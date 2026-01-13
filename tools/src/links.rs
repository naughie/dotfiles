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

#[derive(Clone)]
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

pub async fn get_root() -> Result<PathBuf> {
    let cwd = std::env::current_dir()?;

    let mut dir = &*cwd;

    loop {
        let git = dir.join(".git");
        if matches!(tokio::fs::try_exists(git).await, Ok(true)) {
            return Ok(dir.to_path_buf());
        }

        if let Some(parent) = dir.parent() {
            dir = parent;
        } else {
            return Err(anyhow!("link: could not get root direcotory"));
        }
    }
}

pub struct Ctx {
    pub home: PathBuf,
    pub xdg_config_home: PathBuf,
    pub root: PathBuf,
}

async fn to_resolved_link(name: PathBuf, link_to: PathBuf) -> Result<ResolvedLink> {
    if matches!(tokio::fs::try_exists(&link_to).await, Ok(true)) {
        Ok(ResolvedLink { name, link_to })
    } else {
        Err(anyhow!("link: {}: not found", link_to.display()))
    }
}

impl Ctx {
    pub async fn resolve(&self, configs: &[LinkConfig]) -> Result<Vec<ResolvedLink>> {
        let mut resolved = Vec::new();

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
                    let name = name_root.join(name);
                    let link_to = self.root.join(link_to);

                    resolved.push(to_resolved_link(name, link_to).await?);
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
                    let name_dir = name_root.join(name);
                    let link_to_dir = self.root.join(link_to);

                    let mut read_dir = tokio::fs::read_dir(&link_to_dir).await?;
                    while let Some(entry) = read_dir.next_entry().await? {
                        let fname = entry.file_name();

                        let name = name_dir.join(&fname);
                        let link_to = link_to.join(&fname);

                        resolved.push(to_resolved_link(name, link_to).await?);
                    }
                }
            }
        }

        Ok(resolved)
    }
}

pub async fn configs(root: Option<PathBuf>) -> Result<(Ctx, Vec<LinkConfig>)> {
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
        get_root().await?
    };
    let ctx = Ctx {
        home: get_home()?,
        xdg_config_home: get_xdg_config_home()?,
        root,
    };

    Ok((ctx, links.to_vec()))
}
