use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Serialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct McpInput {
    pub key: String,
    pub label: String,
    pub description: String,
    pub required: bool,
    pub secret: bool,
    pub default: Option<String>,
    pub target: String,
}

#[derive(Serialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct McpPlan {
    pub kind: String,
    pub command: String,
    pub args: Vec<String>,
    pub inputs: Vec<McpInput>,
}

#[derive(Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct McpListing {
    pub id: String,
    pub name: String,
    pub title: String,
    pub description: String,
    pub version: String,
    pub repository: Option<String>,
    pub website: Option<String>,
    pub plan: Option<McpPlan>,
}

#[derive(Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct McpPage {
    pub servers: Vec<McpListing>,
    pub next_cursor: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct McpEntry {
    pub command: String,
    #[serde(default)]
    pub args: Vec<String>,
    #[serde(default)]
    pub env: BTreeMap<String, String>,
}

#[derive(Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct McpInstalled {
    pub name: String,
    pub scope: String,
    pub entry: McpEntry,
}
