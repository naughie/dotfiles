use crate::frontend::{Output, ToolStatus};

use anyhow::{Context, Result, anyhow};

use std::ffi::OsStr;
use std::path::{Path, PathBuf};

use tokio::io::AsyncBufRead;

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
        if perm & 0o111 == 0 {
            Ok(Self::NotExec)
        } else {
            Ok(Self::Exec)
        }
    }
}

async fn cmd_stdout(
    exec: impl AsRef<Path>,
    args: impl IntoIterator<Item = impl AsRef<OsStr>>,
) -> Result<String> {
    use std::process::Stdio;

    let exec = exec.as_ref();
    let mut cmd = tokio::process::Command::new(exec);
    cmd.args(args).kill_on_drop(true).stdin(Stdio::null());
    tracing::info!("run {cmd:?}");

    let output = cmd.output().await?;
    if output.status.success() {
        let mut stdout = String::from_utf8(output.stdout)?;
        let end = stdout.trim_end().len();
        stdout.truncate(end);
        let start = stdout.len() - stdout.trim_start().len();
        stdout.drain(..start);

        Ok(stdout)
    } else {
        Err(anyhow!(
            "{}: exited with {}: {}",
            exec.display(),
            output.status,
            display_bytes(&output.stderr)
        ))
    }
}

async fn cmd_run(
    exec: impl AsRef<Path>,
    args: impl IntoIterator<Item = impl AsRef<OsStr>>,
) -> Result<()> {
    use std::process::Stdio;

    let exec = exec.as_ref();
    let mut command = tokio::process::Command::new(exec);
    command
        .args(args)
        .kill_on_drop(true)
        .stdin(Stdio::null())
        .stdout(Stdio::null());
    tracing::info!("run {command:?}");
    command.stderr(command_stderr());

    let mut child = command.spawn()?;
    let status = wait_command(&mut child).await?;

    if !status.success() {
        return Err(anyhow!("{}: exited with {status}", exec.display()));
    }

    Ok(())
}

fn command_stderr() -> std::process::Stdio {
    if crate::logger::is_tui() {
        std::process::Stdio::piped()
    } else {
        std::process::Stdio::inherit()
    }
}

async fn wait_command(child: &mut tokio::process::Child) -> Result<std::process::ExitStatus> {
    use tokio::io::AsyncReadExt as _;

    if let Some(mut stderr) = child.stderr.take() {
        let logs = async {
            let mut buffer = [0; 4096];

            loop {
                let length = stderr.read(&mut buffer).await?;
                if length == 0 {
                    return Ok::<_, std::io::Error>(());
                }

                tracing::info!(target: "process", "{}", display_bytes(&buffer[..length]));
            }
        };

        let (logs, status) = tokio::join!(logs, child.wait());
        logs?;

        Ok(status?)
    } else {
        Ok(child.wait().await?)
    }
}

pub(super) struct UrlText {
    buffer: [u8; 1024],
    length: usize,
    overflow: Option<String>,
}

impl UrlText {
    pub fn as_str(&self) -> &str {
        match &self.overflow {
            Some(text) => text,
            None => {
                std::str::from_utf8(&self.buffer[..self.length]).expect("formatted URL is UTF-8")
            }
        }
    }
}

impl std::fmt::Display for UrlText {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(self.as_str())
    }
}

pub(super) fn url_text(arguments: std::fmt::Arguments<'_>) -> Result<UrlText> {
    struct Length(usize);

    impl std::fmt::Write for Length {
        fn write_str(&mut self, text: &str) -> std::fmt::Result {
            self.0 += text.len();
            Ok(())
        }
    }

    let mut length = Length(0);
    std::fmt::write(&mut length, arguments)?;

    let mut text = UrlText {
        buffer: [0; 1024],
        length: length.0,
        overflow: None,
    };

    if length.0 > text.buffer.len() {
        let mut overflow = String::with_capacity(length.0);
        std::fmt::write(&mut overflow, arguments)?;
        text.overflow = Some(overflow);
    } else {
        use std::io::Write as _;

        text.buffer.as_mut_slice().write_fmt(arguments)?;
    }

    Ok(text)
}

