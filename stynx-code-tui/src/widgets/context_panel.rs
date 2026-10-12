use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Paragraph, Widget, Wrap},
};

use crate::state::{AppState, DiffLineKind, ToolUseStatus};
use crate::theme;

pub struct FileChange {
    pub path: String,
    pub added: usize,
    pub removed: usize,
}

const MAX_FILES: usize = 8;

pub fn modified_files(state: &AppState) -> Vec<FileChange> {
    let mut files: Vec<FileChange> = Vec::new();
    for message in &state.conversation.messages {
        for tool in &message.tool_uses {
            if !matches!(tool.name.as_str(), "file_edit" | "file_write") || tool.status != ToolUseStatus::Completed {
                continue;
            }
            let Some(path) = tool_path(&tool.input_json) else { continue };
            let (mut added, mut removed) = tool.diff.iter().fold((0, 0), |(a, r), line| match line.kind {
                DiffLineKind::Added => (a + 1, r),
                DiffLineKind::Removed => (a, r + 1),
                DiffLineKind::Context => (a, r),
            });
            if tool.diff.is_empty() && tool.name == "file_write" {
                added = content_lines(&tool.input_json);
                removed = 0;
            }
            match files.iter_mut().find(|file| file.path == path) {
                Some(file) => {
                    file.added += added;
                    file.removed += removed;
                }
                None => files.push(FileChange { path, added, removed }),
            }
        }
    }
    files
}

fn tool_path(input: &str) -> Option<String> {
    let value: serde_json::Value = serde_json::from_str(input).ok()?;
    value.get("file_path").or_else(|| value.get("path"))?.as_str().map(str::to_string)
}

fn content_lines(input: &str) -> usize {
    serde_json::from_str::<serde_json::Value>(input)
        .ok()
        .and_then(|value| value.get("content").and_then(|c| c.as_str()).map(|c| c.lines().count()))
        .unwrap_or(0)
}

fn short_path(path: &str, cwd: &str, max: usize) -> String {
    let relative = path.strip_prefix(cwd).map(|p| p.trim_start_matches('/')).unwrap_or(path);
    let chars: Vec<char> = relative.chars().collect();
    if chars.len() <= max {
        return relative.to_string();
    }
    let keep = max.saturating_sub(1);
    format!("…{}", chars[chars.len() - keep..].iter().collect::<String>())
}

fn compact_tokens(n: u64) -> String {
    match n {
        0..=999 => n.to_string(),
        1_000..=999_999 => format!("{:.1}k", n as f64 / 1_000.0),
        _ => format!("{:.2}M", n as f64 / 1_000_000.0),
    }
}

fn header(title: &str) -> Line<'static> {
    Line::from(Span::styled(
        format!("  {title}"),
        Style::default().fg(theme::TEXT()).add_modifier(Modifier::BOLD),
    ))
}

pub fn panel_lines(state: &AppState, width: usize) -> Vec<Line<'static>> {
    let dim = Style::default().fg(theme::TEXT_MUTED());
    let mut lines = vec![Line::from("")];

    let title = if state.sidebar.title.trim().is_empty() {
        state.cwd.trim_end_matches('/').rsplit('/').next().unwrap_or("stynx").to_string()
    } else {
        state.sidebar.title.clone()
    };
    lines.push(Line::from(Span::styled(
        format!("  {title}"),
        Style::default().fg(theme::TEXT()).add_modifier(Modifier::BOLD),
    )));
    lines.push(Line::from(""));

    lines.push(header("Context"));
    let used = state.last_input_tokens;
    let window = state.context_window.max(1);
    let percent = (used as f64 / window as f64 * 100.0).min(100.0);
    lines.push(Line::from(Span::styled(format!("  {} tokens", compact_tokens(used)), dim)));
    let percent_color = match percent {
        p if p >= 85.0 => theme::ERROR(),
        p if p >= 60.0 => theme::WARNING(),
        _ => theme::TEXT_MUTED(),
    };
    lines.push(Line::from(Span::styled(
        format!("  {percent:.0}% of {} used", compact_tokens(window)),
        Style::default().fg(percent_color),
    )));
    lines.push(Line::from(Span::styled(format!("  ${:.2} spent", state.total_cost), dim)));
    lines.push(Line::from(""));

    lines.push(header("Modified Files"));
    let files = modified_files(state);
    if files.is_empty() {
        lines.push(Line::from(Span::styled("  No changes yet", dim)));
    }
    for file in files.iter().rev().take(MAX_FILES) {
        let stats = format!("+{} -{}", file.added, file.removed).len();
        let room = width.saturating_sub(stats + 4).max(8);
        let mut spans = vec![Span::styled(
            format!("  {} ", short_path(&file.path, &state.cwd, room)),
            Style::default().fg(theme::TEXT()),
        )];
        if file.added > 0 {
            spans.push(Span::styled(format!("+{} ", file.added), Style::default().fg(theme::SUCCESS())));
        }
        if file.removed > 0 {
            spans.push(Span::styled(format!("-{}", file.removed), Style::default().fg(theme::ERROR())));
        }
        lines.push(Line::from(spans));
    }
    if files.len() > MAX_FILES {
        lines.push(Line::from(Span::styled(format!("  +{} more", files.len() - MAX_FILES), dim)));
    }
    lines
}

pub struct ContextPanel<'a> {
    pub state: &'a AppState,
}

impl<'a> Widget for ContextPanel<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        buf.set_style(area, Style::default().bg(theme::BACKGROUND_PANEL()));
        Paragraph::new(panel_lines(self.state, usize::from(area.width)))
            .wrap(Wrap { trim: false })
            .style(Style::default().bg(theme::BACKGROUND_PANEL()))
            .render(area, buf);
    }
}
