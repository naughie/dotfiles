use super::prelude::*;

use std::collections::HashMap;

use anyhow::Error;

pub struct Tool;

fn fname() -> Result<&'static str> {
    if cfg!(target_os = "linux") {
        if cfg!(target_arch = "x86_64") {
            Ok("fnm-linux.zip")
        } else if cfg!(target_arch = "aarch64") {
            Ok("fnm-arm64.zip")
        } else {
            unknown_arch()
        }
    } else if cfg!(target_os = "macos") {
        Ok("fnm-macos.zip")
    } else {
        unknown_os()
    }
}

fn bin_path() -> PathBuf {
    env("FNM_DIR").join("bin/fnm")
}

impl ToolInstall for Tool {
    const NAME: &str = "fnm";
    const DEPENDS_ON: &[&str] = &[];

    fn install_targets() -> impl IntoIterator<Item = PathBuf> {
        [bin_path()]
    }

    async fn already_installed() -> Result<Vec<InstallStatus>> {
        Ok(vec![InstallStatus::check_file(bin_path()).await?])
    }

    async fn get_installed_version() -> Result<Vec<Option<String>>> {
        let ver = cmd_stdout(bin_path(), &["--version"]).await?;
        if let Some(ver) = ver.strip_prefix("fnm ") {
            Ok(vec![Some(format!("v{ver}"))])
        } else {
            Err(anyhow!("fnm: invalid version format: {ver}"))
        }
    }

    async fn get_latest_version() -> Result<Vec<Option<String>>> {
        let ver = github_latest_tag("Schniz/fnm").await?;
        Ok(vec![Some(ver.to_owned())])
    }

    async fn install(_status: Vec<InstallStatus>, _vers: Vec<Option<String>>) -> Result<()> {
        use zip::FuturesAsyncReadCompatExt as _;

        let resp = github_latest_dl("Schniz/fnm", fname()?).await?;
        fs::create_dir_all(env("FNM_DIR").join("bin")).await?;

        let mut zip = zip::ZipFileReader::with_tokio(BufReader::new(resp));
        let mut fnm_installed = false;

        while let Some(mut entry) = zip.next_with_entry().await? {
            {
                let reader = entry.reader_mut();
                if reader.entry().dir()? {
                    zip = entry.skip().await?;
                    continue;
                }

                let path = reader.entry().filename();
                if path.as_bytes() != b"fnm" {
                    return Err(anyhow!("fnm: unknown file path: {path:02x?}"));
                }

                let mut f = BufWriter::new(create_exec(bin_path()).await?);
                let mut reader = reader.compat();
                copy(&mut reader, &mut f).await?;
                f.flush().await?;

                fnm_installed = true;
                break;
            }
        }

        if !fnm_installed {
            return Err(anyhow!(
                "fnm: executable `fnm` does not exist in the received zip"
            ));
        }

        let env = fnm_env().await?;
        install_node(&env, &["install", "--lts"]).await?;
        install_node(&env, &["default", "lts-latest"]).await?;
        install_node(&env, &["use", "lts-latest"]).await?;

        tracing::info!("fnm: install done");
        Ok(())
    }
}

async fn install_node(env: &FnmEnv, args: &[&str]) -> Result<()> {
    use std::process::Stdio;
    use tokio::process::Command;

    let exec = bin_path();

    tracing::info!("fnm: {} {args:?}", exec.display());

    let mut fnm = Command::new(&exec);
    fnm.args(args)
        .kill_on_drop(true)
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null());

    for (key, value) in env {
        fnm.env(key, value);
    }

    let mut fnm = fnm.spawn()?;
    let status = fnm.wait().await?;

    if status.success() {
        Ok(())
    } else {
        let cmd = args.join(" ");
        Err(anyhow!(
            "fnm: fnm {cmd}: exited with nonzero status: {status}"
        ))
    }
}

type FnmEnv = HashMap<String, String>;

async fn fnm_env() -> Result<FnmEnv> {
    use std::process::Stdio;
    use tokio::io::AsyncReadExt as _;
    use tokio::process::Command;

    let exec = bin_path();

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

    let pref = "fnm: fnm env --json";
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
}
