use stynx_code_types::{ContentBlock, Conversation, EngineEvent, Role};
use super::{ConversationState, DiffLine, DiffLineKind, DisplayMessage, DisplayToolUse, InputState, MessagePart, ModalState, ToastState, ToolUseStatus};

/// USD price per million tokens `(input, output)` for a given model.
///
/// Matched on substrings so provider prefixes / date suffixes still resolve
/// (e.g. `deepseek-chat`, `claude-opus-4-20250514`). Falls back to Claude
/// Sonnet rates for unknown models.
fn model_pricing(model: &str) -> (f64, f64) {
    let m = model.to_ascii_lowercase();
    match () {
        // DeepSeek (per their published API pricing, cache-miss rates)
        _ if m.contains("deepseek-reasoner") || m.contains("deepseek-r1") => (0.55, 2.19),
        _ if m.contains("deepseek") => (0.27, 1.10),
        // Anthropic
        _ if m.contains("fable") || m.contains("mythos") => (10.0, 50.0),
        _ if m.contains("opus-5-5") => (4.0, 20.0),
        _ if m.contains("opus-5") || m.contains("opus-4-5") || m.contains("opus-4-6") || m.contains("opus-4-7") || m.contains("opus-4-8") => (5.0, 25.0),
        _ if m.contains("opus") => (15.0, 75.0),
        _ if m.contains("haiku-5") => (0.10, 0.50),
        _ if m.contains("haiku-4-5") => (1.0, 5.0),
        _ if m.contains("haiku") => (0.80, 4.0),
        _ if m.contains("sonnet-5") => (2.0, 10.0),
        _ if m.contains("sonnet") => (3.0, 15.0),
        // OpenAI (common ones)
        _ if m.contains("gpt-4o-mini") => (0.15, 0.60),
        _ if m.contains("gpt-4o") => (2.50, 10.0),
        // Unknown → assume Sonnet-class so the figure is non-zero but sane.
        _ => (3.0, 15.0),
    }
}

#[derive(Clone)]
pub struct SessionSummary {
    pub id: String,
    pub title: String,
    pub updated_at: u64,
    pub pinned: bool,
}

pub struct SidebarState {
    pub visible: bool,
    pub title: String,
    pub session_id: String,
    pub version: String,
    pub sessions: Vec<SessionSummary>,
}

impl SidebarState {
    pub fn new() -> Self {
        Self {
            visible: true,
            title: "New session".to_string(),
            session_id: String::new(),
            version: env!("CARGO_PKG_VERSION").to_string(),
            sessions: Vec::new(),
        }
    }
}

impl Default for SidebarState {
    fn default() -> Self { Self::new() }
}

pub struct AppState {
    pub input: InputState,
    pub conversation: ConversationState,
    pub modal: ModalState,
    pub sidebar: SidebarState,
    pub toasts: ToastState,
    pub model_name: String,
    pub permission_mode: String,
    pub total_cost: f64,
    pub git_branch: Option<String>,
    pub cwd: String,
    pub is_streaming: bool,
    pub spinner_frame: usize,
    pub spinner_tick: u8,
    pub total_input: u64,
    pub total_output: u64,
    pub last_input_tokens: u64,
    pub context_window: u64,
    pub recent_models: Vec<String>,
    pub tool_details: bool,

    pub live_thinking: String,

    pub sub_agents: Vec<SubAgentState>,

    /// Tool calls of the last turn as (kind, failed).
    pub last_summary: Option<Vec<(String, bool)>>,

    pub diff_panel: DiffPanelState,
    /// Set when a tool starts, so the next text segment of the same turn
    /// begins on a new paragraph instead of running into the previous one.
    pub text_break: bool,

    pub is_pending: bool,
    pub elapsed_secs: u64,
    pub stale_warned: bool,
}

#[derive(Default)]
pub struct DiffPanelState {
    pub scroll: usize,
    pub focused: bool,
}

const MAX_QUEUE: usize = 8;

#[derive(Clone)]
pub struct SubAgentState {
    pub label: String,
    pub current: String,
    pub queue: Vec<String>,
    pub step: usize,
}

impl SubAgentState {
    pub fn new(label: String, summary: String) -> Self {
        Self { label, current: summary.clone(), queue: vec![summary], step: 1 }
    }

