use stynx_code_types::ContentBlock;

/// Rough local token estimate (~4 chars per token) for a whole conversation,
/// for deciding when to compact without waiting for real provider usage data.
pub fn estimate_conversation_tokens(conversation: &stynx_code_types::Conversation) -> u64 {
    conversation
        .messages
        .iter()
        .map(|m| estimate_tokens(&m.content))
        .sum::<u64>()
        + conversation.system.as_ref().map_or(0, |s| s.len() as u64 / 4)
}

fn estimate_tokens(blocks: &[ContentBlock]) -> u64 {
    let chars: usize = blocks
        .iter()
        .map(|b| match b {
            ContentBlock::Text { text } => text.len(),
            ContentBlock::ToolUse { name, input, .. } => name.len() + input.to_string().len(),
            ContentBlock::ToolResult { content, .. } => content.len(),
            ContentBlock::Thinking { thinking } => thinking.len(),
            ContentBlock::Image { data, .. } => data.len(),
        })
        .sum();
    (chars as u64) / 4
}
