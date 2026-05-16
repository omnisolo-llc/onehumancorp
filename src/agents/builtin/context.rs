use crate::types::{Message, Role};
use std::sync::Arc;
use dashmap::DashMap;

/// ContextManager handles the conversation history and applies industry-standard
/// optimizations like Observation Masking (JetBrains) and ACON Research Metric.
pub struct ContextManager {
    /// Full conversation history.
    pub messages: Vec<Message>,
    /// Persistent store for full tool outputs, indexed by tool_call_id.
    pub observation_store: Arc<DashMap<String, String>>,
}

impl ContextManager {
    pub fn new() -> Self {
        Self {
            messages: Vec::new(),
            observation_store: Arc::new(DashMap::new()),
        }
    }

    pub fn add_message(&mut self, msg: Message) {
        // If it's a tool message, store the full content in the observation store
        if msg.role == Role::Tool {
            for tr in &msg.tool_results {
                if !tr.tool_call_id.is_empty() && tr.error.is_empty() {
                    self.observation_store.insert(tr.tool_call_id.clone(), tr.content.clone());
                }
            }
        }
        self.messages.push(msg);
    }

    pub fn set_messages(&mut self, messages: Vec<Message>) {
        self.messages = messages;
        // Re-populate observation store from history if needed
        for m in &self.messages {
            if m.role == Role::Tool {
                for tr in &m.tool_results {
                    if !tr.tool_call_id.is_empty() && tr.error.is_empty() {
                        self.observation_store.insert(tr.tool_call_id.clone(), tr.content.clone());
                    }
                }
            }
        }
    }

    /// Returns the messages formatted for the LLM, applying masking and ACON strategies.
    pub fn get_messages_for_llm(
        &self,
        enable_observation_masking: bool,
        masking_threshold: usize,
        masking_size_limit: usize,
        enable_acon: bool,
    ) -> Vec<Message> {
        let mut processed_messages = self.messages.clone();
        let msg_count = processed_messages.len();

        // 1. ACON Research Metric: Prioritize reasoning traces over raw tool outputs.
        // Usually applied when context window is getting full, but here we can apply it
        // to messages older than the last 2 turns if enabled.
        if enable_acon && msg_count > 3 {
            let threshold = msg_count - 2;
            for i in 0..threshold {
                if processed_messages[i].role == Role::Tool {
                    for tr in &mut processed_messages[i].tool_results {
                        if tr.error.is_empty() && !tr.content.is_empty() && !tr.content.starts_with("[ACON:") {
                            tr.content = "[ACON: Tool output omitted to prioritize reasoning traces.]".to_string();
                        }
                    }
                }
            }
        }

        // 2. JetBrains Observation Masking: Hide raw output of old tools but keep tool_calls visible.
        // Upgraded to Recency-Aware Masking.
        if enable_observation_masking {
            for i in 0..msg_count {
                if processed_messages[i].role == Role::Tool {
                    let age = msg_count - i;
                    if age > masking_threshold {
                        for tr in &mut processed_messages[i].tool_results {
                            if tr.error.is_empty() && !tr.content.starts_with("[Observation Masked") && !tr.content.starts_with("[ACON:") {
                                let bytes = tr.content.len();
                                if bytes > masking_size_limit {
                                    let preview_chars = 100;
                                    let char_count = tr.content.chars().count();
                                    if char_count > preview_chars * 2 {
                                        let start_preview: String = tr.content.chars().take(preview_chars).collect();
                                        let end_preview: String = tr.content.chars().skip(char_count - preview_chars).collect();
                                        tr.content = format!(
                                            "[Observation Masked to save context. Output was {} bytes. Preview: {}...{} The tool call itself remains visible. Use 'RecallObservation' with ID '{}' if you need the full output again.]",
                                            bytes, start_preview, end_preview, tr.tool_call_id
                                        );
                                    } else {
                                        tr.content = format!(
                                            "[Observation Masked to save context. Output was {} bytes. The tool call itself remains visible. Use 'RecallObservation' with ID '{}' if you need the full output again.]",
                                            bytes, tr.tool_call_id
                                        );
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        processed_messages
    }

    pub fn recall_observation(&self, tool_call_id: &str) -> Option<String> {
        self.observation_store.get(tool_call_id).map(|c| c.clone())
    }
}
