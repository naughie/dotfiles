use super::prelude::*;

pub struct Tool;

fn bin_path() -> PathBuf {
    env("CARGO_HOME").join("bin/rustup")
}

fn rustc_additional_target() -> Option<&'static str> {
    if cfg!(target_os = "linux") {
        if cfg!(target_arch = "x86_64") {
            Some("x86_64-unknown-linux-musl")
        } else if cfg!(target_arch = "aarch64") {
            Some("aarch64-unknown-linux-musl")
        } else {
            None
        }
    } else {
        None
    }
}

impl ToolInstall for Tool {
    const NAME: &str = "rust";
    const DEPENDS_ON: &[&str] = &[];

    fn install_targets() -> impl IntoIterator<Item = PathBuf> {
        [bin_path()]
    }

    async fn already_installed() -> Result<Vec<InstallStatus>> {
        Ok(vec![InstallStatus::check_file(bin_path()).await?])
    }

    async fn get_installed_version() -> Result<Vec<Option<String>>> {
        Ok(vec![None])
    }

    async fn get_latest_version() -> Result<Vec<Option<String>>> {
        Ok(vec![None])
    }

    async fn install(status: Vec<InstallStatus>, _vers: Vec<Option<String>>) -> Result<()> {
        use futures::StreamExt as _;
        use std::io::Error as IoError;
        use std::process::Stdio;
        use tokio_util::io::StreamReader;

        fn map_err(e: reqwest::Error) -> IoError {
            IoError::other(Box::new(e))
        }

        if status[0] == InstallStatus::Exec {
            cmd_stdout(bin_path(), &["update"]).await?;
            tracing::info!("rust: install done");

            Ok(())
        } else {
            let url = "https://sh.rustup.rs";
            tracing::info!("rust: fetch {url}");
            let resp = retry_get(url, None).await?;
            let status = resp.status();
            if !status.is_success() {
                return Err(anyhow!("rust: failed to fetch installer: {status}"));
            }

            let stream = resp.bytes_stream();

            let stream = stream.map(|chunk| chunk.map_err(map_err));
            let mut stream = StreamReader::new(stream);

            let mut cmd = tokio::process::Command::new("sh");
            cmd.args(["-s", "--", "-y", "--no-modify-path"])
                .kill_on_drop(true)
                .stdin(Stdio::piped())
                .stdout(Stdio::piped())
                .stderr(Stdio::piped());
            if let Some(target) = rustc_additional_target() {
                cmd.args(["--target", target]);
            }
            let mut child = cmd.spawn()?;

            let mut stdin = child.stdin.take().expect("rust: stdin should be piped");
            let copy_fut = async move {
                let res = copy(&mut stream, &mut stdin).await;
                stdin.shutdown().await.ok();
                res.map(|_| ())
            };

            let (copy_res, wait_res) = tokio::join!(copy_fut, child.wait_with_output());
            match (copy_res, wait_res) {
                (Ok(_), Ok(output)) => {
                    let status = output.status;
                    if status.success() {
                        cmd_stdout(bin_path(), &["default", "stable"]).await?;

                        tracing::info!("rust: install done");

                        Ok(())
                    } else {
                        Err(anyhow!(
                            "rust: failed to install: {status}: {}",
                            String::from_utf8_lossy(&output.stderr)
                        ))
                    }
                }
                (Err(e), Ok(output)) => {
                    let status = output.status;
                    if status.success() {
                        Err(anyhow!("rust: could not copy the stream: {e}"))
                    } else {
                        Err(anyhow!(
                            "rust: failed to install: sh exited {status}, copy failed with {e}"
                        ))
                    }
                }
                (Ok(_), Err(e)) => Err(anyhow!("rust: copied the stream, but sh failed with {e}")),
                (Err(copy_err), Err(wait_err)) => Err(anyhow!(
                    "rust: failed to install: copying the stream ({copy_err}), sh ({wait_err})"
                )),
            }
        }
    }
}
