use super::prelude::*;
use super::retry_get;

use serde::Deserialize;
use serde::de::{DeserializeSeed, IgnoredAny, MapAccess, Visitor};

use std::fmt;
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
    env_path("_my_zig_root", "zig")
}

impl ToolInstall for Tool {
    const NAME: &str = "zig";
    const DEPENDS_ON: &[&str] = &[];

    fn install_targets() -> InstallTargets {
        [bin_path()].into()
    }

    async fn get_installed_version(targets: &[PathBuf]) -> Result<Versions> {
        let ver = cmd_stdout(&targets[0], &["version"]).await?;

        Ok([Some(ver)].into())
    }

    async fn get_latest_version() -> Result<Versions> {
        let mut index = get_index().await?;
        let selected = latest_release(&index, json_key()?)?;

        let start = selected.version.as_ptr().addr() - index.as_ptr().addr();
        let end = start + selected.version.len();
        index.truncate(end);
        index.drain(..start);

        Ok([Some(index)].into())
    }

    async fn install(status: InstallStatuses, _vers: Versions) -> Result<()> {
        use futures::StreamExt as _;
        use std::io::Error as IoError;
        use tokio_util::io::StreamReader;

        let key = json_key()?;
        let index = get_index().await?;
        let release = latest_release(&index, key)?;
        let url = release
            .tarball
            .ok_or_else(|| anyhow!("zig: release does not include target {key}"))?;

        let prefix = if let Some((_, fname)) = url.rsplit_once('/')
            && let Some(prefix) = fname.strip_suffix(".tar.xz")
        {
            prefix
        } else {
            return Err(anyhow!("zig: invalid URL format: {url}"));
        };

        tracing::info!("zig: fetch {url}");
        let resp = retry_get(url, None).await?.error_for_status()?;

        let stream = resp.bytes_stream();
        let stream = stream.map(|chunk| chunk.map_err(IoError::other));
        let resp = StreamReader::new(stream);

        let mut dst = env("_my_zig_root");

        let mut tar = tar::Tar::new(xz::XzDecoder::new(resp));
        tracing::info!("zig: unpack into {}", dst.display());

        {
            let status = status[0];
            if status != InstallStatus::NotExists {
                fs::remove_dir_all(&dst).await?;
            }
        }

        fs::create_dir_all(&dst).await?;
        let mut entries = tar.entries()?;

        while let Some(entry) = entries.next().await {
            let mut entry = entry?;

            let path = entry.path()?;

            let normalized = path.strip_prefix(".").unwrap_or(&path);
            let path = if let Ok(path) = normalized.strip_prefix(prefix) {
                path
            } else {
                return Err(anyhow!(
                    "zig: path not started with {prefix:?}: {}",
                    path.display()
                ));
            };

            if path.as_os_str().is_empty() {
                continue;
            }

            let depth = path.components().count();
            let executable = path == Path::new("zig");
            dst.push(path);

            let header = entry.header();
            let entry_type = header.entry_type();

            if entry_type.is_dir() {
                fs::create_dir_all(&dst).await?;
            } else if entry_type.is_file() {
                if let Some(parent) = dst.parent() {
                    fs::create_dir_all(parent).await?;
                }

                let mut f = if executable {
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
                };

                copy(&mut entry, &mut f).await?;
                f.flush().await?;
            } else {
                return Err(anyhow!(
                    "zig: unknown file type {entry_type:?}: {}",
                    path.display()
                ));
            }

            for _ in 0..depth {
                dst.pop();
            }
        }

        tracing::info!("zig: install done");
        Ok(())
    }
}

struct Release<'a> {
    version: &'a str,
    tarball: Option<&'a str>,
}

struct ReleaseSeed(&'static str);

impl<'de> DeserializeSeed<'de> for ReleaseSeed {
    type Value = Option<&'de str>;

    fn deserialize<D: serde::Deserializer<'de>>(
        self,
        deserializer: D,
    ) -> std::result::Result<Self::Value, D::Error> {
        deserializer.deserialize_map(self)
    }
}

impl<'de> Visitor<'de> for ReleaseSeed {
    type Value = Option<&'de str>;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("a Zig release")
    }

    fn visit_map<M: MapAccess<'de>>(
        self,
        mut map: M,
    ) -> std::result::Result<Self::Value, M::Error> {
        #[derive(Deserialize)]
        struct Artifact<'a> {
            tarball: &'a str,
        }

        let mut tarball = None;

        while let Some(key) = map.next_key::<&str>()? {
            if key == self.0 {
                tarball = Some(map.next_value::<Artifact<'de>>()?.tarball);
            } else {
                map.next_value::<IgnoredAny>()?;
            }
        }

        Ok(tarball)
    }
}

struct LatestReleaseSeed(&'static str);

impl<'de> DeserializeSeed<'de> for LatestReleaseSeed {
    type Value = Release<'de>;

    fn deserialize<D: serde::Deserializer<'de>>(
        self,
        deserializer: D,
    ) -> std::result::Result<Self::Value, D::Error> {
        deserializer.deserialize_map(self)
    }
}

impl<'de> Visitor<'de> for LatestReleaseSeed {
    type Value = Release<'de>;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("the Zig release index")
    }

    fn visit_map<M: MapAccess<'de>>(
        self,
        mut map: M,
    ) -> std::result::Result<Self::Value, M::Error> {
        let mut latest = None;
        let mut release = None;

        while let Some(version) = map.next_key::<&str>()? {
            let number = release_number(version);
            if let Some(number) =
                number.filter(|number| latest.is_none_or(|latest| *number > latest))
            {
                latest = Some(number);
                release = Some(Release {
                    version,
                    tarball: map.next_value_seed(ReleaseSeed(self.0))?,
                });
            } else {
                map.next_value::<IgnoredAny>()?;
            }
        }

        release.ok_or_else(|| serde::de::Error::custom("zig: index.json: no stable release found"))
    }
}

fn release_number(version: &str) -> Option<(u32, u32, u32)> {
    let mut parts = version.split('.');
    let major = parts.next()?.parse().ok()?;
    let minor = parts.next()?.parse().ok()?;
    let patch = parts.next()?.parse().ok()?;

    if parts.next().is_some() {
        return None;
    }

    Some((major, minor, patch))
}

fn latest_release<'a>(source: &'a str, target: &'static str) -> Result<Release<'a>> {
    let mut deserializer = serde_json::Deserializer::from_str(source);
    let release = LatestReleaseSeed(target).deserialize(&mut deserializer)?;
    deserializer.end()?;

    Ok(release)
}

async fn get_index() -> Result<String> {
    let url = "https://ziglang.org/download/index.json";
    tracing::info!("zig: fetch {url}");

    let response = retry_get(url, None).await?.error_for_status()?;

    Ok(String::from_utf8(response.bytes().await?.into())?)
}

#[cfg(test)]
mod tests {
    use super::latest_release;

    #[test]
    fn uses_release_number_when_version_field_is_missing() {
        let source = r#"{
            "master": {"version":"0.18.0-dev.1","date":"2026-10-20","docs":""},
            "0.16.1": {"date":"2026-10-21","docs":""},
            "0.17.0": {"date":"2026-10-01","docs":"", "x86_64-linux":{"tarball":"https://ziglang.org/zig.tar.xz"}}
        }"#;

        let release = latest_release(source, "x86_64-linux").unwrap();

        assert_eq!(release.version, "0.17.0");
        assert_eq!(release.tarball, Some("https://ziglang.org/zig.tar.xz"));
    }
}
