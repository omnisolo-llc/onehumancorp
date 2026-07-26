use tonic::{Request, Response, Status};
use uuid::Uuid;
use sqlx::PgPool;

use crate::ohc::chat::chat_service_server::ChatService;
use crate::ohc::chat::{
    CreateInboxRequest, CreateInboxResponse, SendMessageRequest, SendMessageResponse,
    StartConversationRequest, StartConversationResponse, Inbox as ProtoInbox, Conversation as ProtoConversation, Message as ProtoMessage
};

use super::domain::repository::ChatRepository;

pub struct ChatServiceImpl {
    repository: ChatRepository,
}

impl ChatServiceImpl {
    pub fn new(pool: PgPool) -> Self {
        Self {
            repository: ChatRepository::new(pool),
        }
    }
}

#[tonic::async_trait]
impl ChatService for ChatServiceImpl {
    async fn create_inbox(
        &self,
        request: Request<CreateInboxRequest>,
    ) -> Result<Response<CreateInboxResponse>, Status> {
        let req = request.into_inner();
        let tenant_id = Uuid::parse_str(&req.tenant_id)
            .map_err(|_| Status::invalid_argument("Invalid tenant_id"))?;

        let inbox = self
            .repository
            .create_inbox(tenant_id, req.name)
            .await
            .map_err(|e| Status::internal(e.to_string()))?;

        Ok(Response::new(CreateInboxResponse {
            inbox: Some(ProtoInbox {
                id: inbox.id.to_string(),
                tenant_id: inbox.tenant_id.to_string(),
                name: inbox.name,
            }),
        }))
    }

    async fn start_conversation(
        &self,
        request: Request<StartConversationRequest>,
    ) -> Result<Response<StartConversationResponse>, Status> {
        let req = request.into_inner();
        let tenant_id = Uuid::parse_str(&req.tenant_id)
            .map_err(|_| Status::invalid_argument("Invalid tenant_id"))?;
        let inbox_id = Uuid::parse_str(&req.inbox_id)
            .map_err(|_| Status::invalid_argument("Invalid inbox_id"))?;
        let contact_id = Uuid::parse_str(&req.contact_id)
            .map_err(|_| Status::invalid_argument("Invalid contact_id"))?;

        let conversation = self
            .repository
            .create_conversation(tenant_id, inbox_id, contact_id)
            .await
            .map_err(|e| Status::internal(e.to_string()))?;

        Ok(Response::new(StartConversationResponse {
            conversation: Some(ProtoConversation {
                id: conversation.id.to_string(),
                tenant_id: conversation.tenant_id.to_string(),
                inbox_id: conversation.inbox_id.to_string(),
                contact_id: conversation.contact_id.to_string(),
                status: conversation.status,
            }),
        }))
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

        let sender_id = if req.sender_id.is_empty() {
            None
        } else {
            Some(Uuid::parse_str(&req.sender_id)
                .map_err(|_| Status::invalid_argument("Invalid sender_id"))?)
        };

        let message = self
            .repository
            .create_message(
                tenant_id,
                conversation_id,
                req.sender_type,
                sender_id,
                req.content,
            )
            .await
            .map_err(|e| Status::internal(e.to_string()))?;

        Ok(Response::new(SendMessageResponse {
            message: Some(ProtoMessage {
                id: message.id.to_string(),
                tenant_id: message.tenant_id.to_string(),
                conversation_id: message.conversation_id.to_string(),
                sender_type: message.sender_type,
                sender_id: message.sender_id.map(|id| id.to_string()).unwrap_or_default(),
                content: message.content,
            }),
        }))
    }
}







#[cfg(test)]
mod tests {
    use super::*;
    use crate::ohc::chat::{CreateInboxRequest, StartConversationRequest, SendMessageRequest};

    #[tokio::test]
    async fn test_create_inbox_invalid_tenant_id() {
        let pool = sqlx::PgPool::connect_lazy("postgres://dummy").unwrap();
        let service = ChatServiceImpl::new(pool);

        let request = Request::new(CreateInboxRequest {
            tenant_id: "invalid-uuid".to_string(),
            name: "My Inbox".to_string(),
        });

        let result = service.create_inbox(request).await;
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().code(), tonic::Code::InvalidArgument);
    }

    #[tokio::test]
    async fn test_start_conversation_invalid_tenant_id() {
        let pool = sqlx::PgPool::connect_lazy("postgres://dummy").unwrap();
        let service = ChatServiceImpl::new(pool);

        let request = Request::new(StartConversationRequest {
            tenant_id: "invalid-uuid".to_string(),
            inbox_id: Uuid::new_v4().to_string(),
            contact_id: Uuid::new_v4().to_string(),
        });

        let result = service.start_conversation(request).await;
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().code(), tonic::Code::InvalidArgument);
    }

    #[tokio::test]
    async fn test_start_conversation_invalid_inbox_id() {
        let pool = sqlx::PgPool::connect_lazy("postgres://dummy").unwrap();
        let service = ChatServiceImpl::new(pool);

        let request = Request::new(StartConversationRequest {
            tenant_id: Uuid::new_v4().to_string(),
            inbox_id: "invalid-uuid".to_string(),
            contact_id: Uuid::new_v4().to_string(),
        });

        let result = service.start_conversation(request).await;
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().code(), tonic::Code::InvalidArgument);
    }

    #[tokio::test]
    async fn test_send_message_invalid_tenant_id() {
        let pool = sqlx::PgPool::connect_lazy("postgres://dummy").unwrap();
        let service = ChatServiceImpl::new(pool);

        let request = Request::new(SendMessageRequest {
            tenant_id: "invalid-uuid".to_string(),
            conversation_id: Uuid::new_v4().to_string(),
            sender_type: "agent".to_string(),
            sender_id: "".to_string(),
            content: "Hello".to_string(),
        });

        let result = service.send_message(request).await;
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().code(), tonic::Code::InvalidArgument);
    }
}
