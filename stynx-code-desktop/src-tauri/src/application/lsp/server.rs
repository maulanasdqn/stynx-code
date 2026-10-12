use std::collections::HashMap;
use std::path::Path;
use std::process::Stdio;
use std::sync::Arc;
use std::sync::atomic::{AtomicI64, Ordering};
use std::time::Duration;

use serde_json::{Value, json};
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::process::{Child, ChildStdin, Command};
use tokio::sync::{Mutex, oneshot};

use super::framing::{read_message, write_message};
use super::uri::to_uri;

const TIMEOUT: Duration = Duration::from_secs(15);

type Pending = Arc<std::sync::Mutex<HashMap<i64, oneshot::Sender<Result<Value, String>>>>>;

pub struct LspServer {
    stdin: Mutex<ChildStdin>,
    pending: Pending,
    next_id: AtomicI64,
    child: Mutex<Child>,
}

fn capabilities() -> Value {
    json!({
        "textDocument": {
            "synchronization": { "didSave": true, "dynamicRegistration": false },
            "completion": {
                "completionItem": { "snippetSupport": true, "documentationFormat": ["markdown", "plaintext"],
                    "resolveSupport": { "properties": ["documentation", "detail", "additionalTextEdits"] } },
                "contextSupport": true
            },
            "hover": { "contentFormat": ["markdown", "plaintext"] },
            "definition": { "linkSupport": true },
            "publishDiagnostics": { "relatedInformation": false, "versionSupport": false }
        },
        "window": { "workDoneProgress": true },
        "workspace": { "configuration": true, "workspaceFolders": true }
    })
}

impl LspServer {
    pub async fn start<N>(program: &Path, args: &[&str], root: &Path, path_var: Option<&str>, on_notify: N) -> Result<Arc<Self>, String>
    where
        N: Fn(String, Value) + Send + Sync + 'static,
    {
        let mut command = Command::new(program);
        command.args(args).current_dir(root).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).kill_on_drop(true);
        if let Some(path) = path_var {
            command.env("PATH", path);
        }
        let mut child = command.spawn().map_err(|e| format!("failed to start {}: {e}", program.display()))?;
        let stdin = child.stdin.take().ok_or("no stdin")?;
        let stdout = child.stdout.take().ok_or("no stdout")?;
        let stderr_tail: Arc<std::sync::Mutex<Vec<String>>> = Arc::default();
        if let Some(stderr) = child.stderr.take() {
            let tail = Arc::clone(&stderr_tail);
            tokio::spawn(async move {
                let mut lines = BufReader::new(stderr).lines();
                while let Ok(Some(line)) = lines.next_line().await {
                    if let Ok(mut tail) = tail.lock() {
                        tail.push(line);
                        let excess = tail.len().saturating_sub(5);
                        tail.drain(..excess);
                    }
                }
            });
        }
        let server = Arc::new(Self { stdin: Mutex::new(stdin), pending: Arc::default(), next_id: AtomicI64::new(1), child: Mutex::new(child) });
        let reader_server = Arc::clone(&server);
        tokio::spawn(async move {
            let mut reader = BufReader::new(stdout);
            while let Ok(Some(message)) = read_message(&mut reader).await {
                reader_server.dispatch(message, &on_notify).await;
            }
            let pending: Vec<_> = reader_server.pending.lock().map(|mut p| p.drain().collect()).unwrap_or_default();
            for (_, sender) in pending {
                let _ = sender.send(Err("language server exited".into()));
            }
        });
        let uri = to_uri(root);
        let name = root.file_name().map(|n| n.to_string_lossy().to_string()).unwrap_or_default();
        let initialized = server
            .request("initialize", json!({
                "processId": std::process::id(), "rootUri": uri, "capabilities": capabilities(),
                "workspaceFolders": [{ "uri": uri, "name": name }], "clientInfo": { "name": "Stynx" }
            }))
            .await;
        if let Err(error) = initialized {
            tokio::time::sleep(Duration::from_millis(150)).await;
            let tail = stderr_tail.lock().map(|t| t.join(" · ")).unwrap_or_default();
            return Err(if tail.is_empty() { error } else { format!("{error}: {tail}") });
        }
        server.notify("initialized", json!({})).await?;
        Ok(server)
    }

    async fn dispatch<N: Fn(String, Value)>(&self, message: Value, on_notify: &N) {
        let id = message.get("id").cloned();
        match (message.get("method").and_then(Value::as_str), id) {
            (Some(method), Some(id)) => {
                let result = match method {
                    "workspace/configuration" => {
                        let count = message["params"]["items"].as_array().map_or(0, Vec::len);
                        Value::Array(vec![Value::Null; count])
                    }
                    _ => Value::Null,
                };
                let _ = self.send(&json!({ "jsonrpc": "2.0", "id": id, "result": result })).await;
            }
            (Some(method), None) => on_notify(method.to_string(), message.get("params").cloned().unwrap_or(Value::Null)),
            (None, Some(id)) => {
                let Some(id) = id.as_i64() else { return };
                let sender = self.pending.lock().ok().and_then(|mut p| p.remove(&id));
                if let Some(sender) = sender {
                    let outcome = match message.get("error") {
                        Some(error) => Err(error["message"].as_str().unwrap_or("language server error").to_string()),
                        None => Ok(message.get("result").cloned().unwrap_or(Value::Null)),
                    };
                    let _ = sender.send(outcome);
                }
            }
            (None, None) => {}
        }
    }

    async fn send(&self, message: &Value) -> Result<(), String> {
        write_message(&mut *self.stdin.lock().await, message).await
    }

    pub async fn notify(&self, method: &str, params: Value) -> Result<(), String> {
        self.send(&json!({ "jsonrpc": "2.0", "method": method, "params": params })).await
    }

    pub async fn request(&self, method: &str, params: Value) -> Result<Value, String> {
        let id = self.next_id.fetch_add(1, Ordering::Relaxed);
        let (tx, rx) = oneshot::channel();
        self.pending.lock().map_err(|e| e.to_string())?.insert(id, tx);
        self.send(&json!({ "jsonrpc": "2.0", "id": id, "method": method, "params": params })).await?;
        match tokio::time::timeout(TIMEOUT, rx).await {
            Ok(Ok(outcome)) => outcome,
            Ok(Err(_)) => Err("language server closed the request".into()),
            Err(_) => {
                self.pending.lock().map_err(|e| e.to_string())?.remove(&id);
                Err(format!("{method} timed out"))
            }
        }
    }

    pub async fn shutdown(&self) {
        let _ = tokio::time::timeout(Duration::from_secs(2), self.request("shutdown", Value::Null)).await;
        let _ = self.notify("exit", Value::Null).await;
        let _ = self.child.lock().await.kill().await;
    }
}

#[cfg(all(test, unix))]
#[path = "server_test.rs"]
mod tests;
