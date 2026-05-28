use axum::{
    extract::{State, Query, Extension},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;
use crate::auth::User;

#[derive(Clone)]
pub struct AppState {
    pub pool: PgPool,
}

#[derive(Debug, Deserialize)]
pub struct CreateLocationRequest {
    pub r#type: String,
    pub geo_location: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct CreateLocationResponse {
    pub success: bool,
    pub node_id: String,
}

pub async fn handle_create_location(
    State(state): State<AppState>,
    Extension(user): Extension<User>,
    Json(payload): Json<CreateLocationRequest>,
) -> impl IntoResponse {
    let node_id = Uuid::new_v4().to_string();
    let tenant_id = user.organization_id.clone().unwrap_or_else(|| "default_tenant".to_string());
    let loc_type = payload.r#type;
    let geo = payload.geo_location;

    let mut tx = match state.pool.begin().await {
        Ok(tx) => tx,
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to begin transaction").into_response(),
    };

    if let Err(e) = sqlx::query("SELECT set_config('app.current_tenant', $1, true)").bind(&tenant_id)
        .execute(&mut *tx)
        .await
    {
        return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to set tenant context").into_response();
    }

    // 1. Create Location Node
    let result = sqlx::query!(
        "INSERT INTO location_nodes (node_id, tenant_id, type, geo_location, is_active) VALUES ($1, $2, $3, $4, true)",
        node_id, tenant_id, loc_type, geo
    )
    .execute(&mut *tx)
    .await;

    if result.is_err() {
        return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to create location node").into_response();
    }

    // 2. Duplicate catalog for the new location into inventory_ledgers
    // Get all products for the tenant
    let products = sqlx::query!(
        "SELECT id FROM products WHERE tenant_id = $1",
        tenant_id
    )
    .fetch_all(&mut *tx)
    .await;

    if let Ok(products) = products {
        for product in products {
            if let Err(e) = sqlx::query!(
                "INSERT INTO inventory_ledgers (node_id, tenant_id, product_id, available_qty, reserved_qty) VALUES ($1, $2, $3, 0, 0)",
                node_id, tenant_id, product.id
            )
            .execute(&mut *tx)
            .await {
                return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to duplicate catalog").into_response();
            }
        }
    } else {
        return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to fetch products").into_response();
    }

    if let Err(e) = tx.commit().await {
        return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to commit transaction").into_response();
    }

    (
        StatusCode::OK,
        Json(CreateLocationResponse {
            success: true,
            node_id,
        }),
    )
        .into_response()
}

#[derive(Debug, Deserialize)]
pub struct GetInventoryQuery {
    pub node_id: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct InventoryItem {
    pub product_id: String,
    pub available_qty: i32,
}

#[derive(Debug, Serialize)]
pub struct GetInventoryResponse {
    pub items: Vec<InventoryItem>,
}

pub async fn handle_get_inventory(
    State(state): State<AppState>,
    Extension(user): Extension<User>,
    Query(query): Query<GetInventoryQuery>,
) -> impl IntoResponse {
    let tenant_id = user.organization_id.clone().unwrap_or_else(|| "default_tenant".to_string());

    let mut tx = match state.pool.begin().await {
        Ok(tx) => tx,
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to begin transaction").into_response(),
    };

    if let Err(e) = sqlx::query("SELECT set_config('app.current_tenant', $1, true)").bind(&tenant_id)
        .execute(&mut *tx)
        .await
    {
        return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to set tenant context").into_response();
    }

    let items = if let Some(node_id) = query.node_id {
        // Location-specific inventory
        let records = sqlx::query!(
            "SELECT product_id, available_qty FROM inventory_ledgers WHERE tenant_id = $1 AND node_id = $2",
            tenant_id, node_id
        )
        .fetch_all(&mut *tx)
        .await;

        match records {
            Ok(recs) => recs.into_iter().map(|r| InventoryItem {
                product_id: r.product_id,
                available_qty: r.available_qty.unwrap_or(0),
            }).collect(),
            Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to query inventory").into_response(),
        }
    } else {
        // Aggregate inventory
        let records = sqlx::query!(
            "SELECT product_id, SUM(available_qty) as total_qty FROM inventory_ledgers WHERE tenant_id = $1 GROUP BY product_id",
            tenant_id
        )
        .fetch_all(&mut *tx)
        .await;

        match records {
            Ok(recs) => recs.into_iter().map(|r| InventoryItem {
                product_id: r.product_id,
                available_qty: r.total_qty.unwrap_or(0) as i32,
            }).collect(),
            Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to query aggregate inventory").into_response(),
        }
    };

    let _ = tx.commit().await;

    (
        StatusCode::OK,
        Json(GetInventoryResponse { items }),
    )
        .into_response()
}

#[derive(Debug, Deserialize)]
pub struct OfflineSaleItem {
    pub product_id: String,
    pub qty: i32,
}

#[derive(Debug, Deserialize)]
pub struct SyncOfflineSalesRequest {
    pub node_id: String,
    pub sales: Vec<OfflineSaleItem>,
}

#[derive(Debug, Serialize)]
pub struct SyncOfflineSalesResponse {
    pub success: bool,
}

pub async fn handle_sync_offline_sales(
    State(state): State<AppState>,
    Extension(user): Extension<User>,
    Json(payload): Json<SyncOfflineSalesRequest>,
) -> impl IntoResponse {
    let tenant_id = user.organization_id.clone().unwrap_or_else(|| "default_tenant".to_string());
    let node_id = payload.node_id;

    let mut tx = match state.pool.begin().await {
        Ok(tx) => tx,
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to begin transaction").into_response(),
    };

    if let Err(e) = sqlx::query("SELECT set_config('app.current_tenant', $1, true)").bind(&tenant_id)
        .execute(&mut *tx)
        .await
    {
        return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to set tenant context").into_response();
    }

    for sale in payload.sales {
        if let Err(e) = sqlx::query!(
            "UPDATE inventory_ledgers SET available_qty = available_qty - $1 WHERE tenant_id = $2 AND node_id = $3 AND product_id = $4",
            sale.qty, tenant_id, node_id, sale.product_id
        )
        .execute(&mut *tx)
        .await {
            return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to decrement inventory for offline sale").into_response();
        }
    }

    if let Err(e) = tx.commit().await {
        return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to commit offline sales sync").into_response();
    }

    (
        StatusCode::OK,
        Json(SyncOfflineSalesResponse { success: true }),
    )
        .into_response()
}

pub fn router<S>(pool: PgPool) -> Router<S>
where
    S: Clone + Send + Sync + 'static,
{
    let state = AppState { pool };

    Router::new()
        .route("/create", post(handle_create_location))
        .route("/inventory", get(handle_get_inventory))
        .route("/sync_offline_sales", post(handle_sync_offline_sales))
        .with_state(state)
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::http::{Request, StatusCode};
    use tower::ServiceExt;
    use sqlx::postgres::PgPoolOptions;
    use std::env;

    // A placeholder test that can be expanded later
    #[tokio::test]
    async fn test_create_location_response_structure() {
        // Just checking basic compilation of the test module for now
        assert_eq!(true, true);
    }
}
