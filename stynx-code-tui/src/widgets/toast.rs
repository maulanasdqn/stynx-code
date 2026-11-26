use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Clear, Paragraph, Widget},
};

use crate::state::{ToastKind, ToastState};
use crate::theme;

pub struct ToastStack<'a> {
    pub state: &'a ToastState,
}

impl<'a> ToastStack<'a> {
    pub fn new(state: &'a ToastState) -> Self {
        Self { state }
    }
}

impl<'a> Widget for ToastStack<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        if self.state.items.is_empty() {
            return;
        }

        // Solid notify cards with a level-colored left bar, stacked top-right.
        let max_box_w = 48u16.min(area.width.saturating_sub(2));
        if max_box_w < 14 {
            return;
        }
        let bg = theme::BACKGROUND_ELEMENT();
        let right_edge = area.x + area.width.saturating_sub(1);
        let mut y = area.y + 1;

        for toast in &self.state.items {
            let (icon, color) = match toast.kind {
                ToastKind::Info => ("\u{24D8}", theme::ACCENT()),    // ⓘ
                ToastKind::Success => ("\u{2713}", theme::SUCCESS()), // ✓
                ToastKind::Warning => ("\u{25B3}", theme::WARNING()), // △
                ToastKind::Error => ("\u{2717}", theme::ERROR()),     // ✗
            };

            // A "title\nbody" message renders the first line bold as a heading.
            let (title, body) = match toast.message.split_once('\n') {
                Some((t, b)) => (t.trim(), b.trim()),
                None => (toast.message.as_str(), ""),
            };

            // Around the text we reserve: bar + " " + 2-col icon gutter + "  ".
            let text_w = (max_box_w as usize).saturating_sub(6).max(1);
            let mut wrapped: Vec<(String, bool)> = Vec::new(); // (text, is_title)
            for line in wrap_plain(title, text_w) {
                wrapped.push((line, true));
            }
            if !body.is_empty() {
                for line in wrap_plain(body, text_w) {
                    wrapped.push((line, false));
                }
            }

            let longest = wrapped.iter().map(|(l, _)| l.chars().count()).max().unwrap_or(0);
            let box_w = (longest + 6) as u16; // bar + " " + 2-col icon gutter + text + "  "
            let box_h = wrapped.len() as u16 + 2; // padding row above and below

            if y + box_h > area.y + area.height {
                break;
            }

            let x = right_edge.saturating_sub(box_w);
            let fill = Style::default().bg(bg);
            let bar = Span::styled("\u{258C}".to_string(), Style::default().fg(color).bg(bg));

            for row_i in 0..box_h {
                let row = Rect { x, y: y + row_i, width: box_w, height: 1 };
                Clear.render(row, buf);
                let mut spans = vec![bar.clone(), Span::styled(" ".to_string(), fill)];
                let content = row_i.checked_sub(1).and_then(|i| wrapped.get(i as usize));
                if let Some((text, is_title)) = content {
                    let gutter = if row_i == 1 {
                        Span::styled(format!("{icon} "), Style::default().fg(color).bg(bg).add_modifier(Modifier::BOLD))
                    } else {
                        Span::styled("  ".to_string(), fill)
                    };
                    let text_style = if *is_title {
                        Style::default().fg(theme::TEXT()).bg(bg).add_modifier(Modifier::BOLD)
                    } else {
                        Style::default().fg(theme::SUBTLE()).bg(bg)
                    };
                    spans.push(gutter);
                    spans.push(Span::styled(text.clone(), text_style));
                }
                Paragraph::new(Line::from(spans)).style(fill).render(row, buf);
            }

            y += box_h + 1; // one-row gap between cards
        }
    }
}

/// Greedy word-wrap a plain string to `width` columns, hard-splitting any word
/// longer than the line.
pub(crate) fn wrap_plain(s: &str, width: usize) -> Vec<String> {
    let width = width.max(1);
    let mut lines: Vec<String> = Vec::new();
    let mut cur = String::new();

    for word in s.split_whitespace() {
        if cur.is_empty() {
            if word.chars().count() > width {
                let mut chars: Vec<char> = word.chars().collect();
                while chars.len() > width {
                    lines.push(chars[..width].iter().collect());
                    chars.drain(..width);
                }
                cur = chars.into_iter().collect();
            } else {
                cur = word.to_string();
            }
        } else if cur.chars().count() + 1 + word.chars().count() <= width {
            cur.push(' ');
            cur.push_str(word);
        } else {
            lines.push(std::mem::take(&mut cur));
            cur = word.to_string();
        }
    }
    if !cur.is_empty() || lines.is_empty() {
        lines.push(cur);
    }
    lines
}