    pub fn push(&mut self, summary: String) {
        self.step += 1;
        self.current = summary.clone();
        self.queue.push(summary);
        if self.queue.len() > MAX_QUEUE {
            self.queue.remove(0);
        }
    }

    pub fn display_lines(&self) -> usize {
        1 + self.queue.len().min(5)
    }
}

impl AppState {
    pub fn push_recent_model(&mut self, id: &str) {
        self.recent_models.retain(|m| m != id);
        self.recent_models.insert(0, id.to_string());
        if self.recent_models.len() > 8 {
            self.recent_models.truncate(8);
        }
    }

    pub fn cycle_recent_model(&mut self) -> Option<String> {
        if self.recent_models.len() < 2 {
            return None;
        }
        let next = self.recent_models.remove(1);
        self.recent_models.insert(0, next.clone());
        Some(next)
    }
}

impl AppState {
    pub fn new() -> Self {
        Self {
            input: InputState::new(),
            conversation: ConversationState::new(),
            modal: ModalState::new(),
            sidebar: SidebarState::new(),
            toasts: ToastState::new(),
            model_name: String::from("claude-opus-4-8"),
            permission_mode: String::from("Normal"),
            total_cost: 0.0,
            git_branch: None,
            cwd: std::env::current_dir().ok()
                .and_then(|p| p.to_str().map(|s| s.to_string()))
                .unwrap_or_default(),
            is_streaming: false,
            spinner_frame: 0,
            spinner_tick: 0,
            total_input: 0,
            total_output: 0,
            last_input_tokens: 0,
            context_window: 200_000,
            recent_models: Vec::new(),
            tool_details: true,
            live_thinking: String::new(),
            sub_agents: Vec::new(),
            last_summary: None,
            diff_panel: DiffPanelState::default(),
            text_break: false,
            is_pending: false,
            elapsed_secs: 0,
            stale_warned: false,
        }
    }

    pub fn push_user_message(&mut self, text: impl Into<String>) {
        self.last_summary = None;
        self.conversation.messages.push(DisplayMessage {
            role: "user".to_string(),
            content: text.into(),
            thinking: String::new(),
            tool_uses: Vec::new(),
            is_streaming: false, parts: Vec::new(), footer: None,
        });
        self.conversation.auto_scroll = true;
    }

    /// Attach the turn summary to the last assistant message.
    /// Put the turn summary under the current turn's reply. A turn stopped
    /// before any output has no reply yet, so it gets an empty one; the footer
    /// must never land on an earlier turn.
    pub fn set_turn_footer(&mut self, text: impl Into<String>) {
        let messages = &mut self.conversation.messages;
        let turn_start = messages.iter().rposition(|m| m.role == "user").map_or(0, |i| i + 1);
        match messages[turn_start..].iter_mut().rev().find(|m| m.role == "assistant") {
            Some(m) => m.footer = Some(text.into()),
            None => messages.push(DisplayMessage {
                role: "assistant".to_string(), content: String::new(),
                thinking: String::new(), tool_uses: Vec::new(), is_streaming: false,
                parts: Vec::new(), footer: Some(text.into()),
            }),
        }
    }

