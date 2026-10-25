use super::prelude::*;

pub struct Tool;

fn bin_path() -> PathBuf {
    env_path("GOROOT", "bin/go")
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
    const EXTERNALLY_MANAGED: bool = cfg!(target_os = "macos");

    fn install_targets() -> InstallTargets {
        [bin_path()].into()
    }

    async fn get_installed_version(targets: &[PathBuf]) -> Result<Versions> {
        let output = cmd_stdout(&targets[0], &["version"]).await?;

        Ok([Some(version(output, "go version ")?)].into())
    }

    async fn get_latest_version() -> Result<Versions> {
        let url = "https://go.dev/VERSION?m=text";
        tracing::info!("go: fetch {url}");

        let response = retry_get(url, None).await?.error_for_status()?;
        let text = String::from_utf8(response.bytes().await?.into())?;

        Ok([Some(version(text, "")?)].into())
    }

    async fn install(_status: InstallStatuses, vers: Versions) -> Result<()> {
        if Self::EXTERNALLY_MANAGED {
            return Err(anyhow!(
                "go: install the prerequisite with `brew install go`"
            ));
        }

        use futures::StreamExt as _;
        use std::fs::Permissions;
        use std::io::Error as IoError;
        use std::os::unix::fs::PermissionsExt as _;
        use tokio_util::io::StreamReader;

        fn map_err(e: reqwest::Error) -> IoError {
            IoError::other(e)
        }

        let mut root = env("GOROOT");
        let ver = vers[0].as_ref().expect("vers should be non-empty");

        let url = url_text(format_args!("https://go.dev/dl/{ver}.{}", url_ext()?))?;
        tracing::info!("go: fetch {url}");

        let resp = retry_get(url.as_str(), None).await?;
        let status = resp.status();
        if !status.is_success() {
            return Err(anyhow!("go: failed to fetch tar.gz: {status}"));
        }

        clear_toolchain(&root, &[env("GOPATH"), env("GOCACHE")]).await?;

        let stream = resp.bytes_stream();
        let stream = stream.map(|chunk| chunk.map_err(map_err));
        let stream = StreamReader::new(stream);

        {
            let mut tar = tar::Tar::new(gz::GzipDecoder::new(stream));
            let mut entries = tar.entries()?;

            while let Some(entry) = entries.next().await {
                let mut entry = entry?;
                let perm = entry.header().mode()?;
                let perm = Permissions::from_mode(perm);

                let path = entry.path()?;
                let path = path.strip_prefix("go")?;
                if path.as_os_str().is_empty() {
                    continue;
                }

                if path
                    .components()
                    .any(|component| !matches!(component, std::path::Component::Normal(_)))
                {
                    return Err(anyhow!("go: invalid archive path: {}", path.display()));
                }

                let depth = path.components().count();
                root.push(path);

                if let Some(parent) = root.parent() {
                    fs::create_dir_all(parent).await?;
                }

                entry.unpack(&root).await?;
                fs::set_permissions(&root, perm).await?;

                for _ in 0..depth {
                    root.pop();
                }
            }
        }

        root.push("bin");
        let mut read_dir = fs::read_dir(&root).await?;

        while let Some(entry) = read_dir.next_entry().await? {
            fs::set_permissions(entry.path(), Permissions::from_mode(0o755)).await?;
        }

        tracing::info!("go: install done");
        Ok(())
    }
}

async fn clear_toolchain(root: &Path, protected: &[PathBuf]) -> Result<()> {
    fs::create_dir_all(root).await?;
    let mut files = fs::read_dir(root).await?;

    while let Some(file) = files.next_entry().await? {
        let path = file.path();
        if protected
            .iter()
            .any(|protected| protected.starts_with(&path))
        {
            continue;
        }

        if file.file_type().await?.is_dir() {
            remove_dir_all_force(&path).await?;
        } else {
            fs::remove_file(&path).await?;
        }
    }

    Ok(())
}

#[allow(clippy::permissions_set_readonly_false)]
async fn remove_dir_all_force(path: &Path) -> Result<()> {
    fn make_writable(path: &Path) -> Result<()> {
        let metadata = std::fs::symlink_metadata(path)?;
        if metadata.is_symlink() {
            return Ok(());
        }

        let mut permissions = metadata.permissions();
        permissions.set_readonly(false);
        std::fs::set_permissions(path, permissions)?;

        if metadata.is_dir() {
            for entry in std::fs::read_dir(path)? {
                make_writable(&entry?.path())?;
            }
        }

        Ok(())
    }

    make_writable(path)?;
    fs::remove_dir_all(path).await?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{clear_toolchain, fs};

    use std::os::unix::fs::PermissionsExt;
    use std::path::PathBuf;
    use std::time::{SystemTime, UNIX_EPOCH};

    struct TestDirectory(PathBuf);

    impl Drop for TestDirectory {
        fn drop(&mut self) {
            std::fs::remove_dir_all(&self.0).ok();
        }
    }

    #[tokio::test]
    async fn removes_old_toolchain_and_preserves_go_workspace_and_cache() {
        let directory = TestDirectory(std::env::temp_dir().join(format!(
            "dotfiles-go-{}-{}",
            std::process::id(),
            SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos()
        )));

        for (layout, workspace, cache) in [
            ("immediate", "vendor", "cache"),
            ("nested", "state/workspace", "state/cache/build"),
        ] {
            let root = directory.0.join(layout);
            let workspace = root.join(workspace);
            let cache = root.join(cache);

            for path in [
                "bin/go",
                "pkg/tool/compile",
                "src/runtime/runtime.go",
                "VERSION",
            ] {
                let path = root.join(path);
                fs::create_dir_all(path.parent().unwrap()).await.unwrap();
                fs::write(&path, b"old toolchain").await.unwrap();
                fs::set_permissions(&path, std::fs::Permissions::from_mode(0o444))
                    .await
                    .unwrap();
            }

            let module = workspace.join("pkg/mod/example.com/library@v1.0.0/go.mod");
            let binary = workspace.join("bin/gopls");
            let cached = cache.join("artifact");

            for (path, contents) in [
                (&module, b"module example.com/library".as_slice()),
                (&binary, b"gopls".as_slice()),
                (&cached, b"cached artifact".as_slice()),
            ] {
                fs::create_dir_all(path.parent().unwrap()).await.unwrap();
                fs::write(path, contents).await.unwrap();
            }

            fs::set_permissions(&module, std::fs::Permissions::from_mode(0o444))
                .await
                .unwrap();
            fs::symlink(&workspace, root.join("pkg/workspace-link"))
                .await
                .unwrap();

            clear_toolchain(&root, &[workspace, cache]).await.unwrap();

            for path in ["bin", "pkg", "src", "VERSION"] {
                assert!(!fs::try_exists(root.join(path)).await.unwrap());
            }
            assert_eq!(
                fs::read(module).await.unwrap(),
                b"module example.com/library"
            );
            assert_eq!(fs::read(binary).await.unwrap(), b"gopls");
            assert_eq!(fs::read(cached).await.unwrap(), b"cached artifact");
        }
    }
}
