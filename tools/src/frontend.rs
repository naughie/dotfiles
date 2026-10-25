use crate::install::{TOOL_COUNT, ToolSelection};
use crate::tui::{LogHistory, RowWriter, Screen, render_logs};

use anyhow::{Result, bail};
use async_channel::Receiver;

use crossterm::event::{KeyCode, KeyModifiers};
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::{Color, Style};

use std::fmt;
use std::io::{self, IsTerminal as _};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Condvar, Mutex, MutexGuard};
use std::time::Duration;

use tokio::sync::Notify;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ToolStatus {
    InProgress,
    Finished,
    Skipped,
    WaitingForDependencies,
    Error,
}

impl ToolStatus {
    fn label(self) -> &'static str {
        match self {
            Self::InProgress => "In progress",
            Self::Finished => "Finished",
            Self::Skipped => "Skipped",
            Self::WaitingForDependencies => "Waiting for dependencies",
            Self::Error => "Error",
        }
    }

    fn color(self) -> Color {
        match self {
            Self::InProgress => Color::Cyan,
            Self::Finished => Color::Green,
            Self::Skipped => Color::DarkGray,
            Self::WaitingForDependencies => Color::Yellow,
            Self::Error => Color::Red,
        }
    }
}

#[derive(Clone, Copy)]
pub(crate) struct Output<'a> {
    session: Option<&'a Session>,
}

impl<'a> Output<'a> {
    pub const fn cli() -> Self {
        Self { session: None }
    }

    fn tui(session: &'a Session) -> Self {
        Self {
            session: Some(session),
        }
    }

    pub fn is_tui(self) -> bool {
        self.session.is_some()
    }

    pub fn print(self, arguments: fmt::Arguments<'_>) {
        if let Some(session) = self.session {
            lock(&session.state).messages.push_formatted(arguments);
        } else {
            print!("{arguments}");
        }
    }

    pub fn tool_status(self, index: usize, status: ToolStatus) {
        if let Some(session) = self.session
            && let Some(current) = lock(&session.state).statuses.get_mut(index)
        {
            *current = status;
        }
    }

    pub fn selection(self, fallback: ToolSelection) -> ToolSelection {
        self.session
            .map_or(fallback, |session| lock(&session.state).skipped)
    }
}

pub(crate) fn is_terminal() -> bool {
    io::stdin().is_terminal()
        && io::stdout().is_terminal()
        && std::env::var_os("TERM").is_none_or(|term| term != "dumb")
}

fn lock<T>(mutex: &Mutex<T>) -> MutexGuard<'_, T> {
    mutex
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner)
}