struct OutputDisplay<'a>(&'a [u8]);

impl std::fmt::Display for OutputDisplay<'_> {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for chunk in self.0.utf8_chunks() {
            formatter.write_str(chunk.valid())?;
            if !chunk.invalid().is_empty() {
                formatter.write_str("\u{fffd}")?;
            }
        }

        Ok(())
    }
}

fn display_bytes(bytes: &[u8]) -> OutputDisplay<'_> {
    OutputDisplay(bytes)
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
    struct Resp<'a> {
        tag_name: &'a str,
    }

    let url = url_text(format_args!(
        "https://api.github.com/repos/{repo}/releases/latest"
    ))?;
    tracing::info!("fetch {url}");

    let client = reqwest::Client::builder()
        .user_agent("my-toolchain")
        .build()?;
    let resp = retry_get(url.as_str(), Some(client)).await?;
    let status = resp.status();

    if status.is_success() {
        let mut source = String::from_utf8(resp.bytes().await?.into())?;
        let resp: Resp<'_> = serde_json::from_str(&source)?;

        let start = resp.tag_name.as_ptr().addr() - source.as_ptr().addr();
        let end = start + resp.tag_name.len();
        source.truncate(end);
        source.drain(..start);

        Ok(source)
    } else {
        Err(anyhow!("{repo}: {status}"))
    }
}

async fn github_latest_dl(
    repo: &str,
    fname: impl std::fmt::Display,
) -> Result<impl AsyncBufRead + Unpin> {
    use futures::StreamExt as _;
    use std::io::Error as IoError;
    use tokio_util::io::StreamReader;

    fn map_err(e: reqwest::Error) -> IoError {
        IoError::other(e)
    }

    let url = url_text(format_args!(
        "https://github.com/{repo}/releases/latest/download/{fname}"
    ))?;
    tracing::info!("fetch {url}");

    let client = reqwest::Client::builder()
        .user_agent("my-toolchain")
        .build()?;
    let resp = retry_get(url.as_str(), Some(client)).await?;
    let status = resp.status();

    if status.is_success() {
        let stream = resp.bytes_stream();

        let stream = stream.map(|chunk| chunk.map_err(map_err));

        Ok(StreamReader::new(stream))
    } else {
        Err(anyhow!("{repo}: {status}"))
    }
}

#[derive(Debug, Default)]
pub enum ToolItems<T> {
    #[default]
    Empty,
    One([T; 1]),
    Two([T; 2]),
    Three([T; 3]),
}

impl<T> std::ops::Deref for ToolItems<T> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        match self {
            Self::Empty => &[],
            Self::One(items) => items,
            Self::Two(items) => items,
            Self::Three(items) => items,
        }
    }
}

impl<T> std::ops::DerefMut for ToolItems<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        match self {
            Self::Empty => &mut [],
            Self::One(items) => items,
            Self::Two(items) => items,
            Self::Three(items) => items,
        }
    }
}

macro_rules! tool_items {
    ($($length:literal => $variant:ident),*) => {
        $(impl<T> From<[T; $length]> for ToolItems<T> {
            fn from(items: [T; $length]) -> Self {
                Self::$variant(items)
            }
        })*
    };
}

tool_items!(1 => One, 2 => Two, 3 => Three);

pub type InstallTargets = ToolItems<PathBuf>;
pub type InstallStatuses = ToolItems<InstallStatus>;
pub type Versions = ToolItems<Option<String>>;

pub(super) struct ToolDefinition {
    pub name: &'static str,
    pub depends_on: &'static [&'static str],
    pub externally_managed: bool,
}

impl ToolDefinition {
    const fn new<T: ToolInstall>() -> Self {
        Self {
            name: T::NAME,
            depends_on: T::DEPENDS_ON,
            externally_managed: T::EXTERNALLY_MANAGED,
        }
    }
}

