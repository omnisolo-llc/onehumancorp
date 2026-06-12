use axum::{
    extract::{State, Json},
    response::IntoResponse,
    http::StatusCode,
    routing::post,
    Router,
};
use std::sync::Arc;
use serde::{Deserialize, Serialize};
use crate::orchestration::departments::orchestrator::DepartmentOrchestrator;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct AmbassadorWebhookPayload {
    pub tenant_id: String,
    pub message: String,
    pub source: String,
}

#[derive(Serialize)]
pub struct WebhookResponse {
    pub success: bool,
    pub feed_item_id: Option<String>,
}

#[derive(Clone)]
pub struct AppState {
    pub orchestrator: Arc<DepartmentOrchestrator>,
}

pub fn router<S>(orchestrator: Arc<DepartmentOrchestrator>) -> Router<S>
where
    S: Clone + Send + Sync + 'static,
{
    let state = AppState { orchestrator };
    Router::new()
        .route("/", post(handle_ambassador_webhook))
        .with_state(state)
}

async fn handle_ambassador_webhook(
    State(state): State<AppState>,
    Json(payload): Json<AmbassadorWebhookPayload>,
) -> impl IntoResponse {
    let tenant_id = payload.tenant_id.clone();
    let message = payload.message.clone();

    // 1. Generate RAG context
    let query_embedding = match std::env::var("OHC_INBOX_DRAFT_LLM_PROVIDER")
        .or_else(|_| std::env::var("OHC_LLM_PROVIDER"))
        .as_deref()
    {
        Ok("minimax") => {
            let api_key = std::env::var("MINIMAX_API_KEY").unwrap_or_else(|_| "fake-key".to_string());
            crate::minimax::MinimaxClient::new(api_key).generate_embedding(&message).await.unwrap_or_else(|_| vec![0.0; 1536])
        }
        _ => {
            crate::minimax::LocalLLMClient::new().generate_embedding(&message).await.unwrap_or_else(|_| vec![0.0; 1536])
        }
    };

    let memories = state.orchestrator.query_long_term_memory(&tenant_id, &query_embedding, 5).await.unwrap_or_default();

    let mut context_summary = if !memories.is_empty() {
        memories.join("\n")
    } else {
        "No relevant memory found.".to_string()
    };

    if let Ok(inventory_summary) = state.orchestrator.get_inventory_summary(&tenant_id).await {
        context_summary.push_str("\n\n");
        context_summary.push_str(&inventory_summary);
    }

    // 2. Classify intent & draft reply
    let prompt = format!(
        "You are an Ambassador Agent for an SMB. Write one concise, warm customer-service reply for an omnichannel inbox. Do not invent policies, availability, prices, or order state. Use the provided inventory context if asked about product availability. Tenant: {}. Customer message: {}\n\nContext:\n{}",
        tenant_id, message, context_summary
    );
    let compressed_prompt = crate::pricing::compression::reduce_tokens(&prompt);

    let drafted_reply = match std::env::var("OHC_INBOX_DRAFT_LLM_PROVIDER")
        .or_else(|_| std::env::var("OHC_LLM_PROVIDER"))
        .as_deref()
    {
        Ok("minimax") => {
            let api_key = std::env::var("MINIMAX_API_KEY").unwrap_or_else(|_| "fake-key".to_string());
            crate::minimax::MinimaxClient::new(api_key).reason(&compressed_prompt).await.unwrap_or_else(|_| "Thank you for your message. We will get back to you shortly.".to_string())
        }
        _ => {
            crate::minimax::LocalLLMClient::new().reason(&compressed_prompt).await.unwrap_or_else(|_| "Thank you for your message. We will get back to you shortly.".to_string())
        }
    };

    // 3. Insert directly into agent_feed_items
    let feed_item_id = Uuid::new_v4().to_string();

    let context_payload = serde_json::json!({
        "customer_message": message,
        "feature_type": "ambassador_reply",
        "draft_reply": drafted_reply
    });

    // Extract the internal DB from orchestrator
    // We use match &db.store to support both Postgres and Sqlite (for tests)
    let result = match &state.orchestrator.db.store {
        crate::db::DbStore::Postgres => {
            let res = sqlx::query(
                r#"
                INSERT INTO agent_feed_items (id, tenant_id, event_source, context_payload, lifecycle_state, created_at, updated_at)
                VALUES ($1, $2, $3, $4, 'PENDING_APPROVAL', NOW(), NOW())
                "#
            )
            .bind(&feed_item_id)
            .bind(&tenant_id)
            .bind("instagram_dm")
            .bind(sqlx::types::Json(context_payload.clone()))
            .execute(&state.orchestrator.db.pool)
            .await;
            res.map(|_| ())
        },
        crate::db::DbStore::Sqlite(sqlite_pool) => {
            let res = sqlx::query(
                r#"
                INSERT INTO agent_feed_items (id, tenant_id, event_source, context_payload, lifecycle_state, created_at, updated_at)
                VALUES (?, ?, ?, ?, 'PENDING_APPROVAL', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)
                "#
            )
            .bind(&feed_item_id)
            .bind(&tenant_id)
            .bind("instagram_dm")
            .bind(context_payload.to_string())
            .execute(sqlite_pool)
            .await;
            res.map(|_| ())
        }
    };

    match result {
        Ok(_) => {
            // Invalidate cache and publish to WS similar to agent_feed.rs
            let cache = crate::api::agent_feed::get_agent_feed_cache();
            let tag = format!("agent_feed_tenant:{}", tenant_id);
            // It's ok to run cache invalidation concurrently
            let tenant_id_clone = tenant_id.clone();
            let feed_item_id_clone = feed_item_id.clone();
            tokio::spawn(async move {
                cache.invalidate_by_tag(&tag).await;

                // Publish to Redis Pub/Sub
                let client = crate::api::agent_feed::get_redis_client();
                let topic = format!("ohc:feed:{}", tenant_id_clone);
                // Creating a simplified payload to push over WS
                let payload_json = serde_json::json!({
                    "id": feed_item_id_clone,
                    "tenant_id": tenant_id_clone,
                    "event_source": "instagram_dm",
                    "lifecycle_state": "PENDING_APPROVAL"
                }).to_string();

                if let Ok(mut conn) = client.get_multiplexed_async_connection().await {
                    let _: Result<(), _> = redis::AsyncCommands::publish(&mut conn, topic, payload_json).await;
                }
            });

            (StatusCode::OK, Json(WebhookResponse { success: true, feed_item_id: Some(feed_item_id) })).into_response()
        },
        Err(e) => {
            tracing::error!("Failed to insert ambassador agent_feed_item: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(WebhookResponse { success: false, feed_item_id: None })).into_response()
        }
    }
}