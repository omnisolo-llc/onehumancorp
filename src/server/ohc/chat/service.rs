use tonic::{Request, Response, Status};
use uuid::Uuid;
use super::models::{
    chat_service_server::ChatService,
    CreateInboxRequest, CreateInboxResponse,
    CreateChannelRequest, CreateChannelResponse,
    CreateContactRequest, CreateContactResponse,
    StartConversationRequest, StartConversationResponse,
    SendMessageRequest, SendMessageResponse,
    Inbox, Channel, Contact, Conversation, Message,
};
use sqlx::PgPool;

pub struct ChatGrpcService {
    pool: PgPool,
}

impl ChatGrpcService {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[tonic::async_trait]
impl ChatService for ChatGrpcService {
    async fn create_inbox(
        &self,
        request: Request<CreateInboxRequest>,
    ) -> Result<Response<CreateInboxResponse>, Status> {
        let req = request.into_inner();
        let tenant_id = Uuid::parse_str(&req.tenant_id).map_err(|_| Status::invalid_argument("Invalid tenant_id"))?;

        let row = sqlx::query!(
            r#"
            INSERT INTO chat_inboxes (id, tenant_id, name)
            VALUES ($1, $2, $3)
            RETURNING id, tenant_id, name, created_at, updated_at
            "#,
            Uuid::new_v4(),
            tenant_id,
            req.name
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| Status::internal(format!("Database error: {}", e)))?;

        Ok(Response::new(CreateInboxResponse {
            inbox: Some(Inbox {
                id: row.id.to_string(),
                tenant_id: row.tenant_id.to_string(),
                name: row.name,
                created_at: row.created_at.unwrap_or_default().to_string(),
                updated_at: row.updated_at.unwrap_or_default().to_string(),
            }),
        }))
    }

    async fn create_channel(
        &self,
        request: Request<CreateChannelRequest>,
    ) -> Result<Response<CreateChannelResponse>, Status> {
        let req = request.into_inner();
        let tenant_id = Uuid::parse_str(&req.tenant_id).map_err(|_| Status::invalid_argument("Invalid tenant_id"))?;
        let inbox_id = Uuid::parse_str(&req.inbox_id).map_err(|_| Status::invalid_argument("Invalid inbox_id"))?;
        let config_json: serde_json::Value = serde_json::from_str(&req.config).unwrap_or(serde_json::json!({}));

        let row = sqlx::query!(
            r#"
            INSERT INTO chat_channels (id, tenant_id, inbox_id, channel_type, config)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING id, tenant_id, inbox_id, channel_type, config, created_at, updated_at
            "#,
            Uuid::new_v4(),
            tenant_id,
            inbox_id,
            req.channel_type,
            config_json
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| Status::internal(format!("Database error: {}", e)))?;

        Ok(Response::new(CreateChannelResponse {
            channel: Some(Channel {
                id: row.id.to_string(),
                tenant_id: row.tenant_id.to_string(),
                inbox_id: row.inbox_id.to_string(),
                channel_type: row.channel_type,
                config: row.config.unwrap_or_default().to_string(),
                created_at: row.created_at.unwrap_or_default().to_string(),
                updated_at: row.updated_at.unwrap_or_default().to_string(),
            }),
        }))
    }

    async fn create_contact(
        &self,
        request: Request<CreateContactRequest>,
    ) -> Result<Response<CreateContactResponse>, Status> {
        let req = request.into_inner();
        let tenant_id = Uuid::parse_str(&req.tenant_id).map_err(|_| Status::invalid_argument("Invalid tenant_id"))?;

        let row = sqlx::query!(
            r#"
            INSERT INTO chat_contacts (id, tenant_id, name, email, phone)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING id, tenant_id, name, email, phone, created_at, updated_at
            "#,
            Uuid::new_v4(),
            tenant_id,
            req.name,
            req.email,
            req.phone
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| Status::internal(format!("Database error: {}", e)))?;

        Ok(Response::new(CreateContactResponse {
            contact: Some(Contact {
                id: row.id.to_string(),
                tenant_id: row.tenant_id.to_string(),
                name: row.name.unwrap_or_default(),
                email: row.email.unwrap_or_default(),
                phone: row.phone.unwrap_or_default(),
                created_at: row.created_at.unwrap_or_default().to_string(),
                updated_at: row.updated_at.unwrap_or_default().to_string(),
            }),
        }))
    }

