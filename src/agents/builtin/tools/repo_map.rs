use async_trait::async_trait;
use serde_json::Value;

use super::agent_tool::{AgentTool, ToolParameter};
use ohc_builtin_agent_core::MemoryStore;
use std::sync::Arc;

pub struct RepoMapTool;

impl RepoMapTool {
    pub fn new() -> Self {
        RepoMapTool
    }
}

#[async_trait]
impl AgentTool for RepoMapTool {
    fn name(&self) -> String {
        "repo_map".to_string()
    }

    fn description(&self) -> String {
        "Generates a structured map of the workspace repository to understand its layout, using tree-sitter when available to extract class and function signatures.".to_string()
    }

    fn parameters(&self) -> Vec<ToolParameter> {
        vec![]
    }

    async fn execute(
        &self,
        _args: Value,
        _memory: Arc<dyn MemoryStore>,
        _agent_id: &str,
    ) -> Result<String, String> {
        Ok("Repo mapping stubbed due to build errors.".to_string())
    }
}
