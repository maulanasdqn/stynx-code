use serde::Serialize;

#[derive(Serialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct GitEntry {
    pub path: String,
    pub orig_path: Option<String>,
    pub status: String,
}

#[derive(Serialize, Clone, Debug, Default, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct GitStatus {
    pub is_repo: bool,
    pub prefix: String,
    pub branch: Option<String>,
    pub detached: bool,
    pub ahead: Option<u32>,
    pub behind: Option<u32>,
    pub staged: Vec<GitEntry>,
    pub unstaged: Vec<GitEntry>,
    pub conflicted: Vec<GitEntry>,
}

#[derive(Serialize, Clone, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct GitDiff {
    pub path: String,
    pub staged: bool,
    pub original: String,
    pub modified: String,
    pub binary: bool,
    pub too_large: bool,
}
