use std::fmt::{self, Write as _};
use std::io;
use std::time::Duration;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::{Color, Style};
use ratatui::text::Span;

const LOG_CHUNKS: usize = 128;

pub(crate) struct LogHistory {
    chunks: [Vec<u8>; LOG_CHUNKS],
    next: usize,
    length: usize,
}

impl Default for LogHistory {
    fn default() -> Self {
        Self {
            chunks: std::array::from_fn(|_| Vec::new()),
            next: 0,
            length: 0,
        }
    }
}

impl LogHistory {
    pub fn push(&mut self, chunk: Vec<u8>) {
        if chunk.is_empty() {
            return;
        }

        self.chunks[self.next] = chunk;

        self.next = (self.next + 1) % LOG_CHUNKS;
        self.length = (self.length + 1).min(LOG_CHUNKS);
    }

    pub fn push_formatted(&mut self, arguments: fmt::Arguments<'_>) {
        struct Length(usize, bool);

        impl fmt::Write for Length {
            fn write_str(&mut self, text: &str) -> fmt::Result {
                self.0 += text.len();
                if !text.is_empty() {
                    self.1 = text.ends_with('\n');
                }
                Ok(())
            }
        }

        let mut length = Length(0, false);
        fmt::write(&mut length, arguments).ok();

        let chunk = &mut self.chunks[self.next];
        chunk.clear();
        chunk.reserve(length.0 + usize::from(!length.1));
        std::io::Write::write_fmt(chunk, arguments).ok();
        if !chunk.ends_with(b"\n") {
            chunk.push(b'\n');
        }

        self.next = (self.next + 1) % LOG_CHUNKS;
        self.length = (self.length + 1).min(LOG_CHUNKS);
    }

    fn bytes(&self) -> LogBytes<'_> {
        LogBytes {
            history: self,
            chunk: 0,
            byte: 0,
        }
    }
}

struct LogBytes<'a> {
    history: &'a LogHistory,
    chunk: usize,
    byte: usize,
}

impl Iterator for LogBytes<'_> {
    type Item = u8;

    fn next(&mut self) -> Option<u8> {
        while self.chunk < self.history.length {
            let index =
                (self.history.next + LOG_CHUNKS - self.history.length + self.chunk) % LOG_CHUNKS;
            if let Some(&byte) = self.history.chunks[index].get(self.byte) {
                self.byte += 1;
                return Some(byte);
            }

            self.chunk += 1;
            self.byte = 0;
        }

        None
    }
}

#[derive(Default)]
enum Escape {
    #[default]
    None,
    Start,
    Csi,
    Osc,
    OscEnd,
}

struct LogCharacters<'a> {
    bytes: LogBytes<'a>,
    escape: Escape,
    pending: Option<u8>,
}

impl<'a> LogCharacters<'a> {
    fn new(history: &'a LogHistory) -> Self {
        Self {
            bytes: history.bytes(),
            escape: Escape::None,
            pending: None,
        }
    }
}

impl Iterator for LogCharacters<'_> {
    type Item = char;

    fn next(&mut self) -> Option<char> {
        loop {
            let byte = self.pending.take().or_else(|| self.bytes.next())?;

            match self.escape {
                Escape::Start => {
                    self.escape = match byte {
                        b'[' => Escape::Csi,
                        b']' => Escape::Osc,
                        _ => Escape::None,
                    };
                    continue;
                }
                Escape::Csi => {
                    if (0x40..=0x7e).contains(&byte) {
                        self.escape = Escape::None;
                    }
                    continue;
                }
                Escape::Osc => {
                    if byte == 7 {
                        self.escape = Escape::None;
                    } else if byte == 27 {
                        self.escape = Escape::OscEnd;
                    }
                    continue;
                }
                Escape::OscEnd => {
                    self.escape = if byte == b'\\' {
                        Escape::None
                    } else {
                        Escape::Osc
                    };
                    continue;
                }
                Escape::None => {}
            }

            match byte {
                27 => {
                    self.escape = Escape::Start;
                }
                b'\n' => return Some('\n'),
                b'\t' => return Some(' '),
                0..=31 | 127 => {}
                32..=126 => return Some(char::from(byte)),
                _ => {
                    let length = match byte {
                        0xc2..=0xdf => 2,
                        0xe0..=0xef => 3,
                        0xf0..=0xf4 => 4,
                        _ => return Some('\u{fffd}'),
                    };

                    let mut encoded = [byte, 0, 0, 0];
                    for destination in &mut encoded[1..length] {
                        let Some(next) = self.bytes.next() else {
                            return Some('\u{fffd}');
                        };
                        if next & 0xc0 != 0x80 {
                            self.pending = Some(next);
                            return Some('\u{fffd}');
                        }

                        *destination = next;
                    }

                    return Some(
                        std::str::from_utf8(&encoded[..length])
                            .ok()
                            .and_then(|text| text.chars().next())
                            .unwrap_or('\u{fffd}'),
                    );
                }
            }
        }
    }
}

