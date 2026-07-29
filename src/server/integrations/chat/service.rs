use sqlx::PgPool;
use tonic::{Request, Response, Status};
use uuid::Uuid;

use crate::pb::chat_service_server::ChatService;
use crate::pb::{
    CreateContactRequest, CreateContactResponse, CreateConversationRequest,
    CreateConversationResponse, CreateInboxRequest, CreateInboxResponse, CreateMessageRequest,
    CreateMessageResponse, ListConversationsRequest, ListConversationsResponse,
    ListInboxesRequest, ListInboxesResponse, ListMessagesRequest, ListMessagesResponse, Contact, Conversation, Inbox, Message
};

pub struct ChatServiceImpl {
    pool: PgPool,
}

impl ChatServiceImpl {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[tonic::async_trait]
impl ChatService for ChatServiceImpl {
    async fn create_inbox(
        &self,
        request: Request<CreateInboxRequest>,
    ) -> Result<Response<CreateInboxResponse>, Status> {
        let auth_info = request.extensions().get::<::server_auth::orchestration::AuthInfo>().cloned();
        let tenant_id_str = match auth_info {
            Some(i) => i.org_id,
            None => return Err(Status::unauthenticated("Missing authentication context")),
        };
        let tenant_id = Uuid::parse_str(&tenant_id_str)
            .map_err(|_| Status::unauthenticated("Invalid tenant_id format in token"))?;

        let req = request.into_inner();
        let inbox = req.inbox.ok_or_else(|| Status::invalid_argument("Missing inbox"))?;
        let id = Uuid::new_v4();

        let mut tx = self.pool.begin().await.map_err(|e| Status::internal(e.to_string()))?;
        sqlx::query("SELECT set_config('app.current_tenant_id', $1, true)").bind(tenant_id.to_string()).execute(&mut *tx).await.map_err(|e| Status::internal(e.to_string()))?;
        let row = sqlx::query(
            "INSERT INTO chat_inbox (id, tenant_id, name, channel_type) VALUES ($1, $2, $3, $4) RETURNING id, tenant_id, name, channel_type")
            .bind(id)
            .bind(tenant_id)
            .bind(&inbox.name)
            .bind(&inbox.channel_type)
        .fetch_one(&mut *tx)
        .await
        .map_err(|e| Status::internal(e.to_string()))?;
        tx.commit().await.map_err(|e| Status::internal(e.to_string()))?;

        use sqlx::Row;
        let id: Uuid = row.get("id");
        let t_id: Uuid = row.get("tenant_id");
        let name: String = row.get("name");
        let channel_type: String = row.get("channel_type");
        Ok(Response::new(CreateInboxResponse {
            inbox: Some(Inbox {
                id: id.to_string(),
                tenant_id: t_id.to_string(),
                name,
                channel_type,
            }),
        }))
    }

    async fn list_inboxes(
        &self,
        request: Request<ListInboxesRequest>,
    ) -> Result<Response<ListInboxesResponse>, Status> {
        let auth_info = request.extensions().get::<::server_auth::orchestration::AuthInfo>().cloned();
        let tenant_id_str = match auth_info {
            Some(i) => i.org_id,
            None => return Err(Status::unauthenticated("Missing authentication context")),
        };
        let tenant_id = Uuid::parse_str(&tenant_id_str)
            .map_err(|_| Status::unauthenticated("Invalid tenant_id format in token"))?;


        let mut tx = self.pool.begin().await.map_err(|e| Status::internal(e.to_string()))?;
        sqlx::query("SELECT set_config('app.current_tenant_id', $1, true)").bind(tenant_id.to_string()).execute(&mut *tx).await.map_err(|e| Status::internal(e.to_string()))?;
        let rows = sqlx::query(
            "SELECT id, tenant_id, name, channel_type FROM chat_inbox WHERE tenant_id = $1")
            .bind(tenant_id)
        .fetch_all(&mut *tx)
        .await
        .map_err(|e| Status::internal(e.to_string()))?;
        tx.commit().await.map_err(|e| Status::internal(e.to_string()))?;

        use sqlx::Row;
        let inboxes = rows.into_iter().map(|row| {
            let id: Uuid = row.get("id");
            let t_id: Uuid = row.get("tenant_id");
            let name: String = row.get("name");
            let channel_type: String = row.get("channel_type");
            Inbox {
            id: id.to_string(),
            tenant_id: t_id.to_string(),
            name,
            channel_type,
        }}).collect();

        Ok(Response::new(ListInboxesResponse { inboxes }))
    }

