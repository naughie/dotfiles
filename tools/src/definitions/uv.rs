use super::prelude::*;

pub struct Tool;

fn uv_path() -> PathBuf {
    env("UV_INSTALL_DIR").join("uv")
}
fn uvx_path() -> PathBuf {
    env("UV_INSTALL_DIR").join("uvx")
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

    fn install_targets() -> impl IntoIterator<Item = PathBuf> {
        [uv_path(), uvx_path()]
    }

    async fn already_installed() -> Result<Vec<InstallStatus>> {
        Ok(vec![
            InstallStatus::check_file(uv_path()).await?,
            InstallStatus::check_file(uvx_path()).await?,
        ])
    }

    async fn get_installed_version() -> Result<Vec<Option<String>>> {
        let ver = cmd_stdout(uv_path(), &["--version"]).await?;
        let Some(uv_ver) = ver.strip_prefix("uv ") else {
            return Err(anyhow!("uv: invalid version format: {ver}"));
        };

        let ver = cmd_stdout(uvx_path(), &["--version"]).await?;
        let Some(uvx_ver) = ver.strip_prefix("uvx ") else {
            return Err(anyhow!("uvx: invalid version format: {ver}"));
        };

        Ok(vec![Some(uv_ver.to_owned()), Some(uvx_ver.to_owned())])
    }

    async fn get_latest_version() -> Result<Vec<Option<String>>> {
        let ver = github_latest_tag("astral-sh/uv").await?;
        Ok(vec![Some(ver.clone()), Some(ver.clone())])
    }

    async fn install(_status: Vec<InstallStatus>, _vers: Vec<Option<String>>) -> Result<()> {
        let resp = github_latest_dl("astral-sh/uv", fname()?).await?;
        let mut tar = tar::Tar::new(gz::GzipDecoder::new(BufReader::new(resp)));
        let mut entries = tar.entries()?;

        fs::create_dir_all(env("UV_INSTALL_DIR")).await?;

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

            let path = env("UV_INSTALL_DIR").join(fname);
            let mut f = BufWriter::new(create_exec(path).await?);
            copy(&mut entry, &mut f).await?;
            f.flush().await?;
        }

        if !uv_created || !uvx_created {
            return Err(anyhow!(
                "uv: executable `uv` or `uvx` do not exist in the received tar.gz"
            ));
        }

        tracing::info!("uv: install python");
        cmd_stdout(uv_path(), &["python", "install", "--default"]).await?;

        tracing::info!("uv: install done");
        Ok(())
    }
}
