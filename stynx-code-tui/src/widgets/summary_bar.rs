use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Style,
    text::{Line, Span},
    widgets::{Paragraph, Widget},
};

use crate::theme;

/// One quiet line above the input that counts the last turn's tool calls,
/// e.g. "✓ 2 commands · 1 read · 1 edit".
pub struct SummaryBar<'a> {
    /// Tool calls of the last turn as (kind, failed).
    items: &'a [(String, bool)],
}

impl<'a> SummaryBar<'a> {
    pub fn new(items: &'a [(String, bool)]) -> Self {
        Self { items }
    }
}

fn plural(kind: &str, n: usize) -> String {
    if n == 1 {
        return kind.to_string();
    }
    if kind.ends_with("ch") || kind.ends_with('s') {
        format!("{kind}es")
    } else {
        format!("{kind}s")
    }
}

/// Count the calls per kind, keeping the order in which each kind first ran.
fn counts(items: &[(String, bool)]) -> Vec<(&str, usize)> {
    let mut out: Vec<(&str, usize)> = Vec::new();
    for (kind, _) in items {
        match out.iter_mut().find(|(k, _)| *k == kind.as_str()) {
            Some((_, n)) => *n += 1,
            None => out.push((kind.as_str(), 1)),
        }
    }
    out
}

impl<'a> Widget for SummaryBar<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let failed = self.items.iter().filter(|(_, f)| *f).count();
        let muted = Style::default().fg(theme::TEXT_MUTED());
        let (mark, mark_style) = if failed > 0 {
            ("✗", Style::default().fg(theme::ERROR()))
        } else {
            ("✓", Style::default().fg(theme::SUCCESS()))
        };

        let mut spans = vec![Span::styled(format!("   {mark} "), mark_style)];
        for (i, (kind, n)) in counts(self.items).into_iter().enumerate() {
            if i > 0 {
                spans.push(Span::styled(" · ", Style::default().fg(theme::SUBTLE())));
            }
            spans.push(Span::styled(format!("{n} {}", plural(kind, n)), muted));
        }
        if failed > 0 {
            spans.push(Span::styled(" · ", Style::default().fg(theme::SUBTLE())));
            spans.push(Span::styled(format!("{failed} failed"), Style::default().fg(theme::ERROR())));
        }

        Paragraph::new(Line::from(spans))
            .style(Style::default().bg(theme::BACKGROUND()))
            .render(area, buf);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn counts_group_by_kind_in_first_seen_order() {
        let items = vec![
            ("command".to_string(), false),
            ("read".to_string(), false),
            ("command".to_string(), true),
        ];
        assert_eq!(counts(&items), vec![("command", 2), ("read", 1)]);
    }

    #[test]
    fn plurals_read_naturally() {
        assert_eq!(plural("command", 2), "commands");
        assert_eq!(plural("search", 3), "searches");
        assert_eq!(plural("edit", 1), "edit");
        assert_eq!(plural("web search", 2), "web searches");
    }
}
