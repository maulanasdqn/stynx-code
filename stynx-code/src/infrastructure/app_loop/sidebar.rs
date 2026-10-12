use std::sync::Arc;
use std::sync::atomic::AtomicU64;

use stynx_code_commands::expand_message_content;
use stynx_code_engine::{EngineEvent, QueryEngine};
use stynx_code_errors::AppError;
use stynx_code_memory::SessionRepository;
use stynx_code_types::{Conversation, Message, Role};
use stynx_code_tui::{TuiApp, state::app_state::SessionSummary};
use tokio::sync::mpsc;
use tokio::task::JoinHandle;

use crate::infrastructure::run_engine::run_engine_tui;

pub(super) type EngineTask = JoinHandle<Result<Conversation, AppError>>;
pub(super) type EngineSlot = Option<(EngineTask, mpsc::UnboundedReceiver<EngineEvent>, Conversation)>;

pub(super) fn spawn_engine(
    text: &str,
    conversation: &mut Conversation,
    tui: &mut TuiApp,
    engine: &Arc<QueryEngine>,
    total_input: &Arc<AtomicU64>,
    total_output: &Arc<AtomicU64>,
) -> EngineSlot {
    let pre = conversation.clone();
    conversation.push(Message { role: Role::User, content: expand_message_content(text) });
    tui.state.push_user_message(text);
    let (ev_tx, ev_rx) = mpsc::unbounded_channel();
    let task = tokio::spawn({
        let (eng, conv, ti, to) = (
            engine.clone(), conversation.clone(),
            total_input.clone(), total_output.clone(),
        );
        async move { run_engine_tui(&eng, conv, &ti, &to, ev_tx).await }
    });
    Some((task, ev_rx, pre))
}

pub(super) async fn refresh_sidebar_sessions(
    repo: &Arc<dyn SessionRepository>,
    tui: &mut TuiApp,
) {
    let summaries = repo.list().await.unwrap_or_default();
    let current = repo.current().await.ok().flatten();
    tui.state.sidebar.sessions = summaries
        .iter()
        .map(|s| SessionSummary {
            id: s.id.clone(),
            title: s.title.clone(),
            updated_at: s.updated_at,
            pinned: false,
        })
        .collect();
    let Some(id) = current else {
        tui.state.sidebar.title = "New session".to_string();
        return;
    };
    tui.state.sidebar.title = summaries
        .iter()
        .find(|s| s.id == id)
        .map(|s| s.title.clone())
        .unwrap_or_else(|| "New session".to_string());
    tui.state.sidebar.session_id = id;
}
