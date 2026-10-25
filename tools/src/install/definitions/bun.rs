use super::prelude::*;

pub struct Tool;

fn fname() -> Result<&'static str> {
    if cfg!(target_os = "linux") {
        if cfg!(target_arch = "x86_64") {
            Ok("bun-linux-x64.zip")
        } else if cfg!(target_arch = "aarch64") {
            Ok("bun-linux-aarch64.zip")
        } else {
            unknown_arch()
        }
    } else if cfg!(target_os = "macos") {
        if cfg!(target_arch = "x86_64") {
            Ok("bun-darwin-x64.zip")
        } else if cfg!(target_arch = "aarch64") {
            Ok("bun-darwin-aarch64.zip")
        } else {
            unknown_arch()
        }
    } else {
        unknown_os()
    }
}

fn bin_path() -> PathBuf {
    env_path("BUN_INSTALL", "bin/bun")
}

impl ToolInstall for Tool {
    const NAME: &str = "bun";
    const DEPENDS_ON: &[&str] = &[];

    fn install_targets() -> InstallTargets {
        [bin_path()].into()
    }

    async fn get_installed_version(targets: &[PathBuf]) -> Result<Versions> {
        Ok([Some(cmd_stdout(&targets[0], &["--version"]).await?)].into())
    }

    async fn get_latest_version() -> Result<Versions> {
        let tag = github_latest_tag("oven-sh/bun").await?;

        Ok([Some(version(tag, "bun-")?)].into())
    }

    async fn install(_status: InstallStatuses, _vers: Versions) -> Result<()> {
        use zip::FuturesAsyncReadCompatExt as _;

        let resp = github_latest_dl("oven-sh/bun", fname()?).await?;

        let mut zip = zip::ZipFileReader::with_tokio(resp);

        while let Some(mut entry) = zip.next_with_entry().await? {
            {
                let reader = entry.reader_mut();

                if reader.entry().dir()? {
                    zip = entry.skip().await?;
                    continue;
                }

                let path = reader.entry().filename();
                if !path.as_bytes().ends_with(b"/bun") {
                    return Err(anyhow!("bun: unknown file path: {path:02x?}"));
                }

                let mut reader = reader.compat();
                install_exec(&mut reader, bin_path()).await?;

                tracing::info!("bun: install done");
                return Ok(());
            }
        }

        Err(anyhow!(
            "bun: executable `bun` does not exist in the received zip"
        ))
    }
}
