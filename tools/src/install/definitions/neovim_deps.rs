use super::prelude::*;

mod dep {
    pub(super) use super::super::fnm::Tool as Fnm;
    pub(super) use super::super::golang::Tool as Go;
    pub(super) use super::super::rust::Tool as Rust;
    pub(super) use super::super::uv::Tool as Uv;
    pub(super) use super::super::zig::Tool as Zig;
}

use dep::*;

pub struct TreeSitter;

impl ToolInstall for TreeSitter {
    const NAME: &str = "tree-sitter-cli";
    const DEPENDS_ON: &[&str] = &[Rust::NAME];

    fn install_targets() -> InstallTargets {
        [env_path("CARGO_INSTALL_ROOT", "bin/tree-sitter")].into()
    }

    async fn get_installed_version(_targets: &[PathBuf]) -> Result<Versions> {
        Ok([None].into())
    }

    async fn get_latest_version() -> Result<Versions> {
        Ok([None].into())
    }

    async fn install(_status: InstallStatuses, _vers: Versions) -> Result<()> {
        cmd_run(
            env_path("CARGO_HOME", "bin/cargo"),
            &["+stable", "install", "--locked", "tree-sitter-cli"],
        )
        .await?;

        tracing::info!("tree-sitter-cli: install done");
        Ok(())
    }
}

pub struct Texlab;

impl ToolInstall for Texlab {
    const NAME: &str = "texlab";
    const DEPENDS_ON: &[&str] = &[Rust::NAME];

    fn install_targets() -> InstallTargets {
        [env_path("CARGO_INSTALL_ROOT", "bin/texlab")].into()
    }

    async fn get_installed_version(_targets: &[PathBuf]) -> Result<Versions> {
        Ok([None].into())
    }

    async fn get_latest_version() -> Result<Versions> {
        Ok([None].into())
    }

    async fn install(_status: InstallStatuses, _vers: Versions) -> Result<()> {
        let tag = github_latest_tag("latex-lsp/texlab").await?;

        cmd_run(
            env_path("CARGO_HOME", "bin/cargo"),
            &[
                "+stable",
                "install",
                "--locked",
                "--git",
                "https://github.com/latex-lsp/texlab",
                "--tag",
                &tag,
            ],
        )
        .await?;

        tracing::info!("texlab: install done");
        Ok(())
    }
}

pub struct ZigBuild;

impl ToolInstall for ZigBuild {
    const NAME: &str = "cargo-zigbuild";
    const DEPENDS_ON: &[&str] = &[Rust::NAME, Zig::NAME];

    fn install_targets() -> InstallTargets {
        [env_path("CARGO_INSTALL_ROOT", "bin/cargo-zigbuild")].into()
    }

    async fn get_installed_version(_targets: &[PathBuf]) -> Result<Versions> {
        Ok([None].into())
    }

    async fn get_latest_version() -> Result<Versions> {
        Ok([None].into())
    }

    async fn install(_status: InstallStatuses, _vers: Versions) -> Result<()> {
        cmd_run(
            env_path("CARGO_HOME", "bin/cargo"),
            &["+stable", "install", "--locked", "cargo-zigbuild"],
        )
        .await?;

        tracing::info!("cargo-zigbuild: install done");
        Ok(())
    }
}

pub struct TypeScript;

impl ToolInstall for TypeScript {
    const NAME: &str = "typescript";
    const DEPENDS_ON: &[&str] = &[Fnm::NAME];

    fn install_targets() -> InstallTargets {
        [
            env_path("FNM_DIR", "aliases/default/bin/tsc"),
            env_path("FNM_DIR", "aliases/default/bin/typescript-language-server"),
        ]
        .into()
    }

    async fn get_installed_version(_targets: &[PathBuf]) -> Result<Versions> {
        Ok([None, None].into())
    }

    async fn get_latest_version() -> Result<Versions> {
        Ok([None, None].into())
    }

    async fn install(_status: InstallStatuses, _vers: Versions) -> Result<()> {
        run_npm(
            &["install", "-g", "typescript-language-server", "typescript"],
            Self::NAME,
        )
        .await?;

        tracing::info!("typescript: install done");
        Ok(())
    }
}

pub struct NeovimNodeClient;

impl ToolInstall for NeovimNodeClient {
    const NAME: &str = "nvim-node";
    const DEPENDS_ON: &[&str] = &[Fnm::NAME];

