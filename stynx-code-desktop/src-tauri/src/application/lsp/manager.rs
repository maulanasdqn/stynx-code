use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use serde_json::{Value, json};

use super::catalog::{ServerSpec, language_id, resolve, spec_for, typescript_sdk};
use super::server::LspServer;
use super::uri::{from_uri, to_uri};
use crate::domain::ide_lsp::{LspEvent, LspLocation};

pub struct Target {
    pub server: Arc<LspServer>,
    pub uri: String,
    pub root: PathBuf,
}

impl Target {
    fn position(&self, line: u32, character: u32) -> Value {
        json!({ "textDocument": { "uri": self.uri }, "position": { "line": line, "character": character } })
    }

    pub async fn request_at(&self, method: &str, line: u32, character: u32) -> Result<Value, String> {
        self.server.request(method, self.position(line, character)).await
    }

    pub async fn request_doc(&self, method: &str, mut params: Value) -> Result<Value, String> {
        if !params.is_object() {
            params = json!({});
        }
        params["textDocument"] = json!({ "uri": self.uri });
        self.server.request(method, params).await
    }

    pub async fn resolve(&self, item: Value) -> Result<Value, String> {
        self.server.request("completionItem/resolve", item).await
    }

    pub async fn definition(&self, line: u32, character: u32) -> Result<Vec<LspLocation>, String> {
        let items = match self.request_at("textDocument/definition", line, character).await? {
            Value::Array(items) => items,
            Value::Null => Vec::new(),
            single => vec![single],
        };
        Ok(items
            .iter()
            .filter_map(|item| {
                let uri = item.get("targetUri").or_else(|| item.get("uri"))?.as_str()?;
                let range = item.get("targetSelectionRange").or_else(|| item.get("range"))?.clone();
                let absolute = from_uri(uri)?;
                let path = absolute.strip_prefix(&self.root).map(Path::to_path_buf).unwrap_or(absolute);
                Some(LspLocation { path: path.to_string_lossy().replace('\\', "/"), range })
            })
            .collect())
    }
}

enum Slot {
    Ready(Arc<LspServer>),
    Unavailable,
}

pub struct LspManager {
    root: PathBuf,
    path_var: Option<String>,
    servers: HashMap<&'static str, Slot>,
    on_event: Arc<dyn Fn(LspEvent) + Send + Sync>,
}

impl LspManager {
    pub fn new(root: PathBuf, path_var: Option<String>, on_event: Arc<dyn Fn(LspEvent) + Send + Sync>) -> Self {
        Self { root, path_var, servers: HashMap::new(), on_event }
    }

    fn uri(&self, path: &str) -> String {
        to_uri(&self.root.join(path))
    }

    fn status(&self, spec: &ServerSpec, state: &str, message: impl Into<String>) {
        (self.on_event)(LspEvent::Status { server: spec.name.into(), state: state.into(), message: message.into() });
    }

    async fn server(&mut self, path: &str) -> Option<Arc<LspServer>> {
        let spec = spec_for(path)?;
        if let Some(slot) = self.servers.get(spec.id) {
            return match slot {
                Slot::Ready(server) => Some(Arc::clone(server)),
                Slot::Unavailable => None,
            };
        }
        let Some((program, args)) = resolve(spec, self.path_var.as_deref()) else {
            let how = if spec.npm.is_empty() { spec.hint.to_string() } else { "install it from Extensions → Language Servers".into() };
            self.status(spec, "missing", format!("{} not found — {how}", spec.name));
            self.servers.insert(spec.id, Slot::Unavailable);
            return None;
        };
        self.status(spec, "starting", format!("Starting {}…", spec.name));
        let events = Arc::clone(&self.on_event);
        let root = self.root.clone();
        let name = spec.name;
        let notify = move |method: String, params: Value| forward(&events, &root, name, &method, params);
        let init = match typescript_sdk(&self.root).filter(|_| spec.needs_tsdk) {
            Some(sdk) => json!({ "typescript": { "tsdk": sdk.to_string_lossy() } }),
            None => Value::Null,
        };
        match LspServer::start(&program, args, &self.root, self.path_var.as_deref(), init, notify).await {
            Ok(server) => {
                self.status(spec, "ready", format!("{} ready", spec.name));
                self.servers.insert(spec.id, Slot::Ready(Arc::clone(&server)));
                Some(server)
            }
            Err(error) => {
                let error = if error.contains("unavailable for the active toolchain") {
                    format!("{error} — run `rustup component add rust-analyzer`")
                } else {
                    error
                };
                self.status(spec, "error", error);
                self.servers.insert(spec.id, Slot::Unavailable);
                None
            }
        }
    }

    pub async fn open(&mut self, path: &str, text: &str) -> Result<bool, String> {
        let Some(server) = self.server(path).await else { return Ok(false) };
        let doc = json!({ "uri": self.uri(path), "languageId": language_id(path), "version": 1, "text": text });
        server.notify("textDocument/didOpen", json!({ "textDocument": doc })).await?;
        Ok(true)
    }

    pub async fn change(&mut self, path: &str, text: &str, version: i64) -> Result<(), String> {
        let Some(server) = self.server(path).await else { return Ok(()) };
        let params = json!({ "textDocument": { "uri": self.uri(path), "version": version }, "contentChanges": [{ "text": text }] });
        server.notify("textDocument/didChange", params).await
    }

    pub async fn notify_doc(&mut self, path: &str, method: &str) -> Result<(), String> {
        let Some(server) = self.server(path).await else { return Ok(()) };
        server.notify(method, json!({ "textDocument": { "uri": self.uri(path) } })).await
    }

    pub async fn target(&mut self, path: &str) -> Option<Target> {
        let server = self.server(path).await?;
        Some(Target { server, uri: self.uri(path), root: self.root.clone() })
    }

    pub async fn forget(&mut self, id: &str) {
        if let Some(Slot::Ready(server)) = self.servers.remove(id) {
            server.shutdown().await;
        }
    }

    pub async fn shutdown_all(&mut self) {
        for slot in std::mem::take(&mut self.servers).into_values() {
            if let Slot::Ready(server) = slot {
                server.shutdown().await;
            }
        }
    }
}

fn forward(events: &Arc<dyn Fn(LspEvent) + Send + Sync>, root: &Path, name: &str, method: &str, params: Value) {
    match method {
        "textDocument/publishDiagnostics" => {
            let Some(absolute) = params["uri"].as_str().and_then(from_uri) else { return };
            let path = absolute.strip_prefix(root).map(Path::to_path_buf).unwrap_or(absolute);
            let diagnostics = params.get("diagnostics").cloned().unwrap_or(Value::Array(Vec::new()));
            events(LspEvent::Diagnostics { path: path.to_string_lossy().replace('\\', "/"), diagnostics });
        }
        "$/progress" => {
            let value = &params["value"];
            let state = if value["kind"] == "end" { "ready" } else { "busy" };
            let text = [value["title"].as_str(), value["message"].as_str()].into_iter().flatten().collect::<Vec<_>>().join(" ");
            let message = if state == "ready" { format!("{name} ready") } else { format!("{name}: {text}") };
            events(LspEvent::Status { server: name.into(), state: state.into(), message });
        }
        _ => {}
    }
}

#[cfg(test)]
#[path = "manager_test.rs"]
mod tests;
