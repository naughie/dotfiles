use super::prelude::*;

pub struct Tool;

fn fname() -> Result<&'static str> {
    if cfg!(target_os = "linux") {
        if cfg!(target_arch = "x86_64") {
            Ok("fnm-linux.zip")
        } else if cfg!(target_arch = "aarch64") {
            Ok("fnm-arm64.zip")
        } else {
            unknown_arch()
        }
    } else if cfg!(target_os = "macos") {
        Ok("fnm-macos.zip")
    } else {
        unknown_os()
    }
}

fn bin_path() -> PathBuf {
    env_path("FNM_DIR", "bin/fnm")
}

impl ToolInstall for Tool {
    const NAME: &str = "fnm";
    const DEPENDS_ON: &[&str] = &[];

    fn install_targets() -> InstallTargets {
        [
            bin_path(),
            env_path("FNM_DIR", "aliases/default/bin/node"),
            env_path("FNM_DIR", "aliases/default/bin/npm"),
        ]
        .into()
    }

    async fn get_installed_version(targets: &[PathBuf]) -> Result<Versions> {
        let output = cmd_stdout(&targets[0], &["--version"]).await?;

        Ok([Some(version(output, "fnm ")?), None, None].into())
    }

    async fn get_latest_version() -> Result<Versions> {
        let tag = github_latest_tag("Schniz/fnm").await?;

        Ok([Some(version(tag, "")?), None, None].into())
    }

    async fn install(status: InstallStatuses, vers: Versions) -> Result<()> {
        let executable = bin_path();
        let manager_current = if status[0] == InstallStatus::Exec {
            Self::get_installed_version(std::slice::from_ref(&executable))
                .await
                .is_ok_and(|installed| installed.first() == vers.first())
        } else {
            false
        };

        if !manager_current {
            install_manager(&executable).await?;
        }

        cmd_run(&executable, &["install", "--lts"]).await?;
        cmd_run(&executable, &["default", "lts-latest"]).await?;

        tracing::info!("fnm: install done");
        Ok(())
    }
}

async fn install_manager(executable: &Path) -> Result<()> {
    use zip::FuturesAsyncReadCompatExt as _;

    let resp = github_latest_dl("Schniz/fnm", fname()?).await?;

    let mut zip = zip::ZipFileReader::with_tokio(resp);
    let mut fnm_installed = false;

    while let Some(mut entry) = zip.next_with_entry().await? {
        {
            let reader = entry.reader_mut();

            if reader.entry().dir()? {
                zip = entry.skip().await?;
                continue;
            }

            let path = reader.entry().filename();
            if path.as_bytes() != b"fnm" {
                return Err(anyhow!("fnm: unknown file path: {path:02x?}"));
            }

            let mut reader = reader.compat();
            install_exec(&mut reader, executable).await?;

            fnm_installed = true;
            break;
        }
    }

    if !fnm_installed {
        return Err(anyhow!(
            "fnm: executable `fnm` does not exist in the received zip"
        ));
    }

    Ok(())
}
