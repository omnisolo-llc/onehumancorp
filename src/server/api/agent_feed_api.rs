use axum::{extract::State, Json};
use std::sync::Arc;
use crate::hub::Hub;
use tracing::{info, error};
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct ActionCard {
    pub id: String,
    pub tenant_id: String,
    pub agent_id: String,
    pub action_type: String,
    pub payload: serde_json::Value,
    pub status: String,
    pub created_at: String,
}

#[derive(Serialize)]
pub struct GetFeedResponse {
    pub items: Vec<ActionCard>,
}

#[derive(Deserialize)]
pub struct ActionRequest {
    pub id: String,
    pub tenant_id: String,
}

pub fn router<S: Clone + Send + Sync + 'static>(hub: Arc<Hub>) -> axum::Router<S> {
    axum::Router::new()
        .route("/", axum::routing::get(get_feed_handler))
        .route("/approve", axum::routing::post(approve_action_handler))
        .route("/dismiss", axum::routing::post(dismiss_action_handler))
        .with_state(hub)
}

pub async fn get_feed_handler(
    State(_hub): State<Arc<Hub>>,
) -> Result<Json<GetFeedResponse>, (axum::http::StatusCode, String)> {
    info!("get_feed_handler called");
    let pool = crate::db::get_pool();

    // Using a hardcoded tenant for now as auth extractors aren't implemented here yet.
    let tenant_id = "tenant-1";

    // Don't use query! macro as it requires DB connection at compile time
    let rows: Vec<sqlx::postgres::PgRow> = sqlx::query("
        SELECT id, tenant_id, agent_id, action_type, payload, _sync_status as status, created_at
        FROM agent_actions
        WHERE tenant_id = $1 AND _sync_status = 'PENDING'
        ORDER BY created_at DESC
    ")
    .bind(tenant_id)
    .fetch_all(&pool)
    .await
    .map_err(|e| {
        error!("Database error fetching agent feed: {}", e);
        (axum::http::StatusCode::INTERNAL_SERVER_ERROR, "Database error".to_string())
    })?;

    use sqlx::Row;
    let mut items = vec![];
    for row in rows {
        let id: String = row.get::<String, _>("id");
        let tenant_id: String = row.get::<String, _>("tenant_id");
        let agent_id: String = row.get::<String, _>("agent_id");
        let action_type: String = row.get::<String, _>("action_type");
        let payload_str: Option<String> = row.try_get::<String, _>("payload").ok();
        let status: Option<String> = row.try_get::<String, _>("status").ok();

        let created_at: Option<chrono::DateTime<chrono::Utc>> = match row.try_get("created_at") {
            Ok(dt) => Some(dt),
            Err(_) => {
                match row.try_get::<chrono::NaiveDateTime, _>("created_at") {
                    Ok(ndt) => Some(chrono::DateTime::<chrono::Utc>::from_naive_utc_and_offset(ndt, chrono::Utc)),
                    Err(_) => None
                }
            }
        };

        items.push(ActionCard {
            id,
            tenant_id,
            agent_id,
            action_type,
            payload: serde_json::from_str(payload_str.as_deref().unwrap_or("{}")).unwrap_or_else(|_| serde_json::json!({})),
            status: status.unwrap_or_else(|| "PENDING".to_string()),
            created_at: created_at.map(|ts| ts.to_string()).unwrap_or_default(),
        });
    }

    Ok(Json(GetFeedResponse { items }))
}

pub async fn approve_action_handler(
    State(_hub): State<Arc<Hub>>,
    Json(req): Json<ActionRequest>,
) -> Result<Json<serde_json::Value>, (axum::http::StatusCode, String)> {
    info!("approve_action_handler called for id: {}", req.id);
    let pool = crate::db::get_pool();

    let result = sqlx::query("UPDATE agent_actions SET _sync_status = 'APPROVED', updated_at = CURRENT_TIMESTAMP WHERE id = $1 AND tenant_id = $2")
        .bind(&req.id)
        .bind(&req.tenant_id)
        .execute(&pool)
        .await;

    match result {
        Ok(res) if res.rows_affected() > 0 => Ok(Json(serde_json::json!({ "status": "ok" }))),
        Ok(_) => Err((axum::http::StatusCode::NOT_FOUND, "Action not found".to_string())),
        Err(e) => {
            error!("Database error approving action {}: {}", req.id, e);
            Err((axum::http::StatusCode::INTERNAL_SERVER_ERROR, "Database error".to_string()))
        }
    }
}

pub async fn dismiss_action_handler(
    State(_hub): State<Arc<Hub>>,
    Json(req): Json<ActionRequest>,
) -> Result<Json<serde_json::Value>, (axum::http::StatusCode, String)> {
    info!("dismiss_action_handler called for id: {}", req.id);
    let pool = crate::db::get_pool();

    let result = sqlx::query("UPDATE agent_actions SET _sync_status = 'DISMISSED', updated_at = CURRENT_TIMESTAMP WHERE id = $1 AND tenant_id = $2")
        .bind(&req.id)
        .bind(&req.tenant_id)
        .execute(&pool)
        .await;

    match result {
        Ok(res) if res.rows_affected() > 0 => Ok(Json(serde_json::json!({ "status": "ok" }))),
        Ok(_) => Err((axum::http::StatusCode::NOT_FOUND, "Action not found".to_string())),
        Err(e) => {
            error!("Database error dismissing action {}: {}", req.id, e);
            Err((axum::http::StatusCode::INTERNAL_SERVER_ERROR, "Database error".to_string()))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hub::Hub;
    use axum::http::Request;
    use axum::body::Body;
    use tower::ServiceExt;

    #[tokio::test]
    async fn test_get_feed_handler() {
        if let Ok(db_url) = std::env::var("OHC_DATABASE_URL") {
            let pool = sqlx::postgres::PgPoolOptions::new()
                .connect_lazy(&db_url)
                .unwrap();
            let (tx, _) = tokio::sync::mpsc::channel(100);
            let hub = Arc::new(Hub::new(tx, pool));
            let app = router::<()>(hub);

            let req = Request::builder()
                .uri("/")
                .method("GET")
                .body(Body::empty())
                .unwrap();

            let res = app.oneshot(req).await.unwrap();
            assert_eq!(res.status(), 200);
        }
    }
}
