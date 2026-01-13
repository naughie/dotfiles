use super::prelude::*;

pub struct Tool;

fn fname(ver: &str) -> Result<String> {
    if cfg!(target_os = "linux") {
        if cfg!(target_arch = "x86_64") {
            Ok(format!("fish-{ver}-linux-x86_64.tar.xz"))
        } else if cfg!(target_arch = "aarch64") {
            Ok(format!("fish-{ver}-linux-aarch64.tar.xz"))
        } else {
            unknown_arch()
        }
    } else {
        unknown_os()
    }
}

fn bin_path() -> PathBuf {
    env("_FISH_INSTALL").join("fish")
}

impl ToolInstall for Tool {
    const NAME: &str = "fish";
    const DEPENDS_ON: &[&str] = &[];

    fn install_targets() -> impl IntoIterator<Item = PathBuf> {
        [bin_path()]
    }

    async fn already_installed() -> Result<Vec<InstallStatus>> {
        Ok(vec![InstallStatus::check_file(bin_path()).await?])
    }

    async fn get_installed_version() -> Result<Vec<Option<String>>> {
        let ver = cmd_stdout(bin_path(), &["--version"]).await?;
        if let Some((_, ver)) = ver.rsplit_once(' ') {
            Ok(vec![Some(format!("v{ver}")), None])
        } else {
            Err(anyhow!("fish: invalid version format: {ver}"))
        }
    }

    async fn get_latest_version() -> Result<Vec<Option<String>>> {
        let ver = github_latest_tag("fish-shell/fish-shell").await?;
        Ok(vec![Some(ver.to_owned())])
    }

    async fn install(_status: Vec<InstallStatus>, vers: Vec<Option<String>>) -> Result<()> {
        let ver = vers[0].as_ref().expect("vers should be non-empty");
        let url_fname = fname(ver)?;
        let resp = github_latest_dl("fish-shell/fish-shell", &url_fname).await?;
        fs::create_dir_all(env("_FISH_INSTALL")).await?;

        let mut tar = tar::Tar::new(xz::XzDecoder::new(BufReader::new(resp)));
        let mut entries = tar.entries()?;

        while let Some(entry) = entries.next().await {
            let mut entry = entry?;
            let entry_type = entry.header().entry_type();
            if !matches!(entry_type, tar::EntryType::Regular) {
                continue;
            }

            let path = entry.path()?;
            let Some(fname) = path.file_name() else {
                return Err(anyhow!("fish: unknown file path: {}", path.display()));
            };
            if fname != "fish" {
                return Err(anyhow!("fish: unknown file path: {}", path.display()));
            }

            let mut f = BufWriter::new(create_exec(bin_path()).await?);
            copy(&mut entry, &mut f).await?;
            f.flush().await?;

            tracing::info!("fish: install done");
            return Ok(());
        }

        Err(anyhow!(
            "fish: executable `fish` does not exist in the received tar.xz"
        ))
    }
}
