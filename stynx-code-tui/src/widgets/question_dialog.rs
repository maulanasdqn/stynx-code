use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Clear, Paragraph, Widget, Wrap},
};

use crate::state::QaState;
use crate::theme;

pub struct QuestionDialog<'a> {
    pub qa: &'a QaState,
}

impl<'a> QuestionDialog<'a> {
    fn lines(&self) -> Vec<Line<'static>> {
        let qa = self.qa;
        let q = qa.current();
        let panel = theme::BACKGROUND_PANEL();
        let dim = Style::default().fg(theme::TEXT_MUTED()).bg(panel);
        let mut lines = vec![Line::from("")];

        if qa.questions.len() > 1 {
            let mut tabs = vec![Span::styled("  ", dim)];
            for (i, question) in qa.questions.iter().enumerate() {
                let mark = if qa.is_answered(i) { "✓ " } else { "" };
                let style = if i == qa.index {
                    Style::default().fg(theme::BACKGROUND()).bg(theme::PRIMARY()).add_modifier(Modifier::BOLD)
                } else if qa.is_answered(i) {
                    Style::default().fg(theme::SUCCESS()).bg(panel)
                } else {
                    dim
                };
                tabs.push(Span::styled(format!(" {mark}{} ", question.header), style));
                tabs.push(Span::styled(" ", dim));
            }
            lines.push(Line::from(tabs));
            lines.push(Line::from(""));
        }

        lines.push(Line::from(Span::styled(
            format!("  {}", q.question),
            Style::default().fg(theme::TEXT()).bg(panel).add_modifier(Modifier::BOLD),
        )));
        if q.multi_select {
            lines.push(Line::from(Span::styled("  Select all that apply", dim)));
        }
        lines.push(Line::from(""));

        let picks = &qa.picks[qa.index];
        let rows = q.options.iter().map(|o| (o.label.clone(), o.description.clone())).chain(std::iter::once((
            "Other".to_string(),
            None,
        )));
        for (i, (label, description)) in rows.enumerate() {
            let focused = i == qa.cursor;
            let picked = picks.contains(&i);
            let pointer = if focused { "› " } else { "  " };
            let mark = match (q.multi_select, picked) {
                (true, true) => "[x] ",
                (true, false) => "[ ] ",
                (false, true) => "● ",
                (false, false) => "○ ",
            };
            let number = if i < q.options.len() { format!("{}. ", i + 1) } else { "   ".to_string() };
            let label_style = if focused {
                Style::default().fg(theme::PRIMARY()).bg(panel).add_modifier(Modifier::BOLD)
            } else if picked {
                Style::default().fg(theme::TEXT()).bg(panel).add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(theme::TEXT()).bg(panel)
            };
            let mark_style = if picked { Style::default().fg(theme::PRIMARY()).bg(panel) } else { dim };
            let mut spans = vec![
                Span::styled(format!("  {pointer}"), Style::default().fg(theme::PRIMARY()).bg(panel)),
                Span::styled(mark, mark_style),
                Span::styled(number, dim),
                Span::styled(label, label_style),
            ];
            if i == q.options.len() && (focused || picked) {
                spans.push(Span::styled(": ", dim));
                spans.push(Span::styled(qa.other[qa.index].clone(), Style::default().fg(theme::TEXT()).bg(panel)));
                if focused {
                    spans.push(Span::styled("▏", Style::default().fg(theme::PRIMARY()).bg(panel)));
                }
            }
            lines.push(Line::from(spans));
            if let Some(description) = description {
                lines.push(Line::from(Span::styled(format!("           {description}"), dim)));
            }
        }

        lines.push(Line::from(""));
        let key = Style::default().fg(theme::TEXT()).bg(panel);
        let mut hints = vec![
            Span::styled("  ↑↓ ", key),
            Span::styled("move  ", dim),
            Span::styled("enter ", key),
            Span::styled("confirm  ", dim),
        ];
        if q.multi_select {
            hints.push(Span::styled("space ", key));
            hints.push(Span::styled("toggle  ", dim));
        }
        if qa.questions.len() > 1 {
            hints.push(Span::styled("←→ ", key));
            hints.push(Span::styled("question  ", dim));
        }
        hints.push(Span::styled("esc ", key));
        hints.push(Span::styled("cancel", dim));
        lines.push(Line::from(hints));
        lines
    }
}

impl<'a> Widget for QuestionDialog<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let width = 76.min(area.width.saturating_sub(4));
        let lines = self.lines();
        let inner_width = width.saturating_sub(2);
        let content_height = Paragraph::new(lines.clone()).wrap(Wrap { trim: false }).line_count(inner_width) as u16;
        let height = (content_height + 3).min(area.height.saturating_sub(2));
        let x = area.x + area.width.saturating_sub(width) / 2;
        let y = area.y + area.height.saturating_sub(height) / 2;
        let rect = Rect { x, y, width, height };
        Clear.render(rect, buf);

        let header = &self.qa.current().header;
        let block = Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(theme::BORDER_ACTIVE()))
            .style(Style::default().bg(theme::BACKGROUND_PANEL()))
            .title(Span::styled(
                format!(" {header} "),
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