pub(super) struct Installer {
    pub install_targets: InstallTargets,
    pub already_installed: InstallStatuses,
    pub installed_version: Versions,
    pub latest_version: Versions,
}

impl Installer {
    async fn new<T: ToolInstall>(include_latest: bool) -> Result<Self> {
        let install_targets = T::install_targets();
        let already_installed = T::already_installed(&install_targets)
            .await
            .with_context(|| T::NAME)?;

        let installed_version =
            if include_latest && already_installed.iter().all(|&s| s == InstallStatus::Exec) {
                match T::get_installed_version(&install_targets).await {
                    Ok(versions) => versions,
                    Err(error) if T::EXTERNALLY_MANAGED => return Err(error.context(T::NAME)),
                    Err(error) => {
                        tracing::warn!(
                            "{}: could not determine installed version: {error}",
                            T::NAME
                        );
                        Versions::Empty
                    }
                }
            } else {
                Versions::Empty
            };

        let latest_version = if include_latest && !T::EXTERNALLY_MANAGED {
            T::get_latest_version().await.with_context(|| T::NAME)?
        } else {
            Versions::Empty
        };

        Ok(Self {
            install_targets,
            already_installed,
            installed_version,
            latest_version,
        })
    }

    pub fn is_installed(&self) -> bool {
        self.already_installed
            .iter()
            .all(|&s| s == InstallStatus::Exec)
    }

    pub fn is_latest(&self) -> bool {
        self.is_installed()
            && (0..self.install_targets.len()).all(|i| {
                matches!((self.installed_version.get(i), self.latest_version.get(i)),
                (Some(Some(installed)), Some(Some(latest))) if installed == latest)
            })
    }
}

pub trait ToolInstall {
    const NAME: &str;
    const DEPENDS_ON: &[&str];
    const EXTERNALLY_MANAGED: bool = false;

    fn install_targets() -> InstallTargets;

    fn already_installed(
        targets: &[PathBuf],
    ) -> impl Future<Output = Result<InstallStatuses>> + Send {
        async move {
            let mut statuses: InstallStatuses = match targets.len() {
                1 => [InstallStatus::NotExists].into(),
                2 => [InstallStatus::NotExists; 2].into(),
                3 => [InstallStatus::NotExists; 3].into(),
                _ => return Err(anyhow!("{}: invalid installation targets", Self::NAME)),
            };

            for (target, status) in targets.iter().zip(statuses.iter_mut()) {
                *status = InstallStatus::check_file(target).await?;
            }

            Ok(statuses)
        }
    }

    fn get_installed_version(targets: &[PathBuf]) -> impl Future<Output = Result<Versions>> + Send;

    fn get_latest_version() -> impl Future<Output = Result<Versions>> + Send;

    fn install(status: InstallStatuses, vers: Versions) -> impl Future<Output = Result<()>> + Send;
}

async fn inspect<T: ToolInstall>(include_latest: bool) -> Result<Installer> {
    use futures::FutureExt as _;

    match std::panic::AssertUnwindSafe(Installer::new::<T>(include_latest))
        .catch_unwind()
        .await
    {
        Ok(result) => result,
        Err(_) => Err(anyhow!("{}: installation check panicked", T::NAME)),
    }
}

async fn install<T: ToolInstall>(installer: Option<Installer>) -> Option<Result<()>> {
    use futures::FutureExt as _;

    let installer = installer?;

    let result = std::panic::AssertUnwindSafe(async move {
        T::install(installer.already_installed, installer.latest_version).await?;

        if T::already_installed(&installer.install_targets)
            .await?
            .iter()
            .any(|&s| s != InstallStatus::Exec)
        {
            return Err(anyhow!(
                "{}: installation did not create all executables",
                T::NAME
            ));
        }

        Ok(())
    })
    .catch_unwind()
    .await;

    Some(match result {
        Ok(result) => result,
        Err(_) => Err(anyhow!("{}: installer panicked", T::NAME)),
    })
}

