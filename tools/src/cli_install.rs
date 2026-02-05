use super::definitions;
use super::links;
use definitions::InstallStatus;

use anyhow::{Result, anyhow};

use std::collections::BTreeMap;
use std::fmt::{self, Display};
use std::io::{Error as IoError, Write as StdWrite};
use std::path::{Path, PathBuf};

use tokio::sync::broadcast;
use tokio::task::JoinHandle;

use async_channel::{Receiver, Sender};

use tracing::Metadata;
use tracing_subscriber::fmt::MakeWriter;

use owo_colors::Color;

struct LogWriter {
    tx: Sender<Vec<u8>>,
}

impl StdWrite for LogWriter {
    fn write(&mut self, buf: &[u8]) -> Result<usize, IoError> {
        self.tx.send_blocking(buf.to_vec()).ok();
        Ok(buf.len())
    }

    fn flush(&mut self) -> Result<(), IoError> {
        Ok(())
    }
}

impl<'a> MakeWriter<'a> for LogWriter {
    type Writer = Self;

    fn make_writer(&'a self) -> Self::Writer {
        Self {
            tx: self.tx.clone(),
        }
    }

    fn make_writer_for(&'a self, _meta: &Metadata<'_>) -> Self::Writer {
        self.make_writer()
    }
}

fn init_log() -> Result<Receiver<Vec<u8>>> {
    use tracing_subscriber::{EnvFilter, fmt};

    let (tx, rx) = async_channel::bounded(1000);
    let writer = LogWriter { tx };

    fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")),
        )
        .with_writer(writer)
        .init();

    Ok(rx)
}

fn spawn_redirect_logger(logger: &Receiver<Vec<u8>>) -> JoinHandle<Result<()>> {
    let rx = logger.clone();
    tokio::spawn(redirect_logger(rx))
}

async fn redirect_logger(rx: Receiver<Vec<u8>>) -> Result<()> {
    use tokio::io::AsyncWriteExt as _;

    while let Ok(chunk) = rx.recv().await {
        let mut stdout = tokio::io::stdout();
        stdout.write_all(&chunk).await?;
        stdout.flush().await?;
    }

    Ok(())
}

