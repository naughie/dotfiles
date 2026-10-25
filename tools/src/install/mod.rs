mod definitions;

use crate::frontend::{Output, ToolStatus};
use crate::logger::{LogHeader, spawn_redirect_logger};

use anyhow::{Result, bail};
use async_channel::Receiver;
pub(crate) use definitions::TOOL_COUNT;
use definitions::{Installer, TOOLS, ToolDefinition};

use std::io::{BufRead, Write};

#[derive(Debug, Clone, Copy, Default)]
pub struct ToolSelection(u32);

impl ToolSelection {
    pub(crate) fn toggle(&mut self, index: usize) {
        self.0 ^= 1 << index;
    }

    pub(crate) fn toggle_all(&mut self) {
        self.0 ^= (1 << TOOL_COUNT) - 1;
    }

    pub(crate) fn contains(self, index: usize) -> bool {
        self.0 & (1 << index) != 0
    }
}

impl std::str::FromStr for ToolSelection {
    type Err = anyhow::Error;

    fn from_str(value: &str) -> Result<Self> {
        if value == "all" {
            return Ok(Self((1 << TOOL_COUNT) - 1));
        }

        match TOOLS.iter().position(|tool| tool.name == value) {
            Some(index) => Ok(Self(1 << index)),
            None => bail!("unknown tool ID: {value}; use install --list to see available IDs"),
        }
    }
}

#[derive(Debug, Default)]
pub struct InstallOptions {
    pub skip: ToolSelection,
    pub interactive: bool,
}

impl clap::Args for InstallOptions {
    fn augment_args(command: clap::Command) -> clap::Command {
        command
            .arg(
                clap::Arg::new("skip")
                    .long("skip")
                    .value_name("TOOL")
                    .value_delimiter(',')
                    .value_parser(clap::value_parser!(ToolSelection))
                    .action(clap::ArgAction::Append)
                    .help("Skip tools by ID; repeat or separate IDs with commas"),
            )
            .arg(
                clap::Arg::new("interactive")
                    .short('i')
                    .long("interactive")
                    .action(clap::ArgAction::SetTrue)
                    .help("Choose tools to skip before checking the environment"),
            )
    }

    fn augment_args_for_update(command: clap::Command) -> clap::Command {
        Self::augment_args(command)
    }
}

impl clap::FromArgMatches for InstallOptions {
    fn from_arg_matches(matches: &clap::ArgMatches) -> std::result::Result<Self, clap::Error> {
        let mut options = Self::default();
        options.update_from_arg_matches(matches)?;

        Ok(options)
    }

    fn update_from_arg_matches(
        &mut self,
        matches: &clap::ArgMatches,
    ) -> std::result::Result<(), clap::Error> {
        if let Some(selections) = matches.get_many::<ToolSelection>("skip") {
            for selection in selections {
                self.skip.0 |= selection.0;
            }
        }

        self.interactive = matches.get_flag("interactive");

        Ok(())
    }
}

pub(crate) fn tool_names() -> [&'static str; TOOL_COUNT] {
    std::array::from_fn(|index| TOOLS[index].name)
}

pub fn list_tools() {
    for tool in &TOOLS {
        let suffix = if tool.externally_managed {
            " (externally managed)"
        } else {
            ""
        };

        println!("{}{suffix}", tool.name);
    }
}

fn prompt_skips(input: &mut impl BufRead, output: &mut impl Write) -> Result<ToolSelection> {
    write!(
        output,
        "Tools to skip (comma-separated IDs, all, or Enter to keep the selection): "
    )?;
    output.flush()?;

    let mut selection = ToolSelection::default();
    let mut token = [0u8; 64];
    let mut length = 0;
    let mut read_any = false;

    loop {
        let available = input.fill_buf()?;
        if available.is_empty() {
            if !read_any {
                bail!("interactive selection requires input");
            }

            if length != 0 {
                selection.0 |= std::str::from_utf8(&token[..length])?
                    .parse::<ToolSelection>()?
                    .0;
            }

            return Ok(selection);
        }

        let mut consumed = 0;
        let mut finished = false;

        for &byte in available {
            consumed += 1;
            read_any = true;

            if matches!(byte, b',' | b' ' | b'\t' | b'\r' | b'\n') {
                if length != 0 {
                    selection.0 |= std::str::from_utf8(&token[..length])?
                        .parse::<ToolSelection>()?
                        .0;
                    length = 0;
                }

                if byte == b'\n' {
                    finished = true;
                    break;
                }
            } else {
                let Some(destination) = token.get_mut(length) else {
                    bail!("unknown tool ID; use install --list to see available IDs");
                };

                *destination = byte;
                length += 1;
            }
        }

        input.consume(consumed);
        if finished {
            return Ok(selection);
        }
    }
}