mod prelude {
    pub(super) use super::{
        InstallStatus, InstallStatuses, InstallTargets, Result, ToolInstall, Versions, anyhow,
        cmd_run, cmd_stdout, command_stderr, env, github_latest_dl, github_latest_tag, retry_get,
        unknown_arch, unknown_os, url_text, wait_command,
    };

    pub(super) use futures::StreamExt;

    pub(super) use std::path::{Path, PathBuf};

    pub(super) use tokio::{
        fs,
        io::{AsyncWriteExt, copy},
    };

    pub(super) fn env_path(key: &str, suffix: impl AsRef<Path>) -> PathBuf {
        let mut path = env(key);
        path.push(suffix);

        path
    }

    #[derive(Debug)]
    struct InvalidVersion(String);

    impl std::fmt::Display for InvalidVersion {
        fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(formatter, "invalid version format: {}", self.0)
        }
    }

    impl std::error::Error for InvalidVersion {}

    pub(super) fn version(mut output: String, prefix: &str) -> Result<String> {
        let range = output.strip_prefix(prefix).and_then(|value| {
            let value = value.trim_start();
            let value = value.strip_prefix('v').unwrap_or(value);
            let token = value.split_whitespace().next()?;
            let start = output.len() - value.len();
            Some(start..start + token.len())
        });

        let Some(range) = range else {
            return Err(InvalidVersion(output).into());
        };

        output.truncate(range.end);
        output.drain(..range.start);

        Ok(output)
    }

    pub(super) async fn install_exec(
        reader: &mut (impl tokio::io::AsyncRead + Unpin),
        path: impl AsRef<Path>,
    ) -> Result<()> {
        use std::os::unix::fs::PermissionsExt as _;

        let path = path.as_ref();
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).await?;
        }

        let mut file = fs::File::options()
            .write(true)
            .create(true)
            .truncate(true)
            .mode(0o755)
            .open(path)
            .await?;
        file.set_permissions(std::fs::Permissions::from_mode(0o755))
            .await?;

        copy(reader, &mut file).await?;
        file.shutdown().await?;

        Ok(())
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
mod zig;

pub(crate) const TOOL_COUNT: usize = 19;

macro_rules! tool_registry {
    ($($index:literal => $binding:ident: $tool:ty),* $(,)?) => {
        pub(super) const TOOLS: [ToolDefinition; TOOL_COUNT] = [
            $(ToolDefinition::new::<$tool>(),)*
        ];

        pub(super) async fn inspect_all(needed: &[bool; TOOL_COUNT], skipped: &[bool; TOOL_COUNT], output: Output<'_>) -> [Option<Result<Installer>>; TOOL_COUNT] {
            let ($($binding,)*) = tokio::join!($(
                async {
                    if needed[$index] {
                        if !skipped[$index] {
                            output.tool_status($index, ToolStatus::InProgress);
                        }

                        let result = inspect::<$tool>(!skipped[$index]).await;

                        let status = if skipped[$index] {
                            ToolStatus::Skipped
                        } else {
                            match &result {
                                Err(_) => ToolStatus::Error,
                                Ok(installer) if <$tool as ToolInstall>::EXTERNALLY_MANAGED => {
                                    if installer.is_installed() { ToolStatus::Finished } else { ToolStatus::Error }
                                }
                                Ok(installer) if installer.is_latest() => ToolStatus::Finished,
                                Ok(_) if !<$tool as ToolInstall>::DEPENDS_ON.is_empty() => ToolStatus::WaitingForDependencies,
                                Ok(_) => ToolStatus::InProgress,
                            }
                        };

                        output.tool_status($index, status);

                        Some(result)
                    } else {
                        None
                    }
                },
            )*);

            [$($binding,)*]
        }

        pub(super) async fn install_ready(ready: &mut [Option<Installer>; TOOL_COUNT], output: Output<'_>) -> [Option<Result<()>>; TOOL_COUNT] {
            let [$($binding,)*] = ready;

            let ($($binding,)*) = tokio::join!($(
                async {
                    let installer = $binding.take()?;
                    output.tool_status($index, ToolStatus::InProgress);

                    let result = install::<$tool>(Some(installer)).await;

                    output.tool_status($index, if result.as_ref().is_some_and(Result::is_ok) {
                        ToolStatus::Finished
                    } else {
                        ToolStatus::Error
                    });

                    result
                },
            )*);

            [$($binding,)*]
        }
    };
}