async fn install_tools_impl(logger: Receiver<Vec<u8>>) -> Result<()> {
    use owo_colors::OwoColorize as _;
    use owo_colors::colors::{Green, Red};

    println!();
    println!();
    println!();
    println!("### {} ...", "Checking the environment".green());
    println!();
    println!();

    let logger_handle = spawn_redirect_logger(&logger);

    let tools = definitions::list_tools().await?;
    logger_handle.abort();

    let mut ok_tools = Vec::new();
    let mut err_tools = BTreeMap::new();

    for (name, tool) in tools {
        match tool {
            Ok(tool) => {
                ok_tools.push(tool);
            }
            Err(e) => {
                err_tools.insert(name, e);
            }
        }
    }

    if !err_tools.is_empty() {
        println!();
        println!();
        println!("{}", LogHeader("ERROR", Red));
        println!();
        println!("{}", "Could not install the following tools:".red());
        println!();

        for (name, err) in &err_tools {
            println!("- {name}: {err}");
        }

        let rejected = ok_tools.extract_if(.., |tool| {
            tool.depends_on
                .iter()
                .any(|&dep| err_tools.contains_key(dep))
        });
        for tool in rejected {
            let rejected_deps = tool
                .depends_on
                .iter()
                .filter(|&&dep| err_tools.contains_key(dep))
                .copied()
                .collect::<Vec<_>>()
                .join(", ");

            println!("- {}: erroneous dependencies: {rejected_deps}", tool.name);
        }
    }

    println!();
    println!();
    println!("{}", LogHeader("INSTALL INFO", Green));
    println!();

    let mut notifiers = BTreeMap::new();
    for (i, tool) in ok_tools.iter().enumerate() {
        println!("{}{}{}", i.green(), ": ".green(), tool.name.green());

        assert_eq!(tool.install_targets.len(), tool.already_installed.len());

        for (t, target) in tool.install_targets.iter().enumerate() {
            let pref = format_args!("    target[{t}] = {}", target.display());

            let ver = if let Some(Some(ver)) = tool.installed_version.get(t) {
                ver
            } else {
                "<unknown version>"
            };

            let status = &tool.already_installed[t];
            match status {
                InstallStatus::NotExists => println!("{pref} {ver} [{}]", "Not installed".yellow()),
                InstallStatus::NotExec => {
                    println!("{pref} {ver} [{}]", "Invalid file exists".red())
                }
                InstallStatus::Exec => println!("{pref} {ver} [{}]", "Installed".green()),
            }

            if let Some(Some(ver)) = tool.latest_version.get(t) {
                println!("                  latest version = {ver}");
            } else {
                println!("                  latest version = <unknown version>");
            }
        }
        println!();

        let (tx, _rx) = broadcast::channel::<Option<()>>(1);
        notifiers.insert(tool.name, tx);
    }
    let notifiers = notifiers;

    let mut subscribers = BTreeMap::new();
    for tool in &ok_tools {
        let mut subscriptions = Vec::new();
        for &dep in tool.depends_on {
            if let Some(tx) = notifiers.get(dep) {
                let rx = tx.subscribe();
                subscriptions.push((dep, rx));
            } else {
                tracing::warn!("{}: unexpectedly could not subscribe to {dep}", tool.name);
            }
        }
        subscribers.insert(tool.name, subscriptions);
    }

    let mut already_latest = ok_tools
        .extract_if(.., |tool| {
            (0..(tool.install_targets.len())).all(|t| {
                let installed = tool.installed_version.get(t);
                let latest = tool.latest_version.get(t);

                if let Some(Some(installed)) = installed
                    && let Some(Some(latest)) = latest
                {
                    installed == latest
                } else {
                    false
                }
            })
        })
        .peekable();

    if already_latest.peek().is_some() {
        println!();
        println!();
        println!("{}", LogHeader("VERSION CHECK", Green));
        println!();

        println!("{}", "The following tools are up-to-date:".green());
        println!();
    }

    for tool in already_latest {
        print!("- {} (", tool.name);

        for ver in &tool.installed_version {
            assert!(
                ver.is_some(),
                "unexpectedly get None version: {:?}",
                &tool.installed_version
            );
            print!(" {} ", ver.as_ref().unwrap());
        }

        println!(")");

        let tx = &notifiers[tool.name];
        tx.send(Some(())).ok();
    }

    println!();
    println!();
    println!("{}", LogHeader("INSTALL", Green));
    println!();

    if ok_tools.is_empty() {
        println!("No tools to be installed");
        println!();
    } else {
        println!("Start installing ...");
        println!();
    }

    let logger_handle = spawn_redirect_logger(&logger);

    let mut tasks = Vec::new();
    for tool in ok_tools {
        let name = tool.name;

        let tx = notifiers[name].clone();
        let Some(rx) = subscribers.remove(&tool.name) else {
            tracing::error!("Could not get subscriber with {name}");
            continue;
        };

        tasks.push((
            tool.name,
            tokio::spawn(async move {
                for (dep, mut rx) in rx {
                    tracing::info!("{name}: dependency ({dep}): waiting");
                    if rx.recv().await.ok().is_none() {
                        tracing::info!("{name}: dependency ({dep}): failed to install");
                        tx.send(None).ok();
                        return Ok(Err(anyhow!("{name}: Dependency not installed: {dep}")));
                    }
                }
                tracing::info!("{name}: all of the dependencies installed");

                let res = (tool.spawn_install)(tool.already_installed, tool.latest_version).await;

                let status = if matches!(&res, Ok(Ok(_))) {
                    Some(())
                } else {
                    None
                };
                tx.send(status).ok();
                res
            }),
        ));
    }

    let mut results = Vec::new();
    for (name, task) in tasks {
        let res = task.await;
        results.push((name, res));
    }

    logger_handle.abort();

    println!();
    println!();
    println!("{}", LogHeader("SUMMARY", Green));
    println!();
    println!("Done all!");
    println!();

    for (name, res) in results {
        if matches!(&res, Ok(Ok(_))) {
            println!("- {name}: {}", "installed".green());
        } else {
            println!("- {name}: {}: {res:?}", "failed".red());
        }
    }

    println!();

    Ok(())
}

