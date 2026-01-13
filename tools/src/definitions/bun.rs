use super::prelude::*;

pub struct Tool;

fn fname() -> Result<&'static str> {
    if cfg!(target_os = "linux") {
        // musl builds do not work
        if cfg!(target_arch = "x86_64") {
            Ok("bun-linux-x64.zip")
        } else if cfg!(target_arch = "aarch64") {
            Ok("bun-linux-aarch64.zip")
        } else {
            unknown_arch()
        }
    } else if cfg!(target_os = "macos") {
        if cfg!(target_arch = "x86_64") {
            Ok("bun-darwin-x64.zip")
        } else if cfg!(target_arch = "aarch64") {
            Ok("bun-darwin-aarch64.zip")
        } else {
            unknown_arch()
        }
    } else {
        unknown_os()
    }
}

fn bin_path() -> PathBuf {
    env("BUN_INSTALL").join("bin/bun")
}

impl ToolInstall for Tool {
    const NAME: &str = "bun";
    const DEPENDS_ON: &[&str] = &[];

    fn install_targets() -> impl IntoIterator<Item = PathBuf> {
        [bin_path()]
    }

    async fn already_installed() -> Result<Vec<InstallStatus>> {
        Ok(vec![InstallStatus::check_file(bin_path()).await?])
    }

    async fn get_installed_version() -> Result<Vec<Option<String>>> {
        let ver = cmd_stdout(bin_path(), &["--version"]).await?;
        Ok(vec![Some(ver.to_owned())])
    }

    async fn get_latest_version() -> Result<Vec<Option<String>>> {
        let ver = github_latest_tag("oven-sh/bun").await?;
        if let Some(ver) = ver.strip_prefix("bun-v") {
            Ok(vec![Some(ver.to_owned())])
        } else {
            Err(anyhow!("bun: invalid version format: {ver}"))
        }
    }

    async fn install(status: Vec<InstallStatus>, _vers: Vec<Option<String>>) -> Result<()> {
        use zip::FuturesAsyncReadCompatExt as _;

        if status[0] == InstallStatus::Exec {
            cmd_stdout(bin_path(), &["upgrade"]).await?;
            tracing::info!("bun: install done");
            return Ok(());
        }

        let resp = github_latest_dl("oven-sh/bun", fname()?).await?;
        fs::create_dir_all(env("BUN_INSTALL").join("bin")).await?;

        let mut zip = zip::ZipFileReader::with_tokio(BufReader::new(resp));

        while let Some(mut entry) = zip.next_with_entry().await? {
            {
                let reader = entry.reader_mut();
                if reader.entry().dir()? {
                    zip = entry.skip().await?;
                    continue;
                }

                let path = reader.entry().filename();
                if !path.as_bytes().ends_with(b"/bun") {
                    return Err(anyhow!("bun: unknown file path: {path:02x?}"));
                }

                let mut f = BufWriter::new(create_exec(bin_path()).await?);
                let mut reader = reader.compat();
                copy(&mut reader, &mut f).await?;
                f.flush().await?;

                tracing::info!("bun: install done");
                return Ok(());
            }
        }

        Err(anyhow!(
            "bun: executable `bun` does not exist in the received zip"
        ))
    }
}