    /// Rebuild the transcript from a saved conversation. The saved blocks are
    /// replayed as engine events, so a restored turn renders like a live one.
    pub fn restore_conversation(&mut self, conversation: &Conversation) {
        let mut replay = AppState::new();
        replay.cwd = self.cwd.clone();
        let mut tool_names: std::collections::HashMap<&str, &str> = std::collections::HashMap::new();

        for msg in &conversation.messages {
            match msg.role {
                Role::User => {
                    for block in &msg.content {
                        if let ContentBlock::ToolResult { tool_use_id, content, is_error } = block {
                            if let Some(name) = tool_names.get(tool_use_id.as_str()) {
                                replay.apply_engine_event(EngineEvent::ToolResult {
                                    name: name.to_string(),
                                    output: content.clone(),
                                    is_error: is_error.unwrap_or(false),
                                });
                            }
                        }
                    }
                    let text = msg.content.iter().filter_map(|b| match b {
                        ContentBlock::Text { text } => Some(text.as_str()),
                        _ => None,
                    }).collect::<Vec<_>>().join("");
                    if !text.is_empty() {
                        replay.apply_engine_event(EngineEvent::TurnComplete);
                        replay.push_user_message(text);
                    }
                }
                Role::Assistant => {
                    for block in &msg.content {
                        match block {
                            ContentBlock::Text { text } if !text.is_empty() => {
                                replay.apply_engine_event(EngineEvent::TextDelta(text.clone()));
                            }
                            ContentBlock::Thinking { thinking } => {
                                replay.apply_engine_event(EngineEvent::ThinkingDelta(thinking.clone()));
                            }
                            ContentBlock::ToolUse { id, name, input } => {
                                tool_names.insert(id, name);
                                replay.apply_engine_event(EngineEvent::ToolStart { name: name.clone(), id: id.clone() });
                                replay.apply_engine_event(EngineEvent::ToolInput { json_chunk: input.to_string() });
                            }
                            _ => {}
                        }
                    }
                }
            }
        }
        replay.apply_engine_event(EngineEvent::TurnComplete);

        // A call without a saved result was cut off; never leave it spinning.
        for tool in replay.conversation.messages.iter_mut().flat_map(|m| m.tool_uses.iter_mut()) {
            if tool.status == ToolUseStatus::Running {
                tool.status = ToolUseStatus::Error;
                tool.output_preview = "no result (interrupted)".to_string();
            }
        }
        self.conversation.messages = replay.conversation.messages;
    }

    pub fn push_system_message(&mut self, text: impl Into<String>) {
        self.conversation.messages.push(DisplayMessage {
            role: "system".to_string(),
            content: text.into(),
            thinking: String::new(),
            tool_uses: Vec::new(),
            is_streaming: false, parts: Vec::new(), footer: None,
        });
        self.conversation.auto_scroll = true;
    }

