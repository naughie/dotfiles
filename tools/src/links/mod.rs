mod config;
use config::configs;

use crate::frontend::Output;
use crate::logger::{LogHeader, spawn_redirect_logger};
use async_channel::Receiver;

use anyhow::{Result, anyhow};

use std::path::PathBuf;

pub async fn cli_impl(
    logger: Receiver<Vec<u8>>,
    root: Option<PathBuf>,
    output: Output<'_>,
) -> Result<()> {
    let logger_task = (!output.is_tui()).then(|| spawn_redirect_logger(&logger));
    let result = setup_links(root, output).await;

    if let Some(task) = logger_task {
        task.abort();
        task.await.ok();
    }

    result
}

async fn setup_links(root: Option<PathBuf>, output: Output<'_>) -> Result<()> {
    use owo_colors::OwoColorize as _;
    use owo_colors::colors::{Green, Red};

    output.print(format_args!("\n\n{}\n\n", LogHeader("SETUP LINKS", Green)));

    let (ctx, configs) = configs(root).await?;

    output.print(format_args!(
        "- HOME: {}\n- XDG_CONFIG_HOME: {}\n- repository root: {}\n\n",
        ctx.home.display().green(),
        ctx.xdg_config_home.display().green(),
        ctx.root.display().green()
    ));

    let mut failed = false;
    ctx.resolve(&configs, async |config| {
        output.print(format_args!(
            "- link name: {}\n  link to:   {}\n\n",
            config.name.display().green(),
            config.link_to.display().green()
        ));

        if let Err(error) = config.make().await {
            if !failed {
                output.print(format_args!("{}\n\n", LogHeader("LINK ERRORS", Red)));
            }

            output.print(format_args!(
                "- could not create symlink for {}: {}\n",
                config.link_to.display(),
                error.red()
            ));
            failed = true;
        }

        Ok(())
    })
    .await?;

    if failed {
        Err(anyhow!("link: failed to create symlinks"))
    } else {
        Ok(())
    }
}
