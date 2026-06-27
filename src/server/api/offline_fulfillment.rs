use axum::{
    extract::{State, Extension},
    http::StatusCode,
    response::IntoResponse,
    routing::post,
    Json, Router,
};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use std::sync::Arc;
use uuid::Uuid;

// Use Claims since that seems to be the auth model based on lib.rs
use server_common::Claims;

#[derive(Clone)]
pub struct OfflineFulfillmentState {
    pub pool: PgPool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FulfillmentEvent {
    pub order_id: String,
    pub status: String,
    pub crdt_clock: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BatchFulfillmentRequest {
    pub events: Vec<FulfillmentEvent>,
}

pub fn router() -> Router<Arc<OfflineFulfillmentState>> {
    Router::new().route("/api/offline-fulfillment/sync", post(sync_fulfillment_events))
}

async fn sync_fulfillment_events(
    State(state): State<Arc<OfflineFulfillmentState>>,
    Extension(user): Extension<Claims>,
    Json(payload): Json<BatchFulfillmentRequest>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let tenant_id = user.organization_id.clone();

    if tenant_id.is_empty() {
        return Err((StatusCode::UNAUTHORIZED, "Missing tenant ID".to_string()));
    }

    let mut tx = state
        .pool
        .begin()
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    sqlx::query("SET LOCAL app.current_tenant = $1")
        .bind(&tenant_id)
        .execute(&mut *tx)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    for event in payload.events {
        let id = Uuid::new_v4().to_string();

        sqlx::query(
            r#"
            INSERT INTO order_fulfillment_state (id, tenant_id, order_id, status, crdt_clock, updated_at)
            VALUES ($1, $2, $3, $4, $5, CURRENT_TIMESTAMP)
            ON CONFLICT (id) DO UPDATE SET
                status = EXCLUDED.status,
                crdt_clock = EXCLUDED.crdt_clock,
                updated_at = CURRENT_TIMESTAMP
            WHERE order_fulfillment_state.crdt_clock < EXCLUDED.crdt_clock
            "#
        )
        .bind(&id)
        .bind(&tenant_id)
        .bind(&event.order_id)
        .bind(&event.status)
        .bind(event.crdt_clock)
        .execute(&mut *tx)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    }

    tx.commit()
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    // Asynchronously notify Operations Agent of potential conflicts after commit
    let operations_agent = crate::agents::operations_agent::OperationsAgent::new(state.pool.clone());
    let tenant_id_clone = tenant_id.clone();
    let events_clone = payload.events;

    tokio::spawn(async move {
        for event in events_clone {
            if let Err(e) = operations_agent.process_fulfillment_sync(&tenant_id_clone, &event.order_id, &event.status).await {
                tracing::error!("Operations agent failed processing fulfillment sync: {}", e);
            }
        }
    });

    Ok((StatusCode::OK, Json(serde_json::json!({ "status": "synced" }))))
}