struct State {
    title: &'static str,
    tools: Option<[&'static str; TOOL_COUNT]>,
    statuses: [ToolStatus; TOOL_COUNT],
    cursor: usize,
    skipped: ToolSelection,

    selecting: bool,
    finished: bool,
    failed: bool,

    messages: LogHistory,
}

impl State {
    fn render(&self, area: Rect, buffer: &mut Buffer) {
        if area.is_empty() {
            return;
        }

        let mut row = area.y;
        let heading = if self.selecting {
            "Space toggle  a toggle all  Enter start  q cancel"
        } else if self.finished {
            "Complete - Enter/q to close"
        } else {
            "Running - q/Esc/Ctrl-C cancel"
        };
        let heading_color = if self.failed { Color::Red } else { Color::Cyan };
        RowWriter::new(
            buffer,
            Rect::new(area.x, row, area.width, 1),
            Style::default().fg(heading_color),
        )
        .write(format_args!("{heading}"));
        row += 1;

        if let Some(tools) = &self.tools {
            if row < area.bottom() {
                let finished = self
                    .statuses
                    .iter()
                    .filter(|&&s| s == ToolStatus::Finished)
                    .count();
                let skipped = self
                    .statuses
                    .iter()
                    .filter(|&&s| s == ToolStatus::Skipped)
                    .count();
                let errors = self
                    .statuses
                    .iter()
                    .filter(|&&s| s == ToolStatus::Error)
                    .count();

                RowWriter::new(
                    buffer,
                    Rect::new(area.x, row, area.width, 1),
                    Style::default(),
                )
                .write(format_args!(
                    "Finished {finished}  Skipped {skipped}  Error {errors}  Total {TOOL_COUNT}"
                ));
                row += 1;
            }

            let remaining = area.bottom().saturating_sub(row);
            let messages = remaining.min(6) / 2;
            let visible = usize::from(remaining.saturating_sub(messages)).min(TOOL_COUNT);
            let first = self.cursor.saturating_add(1).saturating_sub(visible);

            for (index, name) in tools.iter().enumerate().skip(first).take(visible) {
                let style = Style::default().fg(self.statuses[index].color());
                let focus = if index == self.cursor { ">" } else { " " };
                let mut writer =
                    RowWriter::new(buffer, Rect::new(area.x, row, area.width, 1), style);

                if self.selecting {
                    let selected = if self.skipped.contains(index) {
                        " "
                    } else {
                        "x"
                    };
                    writer.write(format_args!("{focus} [{selected}] {name}"));
                } else {
                    writer.write(format_args!(
                        "{focus} {name:<25} {}",
                        self.statuses[index].label()
                    ));
                }
                row += 1;
            }
        }

        if row < area.bottom() {
            render_logs(
                buffer,
                Rect::new(area.x, row, area.width, area.bottom() - row),
                &self.messages,
                0,
            );
        }
    }
}

enum Startup {
    Pending,
    Ready,
    Failed(io::Error),
}

pub(crate) struct Session {
    state: Mutex<State>,

    startup: Mutex<Startup>,
    started: Condvar,

    selection_ready: Notify,
    cancelled: Notify,
    interrupted: AtomicBool,
}

impl Session {
    pub fn new(
        title: &'static str,
        tools: Option<[&'static str; TOOL_COUNT]>,
        skipped: ToolSelection,
        selecting: bool,
    ) -> Self {
        Self {
            state: Mutex::new(State {
                title,
                tools,
                statuses: std::array::from_fn(|index| {
                    if skipped.contains(index) {
                        ToolStatus::Skipped
                    } else {
                        ToolStatus::WaitingForDependencies
                    }
                }),
                cursor: 0,
                skipped,
                selecting,
                finished: false,
                failed: false,
                messages: LogHistory::default(),
            }),
            startup: Mutex::new(Startup::Pending),
            started: Condvar::new(),
            selection_ready: Notify::new(),
            cancelled: Notify::new(),
            interrupted: AtomicBool::new(false),
        }
    }

    pub fn output(&self) -> Output<'_> {
        Output::tui(self)
    }

    pub fn wait_until_started(&self) -> io::Result<()> {
        let mut startup = lock(&self.startup);

        while matches!(*startup, Startup::Pending) {
            startup = self
                .started
                .wait(startup)
                .unwrap_or_else(std::sync::PoisonError::into_inner);
        }

        match std::mem::replace(&mut *startup, Startup::Ready) {
            Startup::Failed(error) => Err(error),
            _ => Ok(()),
        }
    }

    pub fn cancel(&self) {
        self.interrupted.store(true, Ordering::Release);

        self.cancelled.notify_one();
        self.selection_ready.notify_one();
    }

    pub fn is_interrupted(&self) -> bool {
        self.interrupted.load(Ordering::Acquire)
    }

    pub async fn interrupted(&self) {
        while !self.interrupted.load(Ordering::Acquire) {
            self.cancelled.notified().await;
        }
    }

    pub async fn wait_for_selection(&self) -> Result<()> {
        loop {
            if self.interrupted.load(Ordering::Acquire) {
                bail!("interrupted");
            }
            if !lock(&self.state).selecting {
                return Ok(());
            }

            self.selection_ready.notified().await;
        }
    }

