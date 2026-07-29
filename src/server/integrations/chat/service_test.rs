use crate::pb::{CreateInboxRequest, Inbox, CreateConversationRequest, Conversation};
use tonic::Request;
use uuid::Uuid;

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_create_inbox_request_structure() {
        let mut request = Request::new(CreateInboxRequest {
            inbox: Some(Inbox {
                id: "".to_string(),
                tenant_id: Uuid::new_v4().to_string(),
                name: "Test Inbox".to_string(),
                channel_type: "web_widget".to_string(),
            }),
        });

        let auth_info = ::server_auth::orchestration::AuthInfo {
            user_id: "user_1".to_string(),
            org_id: Uuid::new_v4().to_string(),
            roles: vec![],
        };
        request.extensions_mut().insert(auth_info.clone());

        let req = request.into_inner();
        assert!(req.inbox.is_some());
        let inbox = req.inbox.unwrap();
        assert_eq!(inbox.name, "Test Inbox");
        assert!(Uuid::parse_str(&inbox.tenant_id).is_ok());
    }

    #[tokio::test]
    async fn test_create_conversation_request_structure() {
        let mut request = Request::new(CreateConversationRequest {
            conversation: Some(Conversation {
                id: "".to_string(),
                tenant_id: Uuid::new_v4().to_string(),
                inbox_id: Uuid::new_v4().to_string(),
                contact_id: Uuid::new_v4().to_string(),
                status: "open".to_string(),
            }),
        });

        let auth_info = ::server_auth::orchestration::AuthInfo {
            user_id: "user_1".to_string(),
            org_id: Uuid::new_v4().to_string(),
            roles: vec![],
        };
        request.extensions_mut().insert(auth_info.clone());

        let req = request.into_inner();
        assert!(req.conversation.is_some());
        let conv = req.conversation.unwrap();
        assert_eq!(conv.status, "open");
        assert!(Uuid::parse_str(&conv.tenant_id).is_ok());
    }
}