async fn setup_links_impl(logger: Receiver<Vec<u8>>, root: Option<PathBuf>) -> Result<()> {
    use owo_colors::OwoColorize as _;
    use owo_colors::colors::{Green, Red};

    let _logger_handle = spawn_redirect_logger(&logger);

    println!();
    println!();
    println!("{}", LogHeader("SETUP LINKS", Green));
    println!();

    let (ctx, configs) = links::configs(root).await?;

    println!("- HOME: {}", ctx.home.display().green());
    println!(
        "- XDG_CONFIG_HOME: {}",
        ctx.xdg_config_home.display().green()
    );
    println!("- repository root: {}", ctx.root.display().green());
    println!();

    let configs = ctx.resolve(&configs).await?;
    for config in &configs {
        println!("- link name: {}", config.name.display().green());
        println!("  link to:   {}", config.link_to.display().green());
        println!();
    }

    let mut tasks = Vec::new();

    for config in &configs {
        let config_clone = config.clone();
        tasks.push((
            config,
            tokio::spawn(async move { config_clone.make().await }),
        ));
    }

    let mut errors = Vec::new();

    for (config, task) in tasks {
        match task.await {
            Err(e) => errors.push(anyhow!(
                "could not join for {}: {e}",
                config.link_to.display()
            )),
            Ok(Err(e)) => errors.push(anyhow!(
                "could not create symlink for {}: {e}",
                config.link_to.display()
            )),
            Ok(Ok(())) => {}
        }
    }

    if errors.is_empty() {
        return Ok(());
    }

    println!();
    println!();
    println!("{}", LogHeader("LINK ERRORS", Red));
    println!();
    for err in errors {
        println!("- {}", err.red());
    }

    Err(anyhow!("link: failed to create symlinks"))
}

async fn gen_profile_impl(
    logger: Receiver<Vec<u8>>,
    root: Option<PathBuf>,
    env: &Path,
) -> Result<()> {
    use owo_colors::OwoColorize as _;
    use owo_colors::colors::Green;

    use tokio::fs::File;
    use tokio::io::AsyncWrite;
    use tokio::io::{AsyncBufReadExt as _, AsyncWriteExt as _, BufReader, BufWriter};

    #[derive(Default)]
    struct Ctx {
        public_tmps: Vec<String>,
    }

    async fn write_env(
        dst: &mut (impl AsyncWrite + Unpin),
        path: impl AsRef<Path>,
        ctx: &mut Ctx,
    ) -> Result<()> {
        let path = path.as_ref();
        let src = File::open(path).await?;
        let src = BufReader::new(src);

        let mut lines = src.lines();
        while let Some(line) = lines.next_line().await? {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }

            assert!(
                line.contains('='),
                "{}: invalid env format: no equal sign: {line:?}",
                path.display()
            );

            dst.write_all(b"    ").await?;

            if line.starts_with("__") {
                println!("> local {line}");
                dst.write_all(b"local ").await?;
            } else if line.starts_with('_') {
                println!("> {line}");
                if let Some((key, _)) = line.split_once('=') {
                    ctx.public_tmps.push(key.to_owned());
                }
            } else {
                println!("> export {line}");
                dst.write_all(b"export ").await?;
            }
            dst.write_all(line.as_bytes()).await?;
            dst.write_all(b"\n").await?;
        }
        Ok(())
    }

    let _logger_handle = spawn_redirect_logger(&logger);

    println!();
    println!();
    println!("{}", LogHeader("PROFILE", Green));
    println!();

    let root = if let Some(root) = root {
        root
    } else {
        links::get_root().await?
    };

    println!("- repository root: {}", root.display().green());

    let profile = root.join("shell/profile.sh");
    let profile_exists = tokio::fs::try_exists(&profile).await.is_ok_and(|v| v);
    let generated = root.join("shell/generated.profile.sh");

    if !profile_exists {
        println!("- profile.sh: {}", profile.display().red());
        println!();

        return Err(anyhow!("profile: could not find profile.sh"));
    }

    println!("- profile.sh: {}", profile.display().green());
    println!("- generated.profile.sh: {}", generated.display().green());
    println!();

    let dst = File::create(&generated).await?;
    let mut dst = BufWriter::new(dst);

    dst.write_all(b"# auto-generated in github.com:naughie/dotfiles\n")
        .await?;

    dst.write_all(b"__my_setup_generated_envs() {\n").await?;

    let mut ctx = Ctx::default();

    dst.write_all(b"    ###  CONFIG  ###\n").await?;
    write_env(&mut dst, env.join("env.config"), &mut ctx).await?;
    dst.write_all(b"    ### TEMPLATE ###\n").await?;
    write_env(&mut dst, env.join("env.template"), &mut ctx).await?;

    dst.write_all(b"}\n_my_clear_generated_envs() {\n").await?;

    for env_key in &ctx.public_tmps {
        dst.write_all(b"    unset -v ").await?;
        dst.write_all(env_key.as_bytes()).await?;
        dst.write_all(b"\n").await?;
    }
    dst.write_all(b"    unset -f _my_clear_generated_envs\n")
        .await?;

    dst.write_all(b"}\n__my_setup_generated_envs\nunset -f __my_setup_generated_envs\n")
        .await?;

    dst.shutdown().await?;

    Ok(())
}

