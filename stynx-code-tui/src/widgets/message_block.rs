//! Chat transcript rendering, adapted from OpenCode's session view: user
//! messages in a raised block with an accent bar, assistant text and tool
//! calls as separate rows at a 3-column indent, bash and edit calls as raised
//! blocks, and a one-line summary under each finished turn.

use ratatui::{
    style::{Color, Modifier, Style},
    text::{Line, Span},
};
use unicode_width::UnicodeWidthStr;

use crate::state::{DiffLineKind, DisplayMessage, DisplayToolUse, MessagePart, ToolUseStatus};
use crate::theme;
use super::markdown::{render_md_line_wrapped, is_table_line, render_table_block};
use super::toast::wrap_plain;

/// Left indent of every assistant-side row.
const INDENT: &str = "   ";
/// Most output lines a collapsed bash block shows; the tail is kept.
const BASH_LINES: usize = 10;
/// Most diff lines an edit block shows.
const DIFF_LINES: usize = 40;

enum Row {
    Text(Vec<Line<'static>>),
    Inline(Vec<Line<'static>>),
    Block(Vec<Line<'static>>),
}

pub fn render_message(
    msg: &DisplayMessage,
    width: usize,
    spin: char,
    thinking_active: bool,
    details: bool,
) -> Vec<Line<'static>> {
    let mut lines: Vec<Line<'static>> = Vec::new();
    match msg.role.as_str() {
        "user" => lines.extend(user_block(&msg.content, width)),
        "error" => {
            for (i, raw) in msg.content.lines().enumerate() {
                let text = if i == 0 { format!("{INDENT}Error: {}", raw.trim_end()) } else { format!("{INDENT}{}", raw.trim_end()) };
                lines.push(Line::from(Span::styled(text, Style::default().fg(theme::ERROR()))));
            }
        }
        "system" => {
            for raw in msg.content.lines() {
                lines.push(Line::from(Span::styled(
                    format!("{INDENT}{}", raw.trim_end()),
                    Style::default().fg(theme::TEXT_MUTED()),
                )));
            }
        }
        _ => lines.extend(assistant_rows(msg, width, spin, thinking_active, details)),
    }
    lines.push(Line::from(""));
    lines
}

fn assistant_rows(msg: &DisplayMessage, width: usize, spin: char, thinking_active: bool, details: bool) -> Vec<Line<'static>> {
    let mut rows: Vec<Row> = Vec::new();

    if msg.is_streaming && thinking_active && msg.content.trim().is_empty() && msg.tool_uses.is_empty() {
        rows.push(Row::Inline(vec![Line::from(vec![
            Span::styled(format!("{INDENT}{spin} "), Style::default().fg(theme::TEXT())),
            Span::styled("Thinking", Style::default().fg(theme::TEXT())),
        ])]));
    }
    if !msg.thinking.is_empty() && !msg.is_streaming {
        let count = msg.thinking.lines().filter(|l| !l.trim().is_empty()).count();
        // A leading **bold** line is the thought's title, as in OpenCode.
        let title = msg.thinking.trim_start().strip_prefix("**")
            .and_then(|rest| rest.split_once("**"))
            .map(|(title, _)| format!(": {}", title.trim()))
            .unwrap_or_default();
        let dim_warning = theme::blend(theme::WARNING(), theme::BACKGROUND(), 0.6);
        rows.push(Row::Inline(vec![Line::from(Span::styled(
            clip(&format!("{INDENT}+ Thought{title} · {count} lines"), width),
            Style::default().fg(dim_warning),
        ))]));
    }

    // Messages restored from a saved session have no part order: text first, then tools.
    let fallback: Vec<MessagePart>;
    let parts = if msg.parts.is_empty() {
        let mut parts = vec![MessagePart::Text(msg.content.clone())];
        parts.extend((0..msg.tool_uses.len()).map(MessagePart::Tool));
        fallback = parts;
        &fallback
    } else {
        &msg.parts
    };

    for part in parts {
        match part {
            MessagePart::Text(text) if !text.trim().is_empty() => rows.push(Row::Text(markdown_lines(text, width))),
            MessagePart::Text(_) => {}
            MessagePart::Tool(index) => {
                if let Some(tool) = msg.tool_uses.get(*index) {
                    rows.push(tool_row(tool, width, spin, details));
                }
            }
        }
    }

    let mut lines: Vec<Line<'static>> = Vec::new();
    let mut previous_inline = false;
    for row in rows {
        let (is_inline, row_lines) = match row {
            Row::Inline(l) => (true, l),
            Row::Text(l) | Row::Block(l) => (false, l),
        };
        // One blank line between rows; consecutive one-line tool calls stack.
        if !lines.is_empty() && !(is_inline && previous_inline) {
            lines.push(Line::from(""));
        }
        previous_inline = is_inline;
        lines.extend(row_lines);
    }

    if let Some(footer) = &msg.footer {
        if !lines.is_empty() {
            lines.push(Line::from(""));
        }
        lines.push(Line::from(vec![
            Span::styled(format!("{INDENT}Stynx"), Style::default().fg(theme::PRIMARY())),
            Span::styled(format!(" · {footer}"), Style::default().fg(theme::TEXT_MUTED())),
        ]));
    }
    lines
}

// ── User message ─────────────────────────────────────────────────────────────

fn user_block(content: &str, width: usize) -> Vec<Line<'static>> {
    let raised = theme::BACKGROUND_PANEL();
    let bar = Style::default().fg(theme::PRIMARY()).bg(raised);
    let text = Style::default().fg(theme::TEXT()).bg(raised);
    let text_w = width.saturating_sub(5).max(8);