#[derive(Debug)]
enum Outcome {
    Pending,
    Skipped(bool),
    External,
    UpToDate,
    Installed,
    Failed(anyhow::Error),
    MissingExternal,
    Blocked(&'static str),
    Cycle,
}

impl Outcome {
    fn available(&self) -> bool {
        matches!(
            self,
            Self::Skipped(true) | Self::External | Self::UpToDate | Self::Installed
        )
    }

    fn failed(&self) -> bool {
        matches!(
            self,
            Self::Failed(_) | Self::MissingExternal | Self::Blocked(_) | Self::Cycle
        )
    }
}

impl std::fmt::Display for Outcome {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Skipped(_) => formatter.write_str("skipped"),
            Self::External => formatter.write_str("externally managed"),
            Self::UpToDate => formatter.write_str("up-to-date"),
            Self::Installed => formatter.write_str("installed"),
            Self::Failed(error) => write!(formatter, "failed: {error:#}"),
            Self::MissingExternal => formatter.write_str("failed: required external tool is missing or not executable; install macOS prerequisites with brew install xz fish neovim go"),
            Self::Blocked(dependency) => write!(formatter, "blocked: dependency unavailable: {dependency}"),
            Self::Cycle => formatter.write_str("blocked: dependency cycle"),
            Self::Pending => unreachable!(),
        }
    }
}

fn dependency_state(tool: &ToolDefinition, outcomes: &[Outcome]) -> Option<Outcome> {
    for &dependency in tool.depends_on {
        let Some(index) = TOOLS.iter().position(|tool| tool.name == dependency) else {
            return Some(Outcome::Blocked(dependency));
        };

        if outcomes[index].failed() || matches!(outcomes[index], Outcome::Skipped(false)) {
            return Some(Outcome::Blocked(dependency));
        }

        if !outcomes[index].available() {
            return None;
        }
    }

    Some(Outcome::Installed)
}

async fn execute(
    logger: &Receiver<Vec<u8>>,
    skipped: ToolSelection,
    output: Output<'_>,
) -> [Outcome; TOOL_COUNT] {
    let skipped: [bool; TOOL_COUNT] = std::array::from_fn(|i| skipped.contains(i));
    let mut needed = skipped.map(|skipped| !skipped);

    for (index, tool) in TOOLS.iter().enumerate() {
        if !skipped[index] {
            for dependency in tool.depends_on {
                if let Some(index) = TOOLS.iter().position(|tool| tool.name == *dependency) {
                    needed[index] = true;
                }
            }
        }
    }

    let mut outcomes = std::array::from_fn(|_| Outcome::Pending);
    let mut pending: [Option<Installer>; TOOL_COUNT] = std::array::from_fn(|_| None);

    for (index, tool) in TOOLS.iter().enumerate() {
        output.tool_status(
            index,
            if skipped[index] {
                ToolStatus::Skipped
            } else if tool.depends_on.is_empty() {
                ToolStatus::InProgress
            } else {
                ToolStatus::WaitingForDependencies
            },
        );
    }

    let logger_task = (!output.is_tui()).then(|| spawn_redirect_logger(logger));
    let inspected = definitions::inspect_all(&needed, &skipped, output).await;

    if let Some(logger_task) = logger_task {
        logger_task.abort();
        logger_task.await.ok();
    }

    for (index, inspected) in inspected.into_iter().enumerate() {
        if skipped[index] {
            let available = inspected
                .as_ref()
                .is_some_and(|result| result.as_ref().is_ok_and(Installer::is_installed));
            outcomes[index] = Outcome::Skipped(available);
            continue;
        }

        let installer = match inspected.expect("selected tool is inspected") {
            Ok(installer) => installer,
            Err(error) => {
                outcomes[index] = Outcome::Failed(error);
                continue;
            }
        };

        output.print(format_args!("{}:\n", TOOLS[index].name));
        for (i, target) in installer.install_targets.iter().enumerate() {
            let installed = installer
                .installed_version
                .get(i)
                .and_then(Option::as_deref)
                .unwrap_or("unknown");
            let latest = installer
                .latest_version
                .get(i)
                .and_then(Option::as_deref)
                .unwrap_or("unknown");

            output.print(format_args!(
                "  {}: {:?}, installed {installed}, latest {latest}\n",
                target.display(),
                installer.already_installed[i]
            ));
        }

        if TOOLS[index].externally_managed {
            outcomes[index] = if installer.is_installed() {
                Outcome::External
            } else {
                Outcome::MissingExternal
            };
        } else if installer.is_latest() {
            outcomes[index] = Outcome::UpToDate;
        } else {
            pending[index] = Some(installer);
        }
    }

    let logger_task = (!output.is_tui()).then(|| spawn_redirect_logger(logger));

    while pending.iter().any(Option::is_some) {
        let mut ready = std::array::from_fn(|_| None);
        let mut blocked = false;

        for (index, tool) in TOOLS.iter().enumerate() {
            if pending[index].is_none() {
                continue;
            }

            match dependency_state(tool, &outcomes) {
                Some(Outcome::Installed) => ready[index] = pending[index].take(),
                Some(outcome) => {
                    pending[index] = None;
                    outcomes[index] = outcome;
                    output.tool_status(index, ToolStatus::Error);
                    blocked = true;
                }
                None => {}
            }
        }

        if !ready.iter().any(Option::is_some) {
            if blocked {
                continue;
            }

            for (index, installer) in pending.iter_mut().enumerate() {
                if installer.take().is_some() {
                    outcomes[index] = Outcome::Cycle;
                    output.tool_status(index, ToolStatus::Error);
                }
            }

            break;
        }

        for (index, result) in definitions::install_ready(&mut ready, output)
            .await
            .into_iter()
            .enumerate()
        {
            if let Some(result) = result {
                outcomes[index] = match result {
                    Ok(()) => Outcome::Installed,
                    Err(error) => Outcome::Failed(error),
                };
            }
        }
    }

    if let Some(logger_task) = logger_task {
        logger_task.abort();
        logger_task.await.ok();
    }

    outcomes
}

