use axum::{
    body::Body,
    http::{Request, StatusCode},
    routing::post,
    Router,
};
use serde_json::json;
use tower::ServiceExt;
use crate::api::conversational_webhook::{handle_conversational_webhook_internal, ConversationalWebhookPayload, SessionStore};
use crate::domain::repository::models::ConversationalCheckoutSession;
use std::time::Duration;
use std::sync::Arc;
use tokio::sync::Mutex;

struct MockSessionStore {
    sessions: Arc<Mutex<Vec<ConversationalCheckoutSession>>>,
}

#[async_trait::async_trait]
impl SessionStore for MockSessionStore {
    async fn get_product_price(&self, product_id: &str, _tenant_id: &str) -> Result<Option<i64>, String> {
        if product_id == "p1_db_hit" {
            Ok(Some(2000))
        } else {
            Ok(None)
        }
    }

    async fn save_session(&self, session: &ConversationalCheckoutSession) -> Result<(), String> {
        self.sessions.lock().await.push(session.clone());
        Ok(())
    }
}

#[tokio::test]
async fn test_conversational_webhook_cache_hit() {
    let cache = crate::builder::edge::get_edge_cache();
    cache.set_with_tags(
        "ohc:edge:tenant:t1_test:product:p1_cache_hit",
        json!({"price_cents": 1500}).to_string(),
        vec!["test".to_string()],
        Duration::from_secs(60)
    ).await;

    let payload = ConversationalWebhookPayload {
        tenant_id: "t1_test".to_string(),
        customer_id: "c1_test".to_string(),
        product_id: "p1_cache_hit".to_string(),
    };

    let store = MockSessionStore { sessions: Arc::new(Mutex::new(Vec::new())) };
    let response = handle_conversational_webhook_internal(&store, payload).await;

    let res = response.into_response();
    assert_eq!(res.status(), StatusCode::OK);

    let sessions = store.sessions.lock().await;
    assert_eq!(sessions.len(), 1);
    assert_eq!(sessions[0].amount, 1500);
}

#[tokio::test]
async fn test_conversational_webhook_db_hit() {
    let payload = ConversationalWebhookPayload {
        tenant_id: "t1_test".to_string(),
        customer_id: "c1_test".to_string(),
        product_id: "p1_db_hit".to_string(),
    };

    let store = MockSessionStore { sessions: Arc::new(Mutex::new(Vec::new())) };
    let response = handle_conversational_webhook_internal(&store, payload).await;

    let res = response.into_response();
    assert_eq!(res.status(), StatusCode::OK);

    let sessions = store.sessions.lock().await;
    assert_eq!(sessions.len(), 1);
    assert_eq!(sessions[0].amount, 2000);
}

#[tokio::test]
async fn test_conversational_webhook_isolation() {
    let payload = ConversationalWebhookPayload {
        tenant_id: "t1_test_missing".to_string(),
        customer_id: "c1_test".to_string(),
        product_id: "p1_missing".to_string(),
    };

    let store = MockSessionStore { sessions: Arc::new(Mutex::new(Vec::new())) };
    let response = handle_conversational_webhook_internal(&store, payload).await;

    let res = response.into_response();
    assert_eq!(res.status(), StatusCode::NOT_FOUND);
}
