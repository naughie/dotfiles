use super::prelude::*;

pub struct Tool;

fn uv_path() -> PathBuf {
    env_path("UV_INSTALL_DIR", "uv")
}

fn uvx_path() -> PathBuf {
    env_path("UV_INSTALL_DIR", "uvx")
}

fn fname() -> Result<&'static str> {
    if cfg!(target_os = "linux") {
        if cfg!(target_arch = "x86_64") {
            Ok("uv-x86_64-unknown-linux-musl.tar.gz")
        } else if cfg!(target_arch = "aarch64") {
            Ok("uv-aarch64-unknown-linux-musl.tar.gz")
        } else {
            unknown_arch()
        }
    } else if cfg!(target_os = "macos") {
        if cfg!(target_arch = "x86_64") {
            Ok("uv-x86_64-apple-darwin.tar.gz")
        } else if cfg!(target_arch = "aarch64") {
            Ok("uv-aarch64-apple-darwin.tar.gz")
        } else {
            unknown_arch()
        }
    } else {
        unknown_os()
    }
}

impl ToolInstall for Tool {
    const NAME: &str = "uv";
    const DEPENDS_ON: &[&str] = &[];

    fn install_targets() -> InstallTargets {
        [uv_path(), uvx_path(), python_path()].into()
    }

    async fn get_installed_version(targets: &[PathBuf]) -> Result<Versions> {
        let uv = cmd_stdout(&targets[0], &["--version"]).await?;
        let uvx = cmd_stdout(&targets[1], &["--version"]).await?;

        Ok([Some(version(uv, "uv ")?), Some(version(uvx, "uvx ")?), None].into())
    }

    async fn get_latest_version() -> Result<Versions> {
        let tag = github_latest_tag("astral-sh/uv").await?;

        Ok([Some(version(tag, "")?), None, None].into())
    }

    async fn install(status: InstallStatuses, vers: Versions) -> Result<()> {
        let targets = [uv_path(), uvx_path()];
        let manager_current = if status[..2]
            .iter()
            .all(|status| *status == InstallStatus::Exec)
        {
            Self::get_installed_version(&targets)
                .await
                .is_ok_and(|installed| installed[0] == vers[0] && installed[1] == vers[0])
        } else {
            false
        };

        if !manager_current {
            install_manager(&targets).await?;
        }

        tracing::info!("uv: install python");
        cmd_run(
            &targets[0],
            &["python", "install", "--default", "--upgrade"],
        )
        .await?;

        tracing::info!("uv: install done");
        Ok(())
    }
}

fn python_path() -> PathBuf {
    env_path("UV_PYTHON_BIN_DIR", "python3")
}

async fn install_manager(targets: &[PathBuf]) -> Result<()> {
    let resp = github_latest_dl("astral-sh/uv", fname()?).await?;

    let mut tar = tar::Tar::new(gz::GzipDecoder::new(resp));
    let mut entries = tar.entries()?;

    let mut uv_created = false;
    let mut uvx_created = false;

    while let Some(entry) = entries.next().await {
        let mut entry = entry?;
        let entry_type = entry.header().entry_type();

        if !matches!(entry_type, tar::EntryType::Regular) {
            continue;
        }

        let path = entry.path()?;

        let Some(fname) = path.file_name() else {
            return Err(anyhow!("uv: unknown file path: {}", path.display()));
        };
        if fname == "uv" {
            if uv_created {
                return Err(anyhow!("uv: executable `uv` found twice"));
            }
            uv_created = true;
        } else if fname == "uvx" {
            if uvx_created {
                return Err(anyhow!("uv: executable `uvx` found twice"));
            }
            uvx_created = true;
        } else {
            return Err(anyhow!("uv: unknown file path: {}", path.display()));
        }

        let target = if fname == "uv" {
            &targets[0]
        } else {
            &targets[1]
        };

        install_exec(&mut entry, target).await?;
    }

    if !uv_created || !uvx_created {
        return Err(anyhow!(
            "uv: executable `uv` or `uvx` do not exist in the received tar.gz"
        ));
    }

    Ok(())
}
