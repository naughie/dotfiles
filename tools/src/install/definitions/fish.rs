use super::prelude::*;

pub struct Tool;

struct ArchiveName<'a> {
    version: &'a str,
    arch: &'static str,
}

impl std::fmt::Display for ArchiveName<'_> {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            formatter,
            "fish-{}-linux-{}.tar.xz",
            self.version, self.arch
        )
    }
}

fn fname(version: &str) -> Result<ArchiveName<'_>> {
    if cfg!(target_os = "linux") {
        let arch = if cfg!(target_arch = "x86_64") {
            "x86_64"
        } else if cfg!(target_arch = "aarch64") {
            "aarch64"
        } else {
            return unknown_arch();
        };

        Ok(ArchiveName { version, arch })
    } else {
        unknown_os()
    }
}

fn bin_path() -> PathBuf {
    env_path("_my_fish_install", "fish")
}

impl ToolInstall for Tool {
    const NAME: &str = "fish";
    const DEPENDS_ON: &[&str] = &[];
    const EXTERNALLY_MANAGED: bool = cfg!(target_os = "macos");

    fn install_targets() -> InstallTargets {
        [bin_path()].into()
    }

    async fn get_installed_version(targets: &[PathBuf]) -> Result<Versions> {
        let output = cmd_stdout(&targets[0], &["--version"]).await?;

        Ok([Some(version(output, "fish, version ")?)].into())
    }

    async fn get_latest_version() -> Result<Versions> {
        let tag = github_latest_tag("fish-shell/fish-shell").await?;

        Ok([Some(version(tag, "")?)].into())
    }

    async fn install(_status: InstallStatuses, vers: Versions) -> Result<()> {
        if Self::EXTERNALLY_MANAGED {
            return Err(anyhow!(
                "fish: install the prerequisite with `brew install fish`"
            ));
        }

        let ver = vers[0].as_ref().expect("vers should be non-empty");
        let resp = github_latest_dl("fish-shell/fish-shell", fname(ver)?).await?;

        let mut tar = tar::Tar::new(xz::XzDecoder::new(resp));
        let mut entries = tar.entries()?;

        while let Some(entry) = entries.next().await {
            let mut entry = entry?;
            let entry_type = entry.header().entry_type();

            if !matches!(entry_type, tar::EntryType::Regular) {
                continue;
            }

            let path = entry.path()?;

            let Some(fname) = path.file_name() else {
                return Err(anyhow!("fish: unknown file path: {}", path.display()));
            };
            if fname != "fish" {
                continue;
            }

            install_exec(&mut entry, bin_path()).await?;

            tracing::info!("fish: install done");
            return Ok(());
        }

        Err(anyhow!(
            "fish: executable `fish` does not exist in the received tar.xz"
        ))
    }
}
