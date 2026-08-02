use tonic::{Request, Response, Status};
use uuid::Uuid;
use sqlx::PgPool;

use crate::ohc::inbox::{
    FetchConversationsRequest, FetchConversationsResponse,
    FetchMessagesRequest, FetchMessagesResponse,
    CreateInboxRequest, CreateInboxResponse,
    SendMessageRequest, SendMessageResponse,
    ChatInbox as ProtoChatInbox, OmniMessage,
    inbox_service_server::InboxService,
};
use crate::ohc::chat::service::ChatService;

pub struct InboxServiceHandler {
    chat_service: ChatService,
}

impl InboxServiceHandler {
    pub fn new(chat_service: ChatService) -> Self {
        Self { chat_service }
    }
}

#[tonic::async_trait]
impl InboxService for InboxServiceHandler {
    async fn fetch_conversations(
        &self,
        _request: Request<FetchConversationsRequest>,
    ) -> Result<Response<FetchConversationsResponse>, Status> {
        Err(Status::unimplemented("Not implemented"))
    }

    async fn fetch_messages(
        &self,
        _request: Request<FetchMessagesRequest>,
    ) -> Result<Response<FetchMessagesResponse>, Status> {
        Err(Status::unimplemented("Not implemented"))
    }

    async fn create_inbox(
        &self,
        request: Request<CreateInboxRequest>,
    ) -> Result<Response<CreateInboxResponse>, Status> {
        let req = request.into_inner();
        let tenant_id = Uuid::parse_str(&req.tenant_id).map_err(|_| Status::invalid_argument("Invalid tenant_id"))?;

        let inbox = self.chat_service.create_inbox(tenant_id, &req.name).await.map_err(|e| Status::internal(e.to_string()))?;

        Ok(Response::new(CreateInboxResponse {
            inbox: Some(ProtoChatInbox {
                id: inbox.id.to_string(),
                tenant_id: inbox.tenant_id.to_string(),
                name: inbox.name,
                created_at_unix: inbox.created_at.timestamp(),
                updated_at_unix: inbox.updated_at.timestamp(),
            }),
        }))
    }

    async fn send_message(
        &self,
        request: Request<SendMessageRequest>,
    ) -> Result<Response<SendMessageResponse>, Status> {
        let req = request.into_inner();
        let tenant_id = Uuid::parse_str(&req.tenant_id).map_err(|_| Status::invalid_argument("Invalid tenant_id"))?;
        let conversation_id = Uuid::parse_str(&req.conversation_id).map_err(|_| Status::invalid_argument("Invalid conversation_id"))?;

        let sender_id = if req.sender_id.is_empty() {
            None
        } else {
            Some(Uuid::parse_str(&req.sender_id).map_err(|_| Status::invalid_argument("Invalid sender_id"))?)
        };

        let msg = self.chat_service.send_message(tenant_id, conversation_id, &req.sender_type, sender_id, &req.content).await.map_err(|e| Status::internal(e.to_string()))?;

        Ok(Response::new(SendMessageResponse {
            message: Some(OmniMessage {
                id: msg.id.to_string(),
                tenant_id: msg.tenant_id.to_string(),
                source: "native".to_string(),
                original_content: msg.content,
                translated_content: "".to_string(),
                source_language: "".to_string(),
                target_language: "".to_string(),
                draft_reply: "".to_string(),
                status: "sent".to_string(),
                sender_id: msg.sender_id.map(|id| id.to_string()).unwrap_or_default(),
                customer_id: "".to_string(),
                created_at_unix: msg.created_at.timestamp(),
                updated_at_unix: msg.updated_at.timestamp(),
            }),
        }))
    }
}
