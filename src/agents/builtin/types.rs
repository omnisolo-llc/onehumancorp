use serde::{Deserialize, Serialize};

/// Role in the conversation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Role {
    User,
    Assistant,
    System,
    Tool,
}

impl std::fmt::Display for Role {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Role::User => write!(f, "user"),
            Role::Assistant => write!(f, "assistant"),
            Role::System => write!(f, "system"),
            Role::Tool => write!(f, "tool"),
        }
    }
}

/// A single message in the conversation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub role: Role,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub content: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tool_calls: Vec<ToolCall>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tool_results: Vec<ToolResult>,
}

impl Message {
    pub fn user(content: impl Into<String>) -> Self {
        Self {
            role: Role::User,
            content: content.into(),
            tool_calls: vec![],
            tool_results: vec![],
        }
    }

    pub fn assistant(content: impl Into<String>) -> Self {
        Self {
            role: Role::Assistant,
            content: content.into(),
            tool_calls: vec![],
            tool_results: vec![],
        }
    }
}

/// A tool call requested by the assistant.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolCall {
    pub id: String,
    pub name: String,
    pub arguments: serde_json::Value,
}

/// Result from executing a tool.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolResult {
    pub tool_call_id: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub content: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub error: String,
}

/// Request to the LLM.
#[derive(Debug, Clone)]
pub struct ChatRequest {
    pub model: String,
    pub system: String,
    pub messages: Vec<Message>,
    pub tools: Vec<ToolDefinition>,
    pub max_tokens: i32,
    pub temperature: f32,
}

/// LLM response.
#[derive(Debug, Clone)]
pub struct ChatResponse {
    pub message: Message,
    pub usage: Usage,
    pub stop_reason: String,
}

/// Token usage statistics.
#[derive(Debug, Clone, Default)]
pub struct Usage {
    pub input_tokens: i32,
    pub output_tokens: i32,
}

/// Tool definition for the LLM.
#[derive(Debug, Clone)]
pub struct ToolDefinition {
    pub name: String,
    pub description: String,
    pub parameters: serde_json::Value,
}

/// Errors returned by tool executions, representing different recovery strategies.
#[derive(Debug, Clone)]
pub enum ToolError {
    /// A temporary error (e.g. network failure) that should be retried with backoff.
    Transient(String),
    /// An error that the LLM can fix if we feed it back to it.
    LlmRecoverable(String),
    /// An error requiring the user to intervene (e.g. missing credentials).
    UserFixable(String),
    /// An unexpected error that should abort the execution loop.
    Unexpected(String),
}

impl std::fmt::Display for ToolError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ToolError::Transient(msg) => write!(f, "Transient tool error: {}", msg),
            ToolError::LlmRecoverable(msg) => write!(f, "Tool execution failed: {}", msg),
            ToolError::UserFixable(msg) => write!(f, "User action required: {}", msg),
            ToolError::Unexpected(msg) => write!(f, "Unexpected tool error: {}", msg),
        }
    }
}

impl std::error::Error for ToolError {}
