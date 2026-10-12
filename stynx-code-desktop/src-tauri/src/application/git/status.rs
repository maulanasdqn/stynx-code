use std::path::Path;

use super::cli::{repo_root, run};
use crate::domain::ide_git::{GitEntry, GitStatus};

pub fn status(workspace: &Path) -> Result<GitStatus, String> {
    let Some(root) = repo_root(workspace) else {
        return Ok(GitStatus::default());
    };
    let output = run(&root, &["status", "--porcelain=v2", "-z", "--branch", "--untracked-files=all"], "git status failed")?;
    let mut status = parse(&output.stdout);
    status.is_repo = true;
    status.prefix = workspace
        .strip_prefix(&root)
        .map(|p| p.to_string_lossy().replace('\\', "/"))
        .unwrap_or_default();
    Ok(status)
}

fn entry(path: &str, orig_path: Option<String>, status: char) -> GitEntry {
    GitEntry { path: path.to_string(), orig_path, status: status.to_string() }
}

fn push_ordinary(status: &mut GitStatus, xy: &str, path: &str, orig_path: Option<String>) {
    let mut chars = xy.chars();
    let (x, y) = (chars.next().unwrap_or('.'), chars.next().unwrap_or('.'));
    if x != '.' {
        status.staged.push(entry(path, orig_path.clone(), x));
    }
    if y != '.' {
        status.unstaged.push(entry(path, orig_path, y));
    }
}

pub fn parse(raw: &[u8]) -> GitStatus {
    let fields: Vec<String> = raw
        .split(|byte| *byte == 0)
        .filter(|field| !field.is_empty())
        .map(|field| String::from_utf8_lossy(field).into_owned())
        .collect();
    let mut status = GitStatus::default();
    let mut i = 0;
    while i < fields.len() {
        let (kind, rest) = fields[i].split_once(' ').unwrap_or((fields[i].as_str(), ""));
        match kind {
            "#" => {
                if let Some(name) = rest.strip_prefix("branch.head ") {
                    if name == "(detached)" {
                        status.detached = true;
                    } else {
                        status.branch = Some(name.to_string());
                    }
                } else if let Some(ab) = rest.strip_prefix("branch.ab ") {
                    let mut nums = ab.split_whitespace();
                    status.ahead = nums.next().and_then(|s| s.strip_prefix('+')?.parse().ok());
                    status.behind = nums.next().and_then(|s| s.strip_prefix('-')?.parse().ok());
                }
            }
            "1" => {
                let cols: Vec<&str> = rest.splitn(8, ' ').collect();
                if cols.len() == 8 {
                    push_ordinary(&mut status, cols[0], cols[7], None);
                }
            }
            "2" => {
                let cols: Vec<&str> = rest.splitn(9, ' ').collect();
                i += 1;
                let orig = fields.get(i).cloned();
                if cols.len() == 9 {
                    push_ordinary(&mut status, cols[0], cols[8], orig);
                }
            }
            "u" => {
                let cols: Vec<&str> = rest.splitn(10, ' ').collect();
                if cols.len() == 10 {
                    status.conflicted.push(entry(cols[9], None, 'U'));
                }
            }
            "?" => {
                let path = rest.trim_end_matches('/');
                if !path.is_empty() {
                    status.unstaged.push(entry(path, None, '?'));
                }
            }
            _ => {}
        }
        i += 1;
    }
    status
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_branch_changes_renames_conflicts_and_untracked() {
        let raw = [
            "# branch.oid abc",
            "# branch.head develop",
            "# branch.upstream origin/develop",
            "# branch.ab +2 -1",
            "1 M. N... 100644 100644 100644 aaa bbb src/staged.rs",
            "1 .M N... 100644 100644 100644 aaa bbb src/edited file.rs",
            "1 MM N... 100644 100644 100644 aaa bbb both.rs",
            "2 R. N... 100644 100644 100644 aaa bbb R100 src/new.rs",
            "src/old.rs",
            "u UU N... 100644 100644 100644 100644 a b c conflict.rs",
            "? notes.md",
        ]
        .join("\0");
        let status = parse(raw.as_bytes());
        assert_eq!(status.branch.as_deref(), Some("develop"));
        assert_eq!((status.ahead, status.behind), (Some(2), Some(1)));
        let staged: Vec<_> = status.staged.iter().map(|e| (e.path.as_str(), e.status.as_str())).collect();
        assert_eq!(staged, [("src/staged.rs", "M"), ("both.rs", "M"), ("src/new.rs", "R")]);
        assert_eq!(status.staged[2].orig_path.as_deref(), Some("src/old.rs"));
        let unstaged: Vec<_> = status.unstaged.iter().map(|e| (e.path.as_str(), e.status.as_str())).collect();
        assert_eq!(unstaged, [("src/edited file.rs", "M"), ("both.rs", "M"), ("notes.md", "?")]);
        assert_eq!(status.conflicted[0].path, "conflict.rs");
    }

    #[test]
    fn detached_head_without_upstream() {
        let status = parse(b"# branch.oid abc\0# branch.head (detached)\0");
        assert!(status.detached);
        assert_eq!((status.branch, status.ahead), (None, None));
    }
}