    let mut lines = vec![padded(vec![Span::styled("┃", bar)], width, raised)];
    for raw in content.lines() {
        for row in wrap_plain(raw, text_w) {
            lines.push(padded(vec![Span::styled("┃  ", bar), Span::styled(row, text)], width, raised));
        }
    }
    lines.push(padded(vec![Span::styled("┃", bar)], width, raised));
    lines
}

// ── Assistant text ───────────────────────────────────────────────────────────

fn markdown_lines(text: &str, width: usize) -> Vec<Line<'static>> {
    let mut lines: Vec<Line<'static>> = Vec::new();
    let content_lines: Vec<&str> = text.lines().collect();
    let width = width.saturating_sub(1);
    let mut in_code = false;
    let mut in_mermaid = false;
    let mut prev_blank = true;
    let mut i = 0;
    while i < content_lines.len() {
        let raw = content_lines[i];
        let trimmed_start = raw.trim_start();
        if raw.trim().is_empty() {
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
            let start = i;
            while i < content_lines.len() && is_table_line(content_lines[i]) {
                i += 1;
            }
            lines.extend(render_table_block(&content_lines[start..i], width));
            continue;
        } else {
            lines.extend(render_md_line_wrapped(raw, in_code, width));
        }
        i += 1;
    }
    // The markdown renderer indents by 2; the transcript indents by 3.
    for line in &mut lines {
        line.spans.insert(0, Span::raw(" "));
    }
    lines
}

// ── Tool calls ───────────────────────────────────────────────────────────────

