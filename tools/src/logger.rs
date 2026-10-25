use anyhow::Result;

use std::fmt::{self, Display};
use std::io::{Error as IoError, Write as StdWrite};

use tokio::task::JoinHandle;

use async_channel::{Receiver, Sender};

use tracing::Metadata;
use tracing_subscriber::fmt::MakeWriter;

use owo_colors::Color;

static TUI: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

pub(crate) fn is_tui() -> bool {
    TUI.load(std::sync::atomic::Ordering::Acquire)
}

pub(crate) fn set_tui(enabled: bool) {
    TUI.store(enabled, std::sync::atomic::Ordering::Release);
}

struct LogWriter {
    tx: Sender<Vec<u8>>,
}

impl StdWrite for LogWriter {
    fn write(&mut self, buf: &[u8]) -> Result<usize, IoError> {
        self.tx.send_blocking(buf.to_vec()).ok();

        Ok(buf.len())
    }

    fn flush(&mut self) -> Result<(), IoError> {
        Ok(())
    }
}

impl<'a> MakeWriter<'a> for LogWriter {
    type Writer = Self;

    fn make_writer(&'a self) -> Self::Writer {
        Self {
            tx: self.tx.clone(),
        }
    }

    fn make_writer_for(&'a self, _meta: &Metadata<'_>) -> Self::Writer {
        self.make_writer()
    }
}

pub fn init_log() -> Result<Receiver<Vec<u8>>> {
    use tracing_subscriber::{EnvFilter, fmt};

    let (tx, rx) = async_channel::bounded(1000);
    let writer = LogWriter { tx };

    fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")),
        )
        .with_writer(writer)
        .init();

    Ok(rx)
}

pub fn spawn_redirect_logger(logger: &Receiver<Vec<u8>>) -> JoinHandle<Result<()>> {
    let rx = logger.clone();
    tokio::spawn(redirect_logger(rx))
}

pub async fn redirect_logger(rx: Receiver<Vec<u8>>) -> Result<()> {
    use tokio::io::AsyncWriteExt as _;

    while let Ok(chunk) = rx.recv().await {
        let mut stdout = tokio::io::stdout();
        stdout.write_all(&chunk).await?;
        stdout.flush().await?;
    }

    Ok(())
}

pub struct LogHeader<'a, C>(pub &'a str, pub C);

impl<C: Color> Display for LogHeader<'_, C> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        struct Repeat(char, usize);

        impl Display for Repeat {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                for _ in 0..self.1 {
                    write!(f, "{}", self.0)?;
                }
                Ok(())
            }
        }

        use owo_colors::FgColorDisplay;

        let len = self.0.len();
        let header: FgColorDisplay<C, str> = FgColorDisplay::new(self.0);

        let total_len = 55usize;
        let total_margin = total_len - len - 4;
        let left_margin = total_margin / 2;
        let right_margin = total_margin - left_margin;

        writeln!(f, "{}", Repeat('#', total_len))?;
        writeln!(
            f,
            "##{}{}{}##",
            Repeat(' ', left_margin),
            header,
            Repeat(' ', right_margin),
        )?;
        write!(f, "{}", Repeat('#', total_len))?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::{LogWriter, redirect_logger, spawn_redirect_logger};
    use std::io::Write as _;

    #[tokio::test]
    async fn synchronous_writer_buffers_logs_with_output_detached() {
        let (tx, receiver) = async_channel::bounded(1000);
        let mut writer = LogWriter { tx };

        writer.write_all(b"before attaching").unwrap();
        assert_eq!(receiver.try_recv().unwrap(), b"before attaching");

        let output = spawn_redirect_logger(&receiver);
        output.abort();
        assert!(output.await.unwrap_err().is_cancelled());

        writer.write_all(b"after detaching").unwrap();
        assert_eq!(receiver.try_recv().unwrap(), b"after detaching");

        receiver.close();
        redirect_logger(receiver).await.unwrap();
    }
}
