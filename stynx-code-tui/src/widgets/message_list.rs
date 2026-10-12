use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Paragraph, Widget, Wrap},
};

use ratatui::layout::Alignment;

use crate::state::conversation_state::{BlockKey, RenderedBlock};
use crate::state::ConversationState;
use crate::theme;
use super::message_block::render_message;

pub struct MessageList<'a> {
    pub state: &'a mut ConversationState,
    pub spinner_frame: usize,
    pub tool_details: bool,
    pub thinking_active: bool,
}

impl<'a> MessageList<'a> {
    pub fn new(state: &'a mut ConversationState, spinner_frame: usize) -> Self {
        Self { state, spinner_frame, tool_details: true, thinking_active: false }
    }

    pub fn with_tool_details(mut self, on: bool) -> Self {
        self.tool_details = on;
        self
    }

    pub fn with_thinking_active(mut self, on: bool) -> Self {
        self.thinking_active = on;
        self
    }
}

impl<'a> Widget for MessageList<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        // Align the role accent bar (▌) flush to the left edge (main.x), matching
        // the thinking panel's bar and the tools border. Keep a 1-col right margin.
        let area = Rect {
            x: area.x,
            width: area.width.saturating_sub(1),
            y: area.y + 1,
            height: area.height.saturating_sub(1),
        };

        if self.state.messages.is_empty() {
            draw_empty_state(area, buf);
            return;
        }

        let width = area.width;
        let theme_id = theme::current().id;
        let spin = super::spinner::FRAMES[self.spinner_frame % super::spinner::FRAMES.len()];
        let messages = &self.state.messages;
        let cache = &mut self.state.render_cache;
        cache.truncate(messages.len());

        for (i, msg) in messages.iter().enumerate() {
            let live_spinner = msg.is_streaming && self.thinking_active && msg.content.trim().is_empty();
            let key = BlockKey {
                content_len: msg.content.len(),
                thinking_len: msg.thinking.len(),
                streaming: msg.is_streaming,
                width,
                theme: theme_id,
                spin: if live_spinner { spin } else { ' ' },
            };
            if cache.get(i).is_some_and(|block| block.key == key) {
                continue;
            }
            let mut lines = render_message(msg, usize::from(width), spin, self.thinking_active);
            lines.push(Line::from(""));
            let height = Paragraph::new(lines.clone()).wrap(Wrap { trim: false }).line_count(width);
            let block = RenderedBlock { key, lines, height };
            if i < cache.len() {
                cache[i] = block;
            } else {
                cache.push(block);
            }
        }

        let trailing = 2usize;
        let total_rows: usize = cache.iter().map(|block| block.height).sum::<usize>() + trailing;
        let visible = area.height as usize;
        self.state.total_lines = total_rows;

        let offset = if total_rows <= visible {
            self.state.scroll_offset = 0;
            0
        } else {
            let max_offset = total_rows - visible;
            let offset = if self.state.auto_scroll { max_offset } else { self.state.scroll_offset.min(max_offset) };
            self.state.scroll_offset = offset;
            offset
        };

        let mut lines: Vec<Line<'static>> = Vec::new();
        if total_rows < visible {
            lines.resize(visible - total_rows, Line::from(""));
        }
        let mut row = 0usize;
        let mut skip = 0usize;
        let mut started = false;
        for block in cache.iter() {
            let end = row + block.height;
            if end > offset && row < offset + visible {
                if !started {
                    skip = offset - row.min(offset);
                    started = true;
                }
                lines.extend(block.lines.iter().cloned());
            }
            row = end;
            if row >= offset + visible {
                break;
            }
        }
        lines.push(Line::from(""));
        lines.push(Line::from(""));

        Paragraph::new(lines)
            .wrap(Wrap { trim: false })
            .scroll((u16::try_from(skip).unwrap_or(u16::MAX), 0))
            .render(area, buf);
    }
}


const LOGO_ART: &[&str] = &[
    r" ____   _____ __   __ _   _ __  __",
    r"/ ___| |_   _|\ \ / /| \ | |\ \/ /",
    r"\___ \   | |   \ V / |  \| | \  / ",
    r" ___) |  | |    | |  | |\  | /  \ ",
    r"|____/   |_|    |_|  |_| \_|/_/\_\",
];
const LOGO_SUBTITLE: &str = "c   o   d   e";

const HINTS: &[(&str, &str)] = &[
    ("^P",    "command palette"),
    ("^T",    "focus tools"),
    ("^M",    "switch model"),
    ("/help", "show help"),
];

fn draw_empty_state(area: Rect, buf: &mut Buffer) {
    let mut lines: Vec<Line<'static>> = Vec::new();

    let logo_w = LOGO_ART.iter().map(|s| s.chars().count()).max().unwrap_or(0);
    let pad_to = |s: &str, w: usize| -> String {
        let n = s.chars().count();
        if n >= w { s.to_string() } else {
            let extra = w - n;
            let left = extra / 2;
            let right = extra - left;
            format!("{}{}{}", " ".repeat(left), s, " ".repeat(right))
        }
    };

    let top_pad = area.height.saturating_sub(16) / 3;
    for _ in 0..top_pad { lines.push(Line::from("")); }

    for art in LOGO_ART {
        lines.push(Line::from(Span::styled(
            pad_to(art, logo_w),
            Style::default().fg(theme::PRIMARY()).add_modifier(Modifier::BOLD),
        )));
    }
    lines.push(Line::from(Span::styled(
        pad_to(LOGO_SUBTITLE, logo_w),
        Style::default().fg(theme::PRIMARY()),
    )));
    lines.push(Line::from(""));
    lines.push(Line::from(Span::styled(
        format!("v{}", env!("CARGO_PKG_VERSION")),
        Style::default().fg(theme::TEXT_MUTED()),
    )));
    lines.push(Line::from(""));
    lines.push(Line::from(Span::styled(
        "────────────────────".to_string(),
        Style::default().fg(theme::BORDER()),
    )));
    lines.push(Line::from(""));

    let key_w = HINTS.iter().map(|(k, _)| k.chars().count()).max().unwrap_or(0);
    let label_w = HINTS.iter().map(|(_, l)| l.chars().count()).max().unwrap_or(0);
    for (k, l) in HINTS {
        let key = format!("{:>width$}", k, width = key_w);
        let label = format!("{:<width$}", l, width = label_w);
        lines.push(Line::from(vec![
            Span::styled(key, Style::default().fg(theme::PRIMARY()).add_modifier(Modifier::BOLD)),
            Span::raw("   "),
            Span::styled(label, Style::default().fg(theme::TEXT_MUTED())),
        ]));
    }
    lines.push(Line::from(""));
    lines.push(Line::from(Span::styled(
        "type a message to begin…".to_string(),
        Style::default().fg(theme::TEXT_MUTED()).add_modifier(Modifier::ITALIC),
    )));

    Paragraph::new(lines).alignment(Alignment::Center).render(area, buf);
}
