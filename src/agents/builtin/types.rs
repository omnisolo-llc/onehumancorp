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
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub response_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub previous_response_id: Option<String>,
}

impl Message {
    pub fn user(content: impl Into<String>) -> Self {
        Self {
            role: Role::User,
            content: content.into(),
            tool_calls: vec![],
            tool_results: vec![],
            response_id: None,
            previous_response_id: None,
        }
    }

    pub fn assistant(content: impl Into<String>) -> Self {
        Self {
            role: Role::Assistant,
            content: content.into(),
            tool_calls: vec![],
            tool_results: vec![],
            response_id: None,
            previous_response_id: None,
        }
    }

    pub fn system(content: impl Into<String>) -> Self {
        Self {
            role: Role::System,
            content: content.into(),
            tool_calls: vec![],
            tool_results: vec![],
            response_id: None,
            previous_response_id: None,
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
    pub response_id: Option<String>,
}

/// Token usage statistics.
#[derive(Debug, Clone, Default)]
pub struct Usage {
    pub input_tokens: i32,
    pub output_tokens: i32,
    pub cache_creation_input_tokens: i32,
    pub cache_read_input_tokens: i32,
}

/// Tool definition for the LLM.
#[derive(Debug, Clone)]
#[derive(serde::Serialize)]
pub struct ToolDefinition {
    pub name: String,
    pub description: String,
    pub parameters: serde_json::Value,
}

/// Error Handling (Compounding Error Prevention): LangGraph Mechanic (4-types): 1) Transient, 2) LLM-recoverable, 3) User-fixable, 4) Unexpected
/// 4-tier Error enum for Tool Execution (LangGraph mechanics).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ToolError {
    /// Transient errors (e.g. network timeout). The orchestrator should retry with backoff.
    Transient(String),
    /// Errors the LLM can fix if it sees them (e.g. invalid arguments, missing required param).
    /// Passed back to the LLM as a ToolMessage containing the raw error.
    LlmRecoverable(String),
    /// Errors requiring human intervention. Pauses execution and asks the user.
    UserFixable(String),
    /// Fatal errors. Bubbles up to debug/halt immediately.
    Fatal(String),
    /// Unexpected errors. Bubbles up to debug/halt immediately.
    Unexpected(String),
    /// Yield execution to another agent.
    HandoffRequested(String),
}

impl std::fmt::Display for ToolError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Transient(msg) => write!(f, "Transient error: {}", msg),
            Self::LlmRecoverable(msg) => write!(f, "Recoverable error: {}", msg),
            Self::UserFixable(msg) => write!(f, "User intervention required: {}", msg),
            Self::Fatal(msg) => write!(f, "Fatal error: {}", msg),
            Self::Unexpected(msg) => write!(f, "Unexpected error: {}", msg),
            Self::HandoffRequested(target) => write!(f, "Handoff requested to: {}", target),
        }
    }
}

impl std::error::Error for ToolError {}

use chrono::{DateTime, Utc};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct EmbeddingRecord {
    pub id: String,
    pub tenant_id: String,
    pub agent_id: String,
    pub content: String,
    pub embedding: Vec<f32>,
    pub source_type: String,
    pub created_at: DateTime<Utc>,
    pub last_referenced_at: DateTime<Utc>,
    pub reference_count: i32,
    pub reliability_score: i32,
    pub owner_override: bool,
    pub archived: bool,
    pub metadata: Option<String>,
}

#[async_trait::async_trait]
pub trait LongTermMemory: Send + Sync + std::fmt::Debug {
    async fn retrieve(&self, query: &str, limit: usize) -> Result<Vec<String>, String>;
    async fn store(&self, content: &str, tags: Vec<String>) -> Result<(), String>;
    async fn get_lightweight_index(&self) -> Result<String, String> { Ok("".to_string()) }
    async fn retrieve_topic(&self, _topic_name: &str) -> Result<String, String> { Err("Not implemented".to_string()) }
    async fn search_transcripts(&self, _query: &str, _limit: usize) -> Result<Vec<String>, String> { Ok(vec![]) }
}
