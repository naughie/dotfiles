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
    /// Path to the env file
    #[arg(short, long)]
    env: PathBuf,
}

#[derive(Debug, ArgsDerive)]
pub struct LinkArgs {
    /// Path to the env file
    #[arg(short, long)]
    env: PathBuf,

    /// Path to the repository root
    #[arg(short, long)]
    root: Option<PathBuf>,
}

#[derive(Debug, ArgsDerive)]
pub struct ProfileArgs {
    /// Path to the env file
    #[arg(short, long)]
    env: PathBuf,

    /// Path to the repository root
    #[arg(short, long)]
    root: Option<PathBuf>,
}

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    println!("{args:?}");

    match args.command {
        Command::Install(args) => make::install_tools(&args.env).await?,
        Command::Link(args) => make::setup_links(&args.env, args.root).await?,
        Command::Profile(args) => make::gen_profile(&args.env, args.root).await?,
    }

    Ok(())
}
