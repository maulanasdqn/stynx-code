use ignore::WalkBuilder;

use super::root::WorkspaceRoot;

pub const INDEX_CAP: usize = 20_000;

pub fn index_files(root: &WorkspaceRoot) -> Vec<String> {
    let mut files = Vec::new();
    let walker = WalkBuilder::new(root.path())
        .hidden(false)
        .git_ignore(true)
        .git_global(true)
        .filter_entry(|entry| entry.file_name() != ".git")
        .build();
    for entry in walker.flatten() {
        if files.len() >= INDEX_CAP {
            break;
        }
        if entry.file_type().is_some_and(|kind| kind.is_file())
            && let Some(relative) = root.relative(entry.path())
        {
            files.push(relative);
        }
    }
    files.sort();
    files
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn respects_gitignore() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::create_dir_all(dir.path().join(".git")).unwrap();
        std::fs::create_dir_all(dir.path().join("target")).unwrap();
        std::fs::write(dir.path().join(".gitignore"), "target/\n").unwrap();
        std::fs::write(dir.path().join("main.rs"), "").unwrap();
        std::fs::write(dir.path().join("target/out.o"), "").unwrap();
        let root = WorkspaceRoot::new(dir.path().to_str().unwrap()).unwrap();
        assert_eq!(index_files(&root), [".gitignore", "main.rs"]);
    }
}
