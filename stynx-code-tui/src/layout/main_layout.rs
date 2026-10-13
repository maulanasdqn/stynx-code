use ratatui::layout::{Constraint, Layout, Rect};

pub struct LayoutResult {
    pub tool_history: Option<Rect>,
    pub tool_info: Option<Rect>,
    pub messages: Rect,
    pub thinking: Option<Rect>,
    pub delegate: Option<Rect>,
    pub summary: Option<Rect>,
    pub input: Rect,
    pub footer: Option<Rect>,
}

pub struct MainLayout;

impl MainLayout {
    pub const TOOL_HISTORY_WIDTH: u16 = 40;
    pub const MIN_MAIN_WIDTH: u16 = 60;
    const GUTTER: u16 = 1;

    fn has_sidebar(area: Rect) -> bool {
        area.width > Self::TOOL_HISTORY_WIDTH + Self::MIN_MAIN_WIDTH
    }

    pub fn input_text_width(area: Rect) -> usize {
        let main = if Self::has_sidebar(area) {
            area.width - Self::TOOL_HISTORY_WIDTH - Self::GUTTER * 2
        } else {
            area.width.saturating_sub(Self::GUTTER * 2)
        };
        usize::from(main.saturating_sub(4)).max(1)
    }

    pub fn split(
        area: Rect,
        input_lines: usize,
        thinking_lines: usize,
        delegate_lines: usize,
        has_summary: bool,
        info_lines: usize,
    ) -> LayoutResult {
        let (main, side) = if Self::has_sidebar(area) {
            let chunks = Layout::horizontal([
                Constraint::Length(Self::GUTTER),
                Constraint::Min(1),
                Constraint::Length(Self::GUTTER),
                Constraint::Length(Self::TOOL_HISTORY_WIDTH),
            ])
            .split(area);
            (chunks[1], Some(chunks[3]))
        } else {
            let chunks = Layout::horizontal([
                Constraint::Length(Self::GUTTER),
                Constraint::Min(1),
                Constraint::Length(Self::GUTTER),
            ])
            .split(area);
            (chunks[1], None)
        };

        let (tool_info, tool_history) = match side {
            Some(col) => {
                let info_h = (info_lines as u16 + 1).min(col.height.saturating_sub(6));
                let parts = Layout::vertical([Constraint::Length(info_h), Constraint::Min(1)]).split(col);
                (Some(parts[0]), Some(parts[1]))
            }
            None => (None, None),
        };

        let content = input_lines.clamp(1, 8) as u16;
        let input_h = (content + 2).min(main.height.saturating_sub(5));
        let thinking_h: u16 = if thinking_lines == 0 { 0 } else { (1 + thinking_lines.min(12)) as u16 };
        let delegate_h: u16 = delegate_lines.min(8) as u16;

        let mut constraints: Vec<Constraint> = vec![Constraint::Min(1)];
        if thinking_h > 0 { constraints.push(Constraint::Length(thinking_h)); }
        if delegate_h > 0 { constraints.push(Constraint::Length(delegate_h)); }
        if has_summary { constraints.push(Constraint::Length(1)); }
        constraints.push(Constraint::Length(input_h));
        constraints.push(Constraint::Length(1));

        let rows = Layout::vertical(constraints).split(main);
        let mut idx = 0;
        let messages = rows[idx]; idx += 1;
        let thinking = if thinking_h > 0 { let r = Some(rows[idx]); idx += 1; r } else { None };
        let delegate = if delegate_h > 0 { let r = Some(rows[idx]); idx += 1; r } else { None };
        let summary = if has_summary { let r = Some(rows[idx]); idx += 1; r } else { None };
        let input = rows[idx]; idx += 1;
        let footer = Some(rows[idx]);

        LayoutResult { tool_history, tool_info, messages, thinking, delegate, summary, input, footer }
    }
}