    pub fn apply_engine_event(&mut self, event: EngineEvent) {
        self.is_pending = false;
        match event {
            EngineEvent::TextDelta(text) => {
                self.is_streaming = true;
                let text_break = std::mem::take(&mut self.text_break);
                match self.conversation.messages.last_mut() {
                    Some(m) if m.role == "assistant" && m.is_streaming => {
                        if text_break && !m.content.trim().is_empty() && !m.content.ends_with("\n\n") {
                            let trimmed = m.content.trim_end_matches('\n').len();
                            m.content.truncate(trimmed);
                            m.content.push_str("\n\n");
                        }
                        m.content.push_str(&text);
                        match m.parts.last_mut() {
                            Some(MessagePart::Text(part)) => part.push_str(&text),
                            _ => m.parts.push(MessagePart::Text(text)),
                        }
                    }
                    _ => self.conversation.messages.push(DisplayMessage {
                        role: "assistant".to_string(), content: text.clone(),
                        thinking: String::new(), tool_uses: Vec::new(), is_streaming: true, parts: vec![MessagePart::Text(text)], footer: None,
                    }),
                }
            }
            EngineEvent::ThinkingDelta(text) => {
                self.is_streaming = true;
                self.live_thinking.push_str(&text);
                // Make sure a streaming assistant bubble exists so the "Stynx"
                // header and its thinking indicator render while only reasoning
                // (no text/tools yet) has streamed in.
                if !matches!(self.conversation.messages.last(),
                    Some(m) if m.role == "assistant" && m.is_streaming)
                {
                    self.conversation.messages.push(DisplayMessage {
                        role: "assistant".to_string(), content: String::new(),
                        thinking: String::new(), tool_uses: Vec::new(), is_streaming: true, parts: Vec::new(), footer: None,
                    });
                }
            }
            EngineEvent::ToolStart { name, .. } => {
                self.is_streaming = true;
                self.text_break = true;
                let tool = DisplayToolUse {
                    name,
                    status: ToolUseStatus::Running,
                    output_preview: String::new(),
                    input_json: String::new(),
                    input_summary: String::new(),
                    output_excerpt: Vec::new(),
                    diff: Vec::new(),
                    sub_progress: Vec::new(),
                    live_output: String::new(),
                };
                match self.conversation.messages.last_mut().filter(|m| m.role == "assistant") {
                    Some(m) => {
                        m.parts.push(MessagePart::Tool(m.tool_uses.len()));
                        m.tool_uses.push(tool);
                    }
                    None => self.conversation.messages.push(DisplayMessage {
                        role: "assistant".to_string(), content: String::new(),
                        thinking: String::new(), tool_uses: vec![tool], is_streaming: true, parts: vec![MessagePart::Tool(0)], footer: None,
                    }),
                }
            }
            EngineEvent::ToolInput { json_chunk } => {
                if let Some(m) = self.conversation.messages.last_mut() {
                    if let Some(t) = m.tool_uses.iter_mut().rev()
                        .find(|t| t.status == ToolUseStatus::Running)
                    {
                        t.input_json.push_str(&json_chunk);
                        t.input_summary = summarize_tool_input(&t.name, &t.input_json, &self.cwd);
                    }
                }
            }
            EngineEvent::ToolOutput { name, chunk } => {
                if let Some(m) = self.conversation.messages.last_mut() {
                    if let Some(t) = m.tool_uses.iter_mut().rev()
                        .find(|t| t.name == name && t.status == ToolUseStatus::Running)
                    {
                        t.live_output.push_str(&chunk);
                        // Cap the retained buffer so a noisy command can't grow it forever.
                        if t.live_output.len() > 64_000 {
                            let cut = t.live_output.len() - 48_000;
                            t.live_output = t.live_output.split_off(cut);
                        }
                        let clean = crate::util::strip_ansi(&t.live_output);
                        t.output_preview = clean
                            .lines()
                            .rev()
                            .find(|l| !l.trim().is_empty())
                            .unwrap_or("")
                            .chars()
                            .take(80)
                            .collect();
                        t.output_excerpt = excerpt_lines(&clean, 6, 200);
                    }
                }
            }
            EngineEvent::ToolResult { name, output, is_error } => {
                let clean_output = crate::util::strip_ansi(&output);
                let preview_limit = if is_error { 400 } else { 80 };
                if let Some(m) = self.conversation.messages.last_mut() {
                    // Results arrive in call order, so the oldest running call of
                    // this name owns the result (parallel reads share a name).
                    if let Some(t) = m.tool_uses.iter_mut()
                        .find(|t| t.name == name && t.status == ToolUseStatus::Running) {
                        t.status = if is_error { ToolUseStatus::Error } else { ToolUseStatus::Completed };
                        t.output_preview = clean_output.lines().next().unwrap_or("").chars().take(preview_limit).collect();
                        if t.input_summary.is_empty() {
                            t.input_summary = summarize_tool_input(&t.name, &t.input_json, &self.cwd);
                        }
                        t.output_excerpt = excerpt_lines(&clean_output, 6, 200);
                        if t.name == "file_edit" || t.name == "file_write" {
                            t.diff = build_diff_for(&t.name, &t.input_json);
                        }

                        if matches!(t.name.as_str(), "read" | "grep" | "glob") && !is_error {
                            let n = clean_output.lines().filter(|l| !l.trim().is_empty()).count();
                            if n > 0 && !t.input_summary.contains("(")  {
                                let unit = if n == 1 { "line" } else { "lines" };
                                t.input_summary = format!("{}  ({n} {unit})", t.input_summary);
                            }
                        }
                        if t.name == "ask_user_question" && !is_error {
                            let answer = first_line(clean_output.trim());
                            if !answer.is_empty() {
                                t.input_summary = if t.input_summary.is_empty() {
                                    shorten(answer, 80)
                                } else {
                                    format!("{} → {}", t.input_summary, shorten(answer, 80))
                                };
                            }
                        }
                    }
                }
                // The failed tool line shows its own error; a separate message
                // would also split the turn into two assistant messages.
                if is_error {
                    tracing::error!(tool = %name, output = %clean_output, "tool returned error");
                }
            }
            EngineEvent::TurnComplete => {
                self.is_streaming = false;
                let tool_summary = self.conversation.messages.last().and_then(|m| {
                    if m.role != "assistant" { return None; }
                    if m.tool_uses.is_empty() { return None; }
                    let parts: Vec<(String, bool)> = m.tool_uses.iter()
                        .map(|t| (tool_kind(&t.name).to_string(), t.status == ToolUseStatus::Error))
                        .collect();
                    if parts.is_empty() { None } else { Some(parts) }
                });
                if let Some(m) = self.conversation.messages.last_mut() {
                    m.is_streaming = false;
                    if !self.live_thinking.is_empty() && m.role == "assistant" {
                        if m.thinking.is_empty() {
                            m.thinking = std::mem::take(&mut self.live_thinking);
                        } else {
                            m.thinking.push_str(&self.live_thinking);
                            self.live_thinking.clear();
                        }
                    }
                }
                self.live_thinking.clear();
                if let Some(summary) = tool_summary {
                    self.last_summary = Some(summary);
                    self.conversation.auto_scroll = true;
                }
            }
            EngineEvent::Usage { input_tokens, output_tokens } => {
                if input_tokens > 0 {
                    self.total_input += input_tokens;
                    self.last_input_tokens = input_tokens;
                }
                if output_tokens > 0 { self.total_output += output_tokens; }
                let (in_price, out_price) = model_pricing(&self.model_name);
                self.total_cost = (self.total_input as f64 * in_price
                    + self.total_output as f64 * out_price) / 1_000_000.0;
            }
            EngineEvent::Error(e) => {
                self.is_streaming = false;
                self.conversation.messages.push(DisplayMessage {
                    role: "error".to_string(), content: e,
                    thinking: String::new(), tool_uses: Vec::new(), is_streaming: false, parts: Vec::new(), footer: None,
                });
            }
            EngineEvent::ModeChanged { mode } => {
                self.permission_mode = mode.label().to_string();
            }
            EngineEvent::SubAgentProgress { label, summary } => {
                match self.sub_agents.iter_mut().find(|a| a.label == label) {
                    Some(a) => a.push(summary.clone()),
                    None => self.sub_agents.push(SubAgentState::new(label.clone(), summary.clone())),
                }
                if let Some(m) = self.conversation.messages.last_mut() {
                    if let Some(t) = m.tool_uses.iter_mut().rev()
                        .find(|t| t.status == ToolUseStatus::Running
                            && (t.name == "agent" || t.name == "explore"
                                || t.name.starts_with("delegate_to_")))
                    {
                        t.sub_progress.push(format!("{label}: {summary}"));
                        if t.sub_progress.len() > 50 {
                            let drop = t.sub_progress.len() - 50;
                            t.sub_progress.drain(0..drop);
                        }
                    }
                }
            }
            EngineEvent::SubAgentDone { label } => {
                let last = self
                    .sub_agents
                    .iter()
                    .find(|a| a.label == label)
                    .map(|a| a.current.clone())
                    .filter(|s| !s.trim().is_empty());
                self.sub_agents.retain(|a| a.label != label);
                let msg = match last {
                    Some(s) => format!("{label} done\n{s}"),
                    None => format!("{label} done"),
                };
                self.toasts.success(msg);
            }
            EngineEvent::RetryNotice { attempt, max_attempts, delay_ms, message } => {
                self.toasts.warn(format!(
                    "provider busy ({message}) — retry {attempt}/{max_attempts} in {}s",
                    delay_ms / 1000
                ));
            }
            _ => {}
        }
    }
}

