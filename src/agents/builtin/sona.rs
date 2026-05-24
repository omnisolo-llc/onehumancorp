use ohc_builtin_agent_core::types::{Message, Role};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Ruflo Unique Harness Innovations: SONA neural patterns
/// Self-learning trajectory patterns from successful agent executions.

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Trajectory {
    pub task_description: String,
    pub tool_sequence: Vec<String>,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NeuralPattern {
    pub id: String,
    pub original_task: String,
    pub inferred_intent: String,
    pub tool_sequence: Vec<String>,
}

pub trait PatternStore: Send + Sync {
    fn save_pattern(&self, pattern: NeuralPattern) -> Result<(), String>;
    fn recall_similar(&self, task: &str) -> Result<Vec<NeuralPattern>, String>;
}

pub struct MemoryPatternStore {
    patterns: std::sync::RwLock<HashMap<String, NeuralPattern>>,
}

impl MemoryPatternStore {
    pub fn new() -> Self {
        Self {
            patterns: std::sync::RwLock::new(HashMap::new()),
        }
    }
}

impl PatternStore for MemoryPatternStore {
    fn save_pattern(&self, pattern: NeuralPattern) -> Result<(), String> {
        let mut store = self
            .patterns
            .write()
            .map_err(|_| "Failed to lock pattern store")?;
        store.insert(pattern.id.clone(), pattern);
        Ok(())
    }

    fn recall_similar(&self, task: &str) -> Result<Vec<NeuralPattern>, String> {
        let store = self
            .patterns
            .read()
            .map_err(|_| "Failed to lock pattern store")?;
        let task_lower = task.to_lowercase();

        let mut results = Vec::new();
        for pattern in store.values() {
            // Simple keyword matching for demo purposes
            let match_score = pattern
                .inferred_intent
                .to_lowercase()
                .split_whitespace()
                .filter(|&word| task_lower.contains(word))
                .count();

            if match_score > 0 || task_lower.contains(&pattern.original_task.to_lowercase()) {
                results.push(pattern.clone());
            }
        }

        Ok(results)
    }
}

pub struct SonaPatternManager {
    store: Box<dyn PatternStore>,
}

impl SonaPatternManager {
    pub fn new(store: Box<dyn PatternStore>) -> Self {
        Self { store }
    }

    /// Extracts a trajectory from a completed session's messages and saves it as a NeuralPattern.
    pub fn extract_and_store_pattern(
        &self,
        task_description: &str,
        messages: &[Message],
    ) -> Result<(), String> {
        let mut tool_sequence = Vec::new();

        for msg in messages {
            if msg.role == Role::Assistant {
                for tc in &msg.tool_calls {
                    tool_sequence.push(tc.name.clone());
                }
            }
        }

        if tool_sequence.is_empty() {
            return Ok(()); // Nothing to learn
        }

        let pattern = NeuralPattern {
            id: uuid::Uuid::new_v4().to_string(),
            original_task: task_description.to_string(),
            inferred_intent: format!("Solve task involving {}", task_description), // In a real system, an LLM would infer this
            tool_sequence,
        };

        self.store.save_pattern(pattern)
    }

    /// Recalls previous neural patterns that match the current task.
    pub fn recall_pattern(&self, task: &str) -> Result<Vec<NeuralPattern>, String> {
        self.store.recall_similar(task)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ohc_builtin_agent_core::types::ToolCall;

    #[test]
    fn test_sona_pattern_extraction_and_recall() {
        let store = Box::new(MemoryPatternStore::new());
        let manager = SonaPatternManager::new(store);

        let task = "Analyze data and chart it";

        let messages = vec![
            Message::user(task),
            Message {
                role: Role::Assistant,
                content: String::new(),
                tool_calls: vec![ToolCall {
                    id: "1".to_string(),
                    name: "read_csv".to_string(),
                    arguments: serde_json::Value::Null,
                }],
                tool_results: vec![],
                response_id: None,
                previous_response_id: None,
            },
            Message {
                role: Role::Assistant,
                content: String::new(),
                tool_calls: vec![ToolCall {
                    id: "2".to_string(),
                    name: "generate_chart".to_string(),
                    arguments: serde_json::Value::Null,
                }],
                tool_results: vec![],
                response_id: None,
                previous_response_id: None,
            },
        ];

        // 1. Extract
        assert!(manager.extract_and_store_pattern(task, &messages).is_ok());

        // 2. Recall
        let similar = manager.recall_pattern("I need to chart some data").unwrap();
        assert_eq!(similar.len(), 1);
        let pattern = &similar[0];
        assert_eq!(pattern.tool_sequence, vec!["read_csv", "generate_chart"]);
        assert!(pattern.inferred_intent.contains("chart"));
    }
}
