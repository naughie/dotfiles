use super::prelude::*;

pub struct Tool;

fn bin_path() -> PathBuf {
    env("GOROOT").join("bin/go")
}

fn url_ext() -> Result<&'static str> {
    if cfg!(target_os = "linux") {
        if cfg!(target_arch = "x86_64") {
            Ok("linux-amd64.tar.gz")
        } else {
            unknown_arch()
        }
    } else {
        unknown_os()
    }
}

impl ToolInstall for Tool {
    const NAME: &str = "go";
    const DEPENDS_ON: &[&str] = &[];

    fn install_targets() -> impl IntoIterator<Item = PathBuf> {
        [bin_path()]
    }

    async fn already_installed() -> Result<Vec<InstallStatus>> {
        Ok(vec![InstallStatus::check_file(bin_path()).await?])
    }

    async fn get_installed_version() -> Result<Vec<Option<String>>> {
        let ver = cmd_stdout(bin_path(), &["version"]).await?;
        let Some(ver) = ver.strip_prefix("go version ") else {
            return Err(anyhow!("go: invalid version format: {ver}"));
        };
        if let Some((ver, _)) = ver.split_once(' ') {
            Ok(vec![Some(ver.to_owned())])
        } else {
            Err(anyhow!("go: invalid version format: {ver}"))
        }
    }

    async fn get_latest_version() -> Result<Vec<Option<String>>> {
        let url = "https://go.dev/VERSION?m=text";
        tracing::info!("go: fetch {url}");
        let resp = reqwest::get(url).await?;
        let status = resp.status();
        if status.is_success() {
            let text = resp.text().await?;
            if let Some(line) = text.lines().next() {
                Ok(vec![Some(line.trim().to_owned())])
            } else {
                Err(anyhow!("go: invalid version format: {text}"))
            }
        } else {
            Err(anyhow!("go: could not fetch latest version: {status}"))
        }
    }

    async fn install(status: Vec<InstallStatus>, vers: Vec<Option<String>>) -> Result<()> {
        use futures::StreamExt as _;
        use std::ffi::OsStr;
        use std::fs::Permissions;
        use std::io::Error as IoError;
        use std::os::unix::fs::PermissionsExt as _;
        use tokio_util::io::StreamReader;

        fn map_err(e: reqwest::Error) -> IoError {
            IoError::other(Box::new(e))
        }

        let root = env("GOROOT");
        let Some(root_parent) = root.parent() else {
            return Err(anyhow!(
                "go: GOROOT does not have parent: {}",
                root.display()
            ));
        };

        if status[0] != InstallStatus::NotExists {
            tracing::info!("go: remove GOROOT: {}", root.display());
            remove_dir_all_force(&root).await?;
        }
        fs::create_dir_all(&root).await?;

        let ver = vers[0].as_ref().expect("vers should be non-empty");

        let url = format!("https://go.dev/dl/{ver}.{}", url_ext()?);
        tracing::info!("go: fetch {url}");
        let resp = retry_get(&url, None).await?;
        let status = resp.status();
        if !status.is_success() {
            return Err(anyhow!("go: failed to fetch tar.gz: {status}"));
        }

        let stream = resp.bytes_stream();

        let stream = stream.map(|chunk| chunk.map_err(map_err));
        let stream = StreamReader::new(stream);

        {
            let mut tar = tar::Tar::new(gz::GzipDecoder::new(BufReader::new(stream)));
            let mut entries = tar.entries()?;
            while let Some(entry) = entries.next().await {
                let mut entry = entry?;
                let perm = entry.header().mode()?;
                let perm = Permissions::from_mode(perm);

                let path = entry.path()?;
                let target = root_parent.join(path);

                entry.unpack_in(root_parent).await?;

                tokio::fs::set_permissions(target, perm).await?;
            }
        }

        if root.file_name() != Some(OsStr::new("go")) {
            fs::rename(root_parent.join("go"), &root).await?;
        }

        let mut read_dir = fs::read_dir(root.join("bin")).await?;
        while let Some(entry) = read_dir.next_entry().await? {
            fs::set_permissions(entry.path(), Permissions::from_mode(0o755)).await?;
        }

        tracing::info!("go: install done");
        Ok(())
    }
}

#[allow(clippy::permissions_set_readonly_false)]
async fn remove_dir_all_force(path: &Path) -> Result<()> {
    async fn collect_children(to: &mut Vec<PathBuf>, dir: &Path) -> Result<()> {
        let mut read_dir = fs::read_dir(dir).await?;
        while let Some(entry) = read_dir.next_entry().await? {
            to.push(entry.path());
        }
        Ok(())
    }

    let mut stack = vec![path.to_path_buf()];

    while let Some(path) = stack.pop() {
        let metadata = fs::symlink_metadata(&path).await?;

        let file_type = metadata.file_type();

        if file_type.is_dir() {
            collect_children(&mut stack, &path).await?;
        }

        if !file_type.is_symlink() {
            let mut perm = metadata.permissions();
            perm.set_readonly(false);
            fs::set_permissions(&path, perm).await?;
        }
    }

    tokio::fs::remove_dir_all(path).await?;

    Ok(())
}
