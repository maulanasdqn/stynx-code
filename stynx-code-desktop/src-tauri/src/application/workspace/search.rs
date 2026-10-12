use ignore::WalkBuilder;
use regex::RegexBuilder;
use serde::Serialize;

use super::root::WorkspaceRoot;

const MAX_MATCHES: usize = 2000;
const MAX_FILE: u64 = 2 * 1024 * 1024;
const PREVIEW: usize = 220;

#[derive(Serialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct SearchMatch {
    pub line: usize,
    pub column: usize,
    pub length: usize,
    pub preview: String,
    pub offset: usize,
}

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SearchFile {
    pub path: String,
    pub matches: Vec<SearchMatch>,
}

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SearchResult {
    pub files: Vec<SearchFile>,
    pub truncated: bool,
}

pub fn search(root: &WorkspaceRoot, query: &str, case_sensitive: bool, is_regex: bool) -> Result<SearchResult, String> {
    if query.is_empty() {
        return Ok(SearchResult { files: Vec::new(), truncated: false });
    }
    let pattern = if is_regex { query.to_string() } else { regex::escape(query) };
    let re = RegexBuilder::new(&pattern).case_insensitive(!case_sensitive).build().map_err(|e| e.to_string())?;
    let mut files = Vec::new();
    let mut total = 0;
    let walker = WalkBuilder::new(root.path()).hidden(false).filter_entry(|e| e.file_name() != ".git").build();
    for entry in walker.flatten() {
        if total >= MAX_MATCHES {
            return Ok(SearchResult { files, truncated: true });
        }
        if !entry.file_type().is_some_and(|t| t.is_file()) || entry.metadata().is_ok_and(|m| m.len() > MAX_FILE) {
            continue;
        }
        let Ok(bytes) = std::fs::read(entry.path()) else { continue };
        if bytes[..bytes.len().min(8192)].contains(&0) {
            continue;
        }
        let text = String::from_utf8_lossy(&bytes);
        let mut matches = Vec::new();
        for (index, line) in text.lines().enumerate() {
            for found in re.find_iter(line) {
                let column = line[..found.start()].chars().count();
                let start = line[..found.start()].char_indices().rev().nth(39).map_or(0, |(i, _)| i);
                let offset = line[start..found.start()].chars().count();
                let preview: String = line[start..].trim_end().chars().take(PREVIEW).collect();
                let length = found.as_str().chars().count();
                matches.push(SearchMatch { line: index + 1, column: column + 1, length, preview, offset });
                total += 1;
                if total >= MAX_MATCHES {
                    break;
                }
            }
        }
        if !matches.is_empty()
            && let Some(path) = root.relative(entry.path())
        {
            files.push(SearchFile { path, matches });
        }
    }
    files.sort_by(|a, b| a.path.cmp(&b.path));
    Ok(SearchResult { files, truncated: false })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finds_plain_and_regex_matches_respecting_gitignore() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::create_dir_all(dir.path().join(".git")).unwrap();
        std::fs::write(dir.path().join(".gitignore"), "dist/\n").unwrap();
        std::fs::create_dir_all(dir.path().join("dist")).unwrap();
        std::fs::write(dir.path().join("dist/out.js"), "TODO hidden").unwrap();
        std::fs::write(dir.path().join("a.rs"), "fn main() {\n    // TODO: wire it\n    let todo = 1;\n}\n").unwrap();
        let root = WorkspaceRoot::new(dir.path().to_str().unwrap()).unwrap();
        let plain = search(&root, "todo", false, false).unwrap();
        assert_eq!(plain.files.len(), 1);
        assert_eq!(plain.files[0].matches.iter().map(|m| (m.line, m.column)).collect::<Vec<_>>(), [(2, 8), (3, 9)]);
        let first = &plain.files[0].matches[0];
        let hit: String = first.preview.chars().skip(first.offset).take(first.length).collect();
        assert_eq!(hit, "TODO");
        let strict = search(&root, "TODO", true, false).unwrap();
        assert_eq!(strict.files[0].matches.len(), 1);
        let re = search(&root, r"let \w+ =", true, true).unwrap();
        assert_eq!(re.files[0].matches[0].line, 3);
        assert!(search(&root, "(", true, true).is_err());
    }
}
