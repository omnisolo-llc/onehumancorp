use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

/// OpenHands Unique Harness Innovations: MicroAgents
/// MicroAgents provide highly contextual, repository-specific instructions that are injected
/// dynamically into the prompt when specific triggers (like file extensions or keywords) are matched.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MicroAgent {
    pub name: String,
    pub description: Option<String>,
    pub triggers: Vec<String>, // Regex or simple substring/glob matches
    pub instructions: String,
}

pub struct MicroAgentRegistry {
    pub agents: Vec<MicroAgent>,
}

impl Default for MicroAgentRegistry {
    fn default() -> Self {
        Self::new()
    }
}

impl MicroAgentRegistry {
    pub fn new() -> Self {
        Self { agents: Vec::new() }
    }

    /// Loads MicroAgents from a `.openhands/microagents` or similar directory.
    pub fn load_from_dir(
        &mut self,
        dir: impl AsRef<Path>,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let dir = dir.as_ref();
        if !dir.exists() || !dir.is_dir() {
            return Ok(());
        }

        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_file()
                && (path.extension().and_then(|e| e.to_str()) == Some("yaml")
                    || path.extension().and_then(|e| e.to_str()) == Some("json"))
            {
                let content = fs::read_to_string(&path)?;
                if let Ok(agent) = serde_json::from_str::<MicroAgent>(&content) {
                    self.agents.push(agent);
                }
            }
        }
        Ok(())
    }

    /// Returns the concatenated instructions for all MicroAgents whose triggers match the given context.
    pub fn get_active_instructions(&self, context: &str) -> String {
        let mut active_instructions = Vec::new();

        for agent in &self.agents {
            let matches = agent
                .triggers
                .iter()
                .any(|trigger| context.contains(trigger));
            if matches {
                active_instructions.push(format!(
                    "--- MicroAgent: {} ---\n{}",
                    agent.name, agent.instructions
                ));
            }
        }

        active_instructions.join("\n\n")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;
    use std::fs::File;
    use std::io::Write;

    #[test]
    fn test_microagent_registry_new() {
        let registry = MicroAgentRegistry::new();
        assert!(registry.agents.is_empty());
    }

    #[test]
    fn test_load_from_dir_non_existent() {
        let mut registry = MicroAgentRegistry::new();
        let result = registry.load_from_dir("/path/that/does/not/exist");
        assert!(result.is_ok());
        assert!(registry.agents.is_empty());
    }

    #[test]
    fn test_load_from_dir_valid() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("agent1.json");
        let mut file = File::create(file_path).unwrap();
        let agent_json = r#"{
            "name": "TestAgent",
            "description": "A test agent",
            "triggers": ["test_trigger"],
            "instructions": "Do the test thing."
        }"#;
        writeln!(file, "{}", agent_json).unwrap();

        let mut registry = MicroAgentRegistry::new();
        registry.load_from_dir(dir.path()).unwrap();
        assert_eq!(registry.agents.len(), 1);
        assert_eq!(registry.agents[0].name, "TestAgent");
        assert_eq!(registry.agents[0].triggers, vec!["test_trigger"]);
        assert_eq!(registry.agents[0].instructions, "Do the test thing.");
    }

    #[test]
    fn test_get_active_instructions() {
        let mut registry = MicroAgentRegistry::new();
        registry.agents.push(MicroAgent {
            name: "AgentA".to_string(),
            description: None,
            triggers: vec!["triggerA".to_string()],
            instructions: "Instruction A".to_string(),
        });
        registry.agents.push(MicroAgent {
            name: "AgentB".to_string(),
            description: None,
            triggers: vec!["triggerB".to_string()],
            instructions: "Instruction B".to_string(),
        });

        let ctx = "We have triggerA in context.";
        let instructions = registry.get_active_instructions(ctx);
        assert!(instructions.contains("--- MicroAgent: AgentA ---"));
        assert!(instructions.contains("Instruction A"));
        assert!(!instructions.contains("AgentB"));

        let ctx2 = "We have triggerA and triggerB.";
        let instructions2 = registry.get_active_instructions(ctx2);
        assert!(instructions2.contains("AgentA"));
        assert!(instructions2.contains("AgentB"));

        let ctx3 = "Nothing matches.";
        let instructions3 = registry.get_active_instructions(ctx3);
        assert!(instructions3.is_empty());
    }
}
