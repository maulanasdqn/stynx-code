use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::Widget,
};
use unicode_width::UnicodeWidthStr;

use crate::state::AppState;
use crate::theme;
use crate::widgets::footer::{fmt_elapsed_short, shrink_path};
use crate::widgets::spinner::FRAMES;

pub struct StatusLine<'a> {
    pub state: &'a AppState,
}

fn display_model(model: &str) -> String {
    let base = model.rsplit('/').next().unwrap_or(model);
    let (base, long_context) = match base.strip_suffix("[1m]") {
        Some(stripped) => (stripped, true),
        None => (base, false),
    };
    let Some(rest) = base.strip_prefix("claude-") else { return base.to_string() };
    let mut parts = rest.split('-').filter(|p| !(p.len() >= 6 && p.chars().all(|c| c.is_ascii_digit())));
    let family = parts.next().unwrap_or(rest);
    let version: Vec<&str> = parts.collect();
    let mut name = family[..1].to_uppercase() + &family[1..];
    if !version.is_empty() {
        name.push(' ');
        name.push_str(&version.join("."));
    }
    if long_context {
        name.push_str(" 1M");
    }
    name
}

fn compact(n: u64) -> String {
    match n {
        0..=999 => n.to_string(),
        1_000..=999_999 => format!("{:.1}k", n as f64 / 1_000.0),
        _ => format!("{:.1}M", n as f64 / 1_000_000.0),
    }
}

impl<'a> Widget for StatusLine<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        if area.height == 0 {
            return;
        }
        let s = self.state;
        buf.set_style(area, Style::default().bg(theme::BACKGROUND()));
        let dim = Style::default().fg(theme::TEXT_MUTED());

        let (mode_label, mode_color) = match s.permission_mode.as_str() {
            "Plan" => ("PLAN", theme::WARNING()),
            "Auto-accept" => ("AUTO", theme::SUCCESS()),
            "Bypass" => ("BYPASS", theme::ERROR()),
            _ => ("BUILD", theme::PRIMARY()),
        };
        let mut left = vec![
            Span::styled(
                format!(" {mode_label} "),
                Style::default().fg(theme::BACKGROUND()).bg(mode_color).add_modifier(Modifier::BOLD),
            ),
            Span::styled(format!(" {}", display_model(&s.model_name)), Style::default().fg(theme::TEXT())),
        ];
        let spin = FRAMES[s.spinner_frame % FRAMES.len()];
        if s.is_paused {
            left.push(Span::styled("  paused", Style::default().fg(theme::WARNING())));
        } else if s.is_pending {
            left.push(Span::styled(format!("  {spin} connecting"), dim));
        } else if s.is_streaming {
            left.push(Span::styled(
                format!("  {spin} working {}", fmt_elapsed_short(s.elapsed_secs)),
                Style::default().fg(theme::PRIMARY()),
            ));
        }

        let percent = s.last_input_tokens as f64 / s.context_window.max(1) as f64 * 100.0;
        let mut right_parts = vec![shrink_path(&s.cwd, 32)];
        if let Some(branch) = &s.git_branch {
            right_parts.push(format!("({branch})"));
        }
        right_parts.push(format!("↑{} ↓{}", compact(s.total_input), compact(s.total_output)));
        right_parts.push(format!("{percent:.0}% ctx"));
        right_parts.push(format!("${:.2}", s.total_cost));

        let left_line = Line::from(left);
        let left_width = left_line.width();
        buf.set_line(area.x, area.y, &left_line, area.width);
        while !right_parts.is_empty() {
            let right = format!("{} ", right_parts.join("  "));
            let right_width = right.width();
            if usize::from(area.width) > left_width + right_width + 2 {
                let x = area.x + area.width - right_width as u16;
                buf.set_string(x, area.y, &right, dim);
                break;
            }
            right_parts.remove(0);
        }
    }
}
