use tonic::{Request, Response, Status};
use uuid::Uuid;
use sqlx::PgPool;

use server_ohc::ohc::chat::chat_service_server::ChatService;
use server_ohc::ohc::chat::{
    ListConversationsRequest, ListConversationsResponse,
    GetMessagesRequest, GetMessagesResponse,
    SendMessageRequest, SendMessageResponse,
    Conversation, Message,
};



pub struct ChatServiceImpl {
    pub db: PgPool,
}

#[tonic::async_trait]
impl ChatService for ChatServiceImpl {
    async fn list_conversations(
        &self,
        request: Request<ListConversationsRequest>,
    ) -> Result<Response<ListConversationsResponse>, Status> {
        let req = request.into_inner();
        let tenant_id = Uuid::parse_str(&req.tenant_id)
            .map_err(|_| Status::invalid_argument("Invalid tenant_id"))?;

        let conversations = vec![
            Conversation {
                id: Uuid::new_v4().to_string(),
                tenant_id: tenant_id.to_string(),
                inbox_id: Uuid::new_v4().to_string(),
                contact_id: Uuid::new_v4().to_string(),
                status: "open".to_string(),
                created_at_unix: 0,
                updated_at_unix: 0,
            }
        ];

        Ok(Response::new(ListConversationsResponse { conversations }))
    }

    async fn get_messages(
        &self,
        request: Request<GetMessagesRequest>,
    ) -> Result<Response<GetMessagesResponse>, Status> {
        let req = request.into_inner();
        let tenant_id = Uuid::parse_str(&req.tenant_id)
            .map_err(|_| Status::invalid_argument("Invalid tenant_id"))?;
        let conversation_id = Uuid::parse_str(&req.conversation_id)
            .map_err(|_| Status::invalid_argument("Invalid conversation_id"))?;

        let messages = vec![
            Message {
                id: Uuid::new_v4().to_string(),
                tenant_id: tenant_id.to_string(),
                conversation_id: conversation_id.to_string(),
                inbox_id: Uuid::new_v4().to_string(),
                sender_type: "user".to_string(),
                sender_id: Uuid::new_v4().to_string(),
                content_type: "text".to_string(),
                content: "Hello!".to_string(),
                created_at_unix: 0,
                updated_at_unix: 0,
            }
        ];

        Ok(Response::new(GetMessagesResponse { messages }))
    }

    async fn send_message(
        &self,
        request: Request<SendMessageRequest>,
    ) -> Result<Response<SendMessageResponse>, Status> {
        let req = request.into_inner();
        let tenant_id = Uuid::parse_str(&req.tenant_id)
            .map_err(|_| Status::invalid_argument("Invalid tenant_id"))?;
        let conversation_id = Uuid::parse_str(&req.conversation_id)
            .map_err(|_| Status::invalid_argument("Invalid conversation_id"))?;

        let message = Message {
            id: Uuid::new_v4().to_string(),
            tenant_id: tenant_id.to_string(),
            conversation_id: conversation_id.to_string(),
            inbox_id: Uuid::new_v4().to_string(),
            sender_type: req.sender_type,
            sender_id: req.sender_id,
            content_type: req.content_type,
            content: req.content,
            created_at_unix: 0,
            updated_at_unix: 0,
        };

        Ok(Response::new(SendMessageResponse { message: Some(message) }))
    }
}