struct LogHeader<'a, C>(&'a str, C);

impl<C: Color> Display for LogHeader<'_, C> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        struct Repeat(char, usize);

        impl Display for Repeat {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                for _ in 0..self.1 {
                    write!(f, "{}", self.0)?;
                }
                Ok(())
            }
        }

        use owo_colors::FgColorDisplay;

        let len = self.0.len();
        let header: FgColorDisplay<C, str> = FgColorDisplay::new(self.0);

        let total_len = 55usize;
        let total_margin = total_len - len - 4;
        let left_margin = total_margin / 2;
        let right_margin = total_margin - left_margin;

        writeln!(f, "{}", Repeat('#', total_len))?;
        writeln!(
            f,
            "##{}{}{}##",
            Repeat(' ', left_margin),
            header,
            Repeat(' ', right_margin),
        )?;
        write!(f, "{}", Repeat('#', total_len))?;
        Ok(())
    }
}

async fn run_async_fn(
    env_path: &Path,
    f: impl AsyncFnOnce(Receiver<Vec<u8>>) -> Result<()>,
) -> Result<()> {
    {
        use owo_colors::OwoColorize as _;

        println!("Reading env files:");
        let env_config = env_path.join("env.config");
        let env_template = env_path.join("env.template");

        let config_exists = tokio::fs::try_exists(&env_config).await.is_ok_and(|v| v);
        if config_exists {
            println!("- config:   {}", env_config.display().green());
        } else {
            println!("- config:   {}", env_config.display().red());
        }

        let template_exists = tokio::fs::try_exists(&env_template).await.is_ok_and(|v| v);
        if template_exists {
            println!("- template: {}", env_template.display().green());
        } else {
            println!("- template: {}", env_template.display().red());
        }
        println!();

        if !config_exists || !template_exists {
            return Err(anyhow!("could not read env files: not found"));
        }

        dotenvy::from_path_override(&env_config)?;
        dotenvy::from_path_override(&env_template)?;
    }

    let logger = init_log()?;

    let cli_fut = async move {
        let res = f(logger.clone()).await;
        logger.close();
        redirect_logger(logger).await.ok();
        res
    };

    let ctrl_c = tokio::signal::ctrl_c();

    tokio::select! {
        x = cli_fut => { x?; },
        _ = ctrl_c => {},
    }
    Ok(())
}

pub async fn install_tools(env_path: &Path) -> Result<()> {
    run_async_fn(env_path, install_tools_impl).await
}

pub async fn setup_links(env_path: &Path, root: Option<PathBuf>) -> Result<()> {
    run_async_fn(env_path, async |logger| {
        setup_links_impl(logger, root).await
    })
    .await
}

pub async fn gen_profile(env_path: &Path, root: Option<PathBuf>) -> Result<()> {
    run_async_fn(env_path, async |logger| {
        gen_profile_impl(logger, root, env_path).await
    })
    .await
}
