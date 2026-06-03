use serde::{Deserialize, Serialize};

/// Architectural Decision 7: Harness Thickness
/// Controls the "thickness" of the agent harness.
/// As models get smarter, the harness should get thinner (less prompting, fewer retries, less strict validation).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum HarnessThickness {
    /// Thick: High verbosity, detailed planning prompts, strict validation, high retries.
    /// Recommended for older models (e.g., GPT-3.5, Claude 2).
    Thick,
    /// Medium: Standard verbosity, balanced validation.
    /// Recommended for capable models (e.g., GPT-4, Claude 3 Opus).
    Medium,
    /// Thin: Minimal prompting, relies on model's internal reasoning, minimal retries.
    /// Recommended for cutting-edge reasoning models (e.g., o1, o3-mini).
    Thin,
}

impl Default for HarnessThickness {
    fn default() -> Self {
        Self::Medium
    }
}

impl HarnessThickness {
    /// Adjusts the `max_retries` based on thickness.
    pub fn override_retries(&self, current_retries: usize) -> usize {
        match self {
            Self::Thick => std::cmp::max(current_retries, 3),
            Self::Medium => current_retries,
            Self::Thin => std::cmp::min(current_retries, 1),
        }
    }

    /// Strips excessive planning or verbose system prompts if thickness is Thin.
    pub fn override_system_prompt(&self, prompt: &str) -> String {
        match self {
            Self::Thin => {
                let mut p = prompt.replace("You must think step by step and make a detailed plan.", "");
                p = p.replace("Make a plan before executing.", "");
                p = p.replace("Carefully analyze every single option.", "");
                p.trim().to_string()
            }
            Self::Medium => prompt.to_string(),
            Self::Thick => {
                let mut p = prompt.to_string();
                if !p.contains("think step by step") {
                    p.push_str("\nYou must think step by step and make a detailed plan before acting.");
                }
                p
            }
        }
    }
}
