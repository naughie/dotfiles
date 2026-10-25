use super::prelude::*;

pub struct Tool;

fn bin_path() -> PathBuf {
    env_path("CARGO_HOME", "bin/rustup")
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

    fn install_targets() -> InstallTargets {
        [
            bin_path(),
            env_path("CARGO_HOME", "bin/cargo"),
            env_path("CARGO_HOME", "bin/rustc"),
        ]
        .into()
    }

    async fn already_installed(targets: &[PathBuf]) -> Result<InstallStatuses> {
        let mut statuses = [
            InstallStatus::check_file(&targets[0]).await?,
            InstallStatus::NotExists,
            InstallStatus::NotExists,
        ];

        for (index, tool) in ["cargo", "rustc"].into_iter().enumerate() {
            let mut status = InstallStatus::check_file(&targets[index + 1]).await?;

            if status == InstallStatus::Exec
                && cmd_run(&targets[0], &["run", "stable", tool, "--version"])
                    .await
                    .is_err()
            {
                status = InstallStatus::NotExec;
            }

            statuses[index + 1] = status;
        }

        Ok(statuses.into())
    }

    async fn get_installed_version(_targets: &[PathBuf]) -> Result<Versions> {
        Ok([None, None, None].into())
    }

    async fn get_latest_version() -> Result<Versions> {
        Ok([None, None, None].into())
    }

    async fn install(status: InstallStatuses, _vers: Versions) -> Result<()> {
        use futures::StreamExt as _;
        use std::io::Error as IoError;
        use std::process::Stdio;
        use tokio_util::io::StreamReader;

        fn map_err(e: reqwest::Error) -> IoError {
            IoError::other(e)
        }

        let rustup = bin_path();

        if status[0] == InstallStatus::Exec && cmd_run(&rustup, &["--version"]).await.is_ok() {
            cmd_run(&rustup, &["update", "stable"]).await?;
            configure_stable(&rustup).await?;

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
                .stdout(Stdio::null())
                .stderr(command_stderr());

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

            let (copy_res, wait_res) = tokio::join!(copy_fut, wait_command(&mut child));

            match (copy_res, wait_res) {
                (Ok(()), Ok(status)) if status.success() => {
                    configure_stable(&rustup).await?;

                    tracing::info!("rust: install done");
                    Ok(())
                }
                (copy, wait) => Err(anyhow!(
                    "rust: installer failed: copy={copy:?}, process={wait:?}"
                )),
            }
        }
    }
}

async fn configure_stable(rustup: &Path) -> Result<()> {
    cmd_run(rustup, &["default", "stable"]).await?;

    if let Some(target) = rustc_additional_target() {
        cmd_run(rustup, &["target", "add", "--toolchain", "stable", target]).await?;
    }

    Ok(())
}
