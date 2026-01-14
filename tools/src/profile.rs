use super::links;

use std::path::PathBuf;

use anyhow::{Result, anyhow};

pub async fn find_path(root: Option<PathBuf>) -> Result<PathBuf> {
    use owo_colors::OwoColorize as _;

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

    Ok(generated)
}

pub struct Export {
    pub root: &'static str,
    pub bin: &'static str,
}

pub fn path_exports()
-> impl IntoIterator<Item = Export, IntoIter = impl DoubleEndedIterator<Item = Export>> {
    [
        Export {
            root: "XDG_BIN_HOME",
            bin: "",
        },
        Export {
            root: "GOPATH",
            bin: "/bin",
        },
        Export {
            root: "GOROOT",
            bin: "/bin",
        },
        Export {
            root: "FNM_DIR",
            bin: "/bin",
        },
        Export {
            root: "DENO_INSTALL_ROOT",
            bin: "",
        },
        Export {
            root: "BUN_INSTALL",
            bin: "/bin",
        },
        Export {
            root: "UV_INSTALL_DIR",
            bin: "",
        },
        Export {
            root: "UV_PYTHON_BIN_DIR",
            bin: "",
        },
        Export {
            root: "UV_TOOL_BIN_DIR",
            bin: "",
        },
        Export {
            root: "RBENV_ROOT",
            bin: "/bin",
        },
        Export {
            root: "CARGO_HOME",
            bin: "/bin",
        },
    ]
}
