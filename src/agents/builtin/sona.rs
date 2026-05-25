/// Ruflo Unique Harness Innovations: SONA neural patterns
/// Self-learning trajectory patterns: Recalling historically successful sequences of tools to handle similar future tasks.

#[derive(Debug, Clone, Default)]
pub struct Trajectory {
    pub task_description: String,
    pub tool_sequences: Vec<String>,
    pub outcome: String,
}

pub struct SonaMemory {
    pub trajectories: Vec<Trajectory>,
}

impl SonaMemory {
    pub fn new() -> Self {
        Self {
            trajectories: Vec::new(),
        }
    }

    /// Records a new successful trajectory into the SONA memory
    pub fn record_trajectory(&mut self, task_description: String, tool_sequences: Vec<String>, outcome: String) {
        self.trajectories.push(Trajectory {
            task_description,
            tool_sequences,
            outcome,
        });
    }

    /// Retrieves relevant patterns based on keyword overlap
    pub fn retrieve_relevant_patterns(&self, query_task: &str, top_k: usize) -> Vec<&Trajectory> {
        let query_tokens: Vec<String> = query_task.split_whitespace().map(|s| s.to_lowercase()).collect();

        let mut scored_trajectories: Vec<(&Trajectory, usize)> = self.trajectories.iter().map(|t| {
            let desc_tokens: Vec<String> = t.task_description.split_whitespace().map(|s| s.to_lowercase()).collect();
            let score = query_tokens.iter().filter(|qt| desc_tokens.contains(qt)).count();
            (t, score)
        }).collect();

        // Sort by score descending
        scored_trajectories.sort_by(|a, b| b.1.cmp(&a.1));

        scored_trajectories.into_iter()
            .filter(|&(_, score)| score > 0)
            .take(top_k)
            .map(|(t, _)| t)
            .collect()
    }

    /// Injects retrieved relevant trajectories into the system prompt
    pub fn inject_into_prompt(&self, original_prompt: &str, query_task: &str) -> String {
        let relevant = self.retrieve_relevant_patterns(query_task, 3);
        if relevant.is_empty() {
            return original_prompt.to_string();
        }

        let mut injected = String::new();
        injected.push_str("--- SONA NEURAL PATTERNS (Self-learning trajectory patterns) ---\n");
        injected.push_str("Based on past similar tasks, here are historically successful tool sequences:\n");

        for (i, t) in relevant.iter().enumerate() {
            injected.push_str(&format!("Pattern {}:\n", i + 1));
            injected.push_str(&format!("  Task: {}\n", t.task_description));
            injected.push_str(&format!("  Successful Tools: {:?}\n", t.tool_sequences));
            injected.push_str(&format!("  Outcome: {}\n", t.outcome));
        }
        injected.push_str("----------------------------------------------------------------\n\n");
        injected.push_str(original_prompt);

        injected
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sona_record_and_retrieve() {
        let mut memory = SonaMemory::new();
        memory.record_trajectory(
            "Fix bug in the authentication logic".to_string(),
            vec!["read_file".to_string(), "edit_file".to_string(), "run_tests".to_string()],
            "Bug fixed".to_string()
        );
        memory.record_trajectory(
            "Create a new rust module for networking".to_string(),
            vec!["write_file".to_string(), "cargo_check".to_string()],
            "Module created".to_string()
        );

        let patterns = memory.retrieve_relevant_patterns("fix authentication", 2);
        assert_eq!(patterns.len(), 1);
        assert_eq!(patterns[0].task_description, "Fix bug in the authentication logic");

        let network_patterns = memory.retrieve_relevant_patterns("create rust networking", 2);
        assert_eq!(network_patterns.len(), 1);
        assert_eq!(network_patterns[0].tool_sequences, vec!["write_file", "cargo_check"]);
    }

    #[test]
    fn test_sona_inject_prompt() {
        let mut memory = SonaMemory::new();
        memory.record_trajectory(
            "refactor database schema".to_string(),
            vec!["run_sql".to_string()],
            "success".to_string()
        );

        let original = "You are a helpful AI assistant.";
        let query = "I need to refactor the database";
        let new_prompt = memory.inject_into_prompt(original, query);

        assert!(new_prompt.contains("SONA NEURAL PATTERNS"));
        assert!(new_prompt.contains("refactor database schema"));
        assert!(new_prompt.contains("run_sql"));
        assert!(new_prompt.ends_with("You are a helpful AI assistant."));
    }

    #[test]
    fn test_sona_no_match() {
        let mut memory = SonaMemory::new();
        memory.record_trajectory(
            "Eat an apple".to_string(),
            vec!["eat".to_string()],
            "yummy".to_string()
        );

        let original = "You are an AI.";
        let new_prompt = memory.inject_into_prompt(original, "Build a space shuttle");

        // When there's no match, it should return the original prompt unmodified
        assert_eq!(new_prompt, original);
    }
}