impl Default for AppState {
    fn default() -> Self { Self::new() }
}

/// Short noun for a tool call, used by the turn summary ("2 commands · 1 edit").
fn tool_kind(name: &str) -> &'static str {
    match name {
        "bash" => "command",
        "read" => "read",
        "file_edit" => "edit",
        "file_write" => "write",
        "grep" | "glob" => "search",
        "web_fetch" => "fetch",
        "web_search" => "web search",
        "todo_write" | "todo_read" => "todo update",
        "ask_user_question" => "question",
        n if n == "agent" || n == "explore" || n.starts_with("delegate_to_") => "subagent",
        _ => "tool call",
    }
}

fn try_parse(json: &str) -> Option<serde_json::Value> {
    if json.trim().is_empty() { return None; }
    serde_json::from_str(json).ok()
}

fn shorten(s: &str, max: usize) -> String {
    let trimmed = s.trim();
    if trimmed.chars().count() <= max {
        return trimmed.to_string();
    }
    let mut out: String = trimmed.chars().take(max.saturating_sub(1)).collect();
    out.push('…');
    out
}

fn first_line(s: &str) -> &str {
    s.lines().next().unwrap_or("")
}

/// One-line summary of a tool call's input. Paths under `cwd` are shown relative to it.
pub fn summarize_tool_input(tool: &str, json: &str, cwd: &str) -> String {
    let parsed = try_parse(json);
    let rel = |path: String| -> String {
        match path.strip_prefix(cwd) {
            Some(rest) if !cwd.is_empty() && (rest.is_empty() || rest.starts_with('/')) => {
                let rest = rest.trim_start_matches('/');
                if rest.is_empty() { ".".to_string() } else { rest.to_string() }
            }
            _ => path,
        }
    };
    let get = |k: &str| -> String {
        parsed
            .as_ref()
            .and_then(|v| v.get(k))
            .and_then(|v| v.as_str())
            .map(|s| s.to_string())
            .unwrap_or_default()
    };
    match tool {
        "bash" => {
            if parsed.as_ref().and_then(|v| v.get("list")).and_then(|v| v.as_bool()).unwrap_or(false) {
                return "list background processes".to_string();
            }
            if let Some(h) = parsed.as_ref().and_then(|v| v.get("kill")).and_then(|v| v.as_str()) {
                return format!("kill {h}");
            }
            if let Some(h) = parsed.as_ref().and_then(|v| v.get("status")).and_then(|v| v.as_str()) {
                return format!("status {h}");
            }
            let cmd = get("command");
            if cmd.is_empty() { return String::new(); }
            let bg = parsed.as_ref().and_then(|v| v.get("background")).and_then(|v| v.as_bool()).unwrap_or(false);
            let suffix = if bg { "  &" } else { "" };
            format!("$ {}{suffix}", shorten(first_line(&cmd), 140))
        }
        "read" => {
            let path = rel(get("file_path"));
            if path.is_empty() { return String::new(); }
            let offset = parsed.as_ref().and_then(|v| v.get("offset")).and_then(|v| v.as_u64());
            let limit = parsed.as_ref().and_then(|v| v.get("limit")).and_then(|v| v.as_u64());
            match (offset, limit) {
                (Some(o), Some(l)) => format!("{path}:{o}-{}", o + l),
                (Some(o), None) => format!("{path}:{o}-"),
                _ => path,
            }
        }
        "file_write" => {
            let path = rel(get("file_path"));
            let content_len = parsed
                .as_ref()
                .and_then(|v| v.get("content"))
                .and_then(|v| v.as_str())
                .map(|s| s.lines().count())
                .unwrap_or(0);
            if path.is_empty() { return String::new(); }
            if content_len > 0 { format!("{path}  ({content_len} lines)") } else { path }
        }
        "file_edit" => {
            let path = rel(get("file_path"));
            if path.is_empty() { return String::new(); }
            path
        }
        "glob" => {
            let pattern = get("pattern");
            if pattern.is_empty() { return String::new(); }
            shorten(&pattern, 140)
        }
        "grep" => {
            let pattern = get("pattern");
            let path = rel(get("path"));
            let mut s = shorten(&pattern, 100);
            if !path.is_empty() {
                s.push_str(" in ");
                s.push_str(&shorten(&path, 40));
            }
            s
        }
        "web_fetch" | "web_search" => {
            let url = get("url");
            let q = get("query");
            if !url.is_empty() { shorten(&url, 140) } else { shorten(&q, 140) }
        }
        "delegate_to_intern" => {
            let task = get("task");
            shorten(first_line(&task), 140)
        }
        "agent" | "explore" => {
            let task = get("task");
            shorten(first_line(&task), 140)
        }
        "todo_write" | "todo_read" => String::new(),
        "ask_user_question" => {
            let single = get("question");
            let first = parsed
                .as_ref()
                .and_then(|v| v.get("questions"))
                .and_then(|q| q.get(0))
                .and_then(|q| q.get("question"))
                .and_then(|q| q.as_str())
                .unwrap_or("");
            let q = if single.is_empty() { first } else { single.as_str() };
            shorten(first_line(q), 80)
        }
        _ => {

            parsed
                .as_ref()
                .and_then(|v| v.as_object())
                .and_then(|m| m.values().find_map(|v| v.as_str()))
                .map(|s| shorten(first_line(s), 120))
                .unwrap_or_default()
        }
    }
}

