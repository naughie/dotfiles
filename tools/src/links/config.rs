use anyhow::{Result, anyhow};

use std::ops::ControlFlow;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Copy)]
pub enum LinkRoot {
    Home,
    XdgConfigHome,
}

#[derive(Debug, Clone, Copy)]
pub enum LinkConfig {
    Single {
        root: LinkRoot,
        name: &'static Path,
        link_to: &'static Path,
    },
    Directory {
        root: LinkRoot,
        name: &'static Path,
        link_to: &'static Path,
    },
}

pub struct ResolvedLink {
    pub name: PathBuf,
    pub link_to: PathBuf,
}

impl ResolvedLink {
    pub async fn make(&self) -> Result<()> {
        if remove_existing(&self.name, &self.link_to).await?.is_break() {
            return Ok(());
        }

        create_link(&self.name, &self.link_to).await
    }
}

async fn create_link(name: &Path, link_to: &Path) -> Result<()> {
    tracing::info!(
        "link: {}: creating symlink to {}",
        name.display(),
        link_to.display()
    );

    if let Some(dir) = name.parent() {
        tokio::fs::create_dir_all(dir).await?;
    }

    tokio::fs::symlink(link_to, name).await?;

    Ok(())
}

async fn remove_existing(name: &Path, link_to: &Path) -> Result<ControlFlow<()>> {
    if let Ok(read_link) = tokio::fs::read_link(name).await {
        if read_link == link_to {
            tracing::info!("link: {}: already linked", name.display());

            return Ok(ControlFlow::Break(()));
        } else {
            tracing::info!(
                "link: {}: exists, but linked to the wrong target (expected {}, but found {})",
                name.display(),
                link_to.display(),
                read_link.display()
            );

            tokio::fs::remove_file(name).await?;
            return Ok(ControlFlow::Continue(()));
        }
    }

    let Ok(metadata) = tokio::fs::metadata(name).await else {
        tracing::info!("link: {}: may not exist", name.display());

        return Ok(ControlFlow::Continue(()));
    };

    if metadata.is_dir() {
        tracing::info!("link: {}: is a direcotory", name.display());
        tokio::fs::remove_dir_all(name).await?;
        Ok(ControlFlow::Continue(()))
    } else if metadata.is_file() {
        tracing::info!("link: {}: is a non-directory file", name.display());
        tokio::fs::remove_file(name).await?;
        Ok(ControlFlow::Continue(()))
    } else {
        Err(anyhow!(
            "link: {}: unknown file type: {:?}",
            name.display(),
            metadata.file_type()
        ))
    }
}

fn get_home() -> Result<PathBuf> {
    std::env::home_dir().ok_or_else(|| anyhow!("link: could not get $HOME"))
}

fn get_xdg_config_home() -> Result<PathBuf> {
    std::env::var_os("XDG_CONFIG_HOME")
        .map(From::from)
        .ok_or_else(|| anyhow!("link: could not get $XDG_CONFIG_HOME"))
}

async fn get_git_root() -> Result<PathBuf> {
    let mut directory = std::env::current_dir()?;

    loop {
        directory.push(".git");
        let exists = tokio::fs::try_exists(&directory).await;
        directory.pop();

        if matches!(exists, Ok(true)) {
            return Ok(directory);
        }

        if !directory.pop() {
            return Err(anyhow!("link: could not get root directory"));
        }
    }
}

pub struct Ctx {
    pub home: PathBuf,
    pub xdg_config_home: PathBuf,
    pub root: PathBuf,
}

fn join_path(root: &Path, suffix: &Path) -> PathBuf {
    let mut path = PathBuf::with_capacity(
        root.as_os_str().as_encoded_bytes().len() + suffix.as_os_str().as_encoded_bytes().len() + 1,
    );
    path.push(root);
    path.push(suffix);

    path
}

