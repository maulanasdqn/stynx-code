use serde::Serialize;
use serde_json::Value;

#[derive(Serialize, Clone, Debug)]
#[serde(tag = "type", rename_all = "camelCase", rename_all_fields = "camelCase")]
pub enum LspEvent {
    Diagnostics { path: String, diagnostics: Value },
    Status { server: String, state: String, message: String },
    InstallLog { id: String, line: String },
}

#[derive(Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ServerInfo {
    pub id: String,
    pub name: String,
    pub extensions: Vec<String>,
    pub location: Option<String>,
    pub managed: bool,
    pub installable: bool,
    pub hint: String,
}

#[derive(Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LspLocation {
    pub path: String,
    pub range: Value,
}
