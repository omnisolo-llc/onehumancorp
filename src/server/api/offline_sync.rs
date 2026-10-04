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
    store: &server_auth::Store,
    headers: &axum::http::HeaderMap,
) -> Result<(server_common::Claims, String), axum::response::Response> {
    let mut values = headers.get_all(axum::http::header::AUTHORIZATION).iter();
    let token = values
        .next()
        .filter(|_| values.next().is_none())
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.strip_prefix("Bearer "));
    let Some(token) = token.filter(|t| {
        !t.is_empty()
            && t.len() <= server_auth::MAX_ACCESS_TOKEN_BYTES
            && !t.chars().any(|c| c.is_whitespace() || c.is_ascii_control())
    }) else {
        return Err((StatusCode::UNAUTHORIZED, "Unauthorized").into_response());
    };
    let claims = store
        .validate_token(token)
        .await
        .map_err(|_| (StatusCode::UNAUTHORIZED, "Unauthorized").into_response())?;
    let tenant = server_common::auth_utils::signed_tenant_id(&claims)
        .filter(|tenant| claims.organization_id.as_deref() == Some(tenant.as_str()))
        .ok_or_else(|| {
            (
                StatusCode::UNAUTHORIZED,
                "Tenant-scoped authentication required",
            )
                .into_response()
        })?;
    Ok((claims, tenant))
}

#[derive(Clone)]
pub struct SyncWriteState {
    pub mutations: crate::api::field_ops::records::FieldAccess,
    pub intents: crate::api::field_ops::records::FieldAccess,
}
impl SyncWriteState {
    pub async fn new(store: Arc<server_auth::Store>, configured: Option<&sqlx::PgPool>) -> Self {
        async fn access(
            store: Arc<server_auth::Store>,
            configured: Option<&sqlx::PgPool>,
            relations: &[&str],
        ) -> crate::api::field_ops::records::FieldAccess {
            let pool =
                if let (Some(repository), Some(configured)) = (store.portable_repo(), configured) {
                    server_auth::commit_authority::canonical_pg_data_pool(
                        repository.connection(),
                        configured,
                        relations,
                    )
                    .await
                    .ok()
                } else {
                    None
                };
            crate::api::field_ops::records::FieldAccess { pool, store }
        }
        Self {
            mutations: access(
                store.clone(),
                configured,
                &[
                    "sync_events",
                    "applied_client_mutations",
                    "department_tasks",
                    "ohc_job_queue",
                    "products",
                    "inventory_levels",
                ],
            )
            .await,
            intents: access(store, configured, &["sync_events", "operation_intents"]).await,
        }
    }
}

pub async fn offline_sync_handler(
    State((state, mesh)): State<(
        SyncWriteState,
        Arc<dyn omnisolo_builtin_agent::mesh::transport::MeshTransport>,
    )>,
    headers: axum::http::HeaderMap,
    Json(payload): Json<OfflineSyncRequest>,
) -> impl IntoResponse {
    let (claims, tenant_id) =
        match validate_token_and_get_tenant(&state.mutations.store, &headers).await {
            Ok(t) => t,
            Err(e) => return e,
        };
    let result = durable_sync::sync_authorized_mutations(
        &state.mutations,
        &claims,
        &headers,
        &payload.mutations,
    )
    .await;
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

#[derive(Clone)]
pub struct SyncEventsState {
    pub pool: sqlx::PgPool,
    pub access: crate::api::field_ops::records::FieldAccess,
}
impl SyncEventsState {
    /// Keep the configured pool for existing non-appointment operations. Only
    /// the proven canonical field and receipt objects grant appointment writes.
    pub async fn new(
        pool: sqlx::PgPool,
        field_pool: Option<sqlx::PgPool>,
        store: Arc<server_auth::Store>,
    ) -> Self {
        let canonical = if let (Some(field), Some(repo)) = (field_pool, store.portable_repo()) {
            if server_auth::commit_authority::CanonicalPgAuthority::bind(store.clone(), &field)
                .is_ok()
            {
                server_auth::commit_authority::canonical_pg_data_pool(
                    repo.connection(),
                    &pool,
                    &[
                        "appointments",
                        "department_tasks",
                        "sync_events",
                        "sync_conflict_queue",
                    ],
                )
                .await
                .ok()
            } else {
                None
            }
        } else {
            None
        };
        Self {
            pool,
            access: crate::api::field_ops::records::FieldAccess {
                pool: canonical,
                store,
            },
        }
    }
}

pub async fn sync_events_handler(
    State(state): State<SyncEventsState>,
    headers: axum::http::HeaderMap,
    Json(payload): Json<SyncEventsRequest>,
) -> impl IntoResponse {
    let (claims, tenant_id) =
        match validate_token_and_get_tenant(&state.access.store, &headers).await {
            Ok(identity) => identity,
            Err(error) => return error,
        };
    let result = durable_sync::sync_authorized_events(
        &state,
        &claims,
        &headers,
        &tenant_id,
        &payload.events,
    )
    .await;
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
    State(state): State<SyncWriteState>,
    headers: axum::http::HeaderMap,
    Json(payload): Json<OperationIntentRequest>,
) -> impl IntoResponse {
    let (claims, _) = match validate_token_and_get_tenant(&state.intents.store, &headers).await {
        Ok(t) => t,
        Err(e) => return e,
    };
    let result =
        durable_sync::sync_authorized_intents(&state.intents, &claims, &headers, &payload.intents)
            .await;
    (StatusCode::OK, Json(result)).into_response()
}
