/// Human-in-loop as spectrum -> not binary autonomy vs control
/// Implements a progressive authorization model where confidence scores
/// determine if an action requires human approval, can proceed with notification,
/// or can execute fully autonomously.

use ohc_builtin_agent_core::types::ToolError;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AutonomyLevel {
    /// Human must approve every action
    Manual,
    /// Human approves actions below confidence threshold
    Supervised(u8),
    /// Agent executes fully autonomously
    Autonomous,
}

#[derive(Debug, Clone)]
pub struct HumanInLoopSpectrum {
    pub current_level: AutonomyLevel,
    pub confidence_scores: HashMap<String, u8>,
}

impl HumanInLoopSpectrum {
    pub fn new(level: AutonomyLevel) -> Self {
        Self {
            current_level: level,
            confidence_scores: HashMap::new(),
        }
    }

    pub fn set_confidence(&mut self, tool_name: &str, score: u8) {
        self.confidence_scores.insert(tool_name.to_string(), score);
    }

    pub fn check_authorization(&self, tool_name: &str) -> Result<(), ToolError> {
        match self.current_level {
            AutonomyLevel::Manual => {
                Err(ToolError::UserFixable(format!(
                    "Manual mode: Tool '{}' requires explicit human approval.",
                    tool_name
                )))
            }
            AutonomyLevel::Supervised(threshold) => {
                let score = self.confidence_scores.get(tool_name).copied().unwrap_or(0);
                if score >= threshold {
                    Ok(())
                } else {
                    Err(ToolError::UserFixable(format!(
                        "Supervised mode: Tool '{}' confidence ({}) is below threshold ({}). Human approval required.",
                        tool_name, score, threshold
                    )))
                }
            }
            AutonomyLevel::Autonomous => Ok(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_human_in_loop_spectrum() {
        let mut hil = HumanInLoopSpectrum::new(AutonomyLevel::Supervised(80));
        hil.set_confidence("safe_tool", 95);
        hil.set_confidence("risky_tool", 40);

        assert!(hil.check_authorization("safe_tool").is_ok());

        let res = hil.check_authorization("risky_tool");
        assert!(matches!(res, Err(ToolError::UserFixable(_))));

        let hil_manual = HumanInLoopSpectrum::new(AutonomyLevel::Manual);
        let res_manual = hil_manual.check_authorization("safe_tool");
        assert!(matches!(res_manual, Err(ToolError::UserFixable(_))));

        let hil_auto = HumanInLoopSpectrum::new(AutonomyLevel::Autonomous);
        assert!(hil_auto.check_authorization("risky_tool").is_ok());
    }
}
