use std::path::PathBuf;

use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct InternEntry {
    pub name: String,
    pub provider: String,
    pub model: String,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub base_url: Option<String>,
    #[serde(default)]
    pub api_key_env: Option<String>,
}

pub fn settings_path() -> Option<PathBuf> {
    std::env::var_os("HOME").map(|home| PathBuf::from(home).join(".stynx").join("settings.json"))
}

fn read(path: &PathBuf) -> Result<Map<String, Value>, String> {
    match std::fs::read_to_string(path) {
        Ok(text) => match serde_json::from_str(&text).map_err(|e| format!("{}: {e}", path.display()))? {
            Value::Object(map) => Ok(map),
            _ => Err("settings.json is not an object".into()),
        },
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(Map::new()),
        Err(error) => Err(error.to_string()),
    }
}

fn to_value(entry: &InternEntry) -> Value {
    let mut map = Map::new();
    map.insert("name".into(), entry.name.trim().into());
    map.insert("provider".into(), entry.provider.trim().into());
    map.insert("model".into(), entry.model.trim().into());
    for (key, value) in [("description", &entry.description), ("base_url", &entry.base_url), ("api_key_env", &entry.api_key_env)] {
        if let Some(value) = value.as_deref().map(str::trim).filter(|v| !v.is_empty()) {
            map.insert(key.into(), value.into());
        }
    }
    Value::Object(map)
}

fn from_value(value: &Value) -> Option<InternEntry> {
    let text = |key: &str| value[key].as_str().map(str::to_string);
    Some(InternEntry {
        name: text("name")?,
        provider: text("provider")?,
        model: text("model").unwrap_or_default(),
        description: text("description"),
        base_url: text("base_url"),
        api_key_env: text("api_key_env"),
    })
}

pub fn list_at(path: &PathBuf) -> Result<Vec<InternEntry>, String> {
    let map = read(path)?;
    Ok(map.get("interns").and_then(Value::as_array).map(|list| list.iter().filter_map(from_value).collect()).unwrap_or_default())
}

pub fn save_at(path: &PathBuf, entry: &InternEntry, previous: Option<&str>) -> Result<(), String> {
    if entry.name.trim().is_empty() || entry.provider.trim().is_empty() || entry.model.trim().is_empty() {
        return Err("name, provider and model are required".into());
    }
    if entry.provider == "custom" && entry.base_url.as_deref().is_none_or(|u| u.trim().is_empty()) {
        return Err("a custom provider needs a base URL".into());
    }
    let mut map = read(path)?;
    let list = map.entry("interns").or_insert_with(|| Value::Array(Vec::new())).as_array_mut().ok_or("interns is not a list")?;
    let target = previous.unwrap_or(entry.name.as_str()).to_string();
    let clash = list.iter().any(|v| v["name"] == entry.name.trim() && v["name"] != target.as_str());
    if clash {
        return Err(format!("an intern named {} already exists", entry.name.trim()));
    }
    match list.iter().position(|v| v["name"] == target.as_str()) {
        Some(index) => list[index] = to_value(entry),
        None => list.push(to_value(entry)),
    }
    write(path, map)
}

pub fn delete_at(path: &PathBuf, name: &str) -> Result<(), String> {
    let mut map = read(path)?;
    if let Some(list) = map.get_mut("interns").and_then(Value::as_array_mut) {
        list.retain(|v| v["name"] != name);
    }
    write(path, map)
}

fn write(path: &PathBuf, map: Map<String, Value>) -> Result<(), String> {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    }
    let text = serde_json::to_string_pretty(&Value::Object(map)).map_err(|e| e.to_string())?;
    std::fs::write(path, text + "\n").map_err(|e| e.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn upsert_rename_and_delete_keep_other_settings() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("settings.json");
        std::fs::write(&path, r#"{"main_provider": "claude", "interns": [{"name": "netra", "provider": "custom", "model": "m", "base_url": "https://x"}]}"#).unwrap();
        let entry = InternEntry { name: "fast".into(), provider: "deepseek".into(), model: "deepseek-v4-flash".into(), ..Default::default() };
        save_at(&path, &entry, None).unwrap();
        let renamed = InternEntry { name: "quick".into(), ..entry.clone() };
        save_at(&path, &renamed, Some("fast")).unwrap();
        let names: Vec<_> = list_at(&path).unwrap().into_iter().map(|e| e.name).collect();
        assert_eq!(names, ["netra", "quick"]);
        assert!(save_at(&path, &InternEntry { name: "netra".into(), ..renamed.clone() }, Some("quick")).is_err());
        delete_at(&path, "quick").unwrap();
        let raw: Value = serde_json::from_str(&std::fs::read_to_string(&path).unwrap()).unwrap();
        assert_eq!(raw["main_provider"], "claude");
        assert_eq!(raw["interns"].as_array().unwrap().len(), 1);
        assert!(save_at(&path, &InternEntry { name: "c".into(), provider: "custom".into(), model: "m".into(), ..Default::default() }, None).is_err());
    }
}
