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
    env("_NVIM_INSTALL").join("nvim")
}

impl ToolInstall for Tool {
    const NAME: &str = "nvim";
    const DEPENDS_ON: &[&str] = &[];

    fn install_targets() -> impl IntoIterator<Item = PathBuf> {
        [bin_path()]
    }

    async fn already_installed() -> Result<Vec<InstallStatus>> {
        Ok(vec![InstallStatus::check_file(bin_path()).await?])
    }

    async fn get_installed_version() -> Result<Vec<Option<String>>> {
        let ver = cmd_stdout(bin_path(), &["--version"]).await?;
        if let Some(ver) = ver.lines().next()
            && let Some(ver) = ver.strip_prefix("NVIM ")
        {
            Ok(vec![Some(ver.to_owned())])
        } else {
            Err(anyhow!("nvim: invalid version format: {ver}"))
        }
    }

    async fn get_latest_version() -> Result<Vec<Option<String>>> {
        let ver = github_latest_tag("neovim/neovim").await?;
        Ok(vec![Some(ver)])
    }

    async fn install(_status: Vec<InstallStatus>, _vers: Vec<Option<String>>) -> Result<()> {
        let mut resp = github_latest_dl("neovim/neovim", fname()?).await?;

        fs::create_dir_all(env("_NVIM_INSTALL")).await?;
        let mut f = BufWriter::new(create_exec(bin_path()).await?);
        copy(&mut resp, &mut f).await?;
        f.flush().await?;

        tracing::info!("nvim: install done");

        Ok(())
    }
}