impl InstallOptions {
    pub fn select_interactively(mut self) -> Result<Self> {
        if self.interactive {
            list_tools();

            if self.skip.0 != 0 {
                print!("Already skipped:");
                let mut separator = " ";

                for (index, tool) in TOOLS.iter().enumerate() {
                    if self.skip.contains(index) {
                        print!("{separator}{}", tool.name);
                        separator = ", ";
                    }
                }

                println!();
            }

            self.skip.0 |=
                prompt_skips(&mut std::io::stdin().lock(), &mut std::io::stdout().lock())?.0;
        }

        Ok(self)
    }
}

pub async fn cli_impl(
    logger: Receiver<Vec<u8>>,
    options: InstallOptions,
    output: Output<'_>,
) -> Result<()> {
    output.print(format_args!("Checking the environment ...\n"));

    let outcomes = execute(&logger, options.skip, output).await;

    if !output.is_tui() {
        output.print(format_args!(
            "\n{}\n",
            LogHeader("SUMMARY", owo_colors::colors::Green)
        ));
    } else {
        output.print(format_args!("Summary\n"));
    }

    for (index, (tool, outcome)) in TOOLS.iter().zip(&outcomes).enumerate() {
        let status = match outcome {
            Outcome::Skipped(_) => ToolStatus::Skipped,
            Outcome::External | Outcome::UpToDate | Outcome::Installed => ToolStatus::Finished,
            Outcome::Failed(_)
            | Outcome::MissingExternal
            | Outcome::Blocked(_)
            | Outcome::Cycle => ToolStatus::Error,
            Outcome::Pending => unreachable!(),
        };

        output.tool_status(index, status);
        output.print(format_args!("- {}: {outcome}\n", tool.name));
    }

    if outcomes.iter().any(Outcome::failed) {
        bail!("one or more tools could not be installed");
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn prompt_combines_ids_without_owned_strings() {
        let selected = prompt_skips(
            &mut Cursor::new(b" rust,go\tfnm \r\n"),
            &mut std::io::sink(),
        )
        .unwrap();

        for id in ["rust", "go", "fnm"] {
            let expected: ToolSelection = id.parse().unwrap();
            assert_eq!(selected.0 & expected.0, expected.0);
        }
        assert_eq!(selected.0.count_ones(), 3);

        assert_eq!(
            prompt_skips(&mut Cursor::new(b"\n"), &mut std::io::sink())
                .unwrap()
                .0,
            0
        );
        assert!(prompt_skips(&mut Cursor::new(b""), &mut std::io::sink()).is_err());
        assert!("unknown".parse::<ToolSelection>().is_err());
    }

    #[tokio::test]
    async fn all_skipped_tools_require_no_inspection() {
        let (_sender, logger) = async_channel::bounded(1000);

        assert!(
            execute(&logger, "all".parse().unwrap(), Output::cli())
                .await
                .iter()
                .all(|outcome| matches!(outcome, Outcome::Skipped(false)))
        );
    }

    #[test]
    fn failed_and_skipped_dependencies_propagate() {
        let mut outcomes = std::array::from_fn::<_, TOOL_COUNT, _>(|_| Outcome::Installed);
        let rust = TOOLS.iter().position(|tool| tool.name == "rust").unwrap();
        let tool = ToolDefinition {
            name: "dependent",
            depends_on: &["rust"],
            externally_managed: false,
        };

        outcomes[rust] = Outcome::Skipped(false);
        assert!(matches!(
            dependency_state(&tool, &outcomes),
            Some(Outcome::Blocked("rust"))
        ));

        outcomes[rust] = Outcome::Skipped(true);
        assert!(matches!(
            dependency_state(&tool, &outcomes),
            Some(Outcome::Installed)
        ));

        outcomes[rust] = Outcome::Pending;
        assert!(dependency_state(&tool, &outcomes).is_none());
    }
}
