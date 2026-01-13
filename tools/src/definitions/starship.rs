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
    env("_STARSHIP_INSTALL").join("starship")
}

impl ToolInstall for Tool {
    const NAME: &str = "starship";
    const DEPENDS_ON: &[&str] = &[];

    fn install_targets() -> impl IntoIterator<Item = PathBuf> {
        [bin_path()]
    }

    async fn already_installed() -> Result<Vec<InstallStatus>> {
        Ok(vec![InstallStatus::check_file(bin_path()).await?])
    }

    async fn get_installed_version() -> Result<Vec<Option<String>>> {
        let ver = cmd_stdout(bin_path(), &["--version"]).await?;
        if let Some(line) = ver.lines().next()
            && let Some((_, ver)) = line.split_once(' ')
        {
            Ok(vec![Some(format!("v{ver}")), None])
        } else {
            Err(anyhow!("starship: invalid version format: {ver}"))
        }
    }

    async fn get_latest_version() -> Result<Vec<Option<String>>> {
        let ver = github_latest_tag("starship/starship").await?;
        Ok(vec![Some(ver.to_owned())])
    }

    async fn install(_status: Vec<InstallStatus>, _vers: Vec<Option<String>>) -> Result<()> {
        let resp = github_latest_dl("starship/starship", fname()?).await?;
        fs::create_dir_all(env("_STARSHIP_INSTALL")).await?;

        let mut tar = tar::Tar::new(gz::GzipDecoder::new(BufReader::new(resp)));
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

            let mut f = BufWriter::new(create_exec(bin_path()).await?);
            copy(&mut entry, &mut f).await?;
            f.flush().await?;

            tracing::info!("starship: install done");
            return Ok(());
        }

        Err(anyhow!(
            "starship: executable `starship` does not exist in the received tar.gz"
        ))
    }
}
