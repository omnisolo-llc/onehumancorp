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
            Self::Transient(m) => write!(f, "Transient Error: {}", m),
            Self::LlmRecoverable(m) => write!(f, "Error: {}", m), // Model can read this
            Self::UserFixable(m) => write!(f, "User Intervention Required: {}", m),
            Self::Unexpected(m) => write!(f, "Unexpected Error: {}", m),
        }
    }
}

impl std::error::Error for ToolError {}


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
    ]
}