fn tool_row(tool: &DisplayToolUse, width: usize, spin: char, details: bool) -> Row {
    let is_bash = tool.name == "bash";
    let is_edit = matches!(tool.name.as_str(), "file_edit" | "file_write");
    if details && tool.status != ToolUseStatus::Error {
        if is_bash && !tool.input_summary.is_empty() {
            return Row::Block(bash_block(tool, width, spin));
        }
        if is_edit && !tool.diff.is_empty() {
            return Row::Block(edit_block(tool, width));
        }
    }

    let (glyph, label) = tool_label(tool);
    let (icon, icon_style, text_style) = match tool.status {
        ToolUseStatus::Running => (spin.to_string(), Style::default().fg(theme::TEXT()), Style::default().fg(theme::TEXT_MUTED())),
        ToolUseStatus::Completed => (glyph.to_string(), Style::default().fg(theme::TEXT_MUTED()), Style::default().fg(theme::TEXT_MUTED())),
        ToolUseStatus::Error => ("✗".to_string(), Style::default().fg(theme::ERROR()), Style::default().fg(theme::ERROR())),
    };
    let mut lines = vec![Line::from(vec![
        Span::styled(format!("{INDENT}{icon} "), icon_style),
        Span::styled(clip(&label, width.saturating_sub(6)), text_style),
    ])];

    let detail_style = Style::default().fg(theme::TEXT_MUTED());
    if tool.status == ToolUseStatus::Error && !tool.output_preview.trim().is_empty() {
        for row in wrap_plain(tool.output_preview.trim(), width.saturating_sub(7).max(8)).into_iter().take(3) {
            lines.push(Line::from(Span::styled(format!("{INDENT}  {row}"), Style::default().fg(theme::ERROR()))));
        }
    }
    // Sub-agent progress: the last few steps under the call.
    for step in tool.sub_progress.iter().rev().take(3).rev() {
        lines.push(Line::from(Span::styled(
            format!("{INDENT}  ↳ {}", clip(step, width.saturating_sub(9))),
            detail_style,
        )));
    }
    Row::Inline(lines)
}

/// OpenCode-style glyph and "Tool argument" label for an inline tool line.
fn tool_label(tool: &DisplayToolUse) -> (&'static str, String) {
    let summary = tool.input_summary.trim();
    let with = |name: &str| if summary.is_empty() { name.to_string() } else { format!("{name} {summary}") };
    match tool.name.as_str() {
        "bash" => ("$", summary.strip_prefix("$ ").unwrap_or(summary).to_string()),
        "read" => ("→", with("Read")),
        "grep" => ("✱", with("Grep")),
        "glob" => ("✱", with("Glob")),
        "web_fetch" => ("%", with("WebFetch")),
        "web_search" => ("◈", with("Web Search")),
        "file_edit" => ("←", with("Edit")),
        "file_write" => ("←", with("Write")),
        "todo_write" | "todo_read" => ("⚙", with("Todos")),
        "ask_user_question" => ("?", with("Question")),
        name if name == "agent" || name == "explore" || name.starts_with("delegate_to_") => {
            let who = name.strip_prefix("delegate_to_").unwrap_or(name);
            ("↳", with(&format!("{} Subagent", capitalize(who))))
        }
        other => ("⚙", with(&capitalize(&other.replace('_', " ")))),
    }
}

fn bash_block(tool: &DisplayToolUse, width: usize, spin: char) -> Vec<Line<'static>> {
    let raised = theme::BACKGROUND_PANEL();
    let command = tool.input_summary.strip_prefix("$ ").unwrap_or(&tool.input_summary);
    let lead = if tool.status == ToolUseStatus::Running { format!("{spin} ") } else { "$ ".to_string() };
    let mut body = vec![vec![
        Span::styled(lead, Style::default().fg(theme::TEXT_MUTED()).bg(raised)),
        Span::styled(clip(command, width.saturating_sub(7)), Style::default().fg(theme::TEXT()).bg(raised)),
    ]];

    let output: Vec<&String> = tool.output_excerpt.iter().filter(|l| !l.trim().is_empty()).collect();
    if !output.is_empty() {
        body.push(Vec::new());
        let skipped = output.len().saturating_sub(BASH_LINES);
        if skipped > 0 {
            body.push(vec![Span::styled(format!("({skipped} earlier lines)"), Style::default().fg(theme::TEXT_MUTED()).bg(raised))]);
        }
        for line in output.into_iter().skip(skipped) {
            body.push(vec![Span::styled(clip(line, width.saturating_sub(5)), Style::default().fg(theme::TEXT_MUTED()).bg(raised))]);
        }
    }
    raised_block(body, width)
}

