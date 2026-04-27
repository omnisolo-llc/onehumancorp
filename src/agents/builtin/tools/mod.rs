use serde_json::Value;
use std::sync::Arc;
use tokio::sync::RwLock;

pub mod bash;
pub mod read;
pub mod write;
pub mod edit;
pub mod glob;
pub mod grep;
pub mod webfetch;
pub mod websearch;
pub mod sendmessage;
pub mod todowrite;
pub mod toolsearch;
pub mod task;
pub mod agent_tool;
pub mod sleep;
pub mod marketing;
pub mod finance;
pub mod local_fs_sync;
pub mod ollama;

/// A tool definition and executor — mirrors Go builtin.Tool.
pub struct Tool {
    pub name: String,
    pub description: String,
    pub parameters: Value,
    pub execute: Arc<dyn ToolExecutor>,
}

impl std::fmt::Debug for Tool {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Tool({})", self.name)
    }
}

impl Clone for Tool {
    fn clone(&self) -> Self {
        Self {
            name: self.name.clone(),
            description: self.description.clone(),
            parameters: self.parameters.clone(),
            execute: self.execute.clone(),
        }
    }
}

#[async_trait::async_trait]
pub trait ToolExecutor: Send + Sync {
    async fn execute(
        &self,
        args: Value,
    ) -> Result<String, ToolError>;
}

/// Shared todo list state.
pub type SharedTodos = Arc<RwLock<Vec<todowrite::TodoItem>>>;

/// Shared task store state.
pub type SharedTaskStore = Arc<RwLock<task::TaskStore>>;

/// Shared mailbox for SendMessage.
pub type SharedMailbox = Arc<RwLock<sendmessage::Mailbox>>;

/// Build the default set of all tools.
pub fn all_tools(
    todos: SharedTodos,
    task_store: SharedTaskStore,
    mailbox: SharedMailbox,
) -> Vec<Tool> {
    vec![
        bash::bash_tool(),
        read::read_tool(),
        write::write_tool(),
        edit::edit_tool(),
        glob::glob_tool(),
        grep::grep_tool(),
        webfetch::webfetch_tool(),
        websearch::websearch_tool(),
        sendmessage::sendmessage_tool(mailbox.clone()),
        todowrite::todowrite_tool(todos.clone()),
        todowrite::todoread_tool(todos.clone()),
        toolsearch::toolsearch_tool(),
        task::task_create_tool(task_store.clone()),
        task::task_get_tool(task_store.clone()),
        task::task_list_tool(task_store.clone()),
        task::task_update_tool(task_store.clone()),
        sleep::sleep_tool(),
        marketing::qr_generate_tool(),
        finance::finance_report_tool(),
        agent_tool::agent_stop_tool(),
        agent_tool::agent_status_tool(),
        agent_tool::agent_tool(),
        local_fs_sync::local_fs_sync_tool(),
        ollama::ollama_tool(),
    ]
}

#[derive(Debug)]
pub enum ToolError {
    Transient(String),
    LlmRecoverable(String),
    UserFixable(String),
    Unexpected(String),
}

impl std::fmt::Display for ToolError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Transient(msg) => write!(f, "{}", msg),
            Self::LlmRecoverable(msg) => write!(f, "{}", msg),
            Self::UserFixable(msg) => write!(f, "{}", msg),
            Self::Unexpected(msg) => write!(f, "{}", msg),
        }
    }
}

impl std::error::Error for ToolError {}

impl From<&str> for ToolError {
    fn from(s: &str) -> Self {
        ToolError::LlmRecoverable(s.to_string())
    }
}

impl From<String> for ToolError {
    fn from(s: String) -> Self {
        ToolError::LlmRecoverable(s)
    }
}

impl From<Box<dyn std::error::Error + Send + Sync>> for ToolError {
    fn from(e: Box<dyn std::error::Error + Send + Sync>) -> Self {
        ToolError::LlmRecoverable(e.to_string())
    }
}

impl From<std::io::Error> for ToolError {
    fn from(e: std::io::Error) -> Self {
        ToolError::LlmRecoverable(e.to_string())
    }
}

impl From<serde_json::Error> for ToolError {
    fn from(e: serde_json::Error) -> Self {
        ToolError::LlmRecoverable(e.to_string())
    }
}

impl From<reqwest::Error> for ToolError {
    fn from(e: reqwest::Error) -> Self {
        ToolError::LlmRecoverable(e.to_string())
    }
}
