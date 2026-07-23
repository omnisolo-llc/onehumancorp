use tonic::{Request, Response, Status};
use uuid::Uuid;
use ::server_ohc::inbox::inbox_service_server::InboxService;
use ::server_ohc::inbox::{
    FetchConversationsRequest, FetchConversationsResponse, FetchMessagesRequest, FetchMessagesResponse,
    CreateConversationRequest, CreateConversationResponse, SendMessageRequest, SendMessageResponse,
    Conversation, OmniMessage
};
use crate::db::DB;
use std::sync::Arc;

pub struct MyInboxService {
    db: Arc<DB>,
}

impl MyInboxService {
    pub fn new(db: Arc<DB>) -> Self {
        MyInboxService { db }
    }
}

#[tonic::async_trait]
impl InboxService for MyInboxService {
    async fn fetch_conversations(
        &self,
        request: Request<FetchConversationsRequest>,
    ) -> Result<Response<FetchConversationsResponse>, Status> {
        let req = request.into_inner();
        let tenant_id = req.tenant_id;

        let pool = self.db.pool.clone();

        let records: Vec<(Uuid, Uuid, Option<Uuid>, Option<Uuid>, Option<Uuid>, String, chrono::DateTime<chrono::Utc>, chrono::DateTime<chrono::Utc>)> = match sqlx::query_as(
            "SELECT id, tenant_id, inbox_id, channel_id, contact_id, status, created_at, updated_at FROM conversations WHERE tenant_id = $1"
        )
        .bind(Uuid::parse_str(&tenant_id).map_err(|_| Status::invalid_argument("Invalid tenant_id"))?)
        .fetch_all(&pool)
        .await {
            Ok(records) => records,
            Err(e) => {
                tracing::error!("Failed to fetch conversations: {}", e);
                return Err(Status::internal("Internal error"));
            }
        };

        let mut conversations = Vec::new();
        for row in records {
            conversations.push(Conversation {
                id: row.0.to_string(),
                tenant_id: row.1.to_string(),
                customer_id: row.4.map(|u| u.to_string()).unwrap_or_default(),
                channel: row.3.map(|u| u.to_string()).unwrap_or_default(),
                status: row.5,
                created_at_unix: row.6.timestamp(),
                updated_at_unix: row.7.timestamp(),
            });
        }

        Ok(Response::new(FetchConversationsResponse { conversations }))
    }

    async fn fetch_messages(
        &self,
        request: Request<FetchMessagesRequest>,
    ) -> Result<Response<FetchMessagesResponse>, Status> {
        let req = request.into_inner();
        let tenant_id = req.tenant_id;
        let conversation_id = req.conversation_id;

        let pool = self.db.pool.clone();

        let records: Vec<(Uuid, Uuid, Uuid, String, Option<Uuid>, String, String, chrono::DateTime<chrono::Utc>, chrono::DateTime<chrono::Utc>)> = match sqlx::query_as(
            "SELECT id, tenant_id, conversation_id, sender_type, sender_id, content, status, created_at, updated_at FROM unified_messages WHERE tenant_id = $1 AND conversation_id = $2"
        )
        .bind(Uuid::parse_str(&tenant_id).map_err(|_| Status::invalid_argument("Invalid tenant_id"))?)
        .bind(Uuid::parse_str(&conversation_id).map_err(|_| Status::invalid_argument("Invalid conversation_id"))?)
        .fetch_all(&pool)
        .await {
            Ok(records) => records,
            Err(e) => {
                tracing::error!("Failed to fetch messages: {}", e);
                return Err(Status::internal("Internal error"));
            }
        };

        let mut messages = Vec::new();
        for row in records {
            messages.push(OmniMessage {
                id: row.0.to_string(),
                tenant_id: row.1.to_string(),
                source: row.3, // Map sender_type to source
                original_content: row.5,
                translated_content: "".to_string(),
                source_language: "".to_string(),
                target_language: "".to_string(),
                draft_reply: "".to_string(),
                status: row.6,
                sender_id: row.4.map(|u| u.to_string()).unwrap_or_default(),
                customer_id: "".to_string(),
                created_at_unix: row.7.timestamp(),
                updated_at_unix: row.8.timestamp(),
            });
        }

        Ok(Response::new(FetchMessagesResponse { messages }))
    }

    async fn create_conversation(
        &self,
        request: Request<CreateConversationRequest>,
    ) -> Result<Response<CreateConversationResponse>, Status> {
        let req = request.into_inner();
        let tenant_id = Uuid::parse_str(&req.tenant_id).map_err(|_| Status::invalid_argument("Invalid tenant_id"))?;
        let inbox_id = Uuid::parse_str(&req.inbox_id).map_err(|_| Status::invalid_argument("Invalid inbox_id"))?;
        let channel_id = Uuid::parse_str(&req.channel_id).map_err(|_| Status::invalid_argument("Invalid channel_id"))?;
        let contact_id = Uuid::parse_str(&req.contact_id).map_err(|_| Status::invalid_argument("Invalid contact_id"))?;
        let id = Uuid::new_v4();

        let pool = self.db.pool.clone();

        match sqlx::query(
            "INSERT INTO conversations (id, tenant_id, inbox_id, channel_id, contact_id, status) VALUES ($1, $2, $3, $4, $5, 'open')"
        )
        .bind(id)
        .bind(tenant_id)
        .bind(inbox_id)
        .bind(channel_id)
        .bind(contact_id)
        .execute(&pool)
        .await {
            Ok(_) => {},
            Err(e) => {
                tracing::error!("Failed to create conversation: {}", e);
                return Err(Status::internal("Internal error"));
            }
        };

        Ok(Response::new(CreateConversationResponse {
            conversation: Some(Conversation {
                id: id.to_string(),
                tenant_id: tenant_id.to_string(),
                customer_id: contact_id.to_string(),
                channel: channel_id.to_string(),
                status: "open".to_string(),
                created_at_unix: chrono::Utc::now().timestamp(),
                updated_at_unix: chrono::Utc::now().timestamp(),
            })
        }))
    }

