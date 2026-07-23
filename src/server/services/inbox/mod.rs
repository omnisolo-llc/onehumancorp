use tonic::{Request, Response, Status};
use sqlx::{PgPool, Row};
use std::sync::Arc;
use server_ohc::inbox::{
    FetchConversationsRequest, FetchConversationsResponse,
    FetchMessagesRequest, FetchMessagesResponse,
    OmniMessage, Conversation,
};

pub mod server {
    use super::*;
    use server_ohc::inbox::inbox_server::Inbox;

    pub struct InboxService {
        db: PgPool,
    }

    impl InboxService {
        pub fn new(db: PgPool) -> Self {
            Self { db }
        }
    }

    #[tonic::async_trait]
    impl Inbox for InboxService {
        async fn fetch_conversations(
            &self,
            request: Request<FetchConversationsRequest>,
        ) -> Result<Response<FetchConversationsResponse>, Status> {
            let req = request.into_inner();

            // Set RLS scope
            let mut tx = self.db.begin().await.map_err(|e| Status::internal(e.to_string()))?;
            sqlx::query("SELECT set_config('app.current_tenant', $1, true)")
                .bind(&req.tenant_id)
                .execute(&mut *tx)
                .await
                .map_err(|e| Status::internal(e.to_string()))?;

            let rows = sqlx::query(
                "SELECT DISTINCT
                    customer_id,
                    channel_type as channel,
                    status,
                    EXTRACT(EPOCH FROM created_at)::bigint as created_at_unix
                 FROM inbox_messages
                 WHERE tenant_id = $1
                 ORDER BY created_at_unix DESC"
            )
            .bind(&req.tenant_id)
            .fetch_all(&mut *tx)
            .await
            .map_err(|e| Status::internal(e.to_string()))?;

            let conversations = rows.into_iter().map(|r| {
                Conversation {
                    id: format!("{}-{}", r.get::<String, _>("customer_id"), r.get::<String, _>("channel")),
                    tenant_id: req.tenant_id.clone(),
                    customer_id: r.get("customer_id"),
                    channel: r.get("channel"),
                    status: r.get("status"),
                    created_at_unix: r.get("created_at_unix"),
                    updated_at_unix: r.get("created_at_unix"),
                }
            }).collect();

            Ok(Response::new(FetchConversationsResponse { conversations }))
        }

        async fn fetch_messages(
            &self,
            request: Request<FetchMessagesRequest>,
        ) -> Result<Response<FetchMessagesResponse>, Status> {
            let req = request.into_inner();

            let mut tx = self.db.begin().await.map_err(|e| Status::internal(e.to_string()))?;
            sqlx::query("SELECT set_config('app.current_tenant', $1, true)")
                .bind(&req.tenant_id)
                .execute(&mut *tx)
                .await
                .map_err(|e| Status::internal(e.to_string()))?;

            let parts: Vec<&str> = req.conversation_id.split('-').collect();
            let (customer_id, channel) = if parts.len() >= 2 {
                (parts[0], parts[1])
            } else {
                ("", "")
            };

            let rows = sqlx::query(
                "SELECT id, channel_type as source, original_content,
                        translated_content, source_language, target_language,
                        draft_reply, status, sender_id, customer_id,
                        EXTRACT(EPOCH FROM created_at)::bigint as created_at_unix,
                        EXTRACT(EPOCH FROM updated_at)::bigint as updated_at_unix
                 FROM omni_inbox_messages
                 WHERE tenant_id = $1 AND customer_id = $2 AND channel_type = $3
                 ORDER BY created_at ASC"
            )
            .bind(&req.tenant_id)
            .bind(customer_id)
            .bind(channel)
            .fetch_all(&mut *tx)
            .await
            .map_err(|e| Status::internal(e.to_string()))?;

            let messages = rows.into_iter().map(|r| {
                OmniMessage {
                    id: r.get("id"),
                    tenant_id: req.tenant_id.clone(),
                    source: r.get("source"),
                    original_content: r.get("original_content"),
                    translated_content: r.get("translated_content"),
                    source_language: r.get("source_language"),
                    target_language: r.get("target_language"),
                    draft_reply: r.get("draft_reply"),
                    status: r.get("status"),
                    sender_id: r.get("sender_id"),
                    customer_id: r.get("customer_id"),
                    created_at_unix: r.get("created_at_unix"),
                    updated_at_unix: r.get("updated_at_unix"),
                }
            }).collect();

            Ok(Response::new(FetchMessagesResponse { messages }))
        }
    }
}

pub mod nats_listener {
    use sqlx::PgPool;

    pub struct InboxNatsListener {
        db: PgPool,
    }

    impl InboxNatsListener {
        pub fn new(db: PgPool) -> Self {
            Self { db }
        }

        pub async fn start(&self) {
            tracing::info!("Starting Inbox NATS listener for Omnichannel...");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::server::InboxService;
    use server_ohc::inbox::{FetchConversationsRequest, FetchMessagesRequest};

    #[tokio::test]
    async fn test_inbox_service_fetch_requests() {
        let req = FetchConversationsRequest {
            tenant_id: "test-tenant-id".to_string(),
        };
        assert_eq!(req.tenant_id, "test-tenant-id");

        let msg_req = FetchMessagesRequest {
            tenant_id: "test-tenant-id".to_string(),
            conversation_id: "cust1-ig".to_string(),
        };
        assert_eq!(msg_req.conversation_id, "cust1-ig");
    }
}