fn edit_block(tool: &DisplayToolUse, width: usize) -> Vec<Line<'static>> {
    let raised = theme::BACKGROUND_PANEL();
    let verb = if tool.name == "file_write" { "← Write" } else { "← Edit" };
    let mut body = vec![
        vec![Span::styled(
            clip(&format!("{verb} {}", tool.input_summary.trim()), width.saturating_sub(5)),
            Style::default().fg(theme::TEXT_MUTED()).bg(raised),
        )],
        Vec::new(),
    ];

    let added_bg = theme::blend(theme::SUCCESS(), raised, 0.16);
    let removed_bg = theme::blend(theme::ERROR(), raised, 0.16);
    let code_w = width.saturating_sub(8);
    for line in tool.diff.iter().take(DIFF_LINES) {
        let (sign, sign_color, bg, fg) = match line.kind {
            DiffLineKind::Added => ("+", theme::SUCCESS(), added_bg, theme::TEXT()),
            DiffLineKind::Removed => ("-", theme::ERROR(), removed_bg, theme::TEXT()),
            DiffLineKind::Context => (" ", theme::TEXT_MUTED(), raised, theme::TEXT_MUTED()),
        };
        let code = clip(&line.text.replace('\t', "    "), code_w);
        let fill = code_w.saturating_sub(code.width());
        body.push(vec![
            Span::styled(format!(" {sign} "), Style::default().fg(sign_color).bg(bg)),
            Span::styled(code, Style::default().fg(fg).bg(bg)),
            Span::styled(" ".repeat(fill), Style::default().bg(bg)),
        ]);
    }
    if tool.diff.len() > DIFF_LINES {
        body.push(vec![Span::styled(
            format!("({} more lines)", tool.diff.len() - DIFF_LINES),
            Style::default().fg(theme::TEXT_MUTED()).bg(raised),
        )]);
    }
    raised_block(body, width)
}

/// A raised block: a dark notch bar on the left, one padding row above and
/// below, and the body indented by 2.
fn raised_block(body: Vec<Vec<Span<'static>>>, width: usize) -> Vec<Line<'static>> {
    let raised = theme::BACKGROUND_PANEL();
    let notch = Style::default().fg(theme::BACKGROUND()).bg(raised);
    let mut lines = vec![padded(vec![Span::styled("┃", notch)], width, raised)];
    for row in body {
        let mut spans = vec![Span::styled("┃  ", notch)];
        spans.extend(row);
        lines.push(padded(spans, width, raised));
    }
    lines.push(padded(vec![Span::styled("┃", notch)], width, raised));
    lines
}

// ── Helpers ──────────────────────────────────────────────────────────────────

/// Fill a row to `width` with `bg`, so the block background spans the column.
fn padded(mut spans: Vec<Span<'static>>, width: usize, bg: Color) -> Line<'static> {
    let used: usize = spans.iter().map(|s| s.content.width()).sum();
    if used < width {
        spans.push(Span::styled(" ".repeat(width - used), Style::default().bg(bg)));
    }
    Line::from(spans)
}

/// Cut `text` to `width` columns, marking the cut with an ellipsis.
fn clip(text: &str, width: usize) -> String {
    if text.width() <= width {
        return text.to_string();
    }
    let mut out = String::new();
    for c in text.chars() {
        if out.width() + 2 > width {
            break;
        }
        out.push(c);
    }
    out.push('…');
    out
}

fn capitalize(text: &str) -> String {
    text.split_whitespace()
        .map(|word| {
            let mut chars = word.chars();
            chars.next().map(|first| first.to_uppercase().collect::<String>() + chars.as_str()).unwrap_or_default()
        })
        .collect::<Vec<_>>()
        .join(" ")
}
