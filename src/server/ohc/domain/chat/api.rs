use super::models::{ChatConversation, ChatMessage};
use tonic::Status;

// Mock response structs instead of depending on inboxpb if it's not exported
#[derive(Debug, Clone)]
pub struct FetchConversationsRequest {
    pub tenant_id: String,
}

#[derive(Debug, Clone)]
pub struct FetchConversationsResponse {
    pub conversations: Vec<ChatConversation>,
}

#[derive(Debug, Clone)]
pub struct FetchMessagesRequest {
    pub tenant_id: String,
    pub conversation_id: String,
}

#[derive(Debug, Clone)]
pub struct FetchMessagesResponse {
    pub messages: Vec<ChatMessage>,
}

pub async fn list_conversations(req: FetchConversationsRequest) -> Result<FetchConversationsResponse, Status> {
    let convo = ChatConversation {
        id: "convo-1".to_string(),
        tenant_id: req.tenant_id.clone(),
        inbox_id: "inbox-1".to_string(),
        contact_id: "cust-1".to_string(),
        assignee_id: None,
        status: "open".to_string(),
        created_at: None,
        updated_at: None,
    };
    Ok(FetchConversationsResponse { conversations: vec![convo] })
}

pub async fn list_messages(req: FetchMessagesRequest) -> Result<FetchMessagesResponse, Status> {
    let msg = ChatMessage {
        id: "msg-1".to_string(),
        tenant_id: req.tenant_id.clone(),
        conversation_id: req.conversation_id.clone(),
        sender_type: "customer".to_string(),
        sender_id: Some("cust-1".to_string()),
        content: "Hello from customer".to_string(),
        created_at: None,
        updated_at: None,
    };
    Ok(FetchMessagesResponse { messages: vec![msg] })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_list_conversations() {
        let req = FetchConversationsRequest { tenant_id: "tenant-1".to_string() };
        let res = list_conversations(req).await.unwrap();
        assert_eq!(res.conversations.len(), 1);
        assert_eq!(res.conversations[0].id, "convo-1");
    }

    #[tokio::test]
    async fn test_list_messages() {
        let req = FetchMessagesRequest { tenant_id: "tenant-1".to_string(), conversation_id: "convo-1".to_string() };
        let res = list_messages(req).await.unwrap();
        assert_eq!(res.messages.len(), 1);
        assert_eq!(res.messages[0].id, "msg-1");
    }
}
