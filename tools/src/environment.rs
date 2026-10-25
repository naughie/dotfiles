use crate::frontend::Output;

use anyhow::Result;

use std::path::{Path, PathBuf};

#[derive(Debug)]
struct EnvReadError {
    path: PathBuf,
    source: dotenvy::Error,
}

impl std::fmt::Display for EnvReadError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "could not read {}", self.path.display())
    }
}

impl std::error::Error for EnvReadError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&self.source)
    }
}

pub fn file_names() -> &'static [&'static str] {
    if cfg!(target_os = "macos") {
        &["env.config", "env.template", "env.macos"]
    } else {
        &["env.config", "env.template"]
    }
}

pub fn path_buffer(dir: &Path) -> PathBuf {
    let suffix = file_names()
        .iter()
        .map(|name| name.len())
        .max()
        .unwrap_or(0);

    let mut path = PathBuf::with_capacity(dir.as_os_str().len() + 1 + suffix);
    path.push(dir);

    path
}

pub fn load(dir: &Path, output: Output<'_>) -> Result<()> {
    output.print(format_args!("Reading env files:\n"));

    let mut path = path_buffer(dir);

    for name in file_names() {
        path.push(name);
        if *name != "env.macos" || path.try_exists()? {
            output.print(format_args!("- {}\n", path.display()));
            if let Err(source) = dotenvy::from_path_override(&path) {
                return Err(EnvReadError { path, source }.into());
            }
        }

        path.pop();
    }

    output.print(format_args!("\n"));

    Ok(())
}
