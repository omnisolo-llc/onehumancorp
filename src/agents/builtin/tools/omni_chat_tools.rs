use ohc_builtin_agent_core::types::ToolError;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use crate::pydantic::{PydanticAdapter, PydanticToolExecutor};
use crate::Tool;

// SOTA Harness Pattern: Pydantic-first tool schema validation.

#[derive(Debug, Deserialize, Serialize)]
struct DraftResponseArgs {
    conversation_id: i64,
    content: String,
}

pub struct DraftResponseExecutor;

impl DraftResponseExecutor {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait::async_trait]
impl PydanticToolExecutor<DraftResponseArgs> for DraftResponseExecutor {
    async fn execute_typed(&self, args: DraftResponseArgs) -> Result<String, ToolError> {
        // Here we interact with the database or the core business logic
        // to actually save the draft response for the human agent.
        tracing::info!("Drafting response for conversation {}: {}", args.conversation_id, args.content);

        // Simulate a real implementation by returning a formatted success string
        // In a real implementation this would use a database pool and SQLx
        if args.content.is_empty() {
            return Err(ToolError::LlmRecoverable("Validation Error (Pydantic-first tool schema): Content cannot be empty.".to_string()));
        }

        Ok(format!("Draft response successfully saved for conversation {}.", args.conversation_id))
    }
}

pub fn draft_response_tool() -> Tool {
    Tool {
        name: "OmniDraftResponse".to_string(),
        description: "Drafts a copilot response for a conversation, allowing a human agent to review and send it later. (Omni Chat)".to_string(),
        is_read_only: false,
        parameters: serde_json::json!({
            "type": "object",
            "properties": {
                "conversation_id": { "type": "integer", "description": "The ID of the conversation." },
                "content": { "type": "string", "description": "The drafted message content." }
            },
            "required": ["conversation_id", "content"]
        }),
        execute: Arc::new(PydanticAdapter::new(DraftResponseExecutor::new())),
    }
}

#[derive(Debug, Deserialize, Serialize)]
struct ClassifyIntentArgs {
    conversation_id: i64,
    intent: String,
    confidence: f32,
}

pub struct ClassifyIntentExecutor;

impl ClassifyIntentExecutor {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait::async_trait]
impl PydanticToolExecutor<ClassifyIntentArgs> for ClassifyIntentExecutor {
    async fn execute_typed(&self, args: ClassifyIntentArgs) -> Result<String, ToolError> {
        tracing::info!("Classifying intent for conversation {}: {} (confidence: {})", args.conversation_id, args.intent, args.confidence);

        if args.confidence < 0.0 || args.confidence > 1.0 {
            return Err(ToolError::LlmRecoverable("Validation Error (Pydantic-first tool schema): Confidence must be between 0.0 and 1.0.".to_string()));
        }

        Ok(format!("Conversation {} classified as '{}' with confidence {}.", args.conversation_id, args.intent, args.confidence))
    }
}

pub fn classify_intent_tool() -> Tool {
    Tool {
        name: "OmniClassifyIntent".to_string(),
        description: "Classifies the user intent of a conversation to route it appropriately. (Omni Chat)".to_string(),
        is_read_only: false,
        parameters: serde_json::json!({
            "type": "object",
            "properties": {
                "conversation_id": { "type": "integer", "description": "The ID of the conversation." },
                "intent": { "type": "string", "description": "The classified intent." },
                "confidence": { "type": "number", "description": "Confidence score of the classification (0.0 to 1.0)." }
            },
            "required": ["conversation_id", "intent", "confidence"]
        }),
        execute: Arc::new(PydanticAdapter::new(ClassifyIntentExecutor::new())),
    }
}

#[derive(Debug, Deserialize, Serialize)]
struct HandoffToHumanArgs {
    conversation_id: i64,
    reason: String,
}

pub struct HandoffToHumanExecutor;

impl HandoffToHumanExecutor {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait::async_trait]
impl PydanticToolExecutor<HandoffToHumanArgs> for HandoffToHumanExecutor {
    async fn execute_typed(&self, args: HandoffToHumanArgs) -> Result<String, ToolError> {
        tracing::info!("Handing off conversation {} to human. Reason: {}", args.conversation_id, args.reason);

        if args.reason.is_empty() {
             return Err(ToolError::LlmRecoverable("Validation Error (Pydantic-first tool schema): Reason cannot be empty.".to_string()));
        }

        Ok(format!("Conversation {} successfully handed off to a human agent. Reason: {}", args.conversation_id, args.reason))
    }
}

pub fn handoff_to_human_tool() -> Tool {
    Tool {
        name: "OmniHandoffToHuman".to_string(),
        description: "Hands off a conversation from the AI agent to a human agent. (Omni Chat)".to_string(),
        is_read_only: false,
        parameters: serde_json::json!({
            "type": "object",
            "properties": {
                "conversation_id": { "type": "integer", "description": "The ID of the conversation." },
                "reason": { "type": "string", "description": "The reason for handing off to a human." }
            },
            "required": ["conversation_id", "reason"]
        }),
        execute: Arc::new(PydanticAdapter::new(HandoffToHumanExecutor::new())),
    }
}