pub fn build_diff_for(tool: &str, input_json: &str) -> Vec<DiffLine> {
    let Some(v) = try_parse(input_json) else { return Vec::new(); };
    let max_lines = 14usize;
    let context_lines = 2usize;

    if tool == "file_write" {
        let content = v.get("content").and_then(|c| c.as_str()).unwrap_or("");
        return content
            .lines()
            .take(max_lines)
            .map(|l| DiffLine {
                kind: DiffLineKind::Added,
                text: l.to_string(),
            })
            .collect();
    }

    let old_s = v.get("old_string").and_then(|s| s.as_str()).unwrap_or("");
    let new_s = v.get("new_string").and_then(|s| s.as_str()).unwrap_or("");

    let old_lines: Vec<&str> = old_s.split('\n').collect();
    let new_lines: Vec<&str> = new_s.split('\n').collect();

    let mut p = 0;
    while p < old_lines.len() && p < new_lines.len() && old_lines[p] == new_lines[p] {
        p += 1;
    }

    let mut s = 0;
    while s < old_lines.len() - p && s < new_lines.len() - p
        && old_lines[old_lines.len() - 1 - s] == new_lines[new_lines.len() - 1 - s]
    {
        s += 1;
    }

    let mut out: Vec<DiffLine> = Vec::new();

    let ctx_start = p.saturating_sub(context_lines);
    for line in &old_lines[ctx_start..p] {
        out.push(DiffLine { kind: DiffLineKind::Context, text: line.to_string() });
    }

    for line in &old_lines[p..old_lines.len() - s] {
        out.push(DiffLine { kind: DiffLineKind::Removed, text: line.to_string() });
        if out.len() >= max_lines { return out; }
    }

    for line in &new_lines[p..new_lines.len() - s] {
        out.push(DiffLine { kind: DiffLineKind::Added, text: line.to_string() });
        if out.len() >= max_lines { return out; }
    }

    let ctx_end_start = old_lines.len() - s;
    let ctx_end_stop = (ctx_end_start + context_lines).min(old_lines.len());
    for line in &old_lines[ctx_end_start..ctx_end_stop] {
        out.push(DiffLine { kind: DiffLineKind::Context, text: line.to_string() });
        if out.len() >= max_lines { return out; }
    }

    out
}

