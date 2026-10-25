#![cfg(any(
    all(
        target_arch = "x86_64",
        target_os = "linux",
        any(target_env = "gnu", target_env = "musl")
    ),
    all(target_arch = "aarch64", target_os = "macos"),
))]

use std::path::PathBuf;

use clap::{Args as ArgsDerive, Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(about, long_about = None)]
pub struct Args {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    Install(InstallArgs),
    Link(LinkArgs),
    Profile(ProfileArgs),
}

#[derive(Debug, ArgsDerive)]
pub struct InstallArgs {
    #[arg(
        short,
        long,
        required_unless_present = "list",
        help = "Path to the env directory"
    )]
    env: Option<PathBuf>,

    #[arg(
        long,
        help = "List tool IDs without reading env files or accessing the network"
    )]
    list: bool,

    #[command(flatten)]
    options: make::cli::InstallOptions,
}

#[derive(Debug, ArgsDerive)]
pub struct LinkArgs {
    #[arg(short, long, help = "Path to the env directory")]
    env: PathBuf,

    #[arg(short, long, help = "Path to the repository root")]
    root: Option<PathBuf>,
}

#[derive(Debug, ArgsDerive)]
pub struct ProfileArgs {
    #[arg(short, long, help = "Path to the env directory")]
    env: PathBuf,

    #[arg(short, long, help = "Path to the repository root")]
    root: Option<PathBuf>,
}

#[tokio::main]
pub async fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    match args.command {
        Command::Install(args) => {
            if args.list {
                make::cli::list_tools().await?;
            } else {
                make::cli::install(&args.env.expect("env is required by clap"), args.options)
                    .await?;
            }
        }
        Command::Link(args) => make::cli::links(&args.env, args.root).await?,
        Command::Profile(args) => make::cli::profile(&args.env, args.root).await?,
    }

    Ok(())
}
