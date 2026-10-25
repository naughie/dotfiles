use super::prelude::*;

pub struct Tool;

fn fname() -> Result<&'static str> {
    if cfg!(target_os = "linux") {
        if cfg!(target_arch = "x86_64") {
            Ok("starship-x86_64-unknown-linux-musl.tar.gz")
        } else if cfg!(target_arch = "aarch64") {
            Ok("starship-aarch64-unknown-linux-musl.tar.gz")
        } else {
            unknown_arch()
        }
    } else if cfg!(target_os = "macos") {
        if cfg!(target_arch = "x86_64") {
            Ok("starship-x86_64-apple-darwin.tar.gz")
        } else if cfg!(target_arch = "aarch64") {
            Ok("starship-aarch64-apple-darwin.tar.gz")
        } else {
            unknown_arch()
        }
    } else {
        unknown_os()
    }
}

fn bin_path() -> PathBuf {
    env_path("_my_starship_install", "starship")
}

impl ToolInstall for Tool {
    const NAME: &str = "starship";
    const DEPENDS_ON: &[&str] = &[];

    fn install_targets() -> InstallTargets {
        [bin_path()].into()
    }

    async fn get_installed_version(targets: &[PathBuf]) -> Result<Versions> {
        let output = cmd_stdout(&targets[0], &["--version"]).await?;

        Ok([Some(version(output, "starship ")?)].into())
    }

    async fn get_latest_version() -> Result<Versions> {
        let tag = github_latest_tag("starship/starship").await?;

        Ok([Some(version(tag, "")?)].into())
    }

    async fn install(_status: InstallStatuses, _vers: Versions) -> Result<()> {
        let resp = github_latest_dl("starship/starship", fname()?).await?;

        let mut tar = tar::Tar::new(gz::GzipDecoder::new(resp));
        let mut entries = tar.entries()?;

        while let Some(entry) = entries.next().await {
            let mut entry = entry?;
            let entry_type = entry.header().entry_type();

            if !matches!(entry_type, tar::EntryType::Regular) {
                continue;
            }

            let path = entry.path()?;

            let Some(fname) = path.file_name() else {
                return Err(anyhow!("starship: unknown file path: {}", path.display()));
            };
            if fname != "starship" {
                return Err(anyhow!("starship: unknown file path: {}", path.display()));
            }

            install_exec(&mut entry, bin_path()).await?;

            tracing::info!("starship: install done");
            return Ok(());
        }

        Err(anyhow!(
            "starship: executable `starship` does not exist in the received tar.gz"
        ))
    }
}
