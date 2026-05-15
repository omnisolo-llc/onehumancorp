use super::*;
use ohc_builtin_agent_core::types::ToolResult;

impl Agent {
    pub(crate) fn check_tool_gating(
        tc: &ToolCall,
        is_read_only: bool,
        cfg: &AgentRunConfig,
    ) -> Result<(), ToolError> {
        // Stage 1: Trust establishment at project load
        if !cfg.project_trusted && !is_read_only {
            return Err(ToolError::Fatal(
                "Project not trusted. Mutating tools are disabled.".to_string(),
            ));
        }

        // Stage 2: Permission check before each tool call
        if let Some(allowed) = &cfg.allowed_tools {
            if !allowed.contains(&tc.name) {
                return Err(ToolError::Fatal(format!(
                    "Tool '{}' is not in the allowed list.",
                    tc.name
                )));
            }
        }

        // Stage 3: Explicit user confirmation for high-risk operations
        if cfg.high_risk_tools.contains(&tc.name) && !cfg.approved_tool_calls.contains(&tc.id) {
            return Err(ToolError::UserFixable(format!("High-risk tool '{}' requires explicit user confirmation. Approve this tool call to proceed.", tc.name)));
        }

        Ok(())
    }

    pub(crate) fn validate_schema(args: &serde_json::Value, schema: &serde_json::Value) -> Result<(), String> {
        if let Some(req_array) = schema.get("required").and_then(|v| v.as_array()) {
            if let Some(args_obj) = args.as_object() {
                for req in req_array {
                    if let Some(req_str) = req.as_str() {
                        if !args_obj.contains_key(req_str) {
                            return Err(format!("missing required parameter: '{}'", req_str));
                        }
                    }
                }
            } else if !req_array.is_empty() {
                return Err("arguments must be an object".to_string());
            }
        }

        if let Some(props) = schema.get("properties").and_then(|v| v.as_object()) {
            if let Some(args_obj) = args.as_object() {
                for (k, v) in args_obj {
                    if let Some(prop_schema) = props.get(k) {
                        if let Some(expected_type) =
                            prop_schema.get("type").and_then(|t| t.as_str())
                        {
                            let type_matches = match expected_type {
                                "string" => v.is_string(),
                                "number" | "integer" => v.is_number(),
                                "boolean" => v.is_boolean(),
                                "object" => v.is_object(),
                                "array" => v.is_array(),
                                _ => true, // Unknown type, skip validation for now
                            };
                            if !type_matches {
                                return Err(format!(
                                    "parameter '{}' has invalid type: expected {}",
                                    k, expected_type
                                ));
                            }
                        }
                    }
                }
            }
        }

        Ok(())
    }

    pub(crate) async fn execute_tool(
        &self,
        tc: &ToolCall,
        session_tools: &[Tool],
        current_messages: &[Message],
    ) -> Result<String, ToolError> {
        let tool = session_tools
            .iter()
            .find(|t| t.name == tc.name)
            .ok_or_else(|| ToolError::LlmRecoverable(format!("unknown tool: {}", tc.name)))?;

        let mut args = tc.arguments.clone();
        if tc.name == "spawn_subagent" {
            if let Some(obj) = args.as_object_mut() {
                if obj.get("mode").and_then(|v| v.as_str()) == Some("fork") {
                    if let Ok(context_json) = serde_json::to_string(current_messages) {
                        obj.insert(
                            "parent_context_json".to_string(),
                            serde_json::json!(context_json),
                        );
                    }
                }
            }
        }

        if let Err(e) = Self::validate_schema(&args, &tool.parameters) {
            return Err(ToolError::LlmRecoverable(format!(
                "Tool schema validation failed: {}",
                e
            )));
        }

        tool.execute.execute(args).await
    }

}
