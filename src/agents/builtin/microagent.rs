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
    pub fn load_from_dir(&mut self, dir: impl AsRef<Path>) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let dir = dir.as_ref();
        if !dir.exists() || !dir.is_dir() {
            return Ok(());
        }

        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_file() {
                let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("");
                if ext == "yaml" || ext == "yml" || ext == "json" {
                    let content = fs::read_to_string(&path)?;
                    let result = if ext == "json" {
                        serde_json::from_str::<MicroAgent>(&content).map_err(|e| e.to_string())
                    } else {
                        serde_yaml::from_str::<MicroAgent>(&content).map_err(|e| e.to_string())
                    };

                    if let Ok(agent) = result {
                        self.agents.push(agent);
                    }
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

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_load_microagents() {
        let dir = tempdir().unwrap();
        let agent_dir = dir.path().join("microagents");
        fs::create_dir_all(&agent_dir).unwrap();

        let yaml_content = r#"
name: RustAgent
description: Helper for Rust files
triggers:
  - ".rs"
  - "Cargo.toml"
instructions: "Always run cargo fmt."
"#;
        fs::write(agent_dir.join("rust.yaml"), yaml_content).unwrap();

        let json_content = r#"
{
  "name": "PythonAgent",
  "description": "Helper for Python",
  "triggers": [".py"],
  "instructions": "Use black formatter."
}
"#;
        fs::write(agent_dir.join("python.json"), json_content).unwrap();

        // Also test .yml extension
        let yml_content = r#"
name: JSAGENT
description: Helper for JS files
triggers:
  - ".js"
instructions: "Use eslint."
"#;
        fs::write(agent_dir.join("js.yml"), yml_content).unwrap();

        let mut registry = MicroAgentRegistry::new();
        registry.load_from_dir(&agent_dir).unwrap();

        assert_eq!(registry.agents.len(), 3);

        let rust_agent = registry.agents.iter().find(|a| a.name == "RustAgent").unwrap();
        assert_eq!(rust_agent.triggers, vec![".rs", "Cargo.toml"]);
        assert_eq!(rust_agent.instructions, "Always run cargo fmt.");

        let py_agent = registry.agents.iter().find(|a| a.name == "PythonAgent").unwrap();
        assert_eq!(py_agent.triggers, vec![".py"]);

        let js_agent = registry.agents.iter().find(|a| a.name == "JSAGENT").unwrap();
        assert_eq!(js_agent.triggers, vec![".js"]);

        let active = registry.get_active_instructions("I need help with main.rs and Cargo.toml");
        assert!(active.contains("RustAgent"));
        assert!(!active.contains("PythonAgent"));
        assert!(!active.contains("JSAGENT"));
    }
}