struct TerminalRestore;

impl Drop for TerminalRestore {
    fn drop(&mut self) {
        crossterm::terminal::disable_raw_mode().ok();
        crossterm::execute!(io::stdout(), crossterm::terminal::LeaveAlternateScreen).ok();
    }
}

pub(crate) struct Screen {
    terminal: ratatui::DefaultTerminal,
    _restore: TerminalRestore,
    log_scroll: usize,
    log_page: usize,
}

impl Screen {
    pub fn new() -> io::Result<Self> {
        crossterm::terminal::enable_raw_mode()?;
        let restore = TerminalRestore;

        crossterm::execute!(io::stdout(), crossterm::terminal::EnterAlternateScreen)?;
        let terminal =
            ratatui::Terminal::new(ratatui::backend::CrosstermBackend::new(io::stdout()))?;

        Ok(Self {
            terminal,
            _restore: restore,
            log_scroll: 0,
            log_page: 1,
        })
    }

    pub fn poll(&mut self) -> io::Result<Option<KeyEvent>> {
        while event::poll(Duration::ZERO)? {
            match event::read()? {
                Event::Key(key)
                    if matches!(key.kind, KeyEventKind::Press | KeyEventKind::Repeat) =>
                {
                    match key.code {
                        KeyCode::PageUp => {
                            self.log_scroll = self.log_scroll.saturating_add(self.log_page)
                        }
                        KeyCode::PageDown => {
                            self.log_scroll = self.log_scroll.saturating_sub(self.log_page)
                        }
                        KeyCode::End => self.log_scroll = 0,
                        _ => return Ok(Some(key)),
                    }
                }
                _ => {}
            }
        }

        Ok(None)
    }

    pub fn draw(
        &mut self,
        title: &str,
        logs: &LogHistory,
        render_ui: impl FnOnce(Rect, &mut Buffer),
    ) -> io::Result<()> {
        let scroll = self.log_scroll;
        let mut page = self.log_page;

        self.terminal.draw(|frame| {
            let area = frame.area();
            let upper = area.height.saturating_mul(2) / 3;
            let ui = Rect::new(area.x, area.y, area.width, upper);
            let logging = Rect::new(area.x, area.y + upper, area.width, area.height - upper);

            let buffer = frame.buffer_mut();
            let ui = panel(buffer, ui, title);
            let logging = panel(
                buffer,
                logging,
                "Tracing logs  PgUp/PgDn scroll  End follow",
            );
            page = usize::from(logging.height).max(1);

            render_ui(ui, buffer);
            render_logs(buffer, logging, logs, scroll);
        })?;

        self.log_page = page;
        Ok(())
    }
}

fn panel(buffer: &mut Buffer, area: Rect, title: &str) -> Rect {
    if area.width < 2 || area.height < 2 {
        return Rect::new(area.x, area.y, 0, 0);
    }

    let style = Style::default().fg(Color::DarkGray);
    for x in area.x + 1..area.right() - 1 {
        buffer[(x, area.y)].set_symbol("─").set_style(style);
        buffer[(x, area.bottom() - 1)]
            .set_symbol("─")
            .set_style(style);
    }

    for y in area.y + 1..area.bottom() - 1 {
        buffer[(area.x, y)].set_symbol("│").set_style(style);
        buffer[(area.right() - 1, y)]
            .set_symbol("│")
            .set_style(style);
    }

    for (x, y, symbol) in [
        (area.x, area.y, "┌"),
        (area.right() - 1, area.y, "┐"),
        (area.x, area.bottom() - 1, "└"),
        (area.right() - 1, area.bottom() - 1, "┘"),
    ] {
        buffer[(x, y)].set_symbol(symbol).set_style(style);
    }

    buffer.set_stringn(
        area.x + 2,
        area.y,
        title,
        usize::from(area.width.saturating_sub(4)),
        Style::default().fg(Color::Cyan),
    );

    Rect::new(area.x + 1, area.y + 1, area.width - 2, area.height - 2)
}

