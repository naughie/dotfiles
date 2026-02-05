use super::prelude::*;
use super::retry_get;

use serde::Deserialize;

use std::collections::BTreeMap;
use std::path::Path;

pub struct Tool;

fn json_key() -> Result<&'static str> {
    if cfg!(target_os = "linux") {
        if cfg!(target_arch = "x86_64") {
            Ok("x86_64-linux")
        } else if cfg!(target_arch = "aarch64") {
            Ok("aarch64-linux")
        } else {
            unknown_arch()
        }
    } else if cfg!(target_os = "macos") {
        if cfg!(target_arch = "x86_64") {
            Ok("x86_64-macos")
        } else if cfg!(target_arch = "aarch64") {
            Ok("aarch64-macos")
        } else {
            unknown_arch()
        }
    } else {
        unknown_os()
    }
}

fn bin_path() -> PathBuf {
    env("_my_zig_root").join("zig")
}

impl ToolInstall for Tool {
    const NAME: &str = "zig";
    const DEPENDS_ON: &[&str] = &[];

    fn install_targets() -> impl IntoIterator<Item = PathBuf> {
        [bin_path()]
    }

    async fn already_installed() -> Result<Vec<InstallStatus>> {
        Ok(vec![InstallStatus::check_file(bin_path()).await?])
    }

    async fn get_installed_version() -> Result<Vec<Option<String>>> {
        let ver = cmd_stdout(bin_path(), &["version"]).await?;
        Ok(vec![Some(ver)])
    }

    async fn get_latest_version() -> Result<Vec<Option<String>>> {
        let resp = get_index().await?;
        Ok(vec![resp.version])
    }

    async fn install(status: Vec<InstallStatus>, _vers: Vec<Option<String>>) -> Result<()> {
        use futures::StreamExt as _;
        use std::io::Error as IoError;
        use tokio_util::io::StreamReader;

        let key = json_key()?;
        let ver = get_index().await?;
        let url = &ver.items[key].tarball;

        let prefix = if let Some((_, fname)) = url.rsplit_once('/')
            && let Some(prefix) = fname.strip_suffix(".tar.xz")
        {
            prefix
        } else {
            return Err(anyhow!("zig: invalid URL format: {url}"));
        };

        tracing::info!("zig: fetch {url}");
        let resp = retry_get(url, None).await?;

        let stream = resp.bytes_stream();

        let stream = stream.map(|chunk| chunk.map_err(|e| IoError::other(Box::new(e))));
        let resp = StreamReader::new(stream);

        let dst = env("_my_zig_root");

        let mut tar = tar::Tar::new(xz::XzDecoder::new(BufReader::new(resp)));
        tracing::info!("zig: unpack into {}", dst.display());

        {
            let status = status[0];
            if status != InstallStatus::NotExists {
                fs::remove_dir_all(&dst).await?;
            }
        }

        let prefix_dot = format!("./{prefix}");

        let mut entries = tar.entries()?;
        while let Some(entry) = entries.next().await {
            let mut entry = entry?;

            let path = entry.path()?;

            let path = if let Ok(path) = path.strip_prefix(prefix) {
                path
            } else if let Ok(path) = path.strip_prefix(&prefix_dot) {
                path
            } else {
                return Err(anyhow!(
                    "zig: path not started with {prefix:?}: {}",
                    path.display()
                ));
            };

            let dst = dst.join(path);

            let header = entry.header();
            let entry_type = header.entry_type();

            if entry_type.is_dir() {
                fs::create_dir_all(&dst).await?;
            } else if entry_type.is_file() {
                if let Some(parent) = dst.parent() {
                    fs::create_dir_all(parent).await?;
                }

                let mut f = BufWriter::new(if path == Path::new("zig") {
                    fs::File::options()
                        .write(true)
                        .create(true)
                        .truncate(true)
                        .mode(0o755)
                        .open(&dst)
                        .await?
                } else {
                    fs::File::options()
                        .write(true)
                        .create(true)
                        .truncate(true)
                        .mode(0o644)
                        .open(&dst)
                        .await?
                });
                copy(&mut entry, &mut f).await?;
                f.flush().await?;
            } else {
                return Err(anyhow!(
                    "zig: unknown file type {entry_type:?}: {}",
                    path.display()
                ));
            }
        }

        tracing::info!("zig: install done");
        Ok(())
    }
}

#[derive(Deserialize)]
struct Version {
    #[serde(rename = "version", default)]
    version: Option<String>,
    date: String,
    #[serde(rename = "docs")]
    _docs: String,
    #[serde(rename = "stdDocs", default)]
    _std_docs: Option<String>,
    #[serde(rename = "notes", default)]
    _notes: Option<String>,
    #[serde(flatten)]
    items: BTreeMap<String, Item>,
}
#[derive(Deserialize)]
struct Item {
    tarball: String,
}

async fn get_index() -> Result<Version> {
    fn date_to_int(ver: &str) -> (i32, u8, u8) {
        let mut it = ver.rsplitn(3, '-');
        if let Some(n) = it.next()
            && let Ok(day) = n.parse()
            && let Some(n) = it.next()
            && let Ok(month) = n.parse()
            && let Some(n) = it.next()
            && let Ok(year) = n.parse()
        {
            (year, month, day)
        } else {
            (i32::MIN, 0, 0)
        }
    }

    let url = "https://ziglang.org/download/index.json";
    tracing::info!("zig: fetch {url}");
    let resp = retry_get(url, None).await?;
    let status = resp.status();
    if status.is_success() {
        let resp: BTreeMap<String, Version> = resp.json().await?;

        if let Some((_, item)) = resp
            .into_iter()
            .filter(|(key, _)| &**key != "master")
            .max_by_key(|(_, ver)| date_to_int(&ver.date))
        {
            Ok(item)
        } else {
            Err(anyhow!("zig: index.json: no item found"))
        }
    } else {
        Err(anyhow!("zig: index.json: {status}"))
    }
}
