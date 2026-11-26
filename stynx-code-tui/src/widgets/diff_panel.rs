use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Widget},
};

use crate::state::{AppState, DiffLineKind, ToolUseStatus};
use crate::theme;

/// Side panel that shows the diff of every file edit made in this session,
/// newest first.
pub struct DiffPanel<'a> {
    pub state: &'a mut AppState,
}

impl<'a> DiffPanel<'a> {
    pub fn new(state: &'a mut AppState) -> Self {
        Self { state }
    }
}

fn tool_path(input: &str) -> Option<String> {
    let value: serde_json::Value = serde_json::from_str(input).ok()?;
    value.get("file_path").or_else(|| value.get("path"))?.as_str().map(str::to_string)
}

/// Cut `text` to `width` columns, marking the cut with an ellipsis.
fn clip(text: &str, width: usize) -> String {
    let text = text.replace('\t', "    ");
    if text.chars().count() <= width {
        return text;
    }
    let mut out: String = text.chars().take(width.saturating_sub(1)).collect();
    out.push('…');
    out
}

/// Keep the end of a path, which holds the file name.
fn clip_path(path: &str, width: usize) -> String {
    let chars: Vec<char> = path.chars().collect();
    if chars.len() <= width {
        return path.to_string();
    }
    let keep = width.saturating_sub(1);
    format!("…{}", chars[chars.len() - keep..].iter().collect::<String>())
}

fn diff_lines(state: &AppState, width: usize) -> (Vec<Line<'static>>, usize) {
    let mut lines: Vec<Line<'static>> = Vec::new();
    let mut edits = 0;
    let text_w = width.saturating_sub(4).max(1);

    let tools = state
        .conversation
        .messages
        .iter()
        .flat_map(|message| message.tool_uses.iter())
        .filter(|tool| matches!(tool.name.as_str(), "file_edit" | "file_write"))
        .filter(|tool| tool.status == ToolUseStatus::Completed && !tool.diff.is_empty());

    for tool in tools.collect::<Vec<_>>().into_iter().rev() {
        let path = tool_path(&tool.input_json).unwrap_or_else(|| tool.name.clone());
        let path = path.strip_prefix(&state.cwd).map(|p| p.trim_start_matches('/').to_string()).unwrap_or(path);
        let (added, removed) = tool.diff.iter().fold((0, 0), |(a, r), line| match line.kind {
            DiffLineKind::Added => (a + 1, r),
            DiffLineKind::Removed => (a, r + 1),
            DiffLineKind::Context => (a, r),
        });
        let stats = format!(" +{added} -{removed}");

        if edits > 0 {
            lines.push(Line::from(""));
        }
        edits += 1;
        lines.push(Line::from(vec![
            Span::styled(
                format!("  {}", clip_path(&path, width.saturating_sub(stats.len() + 2).max(8))),
                Style::default().fg(theme::TEXT()).add_modifier(Modifier::BOLD),
            ),
            Span::styled(format!(" +{added}"), Style::default().fg(theme::SUCCESS())),
            Span::styled(format!(" -{removed}"), Style::default().fg(theme::ERROR())),
        ]));

        for line in &tool.diff {
            let (sign, sign_color, text_color) = match line.kind {
                DiffLineKind::Added => ("+", theme::SUCCESS(), theme::TEXT()),
                DiffLineKind::Removed => ("-", theme::ERROR(), theme::TEXT_MUTED()),
                DiffLineKind::Context => (" ", theme::TEXT_MUTED(), theme::TEXT_MUTED()),
            };
            lines.push(Line::from(vec![
                Span::styled(format!("  {sign} "), Style::default().fg(sign_color)),
                Span::styled(clip(&line.text, text_w), Style::default().fg(text_color)),
            ]));
        }
    }
    (lines, edits)
}

impl<'a> Widget for DiffPanel<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        buf.set_style(area, Style::default().bg(theme::BACKGROUND_PANEL()));
        let block = Block::default()
            .borders(Borders::TOP)
            .border_style(Style::default().fg(theme::BORDER()));
        let inner = block.inner(area);
        block.render(area, buf);

        let (body, edits) = diff_lines(self.state, inner.width as usize);
        let mut header_spans = vec![Span::styled(
            "  Diff",
            Style::default().fg(theme::TEXT_MUTED()).add_modifier(Modifier::BOLD),
        )];
        if edits > 0 {
            header_spans.push(Span::styled(format!("  {edits}"), Style::default().fg(theme::TEXT_MUTED())));
        }
        if self.state.diff_panel.focused {
            header_spans.push(Span::styled("  ●", Style::default().fg(theme::PRIMARY())));
        }
        let mut lines: Vec<Line<'static>> = vec![Line::from(header_spans), Line::from("")];

        if body.is_empty() {
            lines.push(Line::from(Span::styled("  No edits yet", Style::default().fg(theme::TEXT_MUTED()))));
            Paragraph::new(lines).render(inner, buf);
            return;
        }

        let visible = (inner.height as usize).saturating_sub(3).max(1);
        let total = body.len();
        let max_scroll = total.saturating_sub(visible);
        let scroll = self.state.diff_panel.scroll.min(max_scroll);
        self.state.diff_panel.scroll = scroll;

        lines.extend(body.into_iter().skip(scroll).take(visible));
        if total > visible {
            lines.push(Line::from(Span::styled(
                format!("  {} / {} lines", (scroll + visible).min(total), total),
                Style::default().fg(theme::SUBTLE()).add_modifier(Modifier::DIM),
            )));
        }
        Paragraph::new(lines).render(inner, buf);
    }
}
