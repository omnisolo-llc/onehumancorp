use tonic::{Request, Response, Status};
use uuid::Uuid;
use chrono::Utc;

pub mod chatpb {
    tonic::include_proto!("ohc.chat");
}

use chatpb::chat_service_server::ChatService;
use chatpb::{FetchInboxesRequest, FetchInboxesResponse, FetchConversationsRequest, FetchConversationsResponse, FetchMessagesRequest, FetchMessagesResponse, SendMessageRequest, SendMessageResponse, ApproveDraftRequest, ApproveDraftResponse};

use crate::services::chat::service::ChatService as DbChatService;

pub struct GrpcChatService {
    db_service: DbChatService,
}

impl GrpcChatService {
    pub fn new(pool: sqlx::PgPool) -> Self {
        Self {
            db_service: DbChatService::new(pool),
        }
    }
}

#[tonic::async_trait]
impl ChatService for GrpcChatService {
    async fn fetch_inboxes(
        &self,
        request: Request<FetchInboxesRequest>,
    ) -> Result<Response<FetchInboxesResponse>, Status> {
        let req = request.into_inner();
        let _tenant_id = Uuid::parse_str(&req.tenant_id).map_err(|_| Status::invalid_argument("Invalid tenant_id"))?;

        let inboxes = vec![];

        Ok(Response::new(FetchInboxesResponse {
            inboxes,
        }))
    }

    async fn fetch_conversations(
        &self,
        request: Request<FetchConversationsRequest>,
    ) -> Result<Response<FetchConversationsResponse>, Status> {
        let req = request.into_inner();
        let _tenant_id = Uuid::parse_str(&req.tenant_id).map_err(|_| Status::invalid_argument("Invalid tenant_id"))?;
        let _inbox_id = Uuid::parse_str(&req.inbox_id).map_err(|_| Status::invalid_argument("Invalid inbox_id"))?;

        let conversations = vec![];
        Ok(Response::new(FetchConversationsResponse {
            conversations,
        }))
    }

    async fn fetch_messages(
        &self,
        request: Request<FetchMessagesRequest>,
    ) -> Result<Response<FetchMessagesResponse>, Status> {
        let req = request.into_inner();
        let _tenant_id = Uuid::parse_str(&req.tenant_id).map_err(|_| Status::invalid_argument("Invalid tenant_id"))?;
        let _conversation_id = Uuid::parse_str(&req.conversation_id).map_err(|_| Status::invalid_argument("Invalid conversation_id"))?;

        let messages = vec![];
        Ok(Response::new(FetchMessagesResponse {
            messages,
        }))
    }

    async fn send_message(
        &self,
        request: Request<SendMessageRequest>,
    ) -> Result<Response<SendMessageResponse>, Status> {
        let req = request.into_inner();
        let tenant_id = Uuid::parse_str(&req.tenant_id).map_err(|_| Status::invalid_argument("Invalid tenant_id"))?;
        let conversation_id = Uuid::parse_str(&req.conversation_id).map_err(|_| Status::invalid_argument("Invalid conversation_id"))?;
        let sender_id = Uuid::parse_str(&req.sender_id).ok();

        let msg = self.db_service.send_message(
            tenant_id,
            conversation_id,
            req.sender_type,
            sender_id,
            req.content.clone(),
        ).await.map_err(|e| Status::internal(e.to_string()))?;

        Ok(Response::new(SendMessageResponse {
            message: Some(chatpb::Message {
                id: msg.id.to_string(),
                tenant_id: msg.tenant_id.to_string(),
                conversation_id: msg.conversation_id.to_string(),
                sender_type: msg.sender_type.clone(),
                sender_id: msg.sender_id.map(|u| u.to_string()).unwrap_or_default(),
                content: msg.content.clone(),
                status: "sent".to_string(),
                draft_reply: "".to_string(),
                created_at: msg.created_at.to_rfc3339(),
                updated_at: msg.updated_at.to_rfc3339(),
            }),
        }))
    }

    async fn approve_draft(
        &self,
        _request: Request<ApproveDraftRequest>,
    ) -> Result<Response<ApproveDraftResponse>, Status> {
        Err(Status::unimplemented("approve_draft not implemented yet"))
    }
}
