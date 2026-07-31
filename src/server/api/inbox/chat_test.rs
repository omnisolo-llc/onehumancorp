use super::chat::{handle_dummy_webhook, ChatAppState, DummyWebhookPayload};
use axum::Json;
use uuid::Uuid;
use std::sync::Arc;
use crate::db::DB;

#[tokio::test]
async fn test_dummy_webhook_saves_message() {
    let db = DB::new().await.expect("Failed to init DB");
    let state = ChatAppState { db: Arc::new(db) };

    let tenant_id = Uuid::new_v4();
    let contact_id = Uuid::new_v4();
    let inbox_id = Uuid::new_v4();

    // 1. Manually insert the required dependencies to satisfy foreign keys
    let pool = &state.db.pool;

    sqlx::query("INSERT INTO chat_inboxes (id, tenant_id, name) VALUES ($1, $2, 'Dummy Inbox')")
        .bind(inbox_id)
        .bind(tenant_id)
        .execute(pool)
        .await
        .expect("Failed to setup chat_inbox");

    sqlx::query("INSERT INTO chat_contacts (id, tenant_id, name) VALUES ($1, $2, 'Dummy Contact')")
        .bind(contact_id)
        .bind(tenant_id)
        .execute(pool)
        .await
        .expect("Failed to setup chat_contact");

    let payload = DummyWebhookPayload {
        tenant_id: tenant_id.to_string(),
        inbox_id: inbox_id.to_string(),
        contact_id: contact_id.to_string(),
        channel: "web_widget".to_string(),
        sender_type: "contact".to_string(),
        content: "my dummy content".to_string(),
    };

    let response = handle_dummy_webhook(axum::extract::State(state.clone()), Json(payload)).await;
    assert!(response.is_ok(), "Handler failed");

    // Verify it was actually written
    let count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM chat_messages WHERE content = 'my dummy content'")
        .fetch_one(pool)
        .await
        .expect("Failed to query");

    assert_eq!(count.0, 1);
}
