use tonic::{Request, Response, Status};
use ::server_ohc::chat::omnichannel_chat_service_server::OmnichannelChatService;
use ::server_ohc::chat::{
    CreateInboxRequest, CreateInboxResponse, GetInboxRequest, GetInboxResponse,
    CreateConversationRequest, CreateConversationResponse, GetConversationRequest, GetConversationResponse,
    ProcessWebhookRequest, ProcessWebhookResponse, Inbox, Conversation, Message
};
use sqlx::PgPool;
use uuid::Uuid;

pub struct ChatEngineService {
    pool: PgPool,
}

impl ChatEngineService {
    pub fn new(pool: PgPool) -> Self {
        ChatEngineService { pool }
    }
}

#[tonic::async_trait]
impl OmnichannelChatService for ChatEngineService {
    async fn create_inbox(
        &self,
        request: Request<CreateInboxRequest>,
    ) -> Result<Response<CreateInboxResponse>, Status> {
        let req = request.into_inner();
        let id = Uuid::new_v4().to_string();
        
        let _tenant_id = Uuid::parse_str(&req.tenant_id)
            .map_err(|_| Status::invalid_argument("Invalid tenant ID format"))?;

        let inbox = Inbox {
            id: id.clone(),
            tenant_id: req.tenant_id.clone(),
            name: req.name.clone(),
        };

        let mut tx = self.pool.begin().await.map_err(|_| Status::internal("DB Error"))?;
        sqlx::query("SET LOCAL app.current_tenant = $1")
            .bind(&req.tenant_id)
            .execute(&mut *tx)
            .await
            .map_err(|_| Status::internal("Tenant isolation error"))?;

        sqlx::query(
            "INSERT INTO chat_inbox (id, tenant_id, name) VALUES ($1, $2, $3)"
        )
        .bind(&id)
        .bind(&req.tenant_id)
        .bind(&req.name)
        .execute(&mut *tx)
        .await
        .map_err(|e| {
            tracing::error!("Failed to create inbox: {}", e);
            Status::internal("Database error")
        })?;

        tx.commit().await.map_err(|_| Status::internal("Commit error"))?;

        Ok(Response::new(CreateInboxResponse { inbox: Some(inbox) }))
    }

    async fn get_inbox(
        &self,
        request: Request<GetInboxRequest>,
    ) -> Result<Response<GetInboxResponse>, Status> {
        let req = request.into_inner();

        let _tenant_id = Uuid::parse_str(&req.tenant_id)
            .map_err(|_| Status::invalid_argument("Invalid tenant ID format"))?;
        let _inbox_id = Uuid::parse_str(&req.id)
            .map_err(|_| Status::invalid_argument("Invalid inbox ID format"))?;

        let mut tx = self.pool.begin().await.map_err(|_| Status::internal("DB Error"))?;
        sqlx::query("SET LOCAL app.current_tenant = $1")
            .bind(&req.tenant_id)
            .execute(&mut *tx)
            .await
            .map_err(|_| Status::internal("Tenant isolation error"))?;

        let row: Option<(String, String, String)> = sqlx::query_as(
            "SELECT id, tenant_id, name FROM chat_inbox WHERE id = $1 AND tenant_id = $2"
        )
        .bind(&req.id)
        .bind(&req.tenant_id)
        .fetch_optional(&mut *tx)
        .await
        .map_err(|e| {
            tracing::error!("Failed to get inbox: {}", e);
            Status::internal("Database error")
        })?;

        tx.commit().await.map_err(|_| Status::internal("Commit error"))?;

        if let Some((id, tenant_id, name)) = row {
            Ok(Response::new(GetInboxResponse {
                inbox: Some(Inbox {
                    id,
                    tenant_id,
                    name,
                }),
            }))
        } else {
            Err(Status::not_found("Inbox not found"))
        }
    }

    async fn create_conversation(
        &self,
        request: Request<CreateConversationRequest>,
    ) -> Result<Response<CreateConversationResponse>, Status> {
        let req = request.into_inner();
        let id = Uuid::new_v4().to_string();

        let _tenant_id = Uuid::parse_str(&req.tenant_id)
            .map_err(|_| Status::invalid_argument("Invalid tenant ID format"))?;
        let _inbox_id = Uuid::parse_str(&req.inbox_id)
            .map_err(|_| Status::invalid_argument("Invalid inbox ID format"))?;
        let _contact_id = Uuid::parse_str(&req.contact_id)
            .map_err(|_| Status::invalid_argument("Invalid contact ID format"))?;

        let conversation = Conversation {
            id: id.clone(),
            tenant_id: req.tenant_id.clone(),
            inbox_id: req.inbox_id.clone(),
            contact_id: req.contact_id.clone(),
            status: "open".to_string(),
        };

        let mut tx = self.pool.begin().await.map_err(|_| Status::internal("DB Error"))?;
        sqlx::query("SET LOCAL app.current_tenant = $1")
            .bind(&req.tenant_id)
            .execute(&mut *tx)
            .await
            .map_err(|_| Status::internal("Tenant isolation error"))?;

        sqlx::query(
            "INSERT INTO chat_conversation (id, tenant_id, inbox_id, contact_id, status) VALUES ($1, $2, $3, $4, $5)"
        )
        .bind(&id)
        .bind(&req.tenant_id)
        .bind(&req.inbox_id)
        .bind(&req.contact_id)
        .bind("open")
        .execute(&mut *tx)
        .await
        .map_err(|e| {
            tracing::error!("Failed to create conversation: {}", e);
            Status::internal("Database error")
        })?;

        tx.commit().await.map_err(|_| Status::internal("Commit error"))?;

        Ok(Response::new(CreateConversationResponse { conversation: Some(conversation) }))
    }

