use std::collections::BTreeSet;
use std::path::Path;
use std::sync::mpsc;
use std::time::Duration;

use notify::{RecommendedWatcher, RecursiveMode, Watcher};

use super::root::WorkspaceRoot;

const DEBOUNCE: Duration = Duration::from_millis(250);
const MAX_PATHS: usize = 4096;
const SKIPPED: [&str; 4] = [".git", "target", "node_modules", "dist"];

pub struct WorkspaceWatcher {
    _watcher: RecommendedWatcher,
}

fn skipped(relative: &str) -> bool {
    relative.split('/').any(|segment| SKIPPED.contains(&segment)) || relative.ends_with(".stynx-tmp")
}

impl WorkspaceWatcher {
    pub fn start<F>(root: WorkspaceRoot, on_change: F) -> Result<Self, String>
    where
        F: Fn(Vec<String>) + Send + 'static,
    {
        let (tx, rx) = mpsc::channel::<notify::Result<notify::Event>>();
        let mut watcher = notify::recommended_watcher(tx).map_err(|error| error.to_string())?;
        watcher.watch(root.path(), RecursiveMode::Recursive).map_err(|error| error.to_string())?;
        std::thread::spawn(move || debounce_loop(&root, &rx, on_change));
        Ok(Self { _watcher: watcher })
    }
}

fn collect(root: &WorkspaceRoot, event: notify::Result<notify::Event>, batch: &mut BTreeSet<String>) {
    let Ok(event) = event else { return };
    for path in event.paths {
        if let Some(relative) = root.relative(Path::new(&path))
            && !skipped(&relative)
            && batch.len() < MAX_PATHS
        {
            batch.insert(relative);
        }
    }
}

fn debounce_loop<F>(root: &WorkspaceRoot, rx: &mpsc::Receiver<notify::Result<notify::Event>>, on_change: F)
where
    F: Fn(Vec<String>),
{
    while let Ok(first) = rx.recv() {
        let mut batch = BTreeSet::new();
        collect(root, first, &mut batch);
        while let Ok(next) = rx.recv_timeout(DEBOUNCE) {
            collect(root, next, &mut batch);
        }
        if !batch.is_empty() {
            on_change(batch.into_iter().collect());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn skips_build_and_vcs_dirs() {
        assert!(skipped("target/debug/app"));
        assert!(skipped(".git/HEAD"));
        assert!(skipped("web/node_modules/x/index.js"));
        assert!(skipped("src/.main.rs.stynx-tmp"));
        assert!(!skipped("src/main.rs"));
    }
}
