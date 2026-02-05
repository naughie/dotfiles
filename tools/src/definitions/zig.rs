use super::prelude::*;
use super::retry_get;

use serde::Deserialize;
use std::collections::BTreeMap;

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
        Ok(vec![Some(resp.version.to_owned())])
    }

    async fn install(status: Vec<InstallStatus>, _vers: Vec<Option<String>>) -> Result<()> {
        use futures::StreamExt as _;
        use std::io::Error as IoError;
        use tokio_util::io::StreamReader;

        let key = json_key()?;
        let ver = get_index().await?;
        let url = &ver.items[key].tarball;

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
                tokio::fs::remove_dir_all(&dst).await?;
            }
        }

        tar.unpack(&dst).await?;

        tracing::info!("zig: install done");
        Ok(())
    }
}

#[derive(Deserialize)]
struct Version {
    version: String,
    date: String,
    #[serde(rename = "docs")]
    _docs: String,
    #[serde(rename = "stdDocs")]
    _std_docs: String,
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