    async fn create_contact(
        &self,
        request: Request<CreateContactRequest>,
    ) -> Result<Response<CreateContactResponse>, Status> {
        let auth_info = request.extensions().get::<::server_auth::orchestration::AuthInfo>().cloned();
        let tenant_id_str = match auth_info {
            Some(i) => i.org_id,
            None => return Err(Status::unauthenticated("Missing authentication context")),
        };
        let tenant_id = Uuid::parse_str(&tenant_id_str)
            .map_err(|_| Status::unauthenticated("Invalid tenant_id format in token"))?;

        let req = request.into_inner();
        let contact = req.contact.ok_or_else(|| Status::invalid_argument("Missing contact"))?;
        let id = Uuid::new_v4();

        let mut tx = self.pool.begin().await.map_err(|e| Status::internal(e.to_string()))?;
        sqlx::query("SELECT set_config('app.current_tenant_id', $1, true)").bind(tenant_id.to_string()).execute(&mut *tx).await.map_err(|e| Status::internal(e.to_string()))?;
        let row = sqlx::query(
            "INSERT INTO chat_contact (id, tenant_id, name, email, phone) VALUES ($1, $2, $3, $4, $5) RETURNING id, tenant_id, name, email, phone")
            .bind(id)
            .bind(tenant_id)
            .bind(&contact.name)
            .bind(&contact.email)
            .bind(&contact.phone)
        .fetch_one(&mut *tx)
        .await
        .map_err(|e| Status::internal(e.to_string()))?;
        tx.commit().await.map_err(|e| Status::internal(e.to_string()))?;

        use sqlx::Row;
        let id: Uuid = row.get("id");
        let t_id: Uuid = row.get("tenant_id");
        let name: Option<String> = row.get("name");
        let email: Option<String> = row.get("email");
        let phone: Option<String> = row.get("phone");
        Ok(Response::new(CreateContactResponse {
            contact: Some(Contact {
                id: id.to_string(),
                tenant_id: t_id.to_string(),
                name: name.unwrap_or_default(),
                email: email.unwrap_or_default(),
                phone: phone.unwrap_or_default(),
            }),
        }))
    }

    async fn create_conversation(
        &self,
        request: Request<CreateConversationRequest>,
    ) -> Result<Response<CreateConversationResponse>, Status> {
        let auth_info = request.extensions().get::<::server_auth::orchestration::AuthInfo>().cloned();
        let tenant_id_str = match auth_info {
            Some(i) => i.org_id,
            None => return Err(Status::unauthenticated("Missing authentication context")),
        };
        let tenant_id = Uuid::parse_str(&tenant_id_str)
            .map_err(|_| Status::unauthenticated("Invalid tenant_id format in token"))?;

        let req = request.into_inner();
        let conv = req.conversation.ok_or_else(|| Status::invalid_argument("Missing conversation"))?;
        let id = Uuid::new_v4();
        let inbox_id = Uuid::parse_str(&conv.inbox_id)
            .map_err(|_| Status::invalid_argument("Invalid inbox_id"))?;
        let contact_id = Uuid::parse_str(&conv.contact_id)
            .map_err(|_| Status::invalid_argument("Invalid contact_id"))?;

        let mut tx = self.pool.begin().await.map_err(|e| Status::internal(e.to_string()))?;
        sqlx::query("SELECT set_config('app.current_tenant_id', $1, true)").bind(tenant_id.to_string()).execute(&mut *tx).await.map_err(|e| Status::internal(e.to_string()))?;
        let row = sqlx::query(
            "INSERT INTO chat_conversation (id, tenant_id, inbox_id, contact_id, status) VALUES ($1, $2, $3, $4, $5) RETURNING id, tenant_id, inbox_id, contact_id, status")
            .bind(id)
            .bind(tenant_id)
            .bind(inbox_id)
            .bind(contact_id)
            .bind(&conv.status)
        .fetch_one(&mut *tx)
        .await
        .map_err(|e| Status::internal(e.to_string()))?;
        tx.commit().await.map_err(|e| Status::internal(e.to_string()))?;

        use sqlx::Row;
        let id: Uuid = row.get("id");
        let t_id: Uuid = row.get("tenant_id");
        let inbox_id: Uuid = row.get("inbox_id");
        let contact_id: Uuid = row.get("contact_id");
        let status: String = row.get("status");
        Ok(Response::new(CreateConversationResponse {
            conversation: Some(Conversation {
                id: id.to_string(),
                tenant_id: t_id.to_string(),
                inbox_id: inbox_id.to_string(),
                contact_id: contact_id.to_string(),
                status,
            }),
        }))
    }

