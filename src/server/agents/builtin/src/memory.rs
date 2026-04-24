use chrono::{DateTime, Utc};
use std::collections::VecDeque;
use std::sync::Mutex;

/// A single completed-task memory record — mirrors Go MemoryEntry.
#[derive(Debug, Clone)]
pub struct MemoryEntry {
    pub task_id: String,
    pub summary: String,
    pub tools_used: Vec<String>,
    pub outcome: String, // "success" | "failure"
    pub duration_s: f64,
    pub lessons: String,
    pub completed_at: DateTime<Utc>,
}

const MAX_MEMORIES_FOR_PROMPT: usize = 5;
const MEMORY_RING_SIZE: usize = 64;

/// Ring-buffer memory store — mirrors Go MemoryStore.
pub struct MemoryStore {
    entries: Mutex<VecDeque<MemoryEntry>>,
    capacity: usize,
}

impl MemoryStore {
    pub fn new(capacity: usize) -> Self {
        let capacity = if capacity == 0 {
            MEMORY_RING_SIZE
        } else {
            capacity
        };
        Self {
            entries: Mutex::new(VecDeque::with_capacity(capacity)),
            capacity,
        }
    }

    pub fn write(&self, entry: MemoryEntry) {
        if entry.task_id.is_empty() {
            return;
        }
        let mut entries = self.entries.lock().unwrap();
        if entries.len() >= self.capacity {
            entries.pop_front();
        }
        entries.push_back(entry);
    }

    /// Returns up to n recent successful memory summaries.
    pub fn recent_successes(&self, n: usize) -> Vec<String> {
        let entries = self.entries.lock().unwrap();
        entries
            .iter()
            .rev()
            .filter(|e| e.outcome == "success" && !e.summary.is_empty())
            .take(n)
            .map(|e| {
                format!(
                    "Past task ({}): {}",
                    e.completed_at.format("%Y-%m-%d"),
                    e.summary
                )
            })
            .collect()
    }
}

impl Default for MemoryStore {
    fn default() -> Self {
        Self::new(MEMORY_RING_SIZE)
    }
}

/// Prepend relevant past successes to system_prompt.
/// Mirrors Go InjectMemoriesIntoPrompt.
pub fn inject_memories_into_prompt(store: &MemoryStore, system_prompt: &str) -> String {
    let memories = store.recent_successes(MAX_MEMORIES_FOR_PROMPT);
    if memories.is_empty() {
        return system_prompt.to_string();
    }
    let mut s = String::new();
    s.push_str("## Relevant past experience\n");
    for m in &memories {
        s.push_str("- ");
        s.push_str(m);
        s.push('\n');
    }
    s.push_str("\n---\n\n");
    s.push_str(system_prompt);
    s
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_memory_write_and_recall() {
        let store = MemoryStore::new(10);
        store.write(MemoryEntry {
            task_id: "t1".to_string(),
            summary: "Fixed bug".to_string(),
            tools_used: vec![],
            outcome: "success".to_string(),
            duration_s: 1.5,
            lessons: String::new(),
            completed_at: Utc::now(),
        });
        let successes = store.recent_successes(5);
        assert_eq!(successes.len(), 1);
        assert!(successes[0].contains("Fixed bug"));
    }

    #[test]
    fn test_memory_ring_buffer() {
        let store = MemoryStore::new(3);
        for i in 0..5 {
            store.write(MemoryEntry {
                task_id: format!("t{}", i),
                summary: format!("Task {}", i),
                tools_used: vec![],
                outcome: "success".to_string(),
                duration_s: 1.0,
                lessons: String::new(),
                completed_at: Utc::now(),
            });
        }
        let successes = store.recent_successes(10);
        assert_eq!(successes.len(), 3); // only last 3 fit
    }

    #[test]
    fn test_inject_memories_empty() {
        let store = MemoryStore::new(10);
        let prompt = "Hello";
        let result = inject_memories_into_prompt(&store, prompt);
        assert_eq!(result, "Hello");
    }

    #[test]
    fn test_inject_memories_non_empty() {
        let store = MemoryStore::new(10);
        store.write(MemoryEntry {
            task_id: "t1".to_string(),
            summary: "memory1".to_string(),
            tools_used: vec![],
            outcome: "success".to_string(),
            duration_s: 1.0,
            lessons: String::new(),
            completed_at: Utc::now(),
        });
        let result = inject_memories_into_prompt(&store, "System prompt");
        assert!(result.contains("## Relevant past experience"));
        assert!(result.contains("System prompt"));
    }
}