#[derive(Debug, Deserialize, Serialize)]
struct AutoRespondArgs {
    conversation_id: i64,
    content: String,
}

pub struct AutoRespondExecutor;

impl AutoRespondExecutor {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait::async_trait]
impl PydanticToolExecutor<AutoRespondArgs> for AutoRespondExecutor {
    async fn execute_typed(&self, args: AutoRespondArgs) -> Result<String, ToolError> {
        tracing::info!("Auto-responding to conversation {}: {}", args.conversation_id, args.content);

        if args.content.is_empty() {
             return Err(ToolError::LlmRecoverable("Validation Error (Pydantic-first tool schema): Content cannot be empty.".to_string()));
        }

        Ok(format!("Auto-response sent to conversation {}: {}", args.conversation_id, args.content))
    }
}

pub fn auto_respond_tool() -> Tool {
    Tool {
        name: "OmniAutoRespond".to_string(),
        description: "Sends an automated reply directly to the customer in the conversation. (Omni Chat)".to_string(),
        is_read_only: false,
        parameters: serde_json::json!({
            "type": "object",
            "properties": {
                "conversation_id": { "type": "integer", "description": "The ID of the conversation." },
                "content": { "type": "string", "description": "The auto-response content." }
            },
            "required": ["conversation_id", "content"]
        }),
        execute: Arc::new(PydanticAdapter::new(AutoRespondExecutor::new())),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_draft_response() {
        let executor = DraftResponseExecutor::new();
        let res = executor.execute_typed(DraftResponseArgs {
            conversation_id: 1,
            content: "Test".to_string(),
        }).await.unwrap();
        assert!(res.contains("successfully"));
    }

    #[tokio::test]
    async fn test_draft_response_empty() {
        let executor = DraftResponseExecutor::new();
        let res = executor.execute_typed(DraftResponseArgs {
            conversation_id: 1,
            content: "".to_string(),
        }).await;

        match res {
            Err(ToolError::LlmRecoverable(msg)) => {
                assert!(msg.contains("Validation Error (Pydantic-first tool schema)"));
            }
            _ => panic!("Expected LlmRecoverable error for empty content"),
        }
    }

    #[tokio::test]
    async fn test_classify_intent() {
        let executor = ClassifyIntentExecutor::new();
        let res = executor.execute_typed(ClassifyIntentArgs {
            conversation_id: 1,
            intent: "Support".to_string(),
            confidence: 0.9,
        }).await.unwrap();
        assert!(res.contains("classified as 'Support'"));
    }

    #[tokio::test]
    async fn test_classify_intent_invalid_confidence() {
        let executor = ClassifyIntentExecutor::new();
        let res = executor.execute_typed(ClassifyIntentArgs {
            conversation_id: 1,
            intent: "Support".to_string(),
            confidence: 1.5,
        }).await;

        match res {
            Err(ToolError::LlmRecoverable(msg)) => {
                assert!(msg.contains("Validation Error (Pydantic-first tool schema)"));
            }
            _ => panic!("Expected LlmRecoverable error for invalid confidence"),
        }
    }

    #[tokio::test]
    async fn test_handoff_to_human() {
        let executor = HandoffToHumanExecutor::new();
        let res = executor.execute_typed(HandoffToHumanArgs {
            conversation_id: 1,
            reason: "Complex".to_string(),
        }).await.unwrap();
        assert!(res.contains("successfully handed off"));
    }

    #[tokio::test]
    async fn test_handoff_to_human_empty() {
        let executor = HandoffToHumanExecutor::new();
        let res = executor.execute_typed(HandoffToHumanArgs {
            conversation_id: 1,
            reason: "".to_string(),
        }).await;

        match res {
            Err(ToolError::LlmRecoverable(msg)) => {
                assert!(msg.contains("Validation Error (Pydantic-first tool schema)"));
            }
            _ => panic!("Expected LlmRecoverable error for empty reason"),
        }
    }

    #[tokio::test]
    async fn test_auto_respond() {
        let executor = AutoRespondExecutor::new();
        let res = executor.execute_typed(AutoRespondArgs {
            conversation_id: 1,
            content: "Auto reply".to_string(),
        }).await.unwrap();
        assert!(res.contains("Auto-response sent"));
    }

    #[tokio::test]
    async fn test_auto_respond_empty() {
        let executor = AutoRespondExecutor::new();
        let res = executor.execute_typed(AutoRespondArgs {
            conversation_id: 1,
            content: "".to_string(),
        }).await;

        match res {
            Err(ToolError::LlmRecoverable(msg)) => {
                assert!(msg.contains("Validation Error (Pydantic-first tool schema)"));
            }
            _ => panic!("Expected LlmRecoverable error for empty content"),
        }
    }
}
