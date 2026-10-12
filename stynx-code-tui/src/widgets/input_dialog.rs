use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Clear, Paragraph, Widget, Wrap},
};

use crate::theme;

pub struct InputDialog<'a> {
    pub title: &'a str,
    pub prompt: &'a str,
    pub buffer: &'a str,
}

impl<'a> InputDialog<'a> {
    pub fn new(title: &'a str, prompt: &'a str, buffer: &'a str) -> Self {
        Self { title, prompt, buffer }
    }
}

impl<'a> Widget for InputDialog<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let w = 76.min(area.width.saturating_sub(4));
        let lines = self.lines();
        let content = Paragraph::new(lines.clone()).wrap(Wrap { trim: false }).line_count(w.saturating_sub(2)) as u16;
        let h = (content + 2).min(area.height.saturating_sub(2));
        let x = area.x + (area.width.saturating_sub(w)) / 2;
        let y = area.y + (area.height.saturating_sub(h)) / 2;
        let rect = Rect { x, y, width: w, height: h };
        Clear.render(rect, buf);

        let block = Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(theme::BORDER_ACTIVE()))
            .style(Style::default().bg(theme::BACKGROUND_PANEL()))
            .title(Span::styled(
                format!(" {} ", self.title),
                Style::default().fg(theme::TEXT()).add_modifier(Modifier::BOLD),
            ));
        let inner = block.inner(rect);
        block.render(rect, buf);

        Paragraph::new(lines)
            .wrap(Wrap { trim: false })
            .style(Style::default().bg(theme::BACKGROUND_PANEL()))
            .render(inner, buf);
    }
}

impl<'a> InputDialog<'a> {
    fn lines(&self) -> Vec<Line<'static>> {
        let mut lines = vec![
            Line::from(""),
        ];
        lines.extend(self.prompt.lines().map(|line| {
            Line::from(Span::styled(
                format!("  {line}"),
                Style::default().fg(theme::TEXT()).bg(theme::BACKGROUND_PANEL()),
            ))
        }));
        lines.extend(vec![
            Line::from(""),
            Line::from(vec![
                Span::styled(
                    "  › ",
                    Style::default().fg(theme::ACCENT()).bg(theme::BACKGROUND_PANEL()),
                ),
                Span::styled(
                    self.buffer.to_string(),
                    Style::default()
                        .fg(theme::TEXT())
                        .bg(theme::BACKGROUND_PANEL())
                        .add_modifier(Modifier::BOLD),
                ),
                Span::styled(
                    "▏",
                    Style::default().fg(theme::ACCENT()).bg(theme::BACKGROUND_PANEL()),
                ),
            ]),
            Line::from(""),
            Line::from(Span::styled(
                "  enter  confirm   esc  cancel ".to_string(),
                Style::default().fg(theme::TEXT_MUTED()).bg(theme::BACKGROUND_PANEL()),
            )),
        ]);
        lines
    }
}
