use anyhow::{Result, anyhow};
use std::ffi::OsStr;
use std::path::{Path, PathBuf};
use tokio::io::AsyncRead;
use tokio::task::JoinHandle;

fn env(key: &str) -> PathBuf {
    std::env::var_os(key)
        .map(From::from)
        .expect("the env must exist")
}

fn unknown_arch<T>() -> Result<T> {
    Err(anyhow!("unknown arch: {}", std::env::consts::ARCH))
}

fn unknown_os<T>() -> Result<T> {
    Err(anyhow!("unknown OS: {}", std::env::consts::OS))
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InstallStatus {
    NotExists,
    NotExec,
    Exec,
}

impl InstallStatus {
    async fn check_file(path: impl AsRef<Path>) -> Result<Self> {
        use std::os::unix::fs::PermissionsExt as _;

        let path = path.as_ref();

        if !tokio::fs::try_exists(path).await? {
            return Ok(Self::NotExists);
        }

        let metadata = tokio::fs::metadata(path).await?;
        if !metadata.is_file() {
            return Err(anyhow!("{} is not a regular file", path.display()));
        }

        let perm = metadata.permissions();
        let perm = perm.mode();
        if perm & 0o100 == 0 {
            Ok(Self::NotExec)
        } else {
            Ok(Self::Exec)
        }
    }
}

async fn cmd_stdout(exec: impl AsRef<Path>, args: &[impl AsRef<OsStr>]) -> Result<String> {
    use std::process::Stdio;

    let exec = exec.as_ref();
    let mut cmd = tokio::process::Command::new(exec);
    cmd.args(args).kill_on_drop(true).stdin(Stdio::null());
    tracing::info!("run {cmd:?}");

    let output = cmd.output().await?;
    if output.status.success() {
        Ok(std::str::from_utf8(&output.stdout)?.trim().to_owned())
    } else {
        Err(anyhow!(
            "{}: exited with {}: {}",
            exec.display(),
            output.status,
            String::from_utf8_lossy(&output.stderr)
        ))
    }
}

async fn retry_get(url: &str, client: Option<reqwest::Client>) -> Result<reqwest::Response> {
    async fn send(url: &str, client: Option<&reqwest::Client>) -> Result<reqwest::Response> {
        let resp = if let Some(client) = client {
            client.get(url).send().await?
        } else {
            reqwest::get(url).await?
        };
        Ok(resp)
    }

    if let Ok(resp) = send(url, client.as_ref()).await {
        return Ok(resp);
    }
    if let Ok(resp) = send(url, client.as_ref()).await {
        return Ok(resp);
    }
    if let Ok(resp) = send(url, client.as_ref()).await {
        return Ok(resp);
    }
    send(url, client.as_ref()).await
}

async fn github_latest_tag(repo: &str) -> Result<String> {
    use serde::Deserialize;
    #[derive(Deserialize)]
    struct Resp {
        tag_name: String,
    }

    let url = format!("https://api.github.com/repos/{repo}/releases/latest");
    tracing::info!("fetch {url}");
    let client = reqwest::Client::builder()
        .user_agent("my-toolchain")
        .build()?;
    let resp = retry_get(&url, Some(client)).await?;
    let status = resp.status();
    if status.is_success() {
        let resp: Resp = resp.json().await?;
        Ok(resp.tag_name)
    } else {
        Err(anyhow!("{repo}: {status}"))
    }
}

async fn github_latest_dl(repo: &str, fname: &str) -> Result<impl AsyncRead> {
    use futures::StreamExt as _;
    use std::io::Error as IoError;
    use tokio_util::io::StreamReader;

    fn map_err(e: reqwest::Error) -> IoError {
        IoError::other(Box::new(e))
    }

    let url = format!("https://github.com/{repo}/releases/latest/download/{fname}");
    tracing::info!("fetch {url}");
    let client = reqwest::Client::builder()
        .user_agent("my-toolchain")
        .build()?;
    let resp = retry_get(&url, Some(client)).await?;
    let status = resp.status();
    if status.is_success() {
        let stream = resp.bytes_stream();

        let stream = stream.map(|chunk| chunk.map_err(map_err));
        Ok(StreamReader::new(stream))
    } else {
        Err(anyhow!("{repo}: {status}"))
    }
}

type BoxFn<A, B, T> = Box<dyn (FnOnce(A, B) -> JoinHandle<T>) + Send>;
pub(super) struct Installer {
    pub(super) name: &'static str,
    pub(super) depends_on: &'static [&'static str],
    pub(super) install_targets: Vec<PathBuf>,
    pub(super) already_installed: Vec<InstallStatus>,
    pub(super) installed_version: Vec<Option<String>>,
    pub(super) latest_version: Vec<Option<String>>,
    pub(super) spawn_install: BoxFn<Vec<InstallStatus>, Vec<Option<String>>, Result<()>>,
}

