use ratatui::{
    style::{Color, Modifier, Style},
    text::{Line, Span},
};

use crate::state::DisplayMessage;
use crate::theme;
use super::markdown::{render_md_line_wrapped, is_table_line, render_table_block};

pub fn render_message(msg: &DisplayMessage, width: usize, spin: char, thinking_active: bool) -> Vec<Line<'static>> {
    let mut lines: Vec<Line<'static>> = Vec::new();
    let bar = "▌";
    let body_indent = "  ";
    match msg.role.as_str() {
        "user" => {
            let start = lines.len();
            for raw in msg.content.lines() {
                lines.push(Line::from(Span::styled(
                    format!("{body_indent}{}", raw.trim_end()),
                    Style::default().fg(theme::TEXT()),
                )));
            }
            stamp_role_bar(&mut lines, start, theme::PRIMARY());
        }
        "error" => {
            lines.push(Line::from(vec![
                Span::styled(bar, Style::default().fg(theme::LOVE()).add_modifier(Modifier::BOLD)),
                Span::styled(" Error", Style::default().fg(theme::LOVE()).add_modifier(Modifier::BOLD)),
            ]));
            for raw in msg.content.lines() {
                lines.push(Line::from(Span::styled(
                    format!("{body_indent}{}", raw.trim_end()),
                    Style::default().fg(theme::LOVE()),
                )));
            }
        }
        "system" => {
            for raw in msg.content.lines() {
                lines.push(Line::from(vec![
                    Span::styled(bar, Style::default().fg(theme::SUBTLE())),
                    Span::styled(
                        format!(" {}", raw.trim_end()),
                        Style::default().fg(theme::MUTED()).add_modifier(Modifier::ITALIC),
                    ),
                ]));
            }
        }
        "done" => {
            let parts: Vec<&str> = msg.content.split(", ").collect();
            for (i, part) in parts.iter().enumerate() {
                let prefix = if i == 0 { "  ✓ " } else { "    " };
                let prefix_style = if i == 0 {
                    Style::default().fg(theme::SUCCESS()).add_modifier(Modifier::BOLD)
                } else {
                    Style::default()
                };
                lines.push(Line::from(vec![
                    Span::styled(prefix, prefix_style),
                    Span::styled(
                        part.to_string(),
                        Style::default().fg(theme::MUTED()).add_modifier(Modifier::ITALIC),
                    ),
                ]));
            }
        }
        _ => {
            let asst_start = lines.len();
            if msg.is_streaming && thinking_active && msg.content.trim().is_empty() {
                lines.push(Line::from(vec![
                    Span::styled(
                        format!("  {spin} "),
                        Style::default().fg(theme::TEXT()).add_modifier(Modifier::BOLD),
                    ),
                    Span::styled(
                        "thinking…",
                        Style::default()
                            .fg(theme::MUTED())
                            .add_modifier(Modifier::ITALIC),
                    ),
                ]));
                lines.push(Line::from(""));
            }
            if !msg.thinking.is_empty() && !msg.is_streaming {
                let lc = msg.thinking.lines().count();
                lines.push(Line::from(Span::styled(
                    format!("  ◈ thinking · {lc} lines"),
                    Style::default()
                        .fg(theme::MUTED())
                        .add_modifier(Modifier::ITALIC | Modifier::DIM),
                )));
                lines.push(Line::from(""));
            }
            let content_lines: Vec<&str> = msg.content.lines().collect();
            let mut in_code = false;
            let mut in_mermaid = false;
            let mut prev_blank = false;
            let mut i = 0;
            while i < content_lines.len() {
                let raw = content_lines[i];
                let trimmed_start = raw.trim_start();
                let is_blank = raw.trim().is_empty();

                if is_blank {
                    if !prev_blank { lines.push(Line::from("")); }
                    prev_blank = true;
                    i += 1;
                    continue;
                }
                prev_blank = false;

                if trimmed_start.starts_with("```mermaid") {
                    in_mermaid = true; in_code = true;
                    lines.push(Line::from(Span::styled("  ╭─ mermaid ", Style::default().fg(theme::IRIS()).add_modifier(Modifier::BOLD))));
                } else if in_mermaid && trimmed_start.starts_with("```") {
                    in_mermaid = false; in_code = false;
                    lines.push(Line::from(Span::styled("  ╰────────── ", Style::default().fg(theme::IRIS()))));
                } else if in_mermaid {
                    lines.push(Line::from(vec![
                        Span::styled("  │ ", Style::default().fg(theme::IRIS())),
                        Span::styled(raw.trim_end().to_string(), Style::default().fg(theme::GOLD())),
                    ]));
                } else if trimmed_start.starts_with("```") {
                    if in_code {
                        in_code = false;
                        lines.push(Line::from(Span::styled("  ╰──", Style::default().fg(theme::OVERLAY()))));
                    } else {
                        in_code = true;
                        let lang = trimmed_start.trim_start_matches('`').trim();
                        let label = if lang.is_empty() { "  ╭─ code".to_string() } else { format!("  ╭─ {lang}") };
                        lines.push(Line::from(Span::styled(label, Style::default().fg(theme::OVERLAY()).add_modifier(Modifier::DIM))));
                    }
                } else if !in_code && is_table_line(raw) {
                    // Gather the whole contiguous table block and render it
                    // aligned (columns padded to a common width).
                    let start = i;
                    while i < content_lines.len() && is_table_line(content_lines[i]) {
                        i += 1;
                    }
                    lines.extend(render_table_block(&content_lines[start..i], width));
                    continue;
                } else {
                    lines.extend(render_md_line_wrapped(raw, in_code, (width).saturating_sub(1)));
                }
                i += 1;
            }
            let _ = in_code;
            stamp_role_bar(&mut lines, asst_start, theme::HL_HIGH());
        }
    }
    lines.push(Line::from(""));
    lines
}

fn stamp_role_bar(lines: &mut [Line<'static>], start: usize, color: Color) {
    let line = match lines.get_mut(start) {
        Some(l) => l,
        None => return,
    };
    if let Some(first) = line.spans.first_mut() {
        let trimmed = first
            .content
            .strip_prefix("  ")
            .or_else(|| first.content.strip_prefix(' '))
            .map(|s| s.to_string());
        if let Some(t) = trimmed {
            first.content = t.into();
        }
    }
    line.spans.insert(
        0,
        Span::styled("▌ ", Style::default().fg(color).add_modifier(Modifier::BOLD)),
    );
}
