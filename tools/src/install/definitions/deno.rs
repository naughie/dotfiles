use super::prelude::*;

pub struct Tool;

fn deno_fname() -> Result<&'static str> {
    if cfg!(target_os = "linux") {
        if cfg!(target_arch = "x86_64") {
            Ok("deno-x86_64-unknown-linux-gnu.zip")
        } else if cfg!(target_arch = "aarch64") {
            Ok("deno-aarch64-unknown-linux-gnu.zip")
        } else {
            unknown_arch()
        }
    } else if cfg!(target_os = "macos") {
        if cfg!(target_arch = "x86_64") {
            Ok("deno-x86_64-apple-darwin.zip")
        } else if cfg!(target_arch = "aarch64") {
            Ok("deno-aarch64-apple-darwin.zip")
        } else {
            unknown_arch()
        }
    } else {
        unknown_os()
    }
}

fn denort_fname() -> Result<&'static str> {
    if cfg!(target_os = "linux") {
        if cfg!(target_arch = "x86_64") {
            Ok("denort-x86_64-unknown-linux-gnu.zip")
        } else if cfg!(target_arch = "aarch64") {
            Ok("denort-aarch64-unknown-linux-gnu.zip")
        } else {
            unknown_arch()
        }
    } else if cfg!(target_os = "macos") {
        if cfg!(target_arch = "x86_64") {
            Ok("denort-x86_64-apple-darwin.zip")
        } else if cfg!(target_arch = "aarch64") {
            Ok("denort-aarch64-apple-darwin.zip")
        } else {
            unknown_arch()
        }
    } else {
        unknown_os()
    }
}

fn deno_path() -> PathBuf {
    env_path("DENO_INSTALL_ROOT", "deno")
}

fn denort_path() -> PathBuf {
    env_path("DENO_INSTALL_ROOT", "denort")
}

impl ToolInstall for Tool {
    const NAME: &str = "deno";
    const DEPENDS_ON: &[&str] = &[];

    fn install_targets() -> InstallTargets {
        [deno_path(), denort_path()].into()
    }

    async fn get_installed_version(targets: &[PathBuf]) -> Result<Versions> {
        let output = cmd_stdout(&targets[0], &["--version"]).await?;

        Ok([Some(version(output, "deno ")?), None].into())
    }

    async fn get_latest_version() -> Result<Versions> {
        let tag = github_latest_tag("denoland/deno").await?;

        Ok([Some(version(tag, "")?), None].into())
    }

    async fn install(_status: InstallStatuses, _vers: Versions) -> Result<()> {
        use zip::FuturesAsyncReadCompatExt as _;

        let resp = github_latest_dl("denoland/deno", deno_fname()?).await?;

        let mut zip = zip::ZipFileReader::with_tokio(resp);
        let mut deno_installed = false;

        while let Some(mut entry) = zip.next_with_entry().await? {
            {
                let reader = entry.reader_mut();

                if reader.entry().dir()? {
                    zip = entry.skip().await?;
                    continue;
                }

                let path = reader.entry().filename();
                if path.as_bytes() != b"deno" {
                    return Err(anyhow!("deno: unknown file path: {path:02x?}"));
                }

                let mut reader = reader.compat();
                install_exec(&mut reader, deno_path()).await?;

                deno_installed = true;
                break;
            }
        }

        if !deno_installed {
            return Err(anyhow!(
                "deno: executable `deno` does not exist in the received zip"
            ));
        }

        let resp = github_latest_dl("denoland/deno", denort_fname()?).await?;

        let mut zip = zip::ZipFileReader::with_tokio(resp);
        let mut denort_installed = false;

        while let Some(mut entry) = zip.next_with_entry().await? {
            {
                let reader = entry.reader_mut();

                if reader.entry().dir()? {
                    zip = entry.skip().await?;
                    continue;
                }

                let path = reader.entry().filename();
                if path.as_bytes() != b"denort" {
                    return Err(anyhow!("denort: unknown file path: {path:02x?}"));
                }

                let mut reader = reader.compat();
                install_exec(&mut reader, denort_path()).await?;

                denort_installed = true;
                break;
            }
        }

        if !denort_installed {
            return Err(anyhow!(
                "deno: executable `denort` does not exist in the received zip"
            ));
        }

        tracing::info!("deno: install done");
        Ok(())
    }
}
