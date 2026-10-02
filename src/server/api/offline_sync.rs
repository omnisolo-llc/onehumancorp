use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[path = "durable_sync.rs"]
pub(super) mod durable_sync;

#[derive(Deserialize, Debug, Clone, Serialize)]
pub struct OfflineMutation {
    pub transaction_id: String,
    pub timestamp: Option<String>,
    pub product_id: String,
    pub quantity_deducted: i32,
    pub amount: Option<i64>, // amount in cents
    pub payment_method: Option<String>,
    pub payment_intent_id: Option<String>,
    pub currency: Option<String>,
    pub mutation_type: Option<String>,
    pub payload: Option<String>,
    pub client_mutation_id: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct OfflineSyncRequest {
    pub mutations: Vec<OfflineMutation>,
}

#[derive(Serialize)]
pub struct OfflineSyncResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pending_reconciliation: Option<Vec<serde_json::Value>>,
    pub success: bool,
    pub failed_count: i32,
}

async fn validate_token_and_get_tenant(
    pool: &sqlx::PgPool,
    headers: &axum::http::HeaderMap,
) -> Result<(String, String), axum::response::Response> {
    let auth_header = headers.get("authorization").and_then(|h| h.to_str().ok());
    let token = match auth_header {
        Some(h) if h.to_lowercase().starts_with("bearer ") => &h[7..],
        _ => return Err((axum::http::StatusCode::UNAUTHORIZED, "Unauthorized").into_response()),
    };

    let repo = std::sync::Arc::new(crate::auth::postgres_store::PgUserRepository::new(
        pool.clone(),
    ));
    let store = std::sync::Arc::new(crate::auth::Store::with_repo(repo));

    let claims = match store.validate_token(token).await {
        Ok(c) => c,
        Err(_) => {
            return Err((axum::http::StatusCode::UNAUTHORIZED, "Unauthorized").into_response());
        }
    };

    let tenant_id = match claims.organization_id {
        Some(id)
            if !id.trim().is_empty() && id.trim() == id && !id.eq_ignore_ascii_case("system") =>
        {
            id
        }
        _ => {
            return Err((
                StatusCode::UNAUTHORIZED,
                "Tenant-scoped authentication required",
            )
                .into_response());
        }
    };
    let agent_id = claims.sub;

    Ok((tenant_id, agent_id))
}

pub async fn offline_sync_handler(
    State((db, mesh)): State<(
        sqlx::PgPool,
        Arc<dyn omnisolo_builtin_agent::mesh::transport::MeshTransport>,
    )>,
    headers: axum::http::HeaderMap,
    Json(payload): Json<OfflineSyncRequest>,
) -> impl IntoResponse {
    let (tenant_id, _) = match validate_token_and_get_tenant(&db, &headers).await {
        Ok(t) => t,
        Err(e) => return e,
    };
    let result = durable_sync::sync_mutations(&db, &tenant_id, &payload.mutations).await;
    invalidate_committed_products(&tenant_id, &result.committed_products).await;
    for product in &result.committed_products {
        let event = ::server_omnisolo::orchestration::TeammateMeshEvent {
            action: "InventoryUpdated".into(),
            agent_id: "system".into(),
            status: String::new(),
            msg_id: uuid::Uuid::new_v4().to_string(),
            payload: serde_json::json!({"product_id":product,"tenant_id":tenant_id})
                .to_string()
                .into_bytes(),
        };
        let _ = mesh.publish("mesh:inventory:updated", event).await;
    }
    (StatusCode::OK, Json(result)).into_response()
}

async fn invalidate_committed_products(tenant: &str, products: &[String]) {
    if products.is_empty() {
        return;
    }
    let cache = crate::builder::edge::get_edge_cache();
    let cdn = crate::utils::edge_caching_middleware::get_cdn_cache();
    cdn.invalidate_by_tag(&format!("tenant-id:{tenant}")).await;
    cache
        .invalidate_by_tag(&format!("tenant-id:{tenant}"))
        .await;
    for product in products {
        cache
            .invalidate_by_tag(&format!("entity:product:{product}"))
            .await;
        cdn.invalidate_by_tag(&format!("entity:product:{product}"))
            .await;
    }
    if let Some(client) = crate::get_redis_client()
        && let Ok(mut conn) = client.get_multiplexed_async_connection().await
    {
        for product in products {
            let payload = serde_json::json!({"event":"inventory.updated","tags":[format!("tenant-id:{tenant}"),format!("entity:product:{product}")]}).to_string();
            let _: Result<(), _> = redis::cmd("PUBLISH")
                .arg("cache_invalidation_events")
                .arg(&payload)
                .query_async(&mut conn)
                .await;
            let _: Result<(), _> = redis::cmd("PUBLISH")
                .arg(format!("inventory:{tenant}"))
                .arg(&payload)
                .query_async(&mut conn)
                .await;
        }
    }
}

#[derive(Deserialize, Debug, Clone, Serialize)]
pub struct SyncEvent {
    pub id: String,
    pub entity_id: String,
    pub entity_type: String,
    pub action_type: String,
    pub payload: serde_json::Value,
    pub base_version: i64,
}

#[derive(Deserialize, Debug, Clone)]
pub struct SyncEventsRequest {
    pub events: Vec<SyncEvent>,
}

#[derive(Serialize)]
pub struct SyncEventsResponse {
    pub success: bool,
    pub applied_count: i32,
    pub conflict_count: i32,
}

pub async fn sync_events_handler(
    State(db): State<sqlx::PgPool>,
    headers: axum::http::HeaderMap,
    Json(payload): Json<SyncEventsRequest>,
) -> impl IntoResponse {
    let (tenant_id, _) = match validate_token_and_get_tenant(&db, &headers).await {
        Ok(t) => t,
        Err(e) => return e,
    };
    let result = durable_sync::sync_events(&db, &tenant_id, &payload.events).await;
    invalidate_committed_products(&tenant_id, &result.committed_products).await;
    (StatusCode::OK, Json(result)).into_response()
}

#[cfg(test)]
mod tests {
    include!("offline_sync_route_test.rs");
}

#[derive(serde::Deserialize, Debug, Clone, serde::Serialize)]
pub struct OperationIntent {
    pub id: String,
    pub action_type: String,
    pub payload: serde_json::Value,
    pub timestamp: Option<String>,
}

#[derive(serde::Deserialize, Debug)]
pub struct OperationIntentRequest {
    pub intents: Vec<OperationIntent>,
}

#[derive(serde::Serialize)]
pub struct OperationIntentResponse {
    pub success: bool,
    pub applied_count: i32,
    pub conflict_count: i32,
    pub failed_count: i32,
}

pub async fn operation_intents_handler(
    State(db): State<sqlx::PgPool>,
    headers: axum::http::HeaderMap,
    Json(payload): Json<OperationIntentRequest>,
) -> impl IntoResponse {
    let (tenant_id, _) = match validate_token_and_get_tenant(&db, &headers).await {
        Ok(t) => t,
        Err(e) => return e,
    };
    let result = durable_sync::sync_intents(&db, &tenant_id, &payload.intents).await;
    (StatusCode::OK, Json(result)).into_response()
}
