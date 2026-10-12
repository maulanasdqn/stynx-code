use unicode_width::UnicodeWidthChar;

pub struct Wrapped {
    pub rows: Vec<String>,
    pub cursor_row: usize,
    pub cursor_col: usize,
}

fn row_ranges(text: &str, width: usize) -> Vec<(usize, usize)> {
    let width = width.max(1);
    let mut rows = Vec::new();
    let mut line_start = 0;
    for line in text.split('\n') {
        let line_end = line_start + line.len();
        let mut start = line_start;
        let mut used = 0;
        let mut last_space: Option<usize> = None;
        let mut index = line_start;
        for c in line.chars() {
            let w = c.width().unwrap_or(0);
            if used + w > width && index > start {
                let cut = match last_space {
                    Some(space) if space > start => space,
                    _ => index,
                };
                rows.push((start, cut));
                start = cut;
                used = text[start..index].chars().map(|ch| ch.width().unwrap_or(0)).sum();
                last_space = None;
            }
            used += w;
            index += c.len_utf8();
            if c == ' ' {
                last_space = Some(index);
            }
        }
        rows.push((start, line_end));
        line_start = line_end + 1;
    }
    rows
}

pub fn wrap(text: &str, cursor: usize, width: usize) -> Wrapped {
    let width = width.max(1);
    let cursor = cursor.min(text.len());
    let ranges = row_ranges(text, width);

    let mut cursor_row = ranges.len().saturating_sub(1);
    for (i, &(start, end)) in ranges.iter().enumerate() {
        let continues = ranges.get(i + 1).is_some_and(|&(next, _)| next == end);
        if start <= cursor && (cursor < end || (cursor == end && !continues)) {
            cursor_row = i;
            break;
        }
    }
    let (row_start, _) = ranges[cursor_row];
    let mut cursor_col: usize = text[row_start..cursor].chars().map(|c| c.width().unwrap_or(0)).sum();

    let mut rows: Vec<String> = ranges.iter().map(|&(s, e)| text[s..e].to_string()).collect();
    if cursor_col >= width {
        cursor_row += 1;
        cursor_col = 0;
        if cursor_row >= rows.len() {
            rows.push(String::new());
        }
    }
    Wrapped { rows, cursor_row, cursor_col }
}

pub fn visual_line_count(text: &str, width: usize) -> usize {
    wrap(text, text.len(), width).rows.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wraps_at_word_boundaries() {
        let w = wrap("hello world again", 17, 12);
        assert_eq!(w.rows, ["hello world ", "again"]);
        assert_eq!((w.cursor_row, w.cursor_col), (1, 5));
    }

    #[test]
    fn breaks_long_words_and_keeps_newlines() {
        let w = wrap("abcdefgh\nxy", 0, 3);
        assert_eq!(w.rows, ["abc", "def", "gh", "xy"]);
        assert_eq!((w.cursor_row, w.cursor_col), (0, 0));
    }

    #[test]
    fn cursor_on_wrap_boundary_moves_to_next_row() {
        let w = wrap("abcdef", 3, 3);
        assert_eq!((w.cursor_row, w.cursor_col), (1, 0));
    }

    #[test]
    fn full_last_row_gets_a_fresh_row_for_the_cursor() {
        let w = wrap("abc", 3, 3);
        assert_eq!(w.rows.len(), 2);
        assert_eq!((w.cursor_row, w.cursor_col), (1, 0));
    }
}
