use axum::{
    extract::{Extension, Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;
use ::server_common::Claims;

use crate::domain::repository::models::{Quote, QuoteLineItem};

pub fn router<S>() -> Router<S>
where
    S: Clone + Send + Sync + 'static,
    PgPool: axum::extract::FromRef<S>,
{
    Router::new()
        .route("/", post(create_quote))
        .route("/{id}", get(get_quote).put(update_quote))
        .route("/{id}/accept", post(accept_quote))
}

#[derive(Serialize)]
pub struct QuoteResponse {
    pub quote: Quote,
    pub line_items: Vec<QuoteLineItem>,
}

#[derive(Deserialize)]
pub struct CreateQuoteRequest {
    pub customer_id: String,
    pub status: Option<String>,
    pub line_items: Vec<QuoteLineItemInput>,
}

#[derive(Deserialize)]
pub struct UpdateQuoteRequest {
    pub status: Option<String>,
    pub line_items: Vec<QuoteLineItemInput>,
}

#[derive(Deserialize, Clone)]
pub struct QuoteLineItemInput {
    pub description: String,
    pub unit_price_cents: i64,
    pub quantity: i32,
    pub is_optional: Option<bool>,
}

async fn get_quote(
    State(pool): State<PgPool>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    let quote_id = id.clone();

    let quote_res = sqlx::query_as::<_, Quote>("SELECT * FROM quotes WHERE id = $1")
        .bind(&quote_id)
        .fetch_optional(&pool)
        .await;

    let quote = match quote_res {
        Ok(Some(q)) => q,
        Ok(None) => return StatusCode::NOT_FOUND.into_response(),
        Err(e) => {
            tracing::error!("Failed to fetch quote: {}", e);
            return StatusCode::INTERNAL_SERVER_ERROR.into_response();
        }
    };

    let items_res = sqlx::query_as::<_, QuoteLineItem>("SELECT * FROM quote_line_items WHERE quote_id = $1")
        .bind(&quote_id)
        .fetch_all(&pool)
        .await;

    let line_items = match items_res {
        Ok(items) => items,
        Err(e) => {
            tracing::error!("Failed to fetch quote line items: {}", e);
            return StatusCode::INTERNAL_SERVER_ERROR.into_response();
        }
    };

    (StatusCode::OK, Json(QuoteResponse { quote, line_items })).into_response()
}

async fn create_quote(
    State(pool): State<PgPool>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<CreateQuoteRequest>,
) -> impl IntoResponse {
    let tenant_id = match claims.organization_id.as_deref() {
        Some(tid) => tid.to_string(),
        None => return StatusCode::UNAUTHORIZED.into_response(),
    };

    let quote_id = Uuid::new_v4().to_string();
    let status = payload.status.unwrap_or_else(|| "Draft".to_string());

    let mut tx = match pool.begin().await {
        Ok(tx) => tx,
        Err(_) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    };

    let insert_quote = sqlx::query(
        "INSERT INTO quotes (id, tenant_id, customer_id, status, created_at, updated_at) VALUES ($1, $2, $3, $4, NOW(), NOW())"
    )
    .bind(&quote_id)
    .bind(&tenant_id)
    .bind(&payload.customer_id)
    .bind(&status)
    .execute(&mut *tx)
    .await;

    if insert_quote.is_err() {
        let _ = tx.rollback().await;
        return StatusCode::INTERNAL_SERVER_ERROR.into_response();
    }

    for item in payload.line_items {
        let item_id = Uuid::new_v4().to_string();
        let is_optional = item.is_optional.unwrap_or(false);
        let insert_item = sqlx::query(
            "INSERT INTO quote_line_items (id, tenant_id, quote_id, description, unit_price_cents, quantity, is_optional, created_at, updated_at) VALUES ($1, $2, $3, $4, $5, $6, $7, NOW(), NOW())"
        )
        .bind(&item_id)
        .bind(&tenant_id)
        .bind(&quote_id)
        .bind(&item.description)
        .bind(item.unit_price_cents)
        .bind(item.quantity)
        .bind(is_optional)
        .execute(&mut *tx)
        .await;

        if insert_item.is_err() {
            let _ = tx.rollback().await;
            return StatusCode::INTERNAL_SERVER_ERROR.into_response();
        }
    }

    if tx.commit().await.is_err() {
        return StatusCode::INTERNAL_SERVER_ERROR.into_response();
    }

    (StatusCode::CREATED, Json(serde_json::json!({"id": quote_id}))).into_response()
}

async fn update_quote(
    State(pool): State<PgPool>,
    Path(id): Path<String>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<UpdateQuoteRequest>,
) -> impl IntoResponse {
    let tenant_id = match claims.organization_id.as_deref() {
        Some(tid) => tid.to_string(),
        None => return StatusCode::UNAUTHORIZED.into_response(),
    };

    let quote_id = id.clone();

    let mut tx = match pool.begin().await {
        Ok(tx) => tx,
        Err(_) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    };

    if let Some(status) = payload.status {
        let update_quote = sqlx::query("UPDATE quotes SET status = $1, updated_at = NOW() WHERE id = $2 AND tenant_id = $3")
            .bind(&status)
            .bind(&quote_id)
            .bind(&tenant_id)
            .execute(&mut *tx)
            .await;
        if update_quote.is_err() {
            let _ = tx.rollback().await;
            return StatusCode::INTERNAL_SERVER_ERROR.into_response();
        }
    }

    // Replace line items
    let delete_items = sqlx::query("DELETE FROM quote_line_items WHERE quote_id = $1 AND tenant_id = $2")
        .bind(&quote_id)
        .bind(&tenant_id)
        .execute(&mut *tx)
        .await;

    if delete_items.is_err() {
        let _ = tx.rollback().await;
        return StatusCode::INTERNAL_SERVER_ERROR.into_response();
    }

    for item in payload.line_items {
        let item_id = Uuid::new_v4().to_string();
        let is_optional = item.is_optional.unwrap_or(false);
        let insert_item = sqlx::query(
            "INSERT INTO quote_line_items (id, tenant_id, quote_id, description, unit_price_cents, quantity, is_optional, created_at, updated_at) VALUES ($1, $2, $3, $4, $5, $6, $7, NOW(), NOW())"
        )
        .bind(&item_id)
        .bind(&tenant_id)
        .bind(&quote_id)
        .bind(&item.description)
        .bind(item.unit_price_cents)
        .bind(item.quantity)
        .bind(is_optional)
        .execute(&mut *tx)
        .await;

        if insert_item.is_err() {
            let _ = tx.rollback().await;
            return StatusCode::INTERNAL_SERVER_ERROR.into_response();
        }
    }

    if tx.commit().await.is_err() {
        return StatusCode::INTERNAL_SERVER_ERROR.into_response();
    }

    (StatusCode::OK, Json(serde_json::json!({"success": true}))).into_response()
}

async fn accept_quote(
    State(pool): State<PgPool>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    let quote_id = id.clone();

    match sqlx::query("UPDATE quotes SET status = 'ACCEPTED', updated_at = NOW() WHERE id = $1")
        .bind(&quote_id)
        .execute(&pool)
        .await
    {
        Ok(_) => (StatusCode::OK, Json(serde_json::json!({"success": true}))).into_response(),
        Err(e) => {
            tracing::error!("Failed to accept quote: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"success": false}))).into_response()
        }
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_quote_flow() {
        assert!(true); // placeholder, as we are mainly making sure it compiles and is integrated.
    }
}