    async fn start_conversation(
        &self,
        request: Request<StartConversationRequest>,
    ) -> Result<Response<StartConversationResponse>, Status> {
        let req = request.into_inner();
        let tenant_id = Uuid::parse_str(&req.tenant_id).map_err(|_| Status::invalid_argument("Invalid tenant_id"))?;
        let inbox_id = Uuid::parse_str(&req.inbox_id).map_err(|_| Status::invalid_argument("Invalid inbox_id"))?;
        let contact_id = Uuid::parse_str(&req.contact_id).map_err(|_| Status::invalid_argument("Invalid contact_id"))?;
        let assignee_id = if req.assignee_id.is_empty() { None } else { Some(Uuid::parse_str(&req.assignee_id).map_err(|_| Status::invalid_argument("Invalid assignee_id"))?) };

        let row = sqlx::query!(
            r#"
            INSERT INTO chat_conversations (id, tenant_id, inbox_id, contact_id, assignee_id, status)
            VALUES ($1, $2, $3, $4, $5, 'open')
            RETURNING id, tenant_id, inbox_id, contact_id, assignee_id, status, created_at, updated_at
            "#,
            Uuid::new_v4(),
            tenant_id,
            inbox_id,
            contact_id,
            assignee_id
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| Status::internal(format!("Database error: {}", e)))?;

        Ok(Response::new(StartConversationResponse {
            conversation: Some(Conversation {
                id: row.id.to_string(),
                tenant_id: row.tenant_id.to_string(),
                inbox_id: row.inbox_id.to_string(),
                contact_id: row.contact_id.to_string(),
                assignee_id: row.assignee_id.unwrap_or_default().to_string(),
                status: row.status,
                created_at: row.created_at.unwrap_or_default().to_string(),
                updated_at: row.updated_at.unwrap_or_default().to_string(),
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
        let sender_id = if req.sender_id.is_empty() { None } else { Some(Uuid::parse_str(&req.sender_id).map_err(|_| Status::invalid_argument("Invalid sender_id"))?) };

        let row = sqlx::query!(
            r#"
            INSERT INTO chat_messages (id, tenant_id, conversation_id, sender_type, sender_id, content)
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING id, tenant_id, conversation_id, sender_type, sender_id, content, created_at, updated_at
            "#,
            Uuid::new_v4(),
            tenant_id,
            conversation_id,
            req.sender_type,
            sender_id,
            req.content
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| Status::internal(format!("Database error: {}", e)))?;

        Ok(Response::new(SendMessageResponse {
            message: Some(Message {
                id: row.id.to_string(),
                tenant_id: row.tenant_id.to_string(),
                conversation_id: row.conversation_id.to_string(),
                sender_type: row.sender_type,
                sender_id: row.sender_id.unwrap_or_default().to_string(),
                content: row.content,
                created_at: row.created_at.unwrap_or_default().to_string(),
                updated_at: row.updated_at.unwrap_or_default().to_string(),
            }),
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::PgPool;
    use tonic::Request;
    use crate::ohc::chat::chat_service_server::ChatService;
    use crate::ohc::chat::{CreateInboxRequest, CreateChannelRequest, CreateContactRequest, StartConversationRequest, SendMessageRequest};

    #[tokio::test]
    async fn test_chat_grpc_service() {
        let database_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| "postgres://postgres:postgres@localhost:5432/ohc".to_string());

        let maybe_pool = PgPool::connect(&database_url).await;
        if maybe_pool.is_err() {
            println!("Skipping db test");
            return;
        }
        let pool = maybe_pool.unwrap();

        // Ensure tables exist for test
        let _ = sqlx::query("
            CREATE TABLE IF NOT EXISTS chat_inboxes (
                id UUID PRIMARY KEY, tenant_id UUID NOT NULL, name TEXT NOT NULL, created_at TIMESTAMPTZ DEFAULT NOW(), updated_at TIMESTAMPTZ DEFAULT NOW()
            );
            CREATE TABLE IF NOT EXISTS chat_channels (
                id UUID PRIMARY KEY, tenant_id UUID NOT NULL, inbox_id UUID NOT NULL, channel_type TEXT NOT NULL, config JSONB DEFAULT '{}'::jsonb, created_at TIMESTAMPTZ DEFAULT NOW(), updated_at TIMESTAMPTZ DEFAULT NOW()
            );
            CREATE TABLE IF NOT EXISTS chat_contacts (
                id UUID PRIMARY KEY, tenant_id UUID NOT NULL, name TEXT, email TEXT, phone TEXT, created_at TIMESTAMPTZ DEFAULT NOW(), updated_at TIMESTAMPTZ DEFAULT NOW()
            );
            CREATE TABLE IF NOT EXISTS chat_conversations (
                id UUID PRIMARY KEY, tenant_id UUID NOT NULL, inbox_id UUID NOT NULL, contact_id UUID NOT NULL, assignee_id UUID, status TEXT NOT NULL DEFAULT 'open', created_at TIMESTAMPTZ DEFAULT NOW(), updated_at TIMESTAMPTZ DEFAULT NOW()
            );
            CREATE TABLE IF NOT EXISTS chat_messages (
                id UUID PRIMARY KEY, tenant_id UUID NOT NULL, conversation_id UUID NOT NULL, sender_type TEXT NOT NULL, sender_id UUID, content TEXT NOT NULL, created_at TIMESTAMPTZ DEFAULT NOW(), updated_at TIMESTAMPTZ DEFAULT NOW()
            );
        ").execute(&pool).await;

        let service = ChatGrpcService::new(pool.clone());
        let tenant_id = Uuid::new_v4().to_string();

        let req = Request::new(CreateInboxRequest {
            tenant_id: tenant_id.clone(),
            name: "Test Inbox".to_string(),
        });

        let res = service.create_inbox(req).await.unwrap().into_inner();
        let inbox = res.inbox.unwrap();
        assert_eq!(inbox.name, "Test Inbox");

        let req2 = Request::new(CreateChannelRequest {
            tenant_id: tenant_id.clone(),
            inbox_id: inbox.id.clone(),
            channel_type: "widget".to_string(),
            config: "{}".to_string(),
        });
        let res2 = service.create_channel(req2).await.unwrap().into_inner();
        let channel = res2.channel.unwrap();
        assert_eq!(channel.channel_type, "widget");

        let req3 = Request::new(CreateContactRequest {
            tenant_id: tenant_id.clone(),
            name: "Test Contact".to_string(),
            email: "test@example.com".to_string(),
            phone: "".to_string(),
        });
        let res3 = service.create_contact(req3).await.unwrap().into_inner();
        let contact = res3.contact.unwrap();
        assert_eq!(contact.name, "Test Contact");

        let req4 = Request::new(StartConversationRequest {
            tenant_id: tenant_id.clone(),
            inbox_id: inbox.id.clone(),
            contact_id: contact.id.clone(),
            assignee_id: "".to_string(),
        });
        let res4 = service.start_conversation(req4).await.unwrap().into_inner();
        let conversation = res4.conversation.unwrap();
        assert_eq!(conversation.status, "open");

        let req5 = Request::new(SendMessageRequest {
            tenant_id: tenant_id.clone(),
            conversation_id: conversation.id.clone(),
            sender_type: "contact".to_string(),
            sender_id: contact.id.clone(),
            content: "Hello from contact".to_string(),
        });
        let res5 = service.send_message(req5).await.unwrap().into_inner();
        let message = res5.message.unwrap();
        assert_eq!(message.content, "Hello from contact");
    }
}
