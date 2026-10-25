#![cfg(any(
    all(
        target_arch = "x86_64",
        target_os = "linux",
        any(target_env = "gnu", target_env = "musl")
    ),
    all(target_arch = "aarch64", target_os = "macos"),
))]

mod cli_impl;
mod environment;
mod frontend;
mod install;
mod links;
mod logger;
mod profile;
mod tui;

pub mod cli {
    pub use crate::cli_impl::{
        gen_profile as profile, install_tools as install, list_tools, setup_links as links,
    };
    pub use crate::install::InstallOptions;
}
