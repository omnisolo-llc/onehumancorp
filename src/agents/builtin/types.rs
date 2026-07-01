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

/// Formats an LLM-Recoverable error string according to the LangGraph 4-tier error handling mechanic.
pub fn format_llm_recoverable_error(tool_name: &str, msg: &str) -> String {
    format!("LLM-Recoverable Tool Error ({}): {}

SOTA Recovery Protocol: Please deeply analyze this validation/execution error, verify your previous arguments against the tool's strict Pydantic JSON schema, correct the arguments, and call the tool again.", tool_name, msg)
}

impl ToolResult {
    /// Creates a standard LLM-Recoverable ToolResult for the LangGraph 4-tier error handling mechanic.
    /// This strictly standardizes how self-correcting feedback is structured to the model.
    pub fn new_llm_recoverable(tool_call_id: String, tool_name: &str, msg: &str) -> Self {
        Self {
            tool_call_id,
            content: String::new(),
            error: format_llm_recoverable_error(tool_name, msg),
        }
    }
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
#[derive(Debug, Clone, serde::Serialize)]
pub struct ToolDefinition {
    pub name: String,
    pub description: String,
    pub parameters: serde_json::Value,
}

/// Master Catalog B.8. Error Handling (Compounding Error Prevention): Stripe limits retries to exactly 2. LangGraph Mechanic (4-types): 1) Transient (retry with backoff), 2) LLM-recoverable (return the raw error as a ToolMessage directly to the model so it can self-correct), 3) User-fixable (interrupt execution and ask user for input), 4) Unexpected (bubble up to debug).
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

/// SOTA Harness Patterns (2025-2026): 5. Human-in-loop as spectrum -> not binary autonomy vs control
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum HumanInLoopSpectrum {
    /// Operates without human intervention except for strictly defined high-risk tools.
    Autonomous,
    /// Requires human approval for tools that modify state (the classic "Restrictive" mode).
    ApprovalOnMutate,
    /// Requires human explicit approval before executing any tool, including read-only ones.
    ApprovalOnAll,
    /// Expects the human to actively review and optionally edit the tool arguments before execution.
    CollaborativeEdit,
    /// Triggers human intervention only under specific conditions (e.g. low confidence or specific triggers, falling back to Autonomous otherwise).
    Supervisory,
}

impl std::fmt::Display for HumanInLoopSpectrum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HumanInLoopSpectrum::Autonomous => write!(f, "Autonomous"),
            HumanInLoopSpectrum::ApprovalOnMutate => write!(f, "ApprovalOnMutate"),
            HumanInLoopSpectrum::ApprovalOnAll => write!(f, "ApprovalOnAll"),
            HumanInLoopSpectrum::CollaborativeEdit => write!(f, "CollaborativeEdit"),
            HumanInLoopSpectrum::Supervisory => write!(f, "Supervisory"),
        }
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, Eq)]
#[derive(Default)]
/// Master Catalog C.5. Permission Architecture: Permissive (auto-approve) vs Restrictive (require approval)
pub enum PermissionArchitecture {
    /// Permissive (auto-approve): All tools are auto-approved unless explicitly in high-risk.
    #[default]
    Permissive,
    /// Restrictive (require approval): All mutating tools require explicit approval.
    Restrictive,
}

impl std::fmt::Display for PermissionArchitecture {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PermissionArchitecture::Permissive => write!(f, "Permissive"),
            PermissionArchitecture::Restrictive => write!(f, "Restrictive"),
        }
    }
}

/// Centralized Pydantic-first tool schema error formatter.
pub fn format_pydantic_error(e: &serde_json::Error, args_str: Option<&str>, custom_instruction: Option<&str>) -> String {
    let error_type = if e.is_data() { "type_error" } else if e.is_syntax() { "syntax_error" } else if e.is_eof() { "eof_error" } else { "unknown_error" };
    let msg_content = format!("{}", e);
    let snippet = args_str.unwrap_or("null");

    let extended_msg_content = if e.is_data() {
        format!("Semantic validation failed: {}", msg_content)
    } else if e.is_syntax() {
        format!("JSON syntax error: {}", msg_content)
    } else if e.is_eof() {
        format!("Incomplete JSON structure (unexpected EOF): {}", msg_content)
    } else {
        msg_content.clone()
    };

    // Inject exact line and column data to provide the LLM with pinpoint error location
    let line = e.line();
    let column = e.column();
    let precise_msg = format!("{} at line {}, column {}", extended_msg_content, line, column);

    let mut pydantic_json_obj = serde_json::json!({
        "type": error_type,
        "loc": ["data", format!("line_{}", line), format!("col_{}", column)],
        "msg": precise_msg,
    });

    if args_str.is_some() {
        pydantic_json_obj.as_object_mut().unwrap().insert("input".to_string(), serde_json::Value::String(snippet.to_string()));
    } else {
        pydantic_json_obj.as_object_mut().unwrap().insert("input".to_string(), serde_json::Value::String("null".to_string()));
    }

    let pydantic_json = serde_json::json!([pydantic_json_obj]);

    let mut msg = format!(
        "Validation Error (Pydantic-first tool schema): Failed to parse arguments.\nReason: {}",
        serde_json::to_string_pretty(&pydantic_json).unwrap_or(msg_content)
    );

    if let Some(instruction) = custom_instruction {
        msg.push_str(&format!("\n{}", instruction));
    } else {
        msg.push_str("\nPlease strictly follow the tool's JSON schema and verify that all required fields are present and of the correct type.");
    }
    msg
}

