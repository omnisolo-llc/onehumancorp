/// Master Catalog B.13. Automation: Hooks (Claude Code Mechanic)
/// Run shell commands automatically when the agent edits files, finishes tasks, or needs input.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tokio::process::Command;
use tokio::io::AsyncWriteExt;
use ohc_builtin_agent_core::types::ToolCall;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum HookType {
    #[serde(rename = "command")]
    Command { command: String, r#if: Option<String> },
    #[serde(rename = "prompt")]
    Prompt { prompt: String },
    #[serde(rename = "agent")]
    Agent { prompt: String, timeout: Option<u64> },
    #[serde(rename = "http")]
    Http { url: String },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HookDefinition {
    pub matcher: Option<String>,
    pub hooks: Vec<HookType>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct HooksConfig {
    #[serde(default)]
    pub hooks: HashMap<String, Vec<HookDefinition>>,
}

#[derive(Debug, Clone, Default)]
pub struct HookExecutor {
    pub config: HooksConfig,
}

impl HookExecutor {
    pub fn new(config: HooksConfig) -> Self {
        Self { config }
    }

    pub async fn execute_pre_tool_use(&self, tool: &ToolCall, cwd: &str) -> Result<(), String> {
        let event_name = "PreToolUse";
        if let Some(definitions) = self.config.hooks.get(event_name) {
            for def in definitions {
                // simple matcher check
                if let Some(matcher) = &def.matcher {
                    if !matcher.is_empty() && !tool.name.contains(matcher) {
                        continue;
                    }
                }

                for hook in &def.hooks {
                    match hook {
                        HookType::Command { command, .. } => {
                            tracing::info!("Executing PreToolUse command hook: {}", command);
                            let input_json = serde_json::json!({
                                "hook_event_name": event_name,
                                "cwd": cwd,
                                "tool_name": tool.name,
                                "tool_input": tool.arguments
                            });
                            let payload = serde_json::to_string(&input_json).unwrap();

                            let mut cmd = Command::new("sh");
                            cmd.arg("-c").arg(command);
                            cmd.current_dir(cwd);
                            cmd.stdin(std::process::Stdio::piped());
                            cmd.stdout(std::process::Stdio::piped());
                            cmd.stderr(std::process::Stdio::piped());

                            let mut child = cmd.spawn().map_err(|e| e.to_string())?;

                            if let Some(mut stdin) = child.stdin.take() {
                                stdin.write_all(payload.as_bytes()).await.map_err(|e| e.to_string())?;
                            }

                            let output = child.wait_with_output().await.map_err(|e| e.to_string())?;

                            if !output.status.success() {
                                let stderr = String::from_utf8_lossy(&output.stderr);
                                if output.status.code() == Some(2) {
                                    return Err(format!("Blocked by hook: {}", stderr));
                                } else {
                                    tracing::warn!("Hook error: {}", stderr);
                                }
                            }
                        }
                        _ => {
                            tracing::warn!("Hook type not yet fully supported in this executor");
                        }
                    }
                }
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_pre_tool_use_hook_allow() {
        let config_json = serde_json::json!({
            "hooks": {
                "PreToolUse": [
                    {
                        "matcher": "Bash",
                        "hooks": [
                            {
                                "type": "command",
                                "command": "cat > /dev/null; true"
                            }
                        ]
                    }
                ]
            }
        });
        let config: HooksConfig = serde_json::from_value(config_json).unwrap();
        let executor = HookExecutor::new(config);

        let tool_call = ToolCall {
            id: "1".to_string(),
            name: "Bash".to_string(),
            arguments: serde_json::json!({"command": "ls"}),
        };

        let res = executor.execute_pre_tool_use(&tool_call, "/tmp").await;
        assert!(res.is_ok());
    }

    #[tokio::test]
    async fn test_pre_tool_use_hook_block() {
        let config_json = serde_json::json!({
            "hooks": {
                "PreToolUse": [
                    {
                        "matcher": "Bash",
                        "hooks": [
                            {
                                "type": "command",
                                "command": "echo 'Blocked!' >&2; sh -c 'exit 2'"
                            }
                        ]
                    }
                ]
            }
        });
        let config: HooksConfig = serde_json::from_value(config_json).unwrap();
        let executor = HookExecutor::new(config);

        let tool_call = ToolCall {
            id: "2".to_string(),
            name: "Bash".to_string(),
            arguments: serde_json::json!({"command": "rm -rf /"}),
        };

        let res = executor.execute_pre_tool_use(&tool_call, "/tmp").await;
        assert!(res.is_err());
        assert!(res.unwrap_err().contains("Blocked by hook: Blocked!"));
    }
}
