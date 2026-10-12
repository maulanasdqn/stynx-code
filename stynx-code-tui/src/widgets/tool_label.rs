pub fn pretty_name(name: &str) -> String {
    if let Some(intern) = name.strip_prefix("delegate_to_") {
        let target = if intern == "all_interns" { "all" } else { intern };
        return format!("→ {target}");
    }
    let pretty = match name {
        "bash" => "Bash",
        "read" => "Read",
        "file_write" => "Write",
        "file_edit" => "Edit",
        "glob" => "Glob",
        "grep" => "Grep",
        "web_fetch" => "WebFetch",
        "web_search" => "WebSearch",
        "todo_write" => "TodoWrite",
        "todo_read" => "TodoRead",
        "ask_user_question" => "AskUser",
        "agent" => "Agent",
        "explore" => "Explore",
        other => other,
    };
    pretty.to_string()
}

pub fn name_cell(name: &str, width: usize) -> String {
    let fitted = truncate_path(name, width.saturating_sub(1));
    format!("{fitted:<width$}")
}

pub fn truncate_path(s: &str, max: usize) -> String {
    use unicode_width::UnicodeWidthChar;
    let total: usize = s.chars().map(|c| c.width().unwrap_or(0)).sum();
    if total <= max { return s.to_string(); }
    if s.contains('/') {
        let parts: Vec<&str> = s.split('/').collect();
        if let Some(last) = parts.last() {
            let last_w: usize = last.chars().map(|c| c.width().unwrap_or(0)).sum();
            if last_w + 4 <= max {
                return format!("…/{last}");
            }
        }
    }
    let mut out = String::new();
    let mut w = 0usize;
    for c in s.chars() {
        let cw = c.width().unwrap_or(0);
        if w + cw + 1 > max { break; }
        w += cw;
        out.push(c);
    }
    out.push('…');
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn intern_delegation_shows_the_intern_name() {
        assert_eq!(pretty_name("delegate_to_netra"), "→ netra");
        assert_eq!(pretty_name("delegate_to_all_interns"), "→ all");
    }

    #[test]
    fn name_cell_always_leaves_a_gap_before_the_summary() {
        let cell = name_cell("delegate_to_something_long", 10);
        assert_eq!(cell.chars().count(), 10);
        assert!(cell.ends_with(' '));
        assert_eq!(name_cell("Read", 10), "Read      ");
    }
}