/// A version of format_pydantic_error that takes a string message instead of a serde_json::Error.
/// Used when validation fails via manual checks rather than serde deserialization.
pub fn format_pydantic_error_string(error_msg: &str, args_str: Option<&str>, custom_instruction: Option<&str>) -> String {
    let snippet = args_str.unwrap_or("null");
    let pydantic_json = serde_json::json!([{
        "type": "value_error",
        "loc": ["data"],
        "msg": error_msg,
        "input": snippet
    }]);

    let mut msg = format!(
        "Validation Error (Pydantic-first tool schema): Failed to parse arguments.\nReason: {}",
        serde_json::to_string_pretty(&pydantic_json).unwrap_or_else(|_| error_msg.to_string())
    );

    if let Some(instruction) = custom_instruction {
        msg.push_str(&format!("\n{}", instruction));
    } else {
        msg.push_str("\nPlease strictly follow the tool's JSON schema and verify that all required fields are present and of the correct type.");
    }
    msg
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_pydantic_error() {
        use serde::Deserialize;

        #[derive(Deserialize, Debug)]
        struct Dummy {
            _field: u32,
        }

        // Test syntax error
        let err_syntax = serde_json::from_str::<Dummy>("{ bad json }").unwrap_err();
        let msg_syntax = format_pydantic_error(&err_syntax, Some("{ bad json }"), None);
        assert!(msg_syntax.contains("Validation Error (Pydantic-first tool schema)"));
        assert!(msg_syntax.contains("JSON syntax error"));
        assert!(msg_syntax.contains("line 1, column"));
        // assert!(msg_syntax.contains("Provided arguments snippet: { bad json }"));

        // Test EOF error
        let err_eof = serde_json::from_str::<Dummy>("{\"_field\": 12").unwrap_err();
        let msg_eof = format_pydantic_error(&err_eof, None, None);
        assert!(msg_eof.contains("Incomplete JSON structure (unexpected EOF)"));
        assert!(msg_eof.contains("line 1, column"));
        assert!(!msg_eof.contains("Provided arguments snippet"));

        // Test semantic/data error
        let err_semantic = serde_json::from_str::<Dummy>("{\"_field\": \"string instead of int\"}").unwrap_err();
        let msg_semantic = format_pydantic_error(&err_semantic, Some("{\"_field\": \"string instead of int\"}"), None);
        assert!(msg_semantic.contains("Semantic validation failed"));
        assert!(msg_semantic.contains("line 1, column"));
        assert!(msg_semantic.contains("line_1"));
    }

    #[test]
    fn test_role_display() {
        assert_eq!(Role::User.to_string(), "user");
        assert_eq!(Role::Assistant.to_string(), "assistant");
        assert_eq!(Role::System.to_string(), "system");
        assert_eq!(Role::Tool.to_string(), "tool");
    }

    #[test]
    fn test_message_constructors() {
        let u_msg = Message::user("Hello");
        assert_eq!(u_msg.role, Role::User);
        assert_eq!(u_msg.content, "Hello");

        let a_msg = Message::assistant("Hi there");
        assert_eq!(a_msg.role, Role::Assistant);
        assert_eq!(a_msg.content, "Hi there");

        let s_msg = Message::system("System instructions");
        assert_eq!(s_msg.role, Role::System);
        assert_eq!(s_msg.content, "System instructions");
    }

    #[test]
    fn test_tool_error_display() {
        assert_eq!(
            ToolError::Transient("timeout".to_string()).to_string(),
            "Transient error: timeout"
        );
        assert_eq!(
            ToolError::LlmRecoverable("bad args".to_string()).to_string(),
            "Recoverable error: bad args"
        );
        assert_eq!(
            ToolError::UserFixable("need approval".to_string()).to_string(),
            "User intervention required: need approval"
        );
        assert_eq!(
            ToolError::Fatal("crash".to_string()).to_string(),
            "Fatal error: crash"
        );
        assert_eq!(
            ToolError::Unexpected("wut".to_string()).to_string(),
            "Unexpected error: wut"
        );
        assert_eq!(
            ToolError::HandoffRequested("agent2".to_string()).to_string(),
            "Handoff requested to: agent2"
        );
    }

    #[test]
    fn test_permission_architecture_default() {
        assert_eq!(
            PermissionArchitecture::default(),
            PermissionArchitecture::Permissive
        );
    }
}

#[cfg(test)]
mod tests_custom {
    use super::*;

    #[test]
    fn test_format_pydantic_error_custom_instruction() {
        use serde::Deserialize;

        #[derive(Deserialize, Debug)]
        struct Dummy {
            _field: u32,
        }

        let err_semantic = serde_json::from_str::<Dummy>("{\"_field\": \"string instead of int\"}").unwrap_err();
        let msg_semantic = format_pydantic_error(&err_semantic, Some("{\"_field\": \"string instead of int\"}"), Some("Please provide an integer value for _field."));

        assert!(msg_semantic.contains("Semantic validation failed"));
        assert!(msg_semantic.contains("Please provide an integer value for _field."));
        assert!(!msg_semantic.contains("Please strictly follow the tool's JSON schema and try again."));
    }
}