    async fn list_conversations(
        &self,
        request: Request<ListConversationsRequest>,
    ) -> Result<Response<ListConversationsResponse>, Status> {
        let auth_info = request.extensions().get::<::server_auth::orchestration::AuthInfo>().cloned();
        let tenant_id_str = match auth_info {
            Some(i) => i.org_id,
            None => return Err(Status::unauthenticated("Missing authentication context")),
        };
        let tenant_id = Uuid::parse_str(&tenant_id_str)
            .map_err(|_| Status::unauthenticated("Invalid tenant_id format in token"))?;

        let req = request.into_inner();
        let inbox_id = Uuid::parse_str(&req.inbox_id)
            .map_err(|_| Status::invalid_argument("Invalid inbox_id"))?;

        let mut tx = self.pool.begin().await.map_err(|e| Status::internal(e.to_string()))?;
        sqlx::query("SELECT set_config('app.current_tenant_id', $1, true)").bind(tenant_id.to_string()).execute(&mut *tx).await.map_err(|e| Status::internal(e.to_string()))?;
        let rows = sqlx::query(
            "SELECT id, tenant_id, inbox_id, contact_id, status FROM chat_conversation WHERE tenant_id = $1 AND inbox_id = $2")
            .bind(tenant_id)
            .bind(inbox_id)
        .fetch_all(&mut *tx)
        .await
        .map_err(|e| Status::internal(e.to_string()))?;
        tx.commit().await.map_err(|e| Status::internal(e.to_string()))?;

        use sqlx::Row;
        let conversations = rows.into_iter().map(|row| {
            let id: Uuid = row.get("id");
            let t_id: Uuid = row.get("tenant_id");
            let inbox_id: Uuid = row.get("inbox_id");
            let contact_id: Uuid = row.get("contact_id");
            let status: String = row.get("status");
            Conversation {
                id: id.to_string(),
                tenant_id: t_id.to_string(),
                inbox_id: inbox_id.to_string(),
                contact_id: contact_id.to_string(),
                status,
        }}).collect();

        Ok(Response::new(ListConversationsResponse { conversations }))
    }

