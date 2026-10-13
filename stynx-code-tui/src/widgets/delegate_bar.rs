use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Paragraph, Widget},
};

use crate::state::SubAgentState;
use crate::theme;
use crate::widgets::spinner::FRAMES;

pub struct DelegateBar<'a> {
    pub agents: &'a [SubAgentState],
    pub spinner_frame: usize,
}

impl<'a> DelegateBar<'a> {
    pub fn new(agents: &'a [SubAgentState], spinner_frame: usize) -> Self {
        Self { agents, spinner_frame }
    }

    pub fn total_lines(agents: &[SubAgentState]) -> usize {
        agents.iter().map(|a| a.display_lines()).sum()
    }
}

impl<'a> Widget for DelegateBar<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        if self.agents.is_empty() { return; }
        let pad = " ";
        let spinner = FRAMES.get(self.spinner_frame % FRAMES.len()).copied().unwrap_or('·');
        let max_rows = area.height as usize;
        let mut lines: Vec<Line<'static>> = Vec::new();

        for agent in self.agents {
            if lines.len() >= max_rows { break; }

            lines.push(Line::from(vec![
                Span::styled(
                    format!("{pad}{spinner} "),
                    Style::default().fg(theme::IRIS()).add_modifier(Modifier::BOLD),
                ),
                Span::styled(
                    agent.label.clone(),
                    Style::default().fg(theme::FOAM()).add_modifier(Modifier::BOLD),
                ),
                Span::styled("  ", Style::default()),
                Span::styled(
                    format!("step {} · {}", agent.step, &agent.current),
                    Style::default().fg(theme::SUBTLE()),
                ),
            ]));

            let tail: Vec<&String> = agent.queue.iter().rev().skip(1).take(4).collect();
            for (i, entry) in tail.iter().enumerate() {
                if lines.len() >= max_rows { break; }
                let dim = if i >= 2 { Modifier::DIM } else { Modifier::empty() };
                lines.push(Line::from(vec![
                    Span::raw(format!("{pad}  ")),
                    Span::styled(
                        "↪ ",
                        Style::default().fg(theme::TEXT_MUTED()).add_modifier(dim),
                    ),
                    Span::styled(
                        truncate(entry, area.width.saturating_sub(8) as usize),
                        Style::default().fg(theme::SUBTLE()).add_modifier(Modifier::ITALIC | dim),
                    ),
                ]));
            }
        }

        Paragraph::new(lines).render(area, buf);
    }
}

fn truncate(s: &str, max: usize) -> String {
    if s.chars().count() <= max {
        return s.to_string();
    }
    let mut out: String = s.chars().take(max.saturating_sub(1)).collect();
    out.push('…');
    out
}