pub fn excerpt_lines(text: &str, max_lines: usize, max_width: usize) -> Vec<String> {
    let mut lines: Vec<String> = text
        .lines()
        .filter(|l| !l.trim().is_empty())
        .take(max_lines + 1)
        .map(|l| {
            if l.chars().count() > max_width {
                let mut s: String = l.chars().take(max_width.saturating_sub(1)).collect();
                s.push('…');
                s
            } else {
                l.to_string()
            }
        })
        .collect();
    let total = text.lines().filter(|l| !l.trim().is_empty()).count();
    if total > max_lines {
        lines.truncate(max_lines);
        lines.push(format!("… +{} more lines", total - max_lines));
    }
    lines
}

#[cfg(test)]
mod text_break_tests {
    use super::*;

    #[test]
    fn text_after_a_tool_starts_a_new_paragraph() {
        let mut state = AppState::new();
        state.apply_engine_event(EngineEvent::TextDelta("Checking the repo.".into()));
        state.apply_engine_event(EngineEvent::ToolStart { name: "read".into(), id: "t1".into() });
        state.apply_engine_event(EngineEvent::TextDelta("README matches.".into()));
        state.apply_engine_event(EngineEvent::TextDelta(" Done.".into()));
        assert_eq!(state.conversation.messages.last().unwrap().content, "Checking the repo.\n\nREADME matches. Done.");
    }

    #[test]
    fn parts_keep_text_and_tools_in_stream_order() {
        let mut state = AppState::new();
        state.apply_engine_event(EngineEvent::TextDelta("Looking.".into()));
        state.apply_engine_event(EngineEvent::ToolStart { name: "read".into(), id: "t1".into() });
        state.apply_engine_event(EngineEvent::ToolStart { name: "grep".into(), id: "t2".into() });
        state.apply_engine_event(EngineEvent::TextDelta("Found ".into()));
        state.apply_engine_event(EngineEvent::TextDelta("it.".into()));
        let parts = &state.conversation.messages.last().unwrap().parts;
        assert!(matches!(&parts[..], [
            MessagePart::Text(a), MessagePart::Tool(0), MessagePart::Tool(1), MessagePart::Text(b),
        ] if a == "Looking." && b == "Found it."));
    }
}

