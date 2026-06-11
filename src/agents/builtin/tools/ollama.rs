use ohc_builtin_agent_core::types::ToolError;
use serde_json::{json, Value};
use serde::Deserialize;
use std::sync::Arc;
use super::{Tool, pydantic::{PydanticToolExecutor, PydanticAdapter}};

#[derive(Deserialize)]
struct OllamaArgs {
    action: String,
    url: Option<String>,
    model_name: Option<String>,
}

struct OllamaExecutor;

#[async_trait::async_trait]
impl PydanticToolExecutor<OllamaArgs> for OllamaExecutor {
    async fn execute_typed(
        &self,
        args: OllamaArgs,
    ) -> Result<String, ToolError> {
        let action = args.action.as_str();
        let url = args.url.as_deref().unwrap_or("http://localhost:11434");

        let client = reqwest::Client::new();

        match action {
            "list_models" => {
                let endpoint = format!("{}/api/tags", url.trim_end_matches('/'));
                let resp = client.get(&endpoint).send().await.map_err(|e| ToolError::LlmRecoverable(e.to_string()))?;
                if !resp.status().is_success() {
                    return Err(ToolError::LlmRecoverable(format!("ollama returned status {}", resp.status())));
                }
                let result: Value = resp.json().await.map_err(|e| ToolError::LlmRecoverable(e.to_string()))?;
                Ok(result.to_string())
            }
            "pull_model" => {
                let model_name = args.model_name.as_deref().ok_or_else(|| ToolError::LlmRecoverable("ollama: model_name is required for pull".to_string()))?;
                let endpoint = format!("{}/api/pull", url.trim_end_matches('/'));
                let payload = json!({
                    "name": model_name,
                    "stream": false,
                });
                let resp = client.post(&endpoint).json(&payload).send().await.map_err(|e| ToolError::LlmRecoverable(e.to_string()))?;
                if !resp.status().is_success() {
                    return Err(ToolError::LlmRecoverable(format!("ollama returned status {}", resp.status())));
                }
                Ok(json!({"status":"pulled"}).to_string())
            }
            "check_health" => {
                let model_name = args.model_name.as_deref().ok_or_else(|| ToolError::LlmRecoverable("ollama: model_name is required for health check".to_string()))?;
                let endpoint = format!("{}/api/generate", url.trim_end_matches('/'));
                let payload = json!({
                    "model": model_name,
                    "prompt": "Hello",
                    "stream": false,
                });
                let resp = client.post(&endpoint).json(&payload).send().await.map_err(|e| ToolError::LlmRecoverable(e.to_string()))?;
                if resp.status().is_success() {
                    Ok(json!({"status":"healthy"}).to_string())
                } else {
                    Err(ToolError::LlmRecoverable(format!("health check failed with status {}", resp.status())))
                }
            }
            _ => Err(ToolError::LlmRecoverable("invalid action".to_string())),
        }
    }
}

pub fn ollama_tool() -> Tool {
    Tool {
        name: "ollama".to_string(),
        description: "Manage local Ollama instances (list, pull, check health).".to_string(),
        is_read_only: true,
        parameters: json!({
            "type": "object",
            "properties": {
                "action": {
                    "type": "string",
                    "description": "Action to perform: list_models, pull_model, check_health"
                },
                "url": {
                    "type": "string",
                    "description": "Ollama URL (default: http://localhost:11434)"
                },
                "model_name": {
                    "type": "string",
                    "description": "Model name (required for pull_model and check_health)"
                }
            },
            "required": ["action"]
        }),
        execute: Arc::new(PydanticAdapter::new(OllamaExecutor)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tools::ToolExecutor;

    #[tokio::test]
    async fn test_ollama_pydantic_adapter_missing_action() {
        let tool = ollama_tool();
        let args = json!({});
        let result = tool.execute.execute(args).await;

        assert!(result.is_err());
        if let Err(ToolError::LlmRecoverable(msg)) = result {
            assert!(msg.contains("Validation Error (Pydantic-first tool schema)"));
            assert!(msg.contains("missing field `action`"));
        } else {
            panic!("Expected LlmRecoverable error for missing required action");
        }
    }

    #[tokio::test]
    async fn test_ollama_pydantic_adapter_invalid_type() {
        let tool = ollama_tool();
        let args = json!({ "action": 123 }); // Action should be a string
        let result = tool.execute.execute(args).await;

        assert!(result.is_err());
        if let Err(ToolError::LlmRecoverable(msg)) = result {
            assert!(msg.contains("Validation Error (Pydantic-first tool schema)"));
        } else {
            panic!("Expected LlmRecoverable error for invalid type");
        }
    }

    #[tokio::test]
    async fn test_ollama_executor_invalid_action() {
        let tool = ollama_tool();
        let args = json!({ "action": "invalid_action" });
        let result = tool.execute.execute(args).await;

        assert!(result.is_err());
        if let Err(ToolError::LlmRecoverable(msg)) = result {
            assert_eq!(msg, "invalid action");
        } else {
            panic!("Expected LlmRecoverable error for invalid action");
        }
    }
}