pub(crate) fn render_logs(buffer: &mut Buffer, area: Rect, history: &LogHistory, scroll: usize) {
    if area.is_empty() {
        return;
    }

    let mut row = 0usize;
    let mut column = 0usize;
    for character in LogCharacters::new(history) {
        if character == '\n' {
            row += 1;
            column = 0;
            continue;
        }

        let mut encoded = [0; 4];
        let width = Span::raw(&*character.encode_utf8(&mut encoded)).width();
        if column + width > usize::from(area.width) {
            row += 1;
            column = 0;
        }
        column += width;
    }

    let total = row + usize::from(column != 0);
    let hidden = total.saturating_sub(usize::from(area.height));
    let top = hidden.saturating_sub(scroll.min(hidden));
    let bottom = (top + usize::from(area.height)).min(total);

    row = 0;
    column = 0;
    for character in LogCharacters::new(history) {
        if character == '\n' {
            row += 1;
            column = 0;
            continue;
        }

        let mut encoded = [0; 4];
        let text = character.encode_utf8(&mut encoded);
        let width = Span::raw(&*text).width();
        if column + width > usize::from(area.width) {
            row += 1;
            column = 0;
        }

        if row >= top && row < bottom && width != 0 && width <= usize::from(area.width) {
            buffer.set_stringn(
                area.x + column as u16,
                area.y + (row - top) as u16,
                &*text,
                width,
                Style::default(),
            );
        }
        column += width;
    }
}

pub(crate) struct RowWriter<'a> {
    buffer: &'a mut Buffer,
    area: Rect,
    column: u16,
    style: Style,
}

impl<'a> RowWriter<'a> {
    pub fn new(buffer: &'a mut Buffer, area: Rect, style: Style) -> Self {
        Self {
            buffer,
            area,
            column: area.x,
            style,
        }
    }

    pub fn write(&mut self, arguments: fmt::Arguments<'_>) {
        self.write_fmt(arguments).ok();
    }
}

impl fmt::Write for RowWriter<'_> {
    fn write_str(&mut self, text: &str) -> fmt::Result {
        if self.area.is_empty() || self.column >= self.area.right() {
            return Ok(());
        }

        self.column = self
            .buffer
            .set_stringn(
                self.column,
                self.area.y,
                text,
                usize::from(self.area.right() - self.column),
                self.style,
            )
            .0;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn log_pane_strips_split_ansi_and_retains_utf8() {
        let mut logs = LogHistory::default();
        logs.push(b"\x1b[3".to_vec());
        logs.push(b"2mINFO\x1b[0m ready\nwide ".to_vec());
        logs.push(b"\xe7\x95".to_vec());
        logs.push(b"\x8c\n".to_vec());

        let area = Rect::new(0, 0, 20, 3);
        let mut buffer = Buffer::empty(area);
        render_logs(&mut buffer, area, &logs, 0);

        assert_eq!(buffer[(0, 0)].symbol(), "I");
        assert_eq!(buffer[(5, 0)].symbol(), "r");
        assert_eq!(buffer[(5, 1)].symbol(), "\u{754c}");
    }

    #[test]
    fn formatted_messages_reuse_retained_chunk_capacity() {
        let mut logs = LogHistory::default();
        logs.push_formatted(format_args!("first {}", 1));
        let allocation = logs.chunks[0].as_ptr();

        for _ in 1..LOG_CHUNKS {
            logs.push_formatted(format_args!("other"));
        }
        logs.push_formatted(format_args!("next {}", 2));

        assert_eq!(logs.chunks[0].as_ptr(), allocation);

        let area = Rect::new(0, 0, 10, 1);
        let mut buffer = Buffer::empty(area);
        render_logs(&mut buffer, area, &logs, 0);

        assert_eq!(buffer[(0, 0)].symbol(), "n");
        assert_eq!(buffer[(5, 0)].symbol(), "2");
    }

    #[test]
    fn log_scrollback_clamps_to_a_full_oldest_page() {
        let mut logs = LogHistory::default();
        logs.push(b"first\nsecond\nthird\n".to_vec());

        let area = Rect::new(0, 0, 10, 2);
        let mut buffer = Buffer::empty(area);
        render_logs(&mut buffer, area, &logs, usize::MAX);

        assert_eq!(buffer[(0, 0)].symbol(), "f");
        assert_eq!(buffer[(0, 1)].symbol(), "s");
    }

    #[test]
    fn small_panels_keep_every_write_within_the_buffer() {
        for width in 0..4 {
            for height in 0..4 {
                let area = Rect::new(0, 0, width, height);
                let mut buffer = Buffer::empty(area);
                let inner = panel(&mut buffer, area, "title");
                render_logs(&mut buffer, inner, &LogHistory::default(), 0);
            }
        }
    }

    #[test]
    fn bounded_log_history_keeps_the_latest_chunks() {
        let mut logs = LogHistory::default();
        for _ in 0..LOG_CHUNKS {
            logs.push(b"old\n".to_vec());
        }
        logs.push(b"new\n".to_vec());

        assert_eq!(logs.length, LOG_CHUNKS);

        let area = Rect::new(0, 0, 10, 1);
        let mut buffer = Buffer::empty(area);
        render_logs(&mut buffer, area, &logs, 0);

        assert_eq!(buffer[(0, 0)].symbol(), "n");
    }
}
