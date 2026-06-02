use ohc_builtin_agent_core::types::ToolError;
use serde_json::{json, Value};
use std::sync::Arc;

use super::{Tool, ToolExecutor};

pub struct ToolSearchExecutor {
    // A list of tools available in the agent's context.
    pub available_tools: Vec<Tool>,
}

#[async_trait::async_trait]
impl ToolExecutor for ToolSearchExecutor {
    async fn execute(
        &self,
        args: Value,
    ) -> Result<String, ToolError> {
        let query = args["query"]
            .as_str()
            .ok_or_else(|| ToolError::LlmRecoverable("toolsearch: query is required".to_string()))?
            .to_lowercase();

        let matches: Vec<String> = self.available_tools
            .iter()
            .filter(|t| {
                t.name.to_lowercase().contains(&query) || t.description.to_lowercase().contains(&query)
            })
            .map(|t| format!("{}: {}", t.name, t.description))
            .collect();

        if matches.is_empty() {
            Ok(format!("No tools found matching '{}'.", query))
        } else {
            Ok(matches.join("\n"))
        }
    }
}

pub fn toolsearch_tool(available_tools: Vec<Tool>) -> Tool {
    Tool {
        name: "ToolSearch".to_string(),
        description: "Search available tools by name or description.".to_string(),
        is_read_only: true,
        parameters: json!({
            "type": "object",
            "properties": {
                "query": {
                    "type": "string",
                    "description": "Search term to find relevant tools."
                }
            },
            "required": ["query"]
        }),
        execute: Arc::new(ToolSearchExecutor { available_tools }),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ohc_builtin_agent_core::types::ToolError;

    struct DummyToolExecutor;
    #[async_trait::async_trait]
    impl ToolExecutor for DummyToolExecutor {
        async fn execute(&self, _args: Value) -> Result<String, ToolError> {
            Ok("dummy".to_string())
        }
    }

    #[tokio::test]
    async fn test_toolsearch_tool_execute() {
        let dummy_executor = Arc::new(DummyToolExecutor);
        let available_tools = vec![
            Tool {
                name: "Alpha".to_string(),
                description: "The first tool.".to_string(),
                is_read_only: true,
                parameters: json!({}),
                execute: dummy_executor.clone(),
            },
            Tool {
                name: "Beta".to_string(),
                description: "The second tool.".to_string(),
                is_read_only: false,
                parameters: json!({}),
                execute: dummy_executor.clone(),
            },
        ];

        let search_tool = toolsearch_tool(available_tools);

        // Valid query matching name
        let args_match = json!({"query": "alpha"});
        let result = search_tool.execute.execute(args_match).await.unwrap();
        assert!(result.contains("Alpha: The first tool."));
        assert!(!result.contains("Beta"));

        // Valid query matching description
        let args_desc = json!({"query": "second"});
        let result_desc = search_tool.execute.execute(args_desc).await.unwrap();
        assert!(result_desc.contains("Beta: The second tool."));
        assert!(!result_desc.contains("Alpha"));

        // Valid query matching none
        let args_none = json!({"query": "gamma"});
        let result_none = search_tool.execute.execute(args_none).await.unwrap();
        assert_eq!(result_none, "No tools found matching 'gamma'.");

        // Missing query error
        let args_err = json!({});
        let result_err = search_tool.execute.execute(args_err).await;
        assert!(result_err.is_err());
        assert_eq!(
            result_err.unwrap_err().to_string(),
            "LLM Recoverable Error: toolsearch: query is required"
        );
    }
}