async fn to_resolved_link(name: PathBuf, link_to: PathBuf) -> Result<ResolvedLink> {
    use std::ffi::OsString;
    use std::os::unix::ffi::OsStringExt as _;

    if !matches!(tokio::fs::try_exists(&link_to).await, Ok(true)) {
        return Err(anyhow!("link: {}: not found", link_to.display()));
    }

    for (count, ancestor) in name.ancestors().skip(1).enumerate() {
        if ancestor == Path::new("/") {
            break;
        }

        if let Ok(relative) = link_to.strip_prefix(ancestor) {
            let relative_len = relative.as_os_str().as_encoded_bytes().len();
            let prefix_len = count * 3;

            let mut bytes = link_to.into_os_string().into_vec();
            let start = bytes.len() - relative_len;
            let len = prefix_len + relative_len;

            if bytes.len() < len {
                bytes.resize(len, 0);
            }

            bytes.copy_within(start..start + relative_len, prefix_len);
            bytes.truncate(len);
            for parent in bytes[..prefix_len].as_chunks_mut::<3>().0 {
                parent.copy_from_slice(b"../");
            }

            return Ok(ResolvedLink {
                name,
                link_to: PathBuf::from(OsString::from_vec(bytes)),
            });
        }
    }

    Ok(ResolvedLink { name, link_to })
}

impl Ctx {
    pub async fn resolve(
        &self,
        configs: &[LinkConfig],
        mut visit: impl AsyncFnMut(ResolvedLink) -> Result<()>,
    ) -> Result<()> {
        for &config in configs {
            match config {
                LinkConfig::Single {
                    root,
                    name,
                    link_to,
                } => {
                    let name_root = match root {
                        LinkRoot::Home => &self.home,
                        LinkRoot::XdgConfigHome => &self.xdg_config_home,
                    };
                    let name = join_path(name_root, name);
                    let link_to = join_path(&self.root, link_to);

                    visit(to_resolved_link(name, link_to).await?).await?;
                }
                LinkConfig::Directory {
                    root,
                    name,
                    link_to,
                } => {
                    let name_root = match root {
                        LinkRoot::Home => &self.home,
                        LinkRoot::XdgConfigHome => &self.xdg_config_home,
                    };
                    let name_dir = join_path(name_root, name);
                    let link_to_dir = join_path(&self.root, link_to);
                    let mut read_dir = tokio::fs::read_dir(&link_to_dir).await?;

                    while let Some(entry) = read_dir.next_entry().await? {
                        let filename = entry.file_name();
                        let name = join_path(&name_dir, filename.as_ref());
                        let link_to = join_path(&link_to_dir, filename.as_ref());

                        visit(to_resolved_link(name, link_to).await?).await?;
                    }
                }
            }
        }

        Ok(())
    }
}

pub async fn configs(root: Option<PathBuf>) -> Result<(Ctx, [LinkConfig; 6])> {
    let links = [
        LinkConfig::Single {
            root: LinkRoot::XdgConfigHome,
            name: Path::new("nvim"),
            link_to: Path::new("nvim"),
        },
        LinkConfig::Single {
            root: LinkRoot::Home,
            name: Path::new(".profile"),
            link_to: Path::new("shell/profile.sh"),
        },
        LinkConfig::Single {
            root: LinkRoot::XdgConfigHome,
            name: Path::new("fish"),
            link_to: Path::new("shell/fish"),
        },
        LinkConfig::Single {
            root: LinkRoot::XdgConfigHome,
            name: Path::new("terminfo"),
            link_to: Path::new("terminfo"),
        },
        LinkConfig::Directory {
            root: LinkRoot::Home,
            name: Path::new("bin"),
            link_to: Path::new("bin"),
        },
        LinkConfig::Directory {
            root: LinkRoot::Home,
            name: Path::new(""),
            link_to: Path::new("misc"),
        },
    ];

    let root = if let Some(root) = root {
        root
    } else {
        get_git_root().await?
    };

    let ctx = Ctx {
        home: get_home()?,
        xdg_config_home: get_xdg_config_home()?,
        root,
    };

    Ok((ctx, links))
}
