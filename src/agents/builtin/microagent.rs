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

impl MicroAgentRegistry {
    pub fn new() -> Self {
        Self { agents: Vec::new() }
    }

    /// Loads MicroAgents from a `.openhands/microagents` or similar directory.
    pub fn load_from_dir(&mut self, dir: impl AsRef<Path>) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let dir = dir.as_ref();
        if !dir.exists() || !dir.is_dir() {
            return Ok(());
        }

        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_file() && (path.extension().and_then(|e| e.to_str()) == Some("yaml") || path.extension().and_then(|e| e.to_str()) == Some("json")) {
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
            let matches = agent.triggers.iter().any(|trigger| context.contains(trigger));
            if matches {
                active_instructions.push(format!("--- MicroAgent: {} ---\n{}", agent.name, agent.instructions));
            }
        }

        active_instructions.join("\n\n")
    }
}
