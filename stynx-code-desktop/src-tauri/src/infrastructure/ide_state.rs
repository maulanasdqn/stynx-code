use tokio::sync::Mutex;

use crate::application::workspace::root::WorkspaceRoot;
use crate::application::workspace::watch::WorkspaceWatcher;

pub struct IdeWorkspace {
    pub root: WorkspaceRoot,
    pub _watcher: Option<WorkspaceWatcher>,
}

#[derive(Default)]
pub struct IdeState(pub Mutex<Option<IdeWorkspace>>);

impl IdeState {
    pub async fn root(&self) -> Result<WorkspaceRoot, String> {
        self.0
            .lock()
            .await
            .as_ref()
            .map(|workspace| workspace.root.clone())
            .ok_or_else(|| "no workspace attached — call ide_attach first".to_string())
    }
}

#[derive(Default)]
pub struct TerminalState(pub std::sync::Mutex<crate::application::terminal::manager::TerminalManager>);
