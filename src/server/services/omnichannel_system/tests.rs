#[cfg(test)]
mod tests {
    use crate::services::omnichannel_system::grpc::OmnichannelServiceImpl;
    use crate::ohc::omnichannel::omnichannel_service_server::OmnichannelService;
    use crate::ohc::omnichannel::{
        CreateInboxRequest, CreateConversationRequest, CreateMessageRequest,
    };
    use tonic::Request;

    #[tokio::test]
    async fn test_create_inbox() {
        let service = OmnichannelServiceImpl::default();
        let req = Request::new(CreateInboxRequest {
            tenant_id: "tenant-1".to_string(),
            name: "Test Inbox".to_string(),
        });

        let res = service.create_inbox(req).await.unwrap().into_inner();
        let inbox = res.inbox.unwrap();
        assert_eq!(inbox.tenant_id, "tenant-1");
        assert_eq!(inbox.name, "Test Inbox");
        assert!(!inbox.id.is_empty());
    }

    #[tokio::test]
    async fn test_create_conversation() {
        let service = OmnichannelServiceImpl::default();
        let req = Request::new(CreateConversationRequest {
            inbox_id: "inbox-1".to_string(),
            contact_id: "contact-1".to_string(),
        });

        let res = service.create_conversation(req).await.unwrap().into_inner();
        let conv = res.conversation.unwrap();
        assert_eq!(conv.inbox_id, "inbox-1");
        assert_eq!(conv.contact_id, "contact-1");
        assert_eq!(conv.status, "open");
        assert!(!conv.id.is_empty());
    }

    #[tokio::test]
    async fn test_create_message() {
        let service = OmnichannelServiceImpl::default();
        let req = Request::new(CreateMessageRequest {
            conversation_id: "conv-1".to_string(),
            content: "Hello World".to_string(),
        });

        let res = service.create_message(req).await.unwrap().into_inner();
        let msg = res.message.unwrap();
        assert_eq!(msg.conversation_id, "conv-1");
        assert_eq!(msg.content, "Hello World");
        assert_eq!(msg.status, "sent");
        assert!(!msg.id.is_empty());
    }
}