#[cfg(test)]
mod restore_tests {
    use super::*;
    use stynx_code_types::Message;

    #[test]
    fn restored_session_keeps_tool_rows_in_one_turn() {
        let cwd = "/work/proj";
        let conversation = Conversation {
            system: None,
            messages: vec![
                Message::user("fix it"),
                Message::assistant(vec![
                    ContentBlock::Text { text: "Looking.".into() },
                    ContentBlock::ToolUse {
                        id: "t1".into(),
                        name: "read".into(),
                        input: serde_json::json!({ "file_path": "/work/proj/src/a.rs" }),
                    },
                    ContentBlock::ToolUse {
                        id: "t2".into(),
                        name: "read".into(),
                        input: serde_json::json!({ "file_path": "/work/proj/src/gone.rs" }),
                    },
                ]),
                Message {
                    role: Role::User,
                    content: vec![
                        ContentBlock::ToolResult { tool_use_id: "t1".into(), content: "fn a() {}".into(), is_error: None },
                        ContentBlock::ToolResult { tool_use_id: "t2".into(), content: "not found".into(), is_error: Some(true) },
                    ],
                },
                Message::assistant(vec![ContentBlock::Text { text: "Done.".into() }]),
            ],
        };

        let mut state = AppState::new();
        state.cwd = cwd.into();
        state.restore_conversation(&conversation);

        let messages = &state.conversation.messages;
        assert_eq!(messages.len(), 2);
        let turn = &messages[1];
        assert!(!turn.is_streaming);
        assert_eq!(turn.tool_uses.len(), 2);
        assert_eq!(turn.tool_uses[0].status, ToolUseStatus::Completed);
        assert_eq!(turn.tool_uses[0].input_summary, "src/a.rs  (1 line)");
        assert_eq!(turn.tool_uses[1].status, ToolUseStatus::Error);
        assert!(matches!(&turn.parts[..], [
            MessagePart::Text(a), MessagePart::Tool(0), MessagePart::Tool(1), MessagePart::Text(b),
        ] if a == "Looking." && b == "Done."));
    }

    #[test]
    fn a_tool_without_a_saved_result_is_not_left_running() {
        let conversation = Conversation {
            system: None,
            messages: vec![
                Message::user("run it"),
                Message::assistant(vec![ContentBlock::ToolUse {
                    id: "t1".into(),
                    name: "bash".into(),
                    input: serde_json::json!({ "command": "sleep 100" }),
                }]),
            ],
        };
        let mut state = AppState::new();
        state.restore_conversation(&conversation);
        assert_eq!(state.conversation.messages[1].tool_uses[0].status, ToolUseStatus::Error);
    }

    #[test]
    fn paths_outside_cwd_stay_absolute() {
        let json = r#"{"file_path":"/work/project-b/x.rs"}"#;
        assert_eq!(summarize_tool_input("read", json, "/work/project"), "/work/project-b/x.rs");
        assert_eq!(summarize_tool_input("read", r#"{"file_path":"/work/project/x.rs"}"#, "/work/project"), "x.rs");
    }
}

#[cfg(test)]
mod footer_tests {
    use super::*;

    #[test]
    fn a_turn_stopped_before_output_keeps_the_previous_footer() {
        let mut state = AppState::new();
        state.push_user_message("first");
        state.apply_engine_event(EngineEvent::TextDelta("Reply.".into()));
        state.apply_engine_event(EngineEvent::TurnComplete);
        state.set_turn_footer("first footer");
        state.push_user_message("second");
        state.set_turn_footer("interrupted");

        let messages = &state.conversation.messages;
        assert_eq!(messages[1].footer.as_deref(), Some("first footer"));
        assert_eq!(messages.last().unwrap().role, "assistant");
        assert_eq!(messages.last().unwrap().footer.as_deref(), Some("interrupted"));
    }
}