impl Installer {
    async fn new<T: ToolInstall + 'static>() -> Result<Self> {
        use anyhow::Error;
        fn map_err<T: ToolInstall>(e: Error) -> Error {
            anyhow!("{}: {e}", T::NAME)
        }

        let install_targets = T::install_targets().into_iter().collect();
        let already_installed = T::already_installed().await.map_err(map_err::<T>)?;

        let installed_version = if already_installed
            .iter()
            .all(|&status| status == InstallStatus::Exec)
        {
            T::get_installed_version().await.map_err(map_err::<T>)?
        } else {
            vec![]
        };

        Ok(Self {
            name: T::NAME,
            depends_on: T::DEPENDS_ON,
            install_targets,
            already_installed,
            installed_version,
            latest_version: T::get_latest_version().await.map_err(map_err::<T>)?,
            spawn_install: Box::new(|status, vers| tokio::spawn(T::install(status, vers))),
        })
    }
}

pub trait ToolInstall {
    const NAME: &str;
    const DEPENDS_ON: &[&str];

    fn install_targets() -> impl IntoIterator<Item = PathBuf>;

    fn already_installed() -> impl Future<Output = Result<Vec<InstallStatus>>>;

    fn get_installed_version() -> impl Future<Output = Result<Vec<Option<String>>>>;

    fn get_latest_version() -> impl Future<Output = Result<Vec<Option<String>>>>;

    fn install(
        status: Vec<InstallStatus>,
        vers: Vec<Option<String>>,
    ) -> impl Future<Output = Result<()>> + Send;
}

mod prelude {
    pub(super) use super::{
        InstallStatus, Result, ToolInstall, anyhow, cmd_stdout, env, github_latest_dl,
        github_latest_tag, retry_get, unknown_arch, unknown_os,
    };
    pub(super) use futures::StreamExt;
    pub(super) use std::path::{Path, PathBuf};
    pub(super) use tokio::{
        fs,
        io::{AsyncWriteExt, BufReader, BufWriter, copy},
    };

    pub(super) async fn create_exec(path: impl AsRef<Path>) -> Result<fs::File> {
        Ok(fs::File::options()
            .write(true)
            .create(true)
            .truncate(true)
            .mode(0o755)
            .open(path)
            .await?)
    }

    pub(super) mod gz {
        pub use async_compression::tokio::bufread::GzipDecoder;
    }
    pub(super) mod xz {
        pub use async_compression::tokio::bufread::XzDecoder;
    }
    pub(super) mod tar {
        pub use tokio_tar::Archive as Tar;
        pub use tokio_tar::EntryType;
    }
    pub(super) mod zip {
        pub use async_zip::base::read::stream::ZipFileReader;
        pub use tokio_util::compat::FuturesAsyncReadCompatExt;
    }
}

macro_rules! _list_tools {
    ($( $tool:ident, )*) => {{
        vec![
            $(
                (
                    <$tool::Tool as ToolInstall>::NAME,
                    tokio::spawn(Installer::new::<$tool::Tool>()),
                ),
            )*
        ]
    }};

    ($to:expr, $in:ident, $( $tool:ident, )*) => {{
        $(
            $to.push((<$in::$tool as ToolInstall>::NAME, tokio::spawn(Installer::new::<$in::$tool>())));
        )*
    }};
}

mod bun;
mod deno;
mod fish;
mod fnm;
mod golang;
mod jaq;
mod neovim;
mod neovim_deps;
mod rust;
mod starship;
mod uv;

pub(super) async fn list_tools() -> Result<Vec<(&'static str, Result<Installer>)>> {
    let mut tasks = _list_tools!(
        bun, deno, fish, fnm, golang, jaq, neovim, rust, starship, uv,
    );

    _list_tools! {
        &mut tasks, neovim_deps,
        TreeSitter,
        Texlab,
        TypeScript,
        NeovimNodeClient,
        Gopls,
        Ruff,
        RustAnalyzer,
    }

    let mut v = Vec::new();

    for (name, task) in tasks {
        v.push((name, task.await?));
    }

    Ok(v)
}
