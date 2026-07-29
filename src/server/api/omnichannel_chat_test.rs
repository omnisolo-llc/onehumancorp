#[cfg(test)]
mod tests {
    use axum::{body::Body, http::{Request, StatusCode}, response::IntoResponse};
    use tower::ServiceExt;
    use sqlx::SqlitePool;
    use crate::db::{DB, DbStore};
    use crate::orchestration::departments::DepartmentOrchestrator;
    use std::sync::Arc;
    use uuid::Uuid;
    use axum::extract::State;
    use axum::Json;
    use server_common::Claims;

    #[tokio::test]
    async fn test_omnichannel_webhook_and_crud() {
        let pool = SqlitePool::connect("sqlite::memory:").await.unwrap();
        let schema = r#"
            CREATE TABLE inboxes (id TEXT PRIMARY KEY, tenant_id TEXT NOT NULL, name TEXT NOT NULL, enable_auto_assignment BOOLEAN, created_at TEXT, updated_at TEXT);
            CREATE TABLE channels (id TEXT PRIMARY KEY, tenant_id TEXT NOT NULL, inbox_id TEXT NOT NULL, provider_type TEXT NOT NULL, credentials TEXT, created_at TEXT, updated_at TEXT);
            CREATE TABLE contacts (id TEXT PRIMARY KEY, tenant_id TEXT NOT NULL, name TEXT, phone_number TEXT, created_at TEXT, updated_at TEXT, UNIQUE(tenant_id, phone_number));
            CREATE TABLE conversations (id TEXT PRIMARY KEY, tenant_id TEXT NOT NULL, inbox_id TEXT NOT NULL, contact_id TEXT, status TEXT NOT NULL, created_at TEXT, updated_at TEXT);
            CREATE TABLE messages (id TEXT PRIMARY KEY, tenant_id TEXT NOT NULL, conversation_id TEXT NOT NULL, content TEXT NOT NULL, sender_type TEXT NOT NULL, created_at TEXT, updated_at TEXT);
            CREATE TABLE ohc_job_queue (id TEXT PRIMARY KEY, tenant_id TEXT, job_type TEXT, payload TEXT, status TEXT);
        "#;
        sqlx::query(schema).execute(&pool).await.unwrap();

        let db = Arc::new(DB {
            pool: sqlx::PgPool::connect_lazy("postgres://dummy").unwrap(),
            store: DbStore::Sqlite(pool.clone()),
        });

        let transport = Arc::new(ohc_builtin_agent::mesh::transport::InProcessTransport::new());
        let mesh = Arc::new(crate::orchestration::mesh::CentrifugeNode::new(transport));
        let orchestrator = Arc::new(DepartmentOrchestrator::new(db.clone(), mesh));
        let state = crate::api::omnichannel_chat::OmnichannelState { db, orchestrator };
        let tenant_id = "tenant-test";

        let claims = axum::Extension(Claims {
            sub: "user-1".into(),
            exp: 0,
            iat: 0,
            organization_id: Some(tenant_id.to_string()),
            username: "user".into(),
            email: "user@example.com".into(),
            roles: vec![],
            session_id: None,
            jti: "jti".into(),
        });

        // Test create_inbox
        let inbox_req = crate::api::omnichannel_chat::CreateInboxReq {
            name: "Test Inbox".into(),
            enable_auto_assignment: Some(false),
        };
        let inbox_res = crate::api::omnichannel_chat::create_inbox(State(state.clone()), claims.clone(), Json(inbox_req)).await.unwrap();
        let inbox = inbox_res.0;
        assert_eq!(inbox.name, "Test Inbox");

        // Test list_inboxes
        let list_inboxes_res = crate::api::omnichannel_chat::list_inboxes(State(state.clone()), claims.clone()).await.unwrap();
        assert_eq!(list_inboxes_res.0.len(), 1);

        // Test create_channel
        let channel_req = crate::api::omnichannel_chat::CreateChannelReq {
            inbox_id: inbox.id,
            provider_type: "whatsapp".into(),
            credentials: None,
        };
        let channel_res = crate::api::omnichannel_chat::create_channel(State(state.clone()), claims.clone(), Json(channel_req)).await.unwrap();
        let channel = channel_res.0;
        assert_eq!(channel.provider_type, "whatsapp");

        // Test list_channels
        let list_channels_res = crate::api::omnichannel_chat::list_channels(State(state.clone()), claims.clone()).await.unwrap();
        assert_eq!(list_channels_res.0.len(), 1);

        // Fire webhook
        let payload = crate::api::omnichannel_chat::WebhookPayload {
            message: "Hello world!".to_string(),
            sender: "+1234567".to_string(),
        };
        let response = crate::api::omnichannel_chat::handle_webhook(
            State(state.clone()),
            axum::extract::Path(channel.id),
            Json(payload),
        ).await.unwrap().0;
        assert!(response.success);

        // Verify Message was inserted
        let msg_count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM messages").fetch_one(&pool).await.unwrap();
        assert_eq!(msg_count, 1);

        // Test list_conversations
        let list_conversations_res = crate::api::omnichannel_chat::list_conversations(State(state.clone()), claims.clone()).await.unwrap();
        assert_eq!(list_conversations_res.0.len(), 1);
        let conv_id = list_conversations_res.0[0].id;

        // Test list_messages
        let list_messages_res = crate::api::omnichannel_chat::list_messages(State(state.clone()), claims.clone()).await.unwrap();
        assert_eq!(list_messages_res.0.len(), 1);

        // Test create_conversation manually
        let create_conv_req = crate::api::omnichannel_chat::CreateConversationReq {
            inbox_id: inbox.id,
            contact_id: None,
        };
        let create_conv_res = crate::api::omnichannel_chat::create_conversation(State(state.clone()), claims.clone(), Json(create_conv_req)).await.unwrap();
        assert_eq!(create_conv_res.0.status, "open");

        // Test create_message manually
        let create_msg_req = crate::api::omnichannel_chat::CreateMessageReq {
            conversation_id: conv_id,
            content: "Reply text".into(),
            sender_type: "agent".into(),
        };
        let create_msg_res = crate::api::omnichannel_chat::create_message(State(state.clone()), claims.clone(), Json(create_msg_req)).await.unwrap();
        assert_eq!(create_msg_res.0.content, "Reply text");
    }
}
