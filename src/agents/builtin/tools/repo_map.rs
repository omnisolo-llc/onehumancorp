use ohc_builtin_agent_core::types::ToolError;
use serde_json::{json, Value};
use std::sync::Arc;
use std::path::PathBuf;
use crate::{Tool, ToolExecutor};

pub struct RepoMapExecutor {
    _workspace_path: PathBuf,
}

impl RepoMapExecutor {
    pub fn new(_workspace_path: PathBuf) -> Self {
        Self { _workspace_path }
    }
}

#[async_trait::async_trait]
impl ToolExecutor for RepoMapExecutor {
    async fn execute(&self, _parameters: Value) -> Result<String, ToolError> {
        Ok("".to_string())
    }
}

pub fn repomap_tool(root_dir: std::path::PathBuf) -> Tool {
    Tool {
        name: "repo_map".to_string(),
        description: "Creates a map of the repository.".to_string(),
        parameters: json!({}),
        is_read_only: true,
        execute: Arc::new(RepoMapExecutor::new(root_dir)),
    }
}

pub fn tree_sitter_parse(_content: &str, _ext: &str) -> Option<Vec<String>> { None }
