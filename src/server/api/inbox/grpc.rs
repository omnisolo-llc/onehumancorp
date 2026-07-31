use sqlx::PgPool;
use tonic::{Request, Response, Status};
use uuid::Uuid;

use crate::services::chat::service::ChatService;
use ::server_ohc::inbox::inbox_service_server::InboxService;
use ::server_ohc::inbox::{
    FetchConversationsRequest, FetchConversationsResponse, FetchMessagesRequest,
    FetchMessagesResponse, OmniMessage, Conversation,
};

pub struct InboxGrpcService {
    chat_service: ChatService,
}

impl InboxGrpcService {
    pub fn new(pool: PgPool) -> Self {
        Self {
            chat_service: ChatService::new(pool),
        }
    }
}

#[tonic::async_trait]
impl InboxService for InboxGrpcService {
    async fn fetch_conversations(
        &self,
        request: Request<FetchConversationsRequest>,
    ) -> Result<Response<FetchConversationsResponse>, Status> {
        let req = request.into_inner();
        let tenant_id = Uuid::parse_str(&req.tenant_id)
            .map_err(|e| Status::invalid_argument(format!("Invalid tenant_id: {}", e)))?;

        let convs = self
            .chat_service
            .fetch_conversations(tenant_id)
            .await
            .map_err(|e| Status::internal(format!("Database error: {}", e)))?;

        let grpc_convs = convs
            .into_iter()
            .map(|c| Conversation {
                id: c.id.to_string(),
                tenant_id: c.tenant_id.to_string(),
                customer_id: c.contact_id.to_string(),
                channel: "omnichannel".to_string(),
                status: c.status,
                created_at_unix: c.created_at.timestamp(),
                updated_at_unix: c.updated_at.timestamp(),
            })
            .collect();

        Ok(Response::new(FetchConversationsResponse {
            conversations: grpc_convs,
        }))
    }

    async fn fetch_messages(
        &self,
        request: Request<FetchMessagesRequest>,
    ) -> Result<Response<FetchMessagesResponse>, Status> {
        let req = request.into_inner();
        let tenant_id = Uuid::parse_str(&req.tenant_id)
            .map_err(|e| Status::invalid_argument(format!("Invalid tenant_id: {}", e)))?;
        let conversation_id = Uuid::parse_str(&req.conversation_id)
            .map_err(|e| Status::invalid_argument(format!("Invalid conversation_id: {}", e)))?;

        let msgs = self
            .chat_service
            .fetch_messages(tenant_id, conversation_id)
            .await
            .map_err(|e| Status::internal(format!("Database error: {}", e)))?;

        let grpc_msgs = msgs
            .into_iter()
            .map(|m| OmniMessage {
                id: m.id.to_string(),
                tenant_id: m.tenant_id.to_string(),
                source: "chat".to_string(),
                original_content: m.content.clone(),
                translated_content: m.content,
                source_language: "English".to_string(),
                target_language: "English".to_string(),
                draft_reply: "".to_string(),
                status: "sent".to_string(),
                sender_id: m.sender_id.map(|id| id.to_string()).unwrap_or_default(),
                customer_id: "".to_string(),
                created_at_unix: m.created_at.timestamp(),
                updated_at_unix: m.updated_at.timestamp(),
            })
            .collect();

        Ok(Response::new(FetchMessagesResponse { messages: grpc_msgs }))
    }
}
