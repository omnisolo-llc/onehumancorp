use ohc_builtin_agent_core::types::ToolError;
use serde_json::json;
use serde::Deserialize;
use std::sync::Arc;

use super::{Tool, pydantic::{PydanticToolExecutor, PydanticAdapter}};

#[derive(Deserialize)]
struct ToolSearchArgs {
    query: String,
}


struct ToolSearchExecutor;

#[async_trait::async_trait]
impl PydanticToolExecutor<ToolSearchArgs> for ToolSearchExecutor {
    async fn execute_typed(&self, args: ToolSearchArgs) -> Result<String, ToolError> {
        let query = args.query.to_lowercase();

        let all_tools = &[
            ("Bash", "Execute shell commands"),
            ("Read", "Read file contents"),
            ("Write", "Write content to a file"),
            ("Edit", "Edit a file by replacing text"),
            ("Glob", "Find files by pattern"),
            ("Grep", "Search file contents with regex"),
            ("WebFetch", "Fetch URL content"),
            ("WebSearch", "Search the web"),
            ("SendMessage", "Send message to parent agent"),
            ("TodoWrite", "Write the task todo list"),
            ("TodoRead", "Read the task todo list"),
            ("ToolSearch", "Search available tools"),
            ("TaskCreate", "Create a new task"),
            ("TaskGet", "Get task details"),
            ("TaskList", "List all tasks"),
            ("TaskUpdate", "Update task status/result"),
            ("Sleep", "Sleep for N seconds"),
            ("Agent", "Spawn a sub-agent task"),
            ("TaskStop", "Stop a running sub-agent"),
            ("TaskStatus", "Get sub-agent task status"),
            (
                "RunWorkflow",
                "Run a built-in multi-agent workflow with phased subagent orchestration",
            ),
        ];

        let matches: Vec<String> = all_tools
            .iter()
            .filter(|(name, desc)| {
                name.to_lowercase().contains(&query) || desc.to_lowercase().contains(&query)
            })
            .map(|(name, desc)| format!("{}: {}", name, desc))
            .collect();

        if matches.is_empty() {
            Ok(format!("No tools found matching '{}'.", query))
        } else {
            Ok(matches.join("\n"))
        }
    }
}

pub fn toolsearch_tool() -> Tool {
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
        execute: Arc::new(PydanticAdapter::new(ToolSearchExecutor)),
    }
}