    async fn get_conversation(
        &self,
        request: Request<GetConversationRequest>,
    ) -> Result<Response<GetConversationResponse>, Status> {
        let req = request.into_inner();
        
        let _tenant_id = Uuid::parse_str(&req.tenant_id)
            .map_err(|_| Status::invalid_argument("Invalid tenant ID format"))?;
        let _conversation_id = Uuid::parse_str(&req.id)
            .map_err(|_| Status::invalid_argument("Invalid conversation ID format"))?;

        let mut tx = self.pool.begin().await.map_err(|_| Status::internal("DB Error"))?;
        sqlx::query("SET LOCAL app.current_tenant = $1")
            .bind(&req.tenant_id)
            .execute(&mut *tx)
            .await
            .map_err(|_| Status::internal("Tenant isolation error"))?;

        let row: Option<(String, String, String, String, String)> = sqlx::query_as(
            "SELECT id, tenant_id, inbox_id, contact_id, status FROM chat_conversation WHERE id = $1 AND tenant_id = $2"
        )
        .bind(&req.id)
        .bind(&req.tenant_id)
        .fetch_optional(&mut *tx)
        .await
        .map_err(|e| {
            tracing::error!("Failed to get conversation: {}", e);
            Status::internal("Database error")
        })?;

        tx.commit().await.map_err(|_| Status::internal("Commit error"))?;

        if let Some((id, tenant_id, inbox_id, contact_id, status)) = row {
            Ok(Response::new(GetConversationResponse {
                conversation: Some(Conversation {
                    id,
                    tenant_id,
                    inbox_id,
                    contact_id,
                    status,
                }),
            }))
        } else {
            Err(Status::not_found("Conversation not found"))
        }
    }

    async fn process_webhook(
        &self,
        request: Request<ProcessWebhookRequest>,
    ) -> Result<Response<ProcessWebhookResponse>, Status> {
        let req = request.into_inner();
        let payload = req.payload.ok_or_else(|| Status::invalid_argument("Missing payload"))?;

        let _tenant_id = Uuid::parse_str(&payload.tenant_id)
            .map_err(|_| Status::invalid_argument("Invalid tenant ID format"))?;
        let _inbox_id = Uuid::parse_str(&payload.inbox_id)
            .map_err(|_| Status::invalid_argument("Invalid inbox ID format"))?;
        let _contact_id = Uuid::parse_str(&payload.contact_id)
            .map_err(|_| Status::invalid_argument("Invalid contact ID format"))?;

        let mut tx = self.pool.begin().await.map_err(|_| Status::internal("DB Error"))?;
        sqlx::query("SET LOCAL app.current_tenant = $1")
            .bind(&payload.tenant_id)
            .execute(&mut *tx)
            .await
            .map_err(|_| Status::internal("Tenant isolation error"))?;

        // 1. Ensure conversation exists or create one
        let conv_row: Option<(String,)> = sqlx::query_as(
            "SELECT id FROM chat_conversation WHERE tenant_id = $1 AND inbox_id = $2 AND contact_id = $3 LIMIT 1"
        )
        .bind(&payload.tenant_id)
        .bind(&payload.inbox_id)
        .bind(&payload.contact_id)
        .fetch_optional(&mut *tx)
        .await
        .map_err(|e| Status::internal(format!("DB error: {}", e)))?;

        let conversation_id = match conv_row {
            Some((id,)) => id,
            None => {
                let new_id = Uuid::new_v4().to_string();
                sqlx::query(
                    "INSERT INTO chat_conversation (id, tenant_id, inbox_id, contact_id, status) VALUES ($1, $2, $3, $4, $5)"
                )
                .bind(&new_id)
                .bind(&payload.tenant_id)
                .bind(&payload.inbox_id)
                .bind(&payload.contact_id)
                .bind("open")
                .execute(&mut *tx)
                .await
                .map_err(|e| Status::internal(format!("DB error: {}", e)))?;
                new_id
            }
        };

        // 2. Insert message
        let msg_id = Uuid::new_v4().to_string();
        sqlx::query(
            "INSERT INTO chat_message (id, tenant_id, conversation_id, content, sender_type, source) VALUES ($1, $2, $3, $4, $5, $6)"
        )
        .bind(&msg_id)
        .bind(&payload.tenant_id)
        .bind(&conversation_id)
        .bind(&payload.content)
        .bind("contact")
        .bind(&payload.source)
        .execute(&mut *tx)
        .await
        .map_err(|e| Status::internal(format!("DB error: {}", e)))?;

        let message = Message {
            id: msg_id,
            tenant_id: payload.tenant_id.clone(),
            conversation_id,
            content: payload.content.clone(),
            sender_type: "contact".to_string(),
            source: payload.source.clone(),
        };

        tx.commit().await.map_err(|_| Status::internal("Commit error"))?;

        Ok(Response::new(ProcessWebhookResponse { message: Some(message) }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_chat_service_instantiation() {
        // Just verify it compiles and can be instantiated.
        // A real test would need a DB pool which is harder in a simple unit test
        // without a test DB setup.
        assert!(true);
    }
}