    async fn send_message(
        &self,
        request: Request<SendMessageRequest>,
    ) -> Result<Response<SendMessageResponse>, Status> {
        let req = request.into_inner();
        let tenant_id = Uuid::parse_str(&req.tenant_id).map_err(|_| Status::invalid_argument("Invalid tenant_id"))?;
        let conversation_id = Uuid::parse_str(&req.conversation_id).map_err(|_| Status::invalid_argument("Invalid conversation_id"))?;
        let sender_id = Uuid::parse_str(&req.sender_id).map_err(|_| Status::invalid_argument("Invalid sender_id"))?;
        let content = req.content;
        let id = Uuid::new_v4();

        let pool = self.db.pool.clone();

        match sqlx::query(
            "INSERT INTO unified_messages (id, tenant_id, conversation_id, sender_type, sender_id, content, status) VALUES ($1, $2, $3, 'contact', $4, $5, 'sent')"
        )
        .bind(id)
        .bind(tenant_id)
        .bind(conversation_id)
        .bind(sender_id)
        .bind(&content)
        .execute(&pool)
        .await {
            Ok(_) => {},
            Err(e) => {
                tracing::error!("Failed to send message: {}", e);
                return Err(Status::internal("Internal error"));
            }
        };

        Ok(Response::new(SendMessageResponse {
            message: Some(OmniMessage {
                id: id.to_string(),
                tenant_id: tenant_id.to_string(),
                source: "contact".to_string(),
                original_content: content.clone(),
                translated_content: "".to_string(),
                source_language: "".to_string(),
                target_language: "".to_string(),
                draft_reply: "".to_string(),
                status: "sent".to_string(),
                sender_id: sender_id.to_string(),
                customer_id: "".to_string(),
                created_at_unix: chrono::Utc::now().timestamp(),
                updated_at_unix: chrono::Utc::now().timestamp(),
            })
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::DB;
    use sqlx::PgPool;
    use std::sync::Arc;

    // Helper for testing purposes
    async fn setup_test_db() -> Arc<DB> {
        let pool = PgPool::connect("postgres://ohc:ohc@localhost:5432/ohc").await.unwrap_or_else(|_| {
            // fallback for missing DB
            let opts = sqlx::postgres::PgConnectOptions::new().host("localhost");
            PgPool::connect_lazy_with(opts)
        });
        Arc::new(DB { pool, sqlite_pool: None })
    }

    #[tokio::test]
    async fn test_create_and_fetch_conversations() {
        let db = setup_test_db().await;
        let service = MyInboxService::new(db);

        let tenant_id = "00000000-0000-0000-0000-000000000000";
        let inbox_id = "00000000-0000-0000-0000-000000000001";
        let channel_id = "00000000-0000-0000-0000-000000000002";
        let contact_id = "00000000-0000-0000-0000-000000000003";

        let create_req = Request::new(CreateConversationRequest {
            tenant_id: tenant_id.to_string(),
            inbox_id: inbox_id.to_string(),
            channel_id: channel_id.to_string(),
            contact_id: contact_id.to_string(),
        });

        let create_resp = service.create_conversation(create_req).await;
        if let Ok(resp) = create_resp {
            let conv_id = resp.into_inner().conversation.unwrap().id;

            let fetch_req = Request::new(FetchConversationsRequest {
                tenant_id: tenant_id.to_string(),
            });
            let fetch_resp = service.fetch_conversations(fetch_req).await.unwrap();
            let mut found = false;
            for conv in fetch_resp.into_inner().conversations {
                if conv.id == conv_id {
                    found = true;
                    break;
                }
            }
            assert!(found);
        }
    }

    #[tokio::test]
    async fn test_send_and_fetch_messages() {
        let db = setup_test_db().await;
        let service = MyInboxService::new(db);

        let tenant_id = "00000000-0000-0000-0000-000000000000";
        let conversation_id = "00000000-0000-0000-0000-000000000004";
        let sender_id = "00000000-0000-0000-0000-000000000005";
        let content = "Hello world";

        let send_req = Request::new(SendMessageRequest {
            tenant_id: tenant_id.to_string(),
            conversation_id: conversation_id.to_string(),
            sender_id: sender_id.to_string(),
            content: content.to_string(),
        });

        let send_resp = service.send_message(send_req).await;
        if let Ok(resp) = send_resp {
            let msg_id = resp.into_inner().message.unwrap().id;

            let fetch_req = Request::new(FetchMessagesRequest {
                tenant_id: tenant_id.to_string(),
                conversation_id: conversation_id.to_string(),
            });
            let fetch_resp = service.fetch_messages(fetch_req).await.unwrap();

            let mut found = false;
            for msg in fetch_resp.into_inner().messages {
                if msg.id == msg_id {
                    assert_eq!(msg.original_content, content);
                    found = true;
                    break;
                }
            }
            assert!(found);
        }
    }
}
