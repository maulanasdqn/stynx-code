use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Paragraph, Widget},
};

use crate::state::InputState;
use crate::theme;
use crate::widgets::input_wrap::wrap;

pub struct InputBox<'a> {
    pub state: &'a InputState,
    pub focused: bool,
}

impl<'a> InputBox<'a> {
    pub fn new(state: &'a InputState, focused: bool) -> Self {
        Self { state, focused }
    }
}

impl<'a> Widget for InputBox<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let border_color = if self.focused { theme::BORDER_ACTIVE() } else { theme::BORDER() };
        let key = Style::default().fg(theme::TEXT());
        let label = Style::default().fg(theme::TEXT_MUTED());
        let hint = Line::from(vec![
            Span::styled(" ↵ ", key),
            Span::styled("send  ", label),
            Span::styled("⇧↵ ", key),
            Span::styled("newline  ", label),
            Span::styled("esc ", key),
            Span::styled("stop ", label),
        ])
        .right_aligned();

        let block = Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(border_color))
            .title_bottom(hint);

        let inner = block.inner(area);

        let text_width = usize::from(inner.width.saturating_sub(2)).max(1);
        let visible_rows = usize::from(inner.height).max(1);
        let text_style = Style::default().fg(theme::TEXT());

        if self.state.buffer.is_empty() {
            let placeholder = if self.state.suggestion.is_empty() {
                " Ask stynx…".to_string()
            } else {
                format!(" {}", self.state.suggestion)
            };
            Paragraph::new(Line::from(Span::styled(placeholder, Style::default().fg(theme::TEXT_MUTED()))))
                .block(block)
                .style(Style::default().bg(theme::BACKGROUND()))
                .render(area, buf);
            if self.focused && inner.width > 1 && inner.height > 0 {
                buf[(inner.x + 1, inner.y)].set_style(Style::default().bg(theme::PRIMARY()).fg(theme::BACKGROUND()));
            }
            return;
        }

        let wrapped = wrap(&self.state.buffer, self.state.cursor_pos, text_width);
        let top = (wrapped.cursor_row + 1).saturating_sub(visible_rows);
        let last_row = wrapped.rows.len().saturating_sub(1);
        let lines: Vec<Line<'static>> = wrapped
            .rows
            .iter()
            .enumerate()
            .skip(top)
            .take(visible_rows)
            .map(|(i, row)| {
                let mut spans = vec![Span::styled(format!(" {row}"), text_style)];
                if i == last_row && !self.state.suggestion.is_empty() {
                    spans.push(Span::styled(
                        self.state.suggestion.clone(),
                        Style::default().fg(theme::MUTED()).add_modifier(Modifier::DIM),
                    ));
                }
                Line::from(spans)
            })
            .collect();

        Paragraph::new(lines)
            .block(block)
            .style(Style::default().bg(theme::BACKGROUND()))
            .render(area, buf);

        if self.focused && inner.width > 0 && inner.height > 0 {
            let row = wrapped.cursor_row - top;
            let cx = inner.x.saturating_add(1).saturating_add(u16::try_from(wrapped.cursor_col).unwrap_or(u16::MAX));
            let cy = inner.y.saturating_add(u16::try_from(row).unwrap_or(u16::MAX));
            let right = inner.x.saturating_add(inner.width);
            let bottom = inner.y.saturating_add(inner.height);
            if cx < right && cy < bottom {
                buf[(cx, cy)].set_style(Style::default().bg(theme::PRIMARY()).fg(theme::BACKGROUND()));
            }
        }
    }
}