tool_registry! {
    0 => bun: bun::Tool,
    1 => deno: deno::Tool,
    2 => fish: fish::Tool,
    3 => fnm: fnm::Tool,
    4 => golang: golang::Tool,
    5 => jaq: jaq::Tool,
    6 => neovim: neovim::Tool,
    7 => rust: rust::Tool,
    8 => starship: starship::Tool,
    9 => uv: uv::Tool,
    10 => zig: zig::Tool,
    11 => tree_sitter: neovim_deps::TreeSitter,
    12 => texlab: neovim_deps::Texlab,
    13 => typescript: neovim_deps::TypeScript,
    14 => nvim_node: neovim_deps::NeovimNodeClient,
    15 => gopls: neovim_deps::Gopls,
    16 => ruff: neovim_deps::Ruff,
    17 => rust_analyzer: neovim_deps::RustAnalyzer,
    18 => zigbuild: neovim_deps::ZigBuild,
}

#[cfg(test)]
mod tests {
    use super::{cmd_run, prelude::version, url_text};

    struct TraceWriter(async_channel::Sender<Vec<u8>>);

    impl std::io::Write for TraceWriter {
        fn write(&mut self, bytes: &[u8]) -> std::io::Result<usize> {
            self.0.send_blocking(bytes.to_vec()).unwrap();

            Ok(bytes.len())
        }

        fn flush(&mut self) -> std::io::Result<()> {
            Ok(())
        }
    }

    struct TuiMode(bool);

    impl Drop for TuiMode {
        fn drop(&mut self) {
            crate::logger::set_tui(self.0);
        }
    }

    #[tokio::test]
    async fn child_stderr_is_forwarded_to_tracing_in_tui_mode() {
        let _mode = TuiMode(crate::logger::is_tui());
        crate::logger::set_tui(true);

        let (sender, receiver) = async_channel::bounded(1000);
        let subscriber = tracing_subscriber::fmt()
            .without_time()
            .with_ansi(false)
            .with_env_filter(tracing_subscriber::EnvFilter::new("process=info"))
            .with_writer(move || TraceWriter(sender.clone()))
            .finish();
        let _subscriber = tracing::subscriber::set_default(subscriber);

        cmd_run("/bin/sh", ["-c", "printf 'captured child stderr' >&2"])
            .await
            .unwrap();

        receiver.close();
        let mut captured = false;

        while let Ok(chunk) = receiver.try_recv() {
            captured |= std::str::from_utf8(&chunk)
                .unwrap()
                .contains("captured child stderr");
        }

        assert!(captured);
    }

    #[test]
    fn version_normalization_reuses_the_input_buffer() {
        for (value, prefix, expected) in [
            ("fish, version 4.2.0\n", "fish, version ", "4.2.0"),
            ("uv 0.13.5 (abcdef)\n", "uv ", "0.13.5"),
            ("NVIM v0.11.0\nBuild type: Release", "NVIM ", "0.11.0"),
            ("go version go1.26.0 linux/amd64", "go version ", "go1.26.0"),
        ] {
            let input = String::from(value);
            let pointer = input.as_ptr();
            let capacity = input.capacity();

            let output = version(input, prefix).unwrap();

            assert_eq!(output, expected);
            assert_eq!(output.as_ptr(), pointer);
            assert_eq!(output.capacity(), capacity);
        }
    }

    #[test]
    fn release_urls_use_inline_storage() {
        let name = "fnm-macos.zip";
        let output = url_text(format_args!(
            "https://github.com/Schniz/fnm/releases/latest/download/{name}"
        ))
        .unwrap();

        assert!(output.overflow.is_none());
        assert_eq!(
            output.as_str(),
            "https://github.com/Schniz/fnm/releases/latest/download/fnm-macos.zip"
        );
    }
}
