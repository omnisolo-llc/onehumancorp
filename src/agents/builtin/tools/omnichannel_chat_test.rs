use super::omnichannel_chat::{omnichannel_chat_tool, OmnichannelChatArgs, OmnichannelChatExecutor};
use super::pydantic::PydanticToolExecutor;
use ohc_builtin_agent_core::types::ToolError;

#[tokio::test]
async fn test_omnichannel_chat_create_contact() {
    let executor = OmnichannelChatExecutor;
    let res = executor
        .execute_typed(OmnichannelChatArgs {
            action: "create_contact".to_string(),
            tenant_id: "tenant-123".to_string(),
            conversation_id: None,
            contact_id: None,
            channel: None,
            message: None,
            tags: None,
        })
        .await
        .unwrap();

    assert!(res.contains("tenant-123"));
    assert!(res.contains("create_contact"));
}

#[tokio::test]
async fn test_omnichannel_chat_invalid_action() {
    let executor = OmnichannelChatExecutor;
    let res = executor
        .execute_typed(OmnichannelChatArgs {
            action: "invalid_action".to_string(),
            tenant_id: "tenant-123".to_string(),
            conversation_id: None,
            contact_id: None,
            channel: None,
            message: None,
            tags: None,
        })
        .await;

    assert!(res.is_err());
    match res.unwrap_err() {
        ToolError::LlmRecoverable(msg) => {
            assert!(msg.contains("Validation Error (Pydantic-first tool schema)"));
        }
        _ => panic!("Expected LlmRecoverable error for invalid action"),
    }
}
