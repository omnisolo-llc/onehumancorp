use tonic::{Request, Response, Status};
use std::sync::Arc;
use crate::domain::repository::omnichannel_chat_repo::OmnichannelChatRepo;
use ::server_ohc::omnichannel::omnichannel_service_server::OmnichannelService;
use ::server_ohc::omnichannel::{
    CreateInboxRequest, GetInboxRequest, CreateConversationRequest, GetConversationRequest,
    CreateMessageRequest, GetMessageRequest, Inbox, Conversation, Message
};

pub struct OmnichannelChatService {
    repo: Arc<OmnichannelChatRepo>,
}

impl OmnichannelChatService {
    pub fn new(repo: Arc<OmnichannelChatRepo>) -> Self {
        Self { repo }
    }
}

#[tonic::async_trait]
impl OmnichannelService for OmnichannelChatService {
    async fn create_inbox(
        &self,
        request: Request<CreateInboxRequest>,
    ) -> Result<Response<Inbox>, Status> {
        let req = request.into_inner();
        match self.repo.create_inbox(req.tenant_id.clone(), req.name.clone()).await {
            Ok(inbox) => Ok(Response::new(Inbox {
                id: inbox.id,
                tenant_id: inbox.tenant_id,
                name: inbox.name,
            })),
            Err(e) => Err(Status::internal(e.to_string())),
        }
    }

    async fn get_inbox(
        &self,
        request: Request<GetInboxRequest>,
    ) -> Result<Response<Inbox>, Status> {
        let req = request.into_inner();
        match self.repo.get_inbox(&req.id).await {
            Ok(inbox) => Ok(Response::new(Inbox {
                id: inbox.id,
                tenant_id: inbox.tenant_id,
                name: inbox.name,
            })),
            Err(e) => Err(Status::not_found(e.to_string())),
        }
    }

    async fn create_conversation(
        &self,
        request: Request<CreateConversationRequest>,
    ) -> Result<Response<Conversation>, Status> {
        let req = request.into_inner();
        match self.repo.create_conversation(req.inbox_id.clone(), req.contact_id.clone(), req.status.clone(), "".to_string()).await {
            Ok(conv) => Ok(Response::new(Conversation {
                id: conv.id,
                inbox_id: conv.inbox_id,
                contact_id: conv.contact_id,
                status: conv.status,
                tenant_id: conv.tenant_id,
            })),
            Err(e) => Err(Status::internal(e.to_string())),
        }
    }

    async fn get_conversation(
        &self,
        request: Request<GetConversationRequest>,
    ) -> Result<Response<Conversation>, Status> {
        let req = request.into_inner();
        match self.repo.get_conversation(&req.id).await {
            Ok(conv) => Ok(Response::new(Conversation {
                id: conv.id,
                inbox_id: conv.inbox_id,
                contact_id: conv.contact_id,
                status: conv.status,
                tenant_id: conv.tenant_id,
            })),
            Err(e) => Err(Status::not_found(e.to_string())),
        }
    }

    async fn create_message(
        &self,
        request: Request<CreateMessageRequest>,
    ) -> Result<Response<Message>, Status> {
        let req = request.into_inner();
        match self.repo.create_message(req.conversation_id.clone(), req.content.clone(), req.status.clone(), "".to_string()).await {
            Ok(msg) => Ok(Response::new(Message {
                id: msg.id,
                conversation_id: msg.conversation_id,
                content: msg.content,
                status: msg.status,
                tenant_id: msg.tenant_id,
            })),
            Err(e) => Err(Status::internal(e.to_string())),
        }
    }

    async fn get_message(
        &self,
        request: Request<GetMessageRequest>,
    ) -> Result<Response<Message>, Status> {
        let req = request.into_inner();
        match self.repo.get_message(&req.id).await {
            Ok(msg) => Ok(Response::new(Message {
                id: msg.id,
                conversation_id: msg.conversation_id,
                content: msg.content,
                status: msg.status,
                tenant_id: msg.tenant_id,
            })),
            Err(e) => Err(Status::not_found(e.to_string())),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::PgPool;

    #[tokio::test]
    async fn test_omnichannel_chat_service() {
        let database_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| "postgres://postgres:postgres@localhost:5432/ohc".to_string());
        let maybe_pool = PgPool::connect(&database_url).await;
        if maybe_pool.is_err() {
            return;
        }
        let pool = maybe_pool.unwrap();
        let db = Arc::new(crate::db::DB { pool: pool.clone(), store: crate::db::DbStore::Postgres });

        let _ = sqlx::query(
            "CREATE TABLE IF NOT EXISTS omnichannel_tenants (id TEXT PRIMARY KEY, name TEXT NOT NULL, created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP, updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP);"
        ).execute(&db.pool).await;
        let _ = sqlx::query(
            "CREATE TABLE IF NOT EXISTS omnichannel_inboxes (id TEXT PRIMARY KEY, tenant_id TEXT NOT NULL, name TEXT NOT NULL, created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP, updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP);"
        ).execute(&db.pool).await;

        let repo = Arc::new(OmnichannelChatRepo::new(db.clone()));
        let service = OmnichannelChatService::new(repo);

        let tenant_id = "test_tenant_id".to_string();
        let name = "test_inbox_name".to_string();

        let _ = sqlx::query("INSERT INTO omnichannel_tenants (id, name) VALUES ($1, $2) ON CONFLICT DO NOTHING")
            .bind(&tenant_id).bind("Test Tenant").execute(&db.pool).await;

        let mut req = Request::new(CreateInboxRequest {
            tenant_id: tenant_id.clone(),
            name: name.clone(),
        });
        req.metadata_mut().insert("tenant_id", tenant_id.parse().unwrap());

        let resp = service.create_inbox(req).await.unwrap();
        assert_eq!(resp.get_ref().name, name);
        assert_eq!(resp.get_ref().tenant_id, tenant_id);
    }
}
