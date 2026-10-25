use super::prelude::*;

pub struct Tool;

fn fname() -> Result<&'static str> {
    if cfg!(target_os = "linux") {
        if cfg!(target_arch = "x86_64") {
            Ok("jaq-x86_64-unknown-linux-musl")
        } else if cfg!(target_arch = "aarch64") {
            Ok("jaq-aarch64-unknown-linux-gnu")
        } else {
            unknown_arch()
        }
    } else if cfg!(target_os = "macos") {
        if cfg!(target_arch = "x86_64") {
            Ok("jaq-x86_64-apple-darwin")
        } else if cfg!(target_arch = "aarch64") {
            Ok("jaq-aarch64-apple-darwin")
        } else {
            unknown_arch()
        }
    } else {
        unknown_os()
    }
}

fn bin_path() -> PathBuf {
    env_path("_my_jaq_install", "jq")
}

impl ToolInstall for Tool {
    const NAME: &str = "jaq";
    const DEPENDS_ON: &[&str] = &[];

    fn install_targets() -> InstallTargets {
        [bin_path()].into()
    }

    async fn get_installed_version(targets: &[PathBuf]) -> Result<Versions> {
        let output = cmd_stdout(&targets[0], &["--version"]).await?;

        Ok([Some(version(output, "jaq ")?)].into())
    }

    async fn get_latest_version() -> Result<Versions> {
        let tag = github_latest_tag("01mf02/jaq").await?;

        Ok([Some(version(tag, "")?)].into())
    }

    async fn install(_status: InstallStatuses, _vers: Versions) -> Result<()> {
        let mut resp = github_latest_dl("01mf02/jaq", fname()?).await?;
        install_exec(&mut resp, bin_path()).await?;

        tracing::info!("jaq: install done");

        Ok(())
    }
}