    fn install_targets() -> InstallTargets {
        [env_path("FNM_DIR", "aliases/default/bin/neovim-node-host")].into()
    }

    async fn get_installed_version(_targets: &[PathBuf]) -> Result<Versions> {
        Ok([None].into())
    }

    async fn get_latest_version() -> Result<Versions> {
        Ok([None].into())
    }

    async fn install(_status: InstallStatuses, _vers: Versions) -> Result<()> {
        run_npm(&["install", "-g", "neovim"], Self::NAME).await?;

        tracing::info!("nvim-node: install done");
        Ok(())
    }
}

pub struct Gopls;

impl ToolInstall for Gopls {
    const NAME: &str = "gopls";
    const DEPENDS_ON: &[&str] = &[Go::NAME];

    fn install_targets() -> InstallTargets {
        [env_path("GOPATH", "bin/gopls")].into()
    }

    async fn get_installed_version(_targets: &[PathBuf]) -> Result<Versions> {
        Ok([None].into())
    }

    async fn get_latest_version() -> Result<Versions> {
        Ok([None].into())
    }

    async fn install(_status: InstallStatuses, _vers: Versions) -> Result<()> {
        cmd_run(
            env_path("GOROOT", "bin/go"),
            &["install", "golang.org/x/tools/gopls@latest"],
        )
        .await?;

        tracing::info!("gopls: install done");
        Ok(())
    }
}

pub struct Ruff;

impl ToolInstall for Ruff {
    const NAME: &str = "ruff";
    const DEPENDS_ON: &[&str] = &[Uv::NAME];

    fn install_targets() -> InstallTargets {
        [env_path("UV_TOOL_BIN_DIR", "ruff")].into()
    }

    async fn get_installed_version(_targets: &[PathBuf]) -> Result<Versions> {
        Ok([None].into())
    }

    async fn get_latest_version() -> Result<Versions> {
        Ok([None].into())
    }

    async fn install(_status: InstallStatuses, _vers: Versions) -> Result<()> {
        cmd_run(
            env_path("UV_INSTALL_DIR", "uv"),
            &["tool", "install", "--upgrade", "--force", "ruff"],
        )
        .await?;

        tracing::info!("ruff: install done");
        Ok(())
    }
}

pub struct RustAnalyzer;

impl ToolInstall for RustAnalyzer {
    const NAME: &str = "rust-analyzer";
    const DEPENDS_ON: &[&str] = &[];

    fn install_targets() -> InstallTargets {
        [env_path("_my_rust_analyzer_install", "rust-analyzer")].into()
    }

    async fn get_installed_version(_targets: &[PathBuf]) -> Result<Versions> {
        Ok([None].into())
    }

    async fn get_latest_version() -> Result<Versions> {
        Ok([None].into())
    }

    async fn install(_status: InstallStatuses, _vers: Versions) -> Result<()> {
        fn fname() -> Result<&'static str> {
            if cfg!(target_os = "linux") {
                if cfg!(target_arch = "x86_64") {
                    Ok("rust-analyzer-x86_64-unknown-linux-gnu.gz")
                } else if cfg!(target_arch = "aarch64") {
                    Ok("rust-analyzer-aarch64-unknown-linux-gnu.gz")
                } else {
                    unknown_arch()
                }
            } else if cfg!(target_os = "macos") {
                if cfg!(target_arch = "x86_64") {
                    Ok("rust-analyzer-x86_64-apple-darwin.gz")
                } else if cfg!(target_arch = "aarch64") {
                    Ok("rust-analyzer-aarch64-apple-darwin.gz")
                } else {
                    unknown_arch()
                }
            } else {
                unknown_os()
            }
        }

        let stream = github_latest_dl("rust-lang/rust-analyzer", fname()?).await?;

        let mut gz = gz::GzipDecoder::new(stream);
        let output = env_path("_my_rust_analyzer_install", "rust-analyzer");
        install_exec(&mut gz, &output).await?;

        tracing::info!("rust-analyzer: install done");
        Ok(())
    }
}

async fn run_npm(args: &[&str], name: &str) -> Result<()> {
    let fnm_args = ["exec", "--using=default", "npm"]
        .into_iter()
        .chain(args.iter().copied());

    cmd_run(env_path("FNM_DIR", "bin/fnm"), fnm_args)
        .await
        .map_err(|error| anyhow!("{name}: {error}"))?;

    Ok(())
}