    pub fn finish(&self, result: &Result<()>) {
        if let Err(error) = result {
            self.output().print(format_args!("Error: {error:#}\n"));
        }

        let mut state = lock(&self.state);
        state.finished = true;
        state.selecting = false;
        state.failed = result.is_err();

        if result.is_err() {
            for status in &mut state.statuses {
                if matches!(
                    status,
                    ToolStatus::InProgress | ToolStatus::WaitingForDependencies
                ) {
                    *status = ToolStatus::Error;
                }
            }
        }
    }

    pub fn run(&self, logger: &Receiver<Vec<u8>>) -> Result<()> {
        let _lifetime = RendererLifetime(self);
        let mut screen = match Screen::new() {
            Ok(screen) => {
                *lock(&self.startup) = Startup::Ready;
                self.started.notify_one();

                screen
            }
            Err(error) => {
                *lock(&self.startup) = Startup::Failed(error);
                self.started.notify_one();

                return Ok(());
            }
        };

        let mut logs = LogHistory::default();

        let result = (|| {
            loop {
                for _ in 0..256 {
                    match logger.try_recv() {
                        Ok(chunk) => logs.push(chunk),
                        Err(_) => break,
                    }
                }

                while let Some(key) = screen.poll()? {
                    let mut state = lock(&self.state);

                    match key.code {
                        KeyCode::Char('q') | KeyCode::Esc => {
                            if state.finished {
                                return Ok(());
                            }

                            self.cancel();
                        }
                        KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                            if state.finished {
                                return Ok(());
                            }

                            self.cancel();
                        }
                        KeyCode::Enter if state.selecting => {
                            state.selecting = false;
                            self.selection_ready.notify_one();
                        }
                        KeyCode::Enter if state.finished => return Ok(()),
                        KeyCode::Up | KeyCode::Char('k') => {
                            state.cursor = state.cursor.saturating_sub(1)
                        }
                        KeyCode::Down | KeyCode::Char('j') => {
                            state.cursor = (state.cursor + 1).min(TOOL_COUNT - 1)
                        }
                        KeyCode::Char(' ') if state.selecting => {
                            let cursor = state.cursor;
                            state.skipped.toggle(cursor);
                        }
                        KeyCode::Char('a') if state.selecting => state.skipped.toggle_all(),
                        _ => {}
                    }

                    if state.selecting {
                        for index in 0..TOOL_COUNT {
                            state.statuses[index] = if state.skipped.contains(index) {
                                ToolStatus::Skipped
                            } else {
                                ToolStatus::WaitingForDependencies
                            };
                        }
                    }
                }

                let state = lock(&self.state);
                if state.finished && self.interrupted.load(Ordering::Acquire) {
                    return Ok(());
                }

                screen.draw(state.title, &logs, |area, buffer| {
                    state.render(area, buffer)
                })?;
                drop(state);

                std::thread::sleep(Duration::from_millis(33));
            }
        })();

        if result.is_err() {
            self.cancel();
        }

        result
    }
}

impl Drop for Session {
    fn drop(&mut self) {
        self.cancel();
    }
}

pub(crate) struct Completion<'a>(pub &'a Session);

impl Drop for Completion<'_> {
    fn drop(&mut self) {
        if !lock(&self.0.state).finished {
            self.0.cancel();

            let mut state = lock(&self.0.state);
            state.finished = true;
            state.failed = true;
        }
    }
}

struct RendererLifetime<'a>(&'a Session);

impl Drop for RendererLifetime<'_> {
    fn drop(&mut self) {
        let mut startup = lock(&self.0.startup);

        if matches!(*startup, Startup::Pending) {
            *startup =
                Startup::Failed(io::Error::other("terminal renderer stopped during startup"));
            self.0.started.notify_one();
        }

        self.0.cancel();
    }
}
