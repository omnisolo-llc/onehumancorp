use tonic::{Request, Response, Status};
use uuid::Uuid;
use crate::ohc::inbox::{
    FetchConversationsRequest, FetchConversationsResponse,
    FetchMessagesRequest, FetchMessagesResponse,
    CreateMessageRequest, CreateMessageResponse,
    ApproveDraftRequest, ApproveDraftResponse,
    inbox_service_server::InboxService,
};
use crate::domain::inbox::dao::InboxDao;

pub struct OHCInboxService {
    pub inbox_dao: InboxDao,
}

#[tonic::async_trait]
impl InboxService for OHCInboxService {
    async fn fetch_conversations(
        &self,
        request: Request<FetchConversationsRequest>,
    ) -> Result<Response<FetchConversationsResponse>, Status> {
        let req = request.into_inner();
        let tenant_id = match Uuid::parse_str(&req.tenant_id) {
            Ok(id) => id,
            Err(_) => return Err(Status::invalid_argument("Invalid tenant_id format")),
        };

        let conversations = self.inbox_dao.fetch_conversations(tenant_id).await.map_err(|e| Status::internal(e.to_string()))?;
        Ok(Response::new(FetchConversationsResponse { conversations }))
    }

    async fn fetch_messages(
        &self,
        request: Request<FetchMessagesRequest>,
    ) -> Result<Response<FetchMessagesResponse>, Status> {
        let req = request.into_inner();
        let tenant_id = match Uuid::parse_str(&req.tenant_id) {
            Ok(id) => id,
            Err(_) => return Err(Status::invalid_argument("Invalid tenant_id format")),
        };
        let conversation_id = match Uuid::parse_str(&req.conversation_id) {
            Ok(id) => id,
            Err(_) => return Err(Status::invalid_argument("Invalid conversation_id format")),
        };

        let messages = self.inbox_dao.fetch_messages(tenant_id, conversation_id).await.map_err(|e| Status::internal(e.to_string()))?;
        Ok(Response::new(FetchMessagesResponse { messages }))
    }

    async fn create_message(
        &self,
        request: Request<CreateMessageRequest>,
    ) -> Result<Response<CreateMessageResponse>, Status> {
        let req = request.into_inner();
        let tenant_id = match Uuid::parse_str(&req.tenant_id) {
            Ok(id) => id,
            Err(_) => return Err(Status::invalid_argument("Invalid tenant_id format")),
        };

        let message = self.inbox_dao.create_message(tenant_id, req).await.map_err(|e| Status::internal(e.to_string()))?;

        // TODO: Broadcast to WebSocket

        Ok(Response::new(CreateMessageResponse { message: Some(message) }))
    }

    async fn approve_draft(
        &self,
        request: Request<ApproveDraftRequest>,
    ) -> Result<Response<ApproveDraftResponse>, Status> {
        let req = request.into_inner();
        let tenant_id = match Uuid::parse_str(&req.tenant_id) {
            Ok(id) => id,
            Err(_) => return Err(Status::invalid_argument("Invalid tenant_id format")),
        };
        let message_id = match Uuid::parse_str(&req.message_id) {
            Ok(id) => id,
            Err(_) => return Err(Status::invalid_argument("Invalid message_id format")),
        };

        let message = self.inbox_dao.approve_draft(tenant_id, message_id).await.map_err(|e| Status::internal(e.to_string()))?;

        // TODO: Process sending the message and broadcast to WebSocket

        Ok(Response::new(ApproveDraftResponse { message: Some(message) }))
    }
}
