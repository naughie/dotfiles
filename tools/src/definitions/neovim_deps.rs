use super::prelude::*;

mod dep {
    pub(super) use super::super::fnm::Tool as Fnm;
    pub(super) use super::super::golang::Tool as Go;
    pub(super) use super::super::rust::Tool as Rust;
    pub(super) use super::super::uv::Tool as Uv;
}
use dep::*;

pub struct TreeSitter;

impl ToolInstall for TreeSitter {
    const NAME: &str = "tree-sitter-cli";
    const DEPENDS_ON: &[&str] = &[Rust::NAME];

    fn install_targets() -> impl IntoIterator<Item = PathBuf> {
        [env("CARGO_INSTALL_ROOT").join("bin/tree-sitter")]
    }

    async fn already_installed() -> Result<Vec<InstallStatus>> {
        Ok(vec![
            InstallStatus::check_file(env("CARGO_INSTALL_ROOT").join("bin/tree-sitter")).await?,
        ])
    }

    async fn get_installed_version() -> Result<Vec<Option<String>>> {
        Ok(vec![None])
    }

    async fn get_latest_version() -> Result<Vec<Option<String>>> {
        Ok(vec![None])
    }

    async fn install(_status: Vec<InstallStatus>, _vers: Vec<Option<String>>) -> Result<()> {
        cmd_stdout(
            env("CARGO_HOME").join("bin/cargo"),
            &["install", "--locked", "tree-sitter-cli"],
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

    fn install_targets() -> impl IntoIterator<Item = PathBuf> {
        [env("CARGO_INSTALL_ROOT").join("bin/texlab")]
    }

    async fn already_installed() -> Result<Vec<InstallStatus>> {
        Ok(vec![
            InstallStatus::check_file(env("CARGO_INSTALL_ROOT").join("bin/texlab")).await?,
        ])
    }

    async fn get_installed_version() -> Result<Vec<Option<String>>> {
        Ok(vec![None])
    }

    async fn get_latest_version() -> Result<Vec<Option<String>>> {
        Ok(vec![None])
    }

    async fn install(_status: Vec<InstallStatus>, _vers: Vec<Option<String>>) -> Result<()> {
        let tag = github_latest_tag("latex-lsp/texlab").await?;
        cmd_stdout(
            env("CARGO_HOME").join("bin/cargo"),
            &[
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

pub struct TypeScript;

impl ToolInstall for TypeScript {
    const NAME: &str = "typescript";
    const DEPENDS_ON: &[&str] = &[Fnm::NAME];

    fn install_targets() -> impl IntoIterator<Item = PathBuf> {
        [
            env("FNM_DIR").join("aliases/default/bin/tsc"),
            env("FNM_DIR").join("aliases/default/bin/typescript-language-server"),
        ]
    }

    async fn already_installed() -> Result<Vec<InstallStatus>> {
        Ok(vec![
            InstallStatus::check_file(env("FNM_DIR").join("aliases/default/bin/tsc")).await?,
            InstallStatus::check_file(
                env("FNM_DIR").join("aliases/default/bin/typescript-language-server"),
            )
            .await?,
        ])
    }

    async fn get_installed_version() -> Result<Vec<Option<String>>> {
        Ok(vec![None, None])
    }

    async fn get_latest_version() -> Result<Vec<Option<String>>> {
        Ok(vec![None, None])
    }

    async fn install(_status: Vec<InstallStatus>, _vers: Vec<Option<String>>) -> Result<()> {
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

    fn install_targets() -> impl IntoIterator<Item = PathBuf> {
        [env("FNM_DIR").join("aliases/default/bin/neovim-node-host")]
    }

    async fn already_installed() -> Result<Vec<InstallStatus>> {
        Ok(vec![
            InstallStatus::check_file(env("FNM_DIR").join("aliases/default/bin/neovim-node-host"))
                .await?,
        ])
    }

    async fn get_installed_version() -> Result<Vec<Option<String>>> {
        Ok(vec![None])
    }

    async fn get_latest_version() -> Result<Vec<Option<String>>> {
        Ok(vec![None])
    }

    async fn install(_status: Vec<InstallStatus>, _vers: Vec<Option<String>>) -> Result<()> {
        run_npm(&["install", "-g", "neovim"], Self::NAME).await?;

        tracing::info!("nvim-node: install done");
        Ok(())
    }
}

pub struct Gopls;

impl ToolInstall for Gopls {
    const NAME: &str = "gopls";
    const DEPENDS_ON: &[&str] = &[Go::NAME];

    fn install_targets() -> impl IntoIterator<Item = PathBuf> {
        [env("GOPATH").join("bin/gopls")]
    }

    async fn already_installed() -> Result<Vec<InstallStatus>> {
        Ok(vec![
            InstallStatus::check_file(env("GOPATH").join("bin/gopls")).await?,
        ])
    }

    async fn get_installed_version() -> Result<Vec<Option<String>>> {
        Ok(vec![None])
    }

    async fn get_latest_version() -> Result<Vec<Option<String>>> {
        Ok(vec![None])
    }

    async fn install(_status: Vec<InstallStatus>, _vers: Vec<Option<String>>) -> Result<()> {
        cmd_stdout(
            env("GOROOT").join("bin/go"),
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

    fn install_targets() -> impl IntoIterator<Item = PathBuf> {
        [env("UV_TOOL_BIN_DIR").join("ruff")]
    }

    async fn already_installed() -> Result<Vec<InstallStatus>> {
        Ok(vec![
            InstallStatus::check_file(env("UV_TOOL_BIN_DIR").join("ruff")).await?,
        ])
    }

    async fn get_installed_version() -> Result<Vec<Option<String>>> {
        Ok(vec![None])
    }

    async fn get_latest_version() -> Result<Vec<Option<String>>> {
        Ok(vec![None])
    }

    async fn install(_status: Vec<InstallStatus>, _vers: Vec<Option<String>>) -> Result<()> {
        cmd_stdout(
            env("UV_INSTALL_DIR").join("uv"),
            &["tool", "install", "ruff@latest"],
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

    fn install_targets() -> impl IntoIterator<Item = PathBuf> {
        [env("_RUST_ANALYZER_INSTALL").join("rust-analyzer")]
    }

    async fn already_installed() -> Result<Vec<InstallStatus>> {
        Ok(vec![
            InstallStatus::check_file(env("_RUST_ANALYZER_INSTALL").join("rust-analyzer")).await?,
        ])
    }

    async fn get_installed_version() -> Result<Vec<Option<String>>> {
        Ok(vec![None])
    }

    async fn get_latest_version() -> Result<Vec<Option<String>>> {
        Ok(vec![None])
    }

    async fn install(_status: Vec<InstallStatus>, _vers: Vec<Option<String>>) -> Result<()> {
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

        let mut gz = gz::GzipDecoder::new(BufReader::new(stream));

        let output = env("_RUST_ANALYZER_INSTALL").join("rust-analyzer");
        let mut output = BufWriter::new(create_exec(&output).await?);
        copy(&mut gz, &mut output).await?;
        output.shutdown().await?;

        tracing::info!("rust-analyzer: install done");
        Ok(())
    }
}

async fn run_npm(args: &[&str], name: &str) -> Result<()> {
    use anyhow::Error;
    use std::collections::HashMap;
    use std::process::Stdio;
    use tokio::io::AsyncReadExt as _;
    use tokio::process::Command;

    type FnmEnv = HashMap<String, String>;

    let exec = env("FNM_DIR").join("bin/fnm");

    let fnm_env = {
        let mut env_setup = Command::new(&exec)
            .args(["env", "--json"])
            .kill_on_drop(true)
            .stdin(Stdio::null())
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
            .spawn()?;

        let mut stdout = env_setup
            .stdout
            .take()
            .expect("fnm: stdout should be piped");

        let stdout_fut = async move {
            let mut buf = Vec::new();
            let res = stdout.read_to_end(&mut buf).await.map_err(Error::from);
            res.and_then(|_| serde_json::from_slice::<FnmEnv>(&buf).map_err(Error::from))
        };

        let proc_fut = async move { env_setup.wait().await };

        let (out, proc) = tokio::join!(stdout_fut, proc_fut);

        let pref = format_args!("{name}: fnm env --json");
        match (out, proc) {
            (Ok(out), Ok(status)) => {
                if status.success() {
                    Ok(out)
                } else {
                    Err(anyhow!("{pref}: exited with nonzero status: {status}"))
                }
            }
            (Err(out), Ok(status)) => {
                if status.success() {
                    Err(anyhow!(
                        "{pref}: exited with 0, but could not deserialize: {out}"
                    ))
                } else {
                    Err(anyhow!("{pref}: exited with nonzero status: {status}"))
                }
            }
            (_, Err(e)) => Err(anyhow!("{pref}: failed to run: {e}")),
        }
    }?;

    let Some(activated_root) = fnm_env.get("FNM_MULTISHELL_PATH") else {
        return Err(anyhow!(
            "{name}: fnm env --json: could not get $FNM_MULTISHELL_PATH: {fnm_env:?}"
        ));
    };

    let mut npm = Command::new(Path::new(activated_root).join("bin/npm"));
    npm.args(args)
        .kill_on_drop(true)
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null());

    let activated_bin = Path::new(activated_root).join("bin");
    for (key, value) in fnm_env {
        npm.env(key, value);
    }

    if let Some(path) = std::env::var_os("PATH") {
        let mut new_path = activated_bin.into_os_string();
        new_path.push(":");
        new_path.push(&path);
        npm.env("PATH", new_path);
    } else {
        npm.env("PATH", activated_bin);
    }

    tracing::info!("{name}: {npm:?}");
    let mut child = npm.spawn()?;
    let status = child.wait().await?;

    if status.success() {
        Ok(())
    } else {
        let cmd = args.join(" ");
        Err(anyhow!(
            "{name}: npm {cmd}: exited with nonzero status: {status}"
        ))
    }
}
