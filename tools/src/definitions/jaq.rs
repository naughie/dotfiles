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
    env("_my_jaq_install").join("jq")
}

impl ToolInstall for Tool {
    const NAME: &str = "jaq";
    const DEPENDS_ON: &[&str] = &[];

    fn install_targets() -> impl IntoIterator<Item = PathBuf> {
        [bin_path()]
    }

    async fn already_installed() -> Result<Vec<InstallStatus>> {
        Ok(vec![InstallStatus::check_file(bin_path()).await?])
    }

    async fn get_installed_version() -> Result<Vec<Option<String>>> {
        let ver = cmd_stdout(bin_path(), &["--version"]).await?;
        if let Some(ver) = ver.strip_prefix("jaq ") {
            if ver.starts_with('v') {
                Ok(vec![Some(ver.to_owned())])
            } else {
                Ok(vec![Some(format!("v{ver}"))])
            }
        } else {
            Err(anyhow!("jaq: invalid version format: {ver}"))
        }
    }

    async fn get_latest_version() -> Result<Vec<Option<String>>> {
        let ver = github_latest_tag("01mf02/jaq").await?;
        Ok(vec![Some(ver)])
    }

    async fn install(_status: Vec<InstallStatus>, _vers: Vec<Option<String>>) -> Result<()> {
        let mut resp = github_latest_dl("01mf02/jaq", fname()?).await?;

        fs::create_dir_all(env("_my_jaq_install")).await?;
        let mut f = BufWriter::new(create_exec(bin_path()).await?);
        copy(&mut resp, &mut f).await?;
        f.flush().await?;

        tracing::info!("jaq: install done");

        Ok(())
    }
}
