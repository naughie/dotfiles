use crate::frontend::Output;
use crate::logger::{LogHeader, spawn_redirect_logger};

use anyhow::{Result, bail};
use async_channel::Receiver;

use std::path::{Path, PathBuf};

use tokio::fs::File;
use tokio::io::{AsyncReadExt as _, AsyncWriteExt as _};

fn repository_root() -> Result<PathBuf> {
    let mut directory = std::env::current_dir()?;
    directory.reserve("shell/generated.profile.sh".len() + 1);

    loop {
        directory.push(".git");
        let exists = directory.try_exists()?;
        directory.pop();

        if exists {
            return Ok(directory);
        }

        if !directory.pop() {
            bail!("profile: could not find the repository root");
        }
    }
}

#[derive(Clone, Copy)]
enum EnvSection {
    Assignments,
    Cleanup,
}

async fn render_line(
    path: &Path,
    index: usize,
    line: &[u8],
    output: &mut File,
    section: EnvSection,
) -> Result<()> {
    let line = std::str::from_utf8(line)?.trim();
    if line.is_empty() || line.starts_with('#') {
        return Ok(());
    }

    let assignment = line.strip_prefix("export ").unwrap_or(line).trim();
    let Some((key, value)) = assignment.split_once('=') else {
        bail!(
            "{}:{index}: expected an environment assignment",
            path.display()
        );
    };

    let key = key.trim();
    if key.is_empty()
        || !key
            .bytes()
            .enumerate()
            .all(|(i, c)| c == b'_' || c.is_ascii_alphabetic() || (i > 0 && c.is_ascii_digit()))
    {
        bail!(
            "{}:{index}: invalid environment variable name",
            path.display()
        );
    }

    match section {
        EnvSection::Cleanup => {
            if key.starts_with('_') && !key.starts_with("__") {
                output.write_all(b"    unset -v ").await?;
                output.write_all(key.as_bytes()).await?;
                output.write_all(b"\n").await?;
            }
        }
        EnvSection::Assignments => {
            output.write_all(b"    ").await?;
            if key.starts_with("__") {
                output.write_all(b"local ").await?;
            } else if !key.starts_with('_') {
                output.write_all(b"export ").await?;
            }

            output.write_all(key.as_bytes()).await?;
            output.write_all(b"=").await?;
            output.write_all(value.trim().as_bytes()).await?;
            output.write_all(b"\n").await?;
        }
    }

    Ok(())
}

async fn render_env(
    path: &Path,
    output: &mut File,
    buffer: &mut [u8; 4096],
    overflow: &mut Vec<u8>,
    section: EnvSection,
) -> Result<()> {
    let mut source = File::open(path).await?;
    let mut used = 0;
    let mut index = 0;

    loop {
        if used == buffer.len() {
            let mut next = [0];
            let count = source.read(&mut next).await?;
            if count == 0 || next[0] == b'\n' {
                index += 1;
                if overflow.is_empty() {
                    render_line(path, index, buffer, output, section).await?;
                } else {
                    overflow.extend_from_slice(buffer);
                    render_line(path, index, overflow, output, section).await?;
                    overflow.clear();
                }
                used = 0;

                if count == 0 {
                    break;
                }
            } else {
                overflow.reserve(buffer.len() + 1);
                overflow.extend_from_slice(buffer);
                overflow.push(next[0]);
                used = 0;
            }
        }

        let count = source.read(&mut buffer[used..]).await?;
        if count == 0 {
            if used != 0 || !overflow.is_empty() {
                index += 1;
                if overflow.is_empty() {
                    render_line(path, index, &buffer[..used], output, section).await?;
                } else {
                    overflow.extend_from_slice(&buffer[..used]);
                    render_line(path, index, overflow, output, section).await?;
                    overflow.clear();
                }
            }
            break;
        }

        let available = used + count;
        let mut start = 0;

        for end in 0..available {
            if buffer[end] == b'\n' {
                index += 1;
                if overflow.is_empty() {
                    render_line(path, index, &buffer[start..end], output, section).await?;
                } else {
                    overflow.extend_from_slice(&buffer[start..end]);
                    render_line(path, index, overflow, output, section).await?;
                    overflow.clear();
                }

                start = end + 1;
            }
        }

        buffer.copy_within(start..available, 0);
        used = available - start;
    }

    Ok(())
}

pub async fn cli_impl(
    logger: Receiver<Vec<u8>>,
    root: Option<PathBuf>,
    env: &Path,
    output: Output<'_>,
) -> Result<()> {
    let logger_task = (!output.is_tui()).then(|| spawn_redirect_logger(&logger));

    let result = generate_profile(root, env, output).await;

    if let Some(task) = logger_task {
        task.abort();
        task.await.ok();
    }

    result
}

async fn generate_profile(root: Option<PathBuf>, env: &Path, display: Output<'_>) -> Result<()> {
    let mut generated = match root {
        Some(root) => root,
        None => repository_root()?,
    };

    display.print(format_args!(
        "\n{}\n\n",
        LogHeader("PROFILE", owo_colors::colors::Green)
    ));
    display.print(format_args!("- repository root: {}\n", generated.display()));

    generated.reserve("shell/generated.profile.sh".len() + 1);
    generated.push("shell/profile.sh");
    if !generated.try_exists()? {
        bail!("profile: could not find {}", generated.display());
    }

    generated.set_file_name("generated.profile.sh");
    display.print(format_args!(
        "- generated profile: {}\n",
        generated.display()
    ));

    let mut path = crate::environment::path_buffer(env);
    let mut output = File::create(generated).await?;
    let mut buffer = [0; 4096];
    let mut overflow = Vec::new();

    output.write_all(b"__my_setup_generated_envs() {\n").await?;

    for section in [EnvSection::Assignments, EnvSection::Cleanup] {
        if matches!(section, EnvSection::Cleanup) {
            output
                .write_all(b"}\n_my_clear_generated_envs() {\n")
                .await?;
        }

        for name in crate::environment::file_names() {
            path.push(name);
            if *name != "env.macos" || path.try_exists()? {
                render_env(&path, &mut output, &mut buffer, &mut overflow, section).await?;
            }

            path.pop();
        }
    }

    output.write_all(b"    unset -f _my_clear_generated_envs\n}\n__my_setup_generated_envs\nunset -f __my_setup_generated_envs\n").await?;
    output.shutdown().await?;

    Ok(())
}
