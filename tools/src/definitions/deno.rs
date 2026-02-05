use super::prelude::*;

pub struct Tool;

fn deno_fname() -> Result<&'static str> {
    if cfg!(target_os = "linux") {
        if cfg!(target_arch = "x86_64") {
            Ok("deno-x86_64-unknown-linux-gnu.zip")
        } else if cfg!(target_arch = "aarch64") {
            Ok("deno-aarch64-unknown-linux-gnu.zip")
        } else {
            unknown_arch()
        }
    } else if cfg!(target_os = "macos") {
        if cfg!(target_arch = "x86_64") {
            Ok("deno-x86_64-apple-darwin.zip")
        } else if cfg!(target_arch = "aarch64") {
            Ok("deno-aarch64-apple-darwin.zip")
        } else {
            unknown_arch()
        }
    } else {
        unknown_os()
    }
}

fn denort_fname() -> Result<&'static str> {
    if cfg!(target_os = "linux") {
        if cfg!(target_arch = "x86_64") {
            Ok("denort-x86_64-unknown-linux-gnu.zip")
        } else if cfg!(target_arch = "aarch64") {
            Ok("denort-aarch64-unknown-linux-gnu.zip")
        } else {
            unknown_arch()
        }
    } else if cfg!(target_os = "macos") {
        if cfg!(target_arch = "x86_64") {
            Ok("denort-x86_64-apple-darwin.zip")
        } else if cfg!(target_arch = "aarch64") {
            Ok("denort-aarch64-apple-darwin.zip")
        } else {
            unknown_arch()
        }
    } else {
        unknown_os()
    }
}

fn deno_path() -> PathBuf {
    env("DENO_INSTALL_ROOT").join("deno")
}
fn denort_path() -> PathBuf {
    env("DENO_INSTALL_ROOT").join("denort")
}

impl ToolInstall for Tool {
    const NAME: &str = "deno";
    const DEPENDS_ON: &[&str] = &[];

    fn install_targets() -> impl IntoIterator<Item = PathBuf> {
        [deno_path(), denort_path()]
    }

    async fn already_installed() -> Result<Vec<InstallStatus>> {
        Ok(vec![
            InstallStatus::check_file(deno_path()).await?,
            InstallStatus::check_file(denort_path()).await?,
        ])
    }

    async fn get_installed_version() -> Result<Vec<Option<String>>> {
        let ver = cmd_stdout(deno_path(), &["--version"]).await?;
        let Some(ver) = ver.strip_prefix("deno ") else {
            return Err(anyhow!("deno: invalid version format: {ver}"));
        };
        let Some((ver, _)) = ver.split_once(' ') else {
            return Err(anyhow!("deno: invalid version format: deno {ver}"));
        };
        let ver = format!("v{ver}");
        Ok(vec![Some(ver.clone()), Some(ver)])
    }

    async fn get_latest_version() -> Result<Vec<Option<String>>> {
        let ver = github_latest_tag("denoland/deno").await?;
        Ok(vec![Some(ver.to_owned()), Some(ver)])
    }

    async fn install(status: Vec<InstallStatus>, _vers: Vec<Option<String>>) -> Result<()> {
        use zip::FuturesAsyncReadCompatExt as _;

        if status[0] == InstallStatus::Exec {
            cmd_stdout(deno_path(), &["upgrade"]).await?;
            tracing::info!("deno: install done");
            return Ok(());
        }

        let resp = github_latest_dl("denoland/deno", deno_fname()?).await?;
        fs::create_dir_all(env("DENO_INSTALL_ROOT")).await?;

        let mut zip = zip::ZipFileReader::with_tokio(BufReader::new(resp));
        let mut deno_installed = false;

        while let Some(mut entry) = zip.next_with_entry().await? {
            {
                let reader = entry.reader_mut();
                if reader.entry().dir()? {
                    zip = entry.skip().await?;
                    continue;
                }

                let path = reader.entry().filename();
                if path.as_bytes() != b"deno" {
                    return Err(anyhow!("deno: unknown file path: {path:02x?}"));
                }

                let mut f = BufWriter::new(create_exec(deno_path()).await?);
                let mut reader = reader.compat();
                copy(&mut reader, &mut f).await?;
                f.flush().await?;

                deno_installed = true;
                break;
            }
        }
        if !deno_installed {
            return Err(anyhow!(
                "deno: executable `deno` does not exist in the received zip"
            ));
        }

        let resp = github_latest_dl("denoland/deno", denort_fname()?).await?;

        let mut zip = zip::ZipFileReader::with_tokio(BufReader::new(resp));
        let mut denort_installed = false;

        while let Some(mut entry) = zip.next_with_entry().await? {
            {
                let reader = entry.reader_mut();
                if reader.entry().dir()? {
                    zip = entry.skip().await?;
                    continue;
                }

                let path = reader.entry().filename();
                if path.as_bytes() != b"denort" {
                    return Err(anyhow!("denort: unknown file path: {path:02x?}"));
                }

                let mut f = BufWriter::new(create_exec(denort_path()).await?);
                let mut reader = reader.compat();
                copy(&mut reader, &mut f).await?;
                f.flush().await?;

                denort_installed = true;
                break;
            }
        }
        if !denort_installed {
            return Err(anyhow!(
                "deno: executable `denort` does not exist in the received zip"
            ));
        }

        tracing::info!("deno: install done");
        Ok(())
    }
}
