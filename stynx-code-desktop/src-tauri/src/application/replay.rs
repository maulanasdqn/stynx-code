use std::collections::HashMap;

use stynx_code_types::{ContentBlock, Conversation, Role};

use crate::domain::models::{ImagePayload, Turn};

const REFERENCES_END: &str = "</reference_documents>";

pub fn replay(conversation: &Conversation) -> Vec<Turn> {
    let mut turns: Vec<Turn> = Vec::new();
    let mut tool_index: HashMap<String, usize> = HashMap::new();

    for message in &conversation.messages {
        let role = match message.role {
            Role::User => "user",
            Role::Assistant => "assistant",
        };
        let mut text = String::new();
        let mut images = Vec::new();

        for block in &message.content {
            match block {
                ContentBlock::Text { text: chunk } => text.push_str(chunk),
                ContentBlock::Image { media_type, data } => images.push(ImagePayload {
                    media_type: media_type.clone(),
                    data: data.clone(),
                }),
                ContentBlock::Thinking { thinking } if !thinking.trim().is_empty() => {
                    turns.push(Turn { role: "thinking".into(), text: thinking.clone(), ..Turn::default() });
                }
                ContentBlock::ToolUse { id, name, input } => {
                    flush(&mut turns, role, &mut text, &mut images);
                    tool_index.insert(id.clone(), turns.len());
                    turns.push(Turn {
                        role: "tool".into(),
                        tool_name: Some(name.clone()),
                        tool_input: Some(input.to_string()),
                        ..Turn::default()
                    });
                }
                ContentBlock::ToolResult { tool_use_id, content, is_error } => {
                    if let Some(&index) = tool_index.get(tool_use_id) {
                        turns[index].tool_output = Some(content.clone());
                        turns[index].is_error = is_error.unwrap_or(false);
                    }
                }
                ContentBlock::Thinking { .. } => {}
            }
        }
        flush(&mut turns, role, &mut text, &mut images);
    }
    turns
}

fn flush(turns: &mut Vec<Turn>, role: &str, text: &mut String, images: &mut Vec<ImagePayload>) {
    let visible = match text.find(REFERENCES_END) {
        Some(end) => text[end + REFERENCES_END.len()..].trim().to_string(),
        None => text.trim().to_string(),
    };
    if visible.is_empty() && images.is_empty() {
        text.clear();
        return;
    }
    turns.push(Turn {
        role: role.into(),
        text: visible,
        images: std::mem::take(images),
        ..Turn::default()
    });
    text.clear();
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use stynx_code_types::Message;

    fn message(role: Role, content: Vec<ContentBlock>) -> Message {
        Message { role, content }
    }

    #[test]
    fn pairs_tool_results_and_hides_references() {
        let conversation = Conversation {
            system: None,
            messages: vec![
                message(Role::User, vec![ContentBlock::Text {
                    text: "intro\n<reference_documents>\ndoc\n</reference_documents>\n\nfix it".into(),
                }]),
                message(Role::Assistant, vec![
                    ContentBlock::Thinking { thinking: "plan".into() },
                    ContentBlock::Text { text: "Running".into() },
                    ContentBlock::ToolUse { id: "t1".into(), name: "bash".into(), input: json!({"command": "ls"}) },
                ]),
                message(Role::User, vec![ContentBlock::ToolResult {
                    tool_use_id: "t1".into(),
                    content: "a.txt".into(),
                    is_error: None,
                }]),
            ],
        };
        let turns = replay(&conversation);
        let roles: Vec<&str> = turns.iter().map(|turn| turn.role.as_str()).collect();
        assert_eq!(roles, ["user", "thinking", "assistant", "tool"]);
        assert_eq!(turns[0].text, "fix it");
        assert_eq!(turns[3].tool_output.as_deref(), Some("a.txt"));
        assert!(!turns[3].is_error);
    }
}
