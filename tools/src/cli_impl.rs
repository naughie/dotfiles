use crate::frontend::{self, Completion, Output, Session, ToolStatus};
use crate::install::{self, InstallOptions, TOOL_COUNT, ToolSelection};
use crate::links;
use crate::logger::{init_log, redirect_logger, set_tui};
use crate::profile;

use anyhow::Result;
use async_channel::Receiver;

use std::path::{Path, PathBuf};

async fn run_cli(
    env_path: Option<&Path>,
    f: impl for<'a> AsyncFnOnce(Receiver<Vec<u8>>, Output<'a>) -> Result<()>,
) -> Result<()> {
    let output = Output::cli();
    if let Some(env_path) = env_path {
        crate::environment::load(env_path, output)?;
    }

    let logger = init_log()?;

    let result = tokio::select! {
        result = f(logger.clone(), output) => result,
        result = tokio::signal::ctrl_c() => match result {
            Ok(()) => Err(anyhow::anyhow!("interrupted")),
            Err(error) => Err(error.into()),
        },
    };

    logger.close();
    redirect_logger(logger).await.ok();

    result
}

struct TuiMode;

impl Drop for TuiMode {
    fn drop(&mut self) {
        set_tui(false);
    }
}

async fn run_command(
    env_path: Option<&Path>,
    title: &'static str,
    tools: Option<[&'static str; TOOL_COUNT]>,
    skipped: ToolSelection,
    interactive: bool,
    tui: bool,
    f: impl for<'a> AsyncFnOnce(Receiver<Vec<u8>>, Output<'a>) -> Result<()>,
) -> Result<()> {
    if !tui {
        return run_cli(env_path, f).await;
    }

    tokio::task::block_in_place(|| {
        let session = Session::new(title, tools, skipped, interactive);
        let output = session.output();
        let environment = match env_path {
            Some(path) => crate::environment::load(path, output),
            None => Ok(()),
        };

        set_tui(true);
        let _mode = TuiMode;
        let logger = init_log()?;
        let mut interrupt =
            tokio::signal::unix::signal(tokio::signal::unix::SignalKind::interrupt())?;

        std::thread::scope(|scope| {
            let renderer = scope.spawn(|| session.run(&logger));
            let _completion = Completion(&session);
            session.wait_until_started()?;

            let work = async {
                environment?;
                session.wait_for_selection().await?;

                f(logger.clone(), output).await
            };

            let result = tokio::runtime::Handle::current().block_on(async {
                let signal = interrupt.recv();
                tokio::pin!(signal);

                let mut result = tokio::select! {
                    result = work => result,
                    () = session.interrupted() => Err(anyhow::anyhow!("interrupted")),
                    result = &mut signal => {
                        session.cancel();
                        match result {
                            Some(()) => Err(anyhow::anyhow!("interrupted")),
                            None => Err(anyhow::anyhow!("interrupt handler closed")),
                        }
                    },
                };

                session.finish(&result);
                logger.close();

                if !session.is_interrupted() {
                    tokio::select! {
                        () = session.interrupted() => {},
                        signal = &mut signal => {
                            session.cancel();
                            result = match signal {
                                Some(()) => Err(anyhow::anyhow!("interrupted")),
                                None => Err(anyhow::anyhow!("interrupt handler closed")),
                            };
                        },
                    }
                }

                result
            });

            match renderer.join() {
                Ok(rendered) => rendered?,
                Err(payload) => std::panic::resume_unwind(payload),
            }

            result
        })
    })
}

pub async fn install_tools(env_path: &Path, options: InstallOptions) -> Result<()> {
    let tui = frontend::is_terminal();
    let options = if tui {
        options
    } else {
        options.select_interactively()?
    };
    let skipped = options.skip;
    let interactive = options.interactive;

    run_command(
        Some(env_path),
        "Install tools",
        Some(install::tool_names()),
        skipped,
        interactive,
        tui,
        async |logger, output| {
            let mut options = options;
            options.skip = output.selection(options.skip);

            install::cli_impl(logger, options, output).await
        },
    )
    .await
}

pub async fn setup_links(env_path: &Path, root: Option<PathBuf>) -> Result<()> {
    run_command(
        Some(env_path),
        "Link configuration",
        None,
        ToolSelection::default(),
        false,
        frontend::is_terminal(),
        async |logger, output| links::cli_impl(logger, root, output).await,
    )
    .await
}

pub async fn gen_profile(env_path: &Path, root: Option<PathBuf>) -> Result<()> {
    run_command(
        Some(env_path),
        "Generate profile",
        None,
        ToolSelection::default(),
        false,
        frontend::is_terminal(),
        async |logger, output| profile::cli_impl(logger, root, env_path, output).await,
    )
    .await
}

pub async fn list_tools() -> Result<()> {
    let tui = frontend::is_terminal();
    if !tui {
        install::list_tools();
        return Ok(());
    }

    run_command(
        None,
        "Available tools",
        Some(install::tool_names()),
        ToolSelection::default(),
        false,
        tui,
        async |_, output| {
            for index in 0..TOOL_COUNT {
                output.tool_status(index, ToolStatus::Finished);
            }

            Ok(())
        },
    )
    .await
}
