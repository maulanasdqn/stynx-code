use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Paragraph, Widget},
};

use crate::theme;
use crate::widgets::footer::{fmt_elapsed_short, pretty_model, shrink_path};
use crate::widgets::spinner::FRAMES;

pub struct SidebarInfo<'a> {
    pub cwd: &'a str,
    pub model: &'a str,
    pub mode: &'a str,
    pub cost: f64,
    pub git_branch: Option<&'a str>,
    pub is_streaming: bool,
    pub is_pending: bool,
    pub is_paused: bool,
    pub spinner_frame: usize,
    pub elapsed_secs: u64,
}

pub fn section_header(title: &str) -> Line<'static> {
    Line::from(Span::styled(
        format!("  {title}"),
        Style::default().fg(theme::TEXT_MUTED()).add_modifier(Modifier::BOLD),
    ))
}

fn row(icon: &str, icon_color: Color, value: impl Into<String>, value_style: Style) -> Line<'static> {
    Line::from(vec![
        Span::styled(format!("  {icon} "), Style::default().fg(icon_color)),
        Span::styled(value.into(), value_style),
    ])
}

fn display_model(model: &str) -> String {
    let base = model.rsplit('/').next().unwrap_or(model);
    let (base, long_context) = match base.strip_suffix("[1m]") {
        Some(stripped) => (stripped, true),
        None => (base, false),
    };
    let Some(rest) = base.strip_prefix("claude-") else {
        return pretty_model(model);
    };
    let mut parts = rest.split('-').filter(|p| !(p.len() >= 6 && p.chars().all(|c| c.is_ascii_digit())));
    let family = parts.next().unwrap_or(rest);
    let version: Vec<&str> = parts.collect();
    let mut name = family[..1].to_uppercase() + &family[1..];
    if !version.is_empty() {
        name.push(' ');
        name.push_str(&version.join("."));
    }
    if long_context {
        name.push_str(" · 1M");
    }
    name
}

fn project_name(cwd: &str) -> String {
    cwd.trim_end_matches('/')
        .rsplit('/')
        .next()
        .filter(|s| !s.is_empty())
        .unwrap_or(cwd)
        .to_string()
}

impl<'a> Widget for SidebarInfo<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        if area.height == 0 || area.width < 4 {
            return;
        }
        let bg = theme::BACKGROUND_PANEL();
        buf.set_style(area, Style::default().bg(bg));

        let w = area.width as usize;
        let text = Style::default().fg(theme::TEXT());
        let dim = Style::default().fg(theme::TEXT_MUTED());
        let mut lines: Vec<Line<'static>> = vec![Line::from(""), section_header("Workspace")];

        lines.push(row(
            "\u{25CF}",
            theme::PRIMARY(),
            project_name(self.cwd),
            text.add_modifier(Modifier::BOLD),
        ));
        if let Some(branch) = self.git_branch {
            lines.push(row("\u{E0A0}", theme::TEXT_MUTED(), branch.to_string(), dim));
        }
        lines.push(row("\u{F07C}", theme::TEXT_MUTED(), shrink_path(self.cwd, w.saturating_sub(6)), dim));

        lines.push(Line::from(""));
        lines.push(section_header("Session"));
        lines.push(row("\u{25C7}", theme::PRIMARY(), display_model(self.model), text));

        let (mode_icon, mode_color) = match self.mode {
            "Auto-accept" => ("\u{26A1}", theme::WARNING()),
            "Plan" => ("\u{25C6}", theme::PRIMARY()),
            "Bypass" => ("\u{26A0}", theme::ERROR()),
            _ => ("\u{25CB}", theme::TEXT_MUTED()),
        };
        lines.push(row(mode_icon, mode_color, self.mode.to_string(), Style::default().fg(mode_color)));
        lines.push(row("$", theme::TEXT_MUTED(), format!("{:.4}", self.cost), dim));

        let spin = FRAMES[self.spinner_frame % FRAMES.len()].to_string();
        let status = if self.is_paused {
            Some(row("\u{23F8}", theme::WARNING(), "Paused", Style::default().fg(theme::WARNING())))
        } else if self.is_pending {
            Some(row(&spin, theme::TEXT_MUTED(), "Connecting…", dim))
        } else if self.is_streaming {
            Some(row(
                &spin,
                theme::PRIMARY(),
                format!("Thinking  {}", fmt_elapsed_short(self.elapsed_secs)),
                Style::default().fg(theme::PRIMARY()),
            ))
        } else {
            Some(row("\u{25CF}", theme::SUCCESS(), "Ready", dim))
        };
        lines.extend(status);

        Paragraph::new(lines).style(Style::default().bg(bg)).render(area, buf);
    }
}
