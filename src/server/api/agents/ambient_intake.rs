use axum::{
    extract::{State, Form},
    response::IntoResponse,
    http::StatusCode,
    routing::post,
    Router,
    Json,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use sqlx::PgPool;
use uuid::Uuid;
use chrono::Utc;
use crate::orchestration::departments::orchestrator::DepartmentOrchestrator;
use redis::AsyncCommands;
use ohc_builtin_agent::LLMAgent;

#[derive(Deserialize)]
pub struct IntakeRequest {
    pub tenant_id: String,
    pub customer_id: Option<String>,
    pub channel: String,
    pub message: String,
}

#[derive(Serialize)]
pub struct IntakeResponse {
    pub success: bool,
    pub feed_item_id: Option<String>,
}

pub fn router<S>(pool: PgPool, orchestrator: Arc<DepartmentOrchestrator>) -> Router<S>
where
    S: Clone + Send + Sync + 'static,
{
    Router::new()
        .route("/", post(handle_intake))
        .with_state((pool, orchestrator))
}

async fn handle_intake(
    State((pool, _orchestrator)): State<(PgPool, Arc<DepartmentOrchestrator>)>,
    Form(payload): Form<IntakeRequest>,
) -> impl IntoResponse {
    let tenant_id = payload.tenant_id;
    let channel = payload.channel;
    let message = payload.message;

    let redis_url = std::env::var("REDIS_URL").unwrap_or_else(|_| "redis://localhost:6379".to_string());
    let client = redis::Client::open(redis_url).unwrap();
    let mut con = match client.get_multiplexed_async_connection().await {
        Ok(c) => c,
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, Json(IntakeResponse { success: false, feed_item_id: None })).into_response(),
    };

    let lock_key = format!("ohc:lock:{}:schedule", tenant_id);
    let lock_acquired: bool = redis::cmd("SET")
        .arg(&lock_key)
        .arg("LOCKED")
        .arg("NX")
        .arg("EX")
        .arg(60)
        .query_async(&mut con).await.unwrap_or(false);

    if !lock_acquired {
        return (StatusCode::CONFLICT, Json(IntakeResponse { success: false, feed_item_id: None })).into_response();
    }

    let llm_agent = LLMAgent::new("You are a helpful intake assistant. Extract the estimated cost based on the service requested. If the user mentions 'sink', estimate 15000 cents. Otherwise, default to 10000 cents. Output ONLY a valid JSON object like: {\"price\": 15000, \"service\": \"Sink Repair\"}.").unwrap_or_else(|_| LLMAgent::new("").unwrap());

    let llm_response = llm_agent.generate(&message, std::time::Duration::from_secs(10)).await.unwrap_or_else(|_| "{\"price\": 10000, \"service\": \"General Repair\"}".to_string());

    let parsed: serde_json::Value = serde_json::from_str(&llm_response).unwrap_or_else(|_| serde_json::json!({"price": 10000, "service": "General Service"}));

    let suggested_price = parsed.get("price").and_then(|v| v.as_i64()).unwrap_or(10000);
    let service_description = parsed.get("service").and_then(|v| v.as_str()).unwrap_or("General Service");

    let start_time = Utc::now() + chrono::Duration::days(1);
    let end_time = start_time + chrono::Duration::hours(2);

    let draft_quote_id = Uuid::new_v4();
    let customer_id = Uuid::parse_str(payload.customer_id.as_deref().unwrap_or("")).unwrap_or_else(|_| Uuid::new_v4());

    let mut tx = pool.begin().await.unwrap();

    let _ = sqlx::query(
        "INSERT INTO quotes (id, tenant_id, customer_id, status, created_at, updated_at) VALUES ($1, $2, $3, 'DRAFT', NOW(), NOW())"
    )
    .bind(&draft_quote_id)
    .bind(&tenant_id)
    .bind(&customer_id)
    .execute(&mut *tx).await;

    let _ = sqlx::query(
        "INSERT INTO quote_line_items (id, quote_id, description, unit_price_cents, quantity, is_optional, created_at, updated_at) VALUES ($1, $2, $3, $4, 1, false, NOW(), NOW())"
    )
    .bind(Uuid::new_v4())
    .bind(&draft_quote_id)
    .bind(service_description)
    .bind(suggested_price)
    .execute(&mut *tx).await;

    let feed_item_id = Uuid::new_v4().to_string();
    let context_payload = serde_json::json!({
        "channel": channel,
        "message": message,
        "customer_id": customer_id.to_string(),
    });

    let proposed_action = serde_json::json!({
        "action_type": "Draft Quote & Proposed Booking",
        "quote_id": draft_quote_id.to_string(),
        "proposed_start_time": start_time.to_rfc3339(),
        "proposed_end_time": end_time.to_rfc3339(),
        "total_amount_cents": suggested_price,
        "description": "Agent drafted a quote based on incoming request."
    });

    let _ = sqlx::query(
        "INSERT INTO agent_feed_items (id, tenant_id, event_source, context_payload, proposed_action, lifecycle_state, created_at, updated_at) VALUES ($1, $2, $3, $4, $5, 'PENDING_APPROVAL', NOW(), NOW())"
    )
    .bind(&feed_item_id)
    .bind(&tenant_id)
    .bind("ambient_intake")
    .bind(sqlx::types::Json(context_payload))
    .bind(sqlx::types::Json(proposed_action))
    .execute(&mut *tx).await;

    tx.commit().await.unwrap();

    let _: () = redis::cmd("DEL").arg(&lock_key).query_async(&mut con).await.unwrap_or(());

    (StatusCode::OK, Json(IntakeResponse { success: true, feed_item_id: Some(feed_item_id) })).into_response()
}
