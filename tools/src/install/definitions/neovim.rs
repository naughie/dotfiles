use super::prelude::*;

pub struct Tool;

fn fname() -> Result<&'static str> {
    if cfg!(target_os = "linux") {
        if cfg!(target_arch = "x86_64") {
            Ok("nvim-linux-x86_64.appimage")
        } else if cfg!(target_arch = "aarch64") {
            Ok("nvim-linux-arm64.appimage")
        } else {
            unknown_arch()
        }
    } else {
        unknown_os()
    }
}

fn bin_path() -> PathBuf {
    env_path("_my_nvim_install", "nvim")
}

impl ToolInstall for Tool {
    const NAME: &str = "nvim";
    const DEPENDS_ON: &[&str] = &[];
    const EXTERNALLY_MANAGED: bool = cfg!(target_os = "macos");

    fn install_targets() -> InstallTargets {
        [bin_path()].into()
    }

    async fn get_installed_version(targets: &[PathBuf]) -> Result<Versions> {
        let output = cmd_stdout(&targets[0], &["--version"]).await?;

        Ok([Some(version(output, "NVIM ")?)].into())
    }

    async fn get_latest_version() -> Result<Versions> {
        let tag = github_latest_tag("neovim/neovim").await?;

        Ok([Some(version(tag, "")?)].into())
    }

    async fn install(_status: InstallStatuses, _vers: Versions) -> Result<()> {
        if Self::EXTERNALLY_MANAGED {
            return Err(anyhow!(
                "neovim: install the prerequisite with `brew install neovim`"
            ));
        }

        let mut resp = github_latest_dl("neovim/neovim", fname()?).await?;

        install_exec(&mut resp, bin_path()).await?;

        tracing::info!("nvim: install done");

        Ok(())
    }
}