    async fn create_message(
        &self,
        request: Request<CreateMessageRequest>,
    ) -> Result<Response<CreateMessageResponse>, Status> {
        let auth_info = request.extensions().get::<::server_auth::orchestration::AuthInfo>().cloned();
        let tenant_id_str = match auth_info {
            Some(i) => i.org_id,
            None => return Err(Status::unauthenticated("Missing authentication context")),
        };
        let tenant_id = Uuid::parse_str(&tenant_id_str)
            .map_err(|_| Status::unauthenticated("Invalid tenant_id format in token"))?;

        let req = request.into_inner();
        let msg = req.message.ok_or_else(|| Status::invalid_argument("Missing message"))?;
        let id = Uuid::new_v4();
        let conversation_id = Uuid::parse_str(&msg.conversation_id)
            .map_err(|_| Status::invalid_argument("Invalid conversation_id"))?;

        let sender_id = if msg.sender_id.is_empty() {
            None
        } else {
            Some(Uuid::parse_str(&msg.sender_id)
                .map_err(|_| Status::invalid_argument("Invalid sender_id"))?)
        };

        let mut tx = self.pool.begin().await.map_err(|e| Status::internal(e.to_string()))?;
        sqlx::query("SELECT set_config('app.current_tenant_id', $1, true)").bind(tenant_id.to_string()).execute(&mut *tx).await.map_err(|e| Status::internal(e.to_string()))?;
        let row = sqlx::query(
            "INSERT INTO chat_message (id, tenant_id, conversation_id, content, sender_type, sender_id) VALUES ($1, $2, $3, $4, $5, $6) RETURNING id, tenant_id, conversation_id, content, sender_type, sender_id")
            .bind(id)
            .bind(tenant_id)
            .bind(conversation_id)
            .bind(&msg.content)
            .bind(&msg.sender_type)
            .bind(sender_id)
        .fetch_one(&mut *tx)
        .await
        .map_err(|e| Status::internal(e.to_string()))?;
        tx.commit().await.map_err(|e| Status::internal(e.to_string()))?;

        use sqlx::Row;
        let id: Uuid = row.get("id");
        let t_id: Uuid = row.get("tenant_id");
        let conv_id: Uuid = row.get("conversation_id");
        let content: String = row.get("content");
        let sender_type: String = row.get("sender_type");
        let sender_id: Option<Uuid> = row.get("sender_id");
        Ok(Response::new(CreateMessageResponse {
            message: Some(Message {
                id: id.to_string(),
                tenant_id: t_id.to_string(),
                conversation_id: conv_id.to_string(),
                content,
                sender_type,
                sender_id: sender_id.map(|u| u.to_string()).unwrap_or_default(),
            }),
        }))
    }

    async fn list_messages(
        &self,
        request: Request<ListMessagesRequest>,
    ) -> Result<Response<ListMessagesResponse>, Status> {
        let auth_info = request.extensions().get::<::server_auth::orchestration::AuthInfo>().cloned();
        let tenant_id_str = match auth_info {
            Some(i) => i.org_id,
            None => return Err(Status::unauthenticated("Missing authentication context")),
        };
        let tenant_id = Uuid::parse_str(&tenant_id_str)
            .map_err(|_| Status::unauthenticated("Invalid tenant_id format in token"))?;

        let req = request.into_inner();
        let conversation_id = Uuid::parse_str(&req.conversation_id)
            .map_err(|_| Status::invalid_argument("Invalid conversation_id"))?;

        let mut tx = self.pool.begin().await.map_err(|e| Status::internal(e.to_string()))?;
        sqlx::query("SELECT set_config('app.current_tenant_id', $1, true)").bind(tenant_id.to_string()).execute(&mut *tx).await.map_err(|e| Status::internal(e.to_string()))?;
        let rows = sqlx::query(
            "SELECT id, tenant_id, conversation_id, content, sender_type, sender_id FROM chat_message WHERE tenant_id = $1 AND conversation_id = $2 ORDER BY created_at ASC")
            .bind(tenant_id)
            .bind(conversation_id)
        .fetch_all(&mut *tx)
        .await
        .map_err(|e| Status::internal(e.to_string()))?;
        tx.commit().await.map_err(|e| Status::internal(e.to_string()))?;

        use sqlx::Row;
        let messages = rows.into_iter().map(|row| {
            let id: Uuid = row.get("id");
            let t_id: Uuid = row.get("tenant_id");
            let conv_id: Uuid = row.get("conversation_id");
            let content: String = row.get("content");
            let sender_type: String = row.get("sender_type");
            let sender_id: Option<Uuid> = row.get("sender_id");
            Message {
            id: id.to_string(),
            tenant_id: t_id.to_string(),
            conversation_id: conv_id.to_string(),
            content,
            sender_type,
            sender_id: sender_id.map(|u| u.to_string()).unwrap_or_default(),
        }}).collect();

        Ok(Response::new(ListMessagesResponse { messages }))
    }
}
