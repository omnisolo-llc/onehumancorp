use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post, put},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

use crate::domain::repository::models::{Quote, QuoteLineItem};

pub fn router<S>() -> Router<S>
where
    S: Clone + Send + Sync + 'static,
    PgPool: axum::extract::FromRef<S>,
{
    Router::new()
        .route("/{id}", get(get_quote))
        .route("/{id}", put(update_quote))
        .route("/{id}/accept", post(accept_quote))
}

#[derive(Serialize)]
pub struct QuoteResponse {
    pub quote: Quote,
    pub line_items: Vec<QuoteLineItem>,
}

#[derive(Deserialize)]
pub struct UpdateQuoteRequest {
    pub line_items: Vec<QuoteLineItemPayload>,
}

#[derive(Deserialize)]
pub struct QuoteLineItemPayload {
    pub description: String,
    pub unit_price_cents: i64,
    pub quantity: i32,
    #[serde(default)]
    pub is_optional: bool,
}

async fn get_quote(
    State(pool): State<PgPool>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    let quote_id = match Uuid::parse_str(&id) {
        Ok(uid) => uid,
        Err(_) => return StatusCode::BAD_REQUEST.into_response(),
    };

    let quote_res = sqlx::query_as::<_, Quote>("SELECT * FROM quotes WHERE id = $1")
        .bind(quote_id)
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
        .bind(quote_id)
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

async fn update_quote(
    State(pool): State<PgPool>,
    Path(id): Path<String>,
    Json(payload): Json<UpdateQuoteRequest>,
) -> impl IntoResponse {
    let quote_id = match Uuid::parse_str(&id) {
        Ok(uid) => uid,
        Err(_) => return StatusCode::BAD_REQUEST.into_response(),
    };

    let mut tx = match pool.begin().await {
        Ok(tx) => tx,
        Err(e) => {
            tracing::error!("Failed to begin transaction: {}", e);
            return StatusCode::INTERNAL_SERVER_ERROR.into_response();
        }
    };

    if let Err(e) = sqlx::query("DELETE FROM quote_line_items WHERE quote_id = $1")
        .bind(quote_id)
        .execute(&mut *tx)
        .await
    {
        tracing::error!("Failed to delete old quote line items: {}", e);
        return StatusCode::INTERNAL_SERVER_ERROR.into_response();
    }

    let mut total_amount_cents = 0;
    for item in &payload.line_items {
        total_amount_cents += item.unit_price_cents * item.quantity as i64;
        let item_id = Uuid::new_v4();
        if let Err(e) = sqlx::query("INSERT INTO quote_line_items (id, quote_id, description, unit_price_cents, quantity, is_optional) VALUES ($1, $2, $3, $4, $5, $6)")
            .bind(item_id)
            .bind(quote_id)
            .bind(&item.description)
            .bind(item.unit_price_cents)
            .bind(item.quantity)
            .bind(item.is_optional)
            .execute(&mut *tx)
            .await
        {
            tracing::error!("Failed to insert new quote line item: {}", e);
            return StatusCode::INTERNAL_SERVER_ERROR.into_response();
        }
    }

    let deposit_amount = (total_amount_cents as f64 * 0.20) as i64;

    if let Err(e) = sqlx::query("UPDATE quotes SET total_amount = $1, required_deposit = $2, updated_at = NOW() WHERE id = $3")
        .bind(total_amount_cents)
        .bind(deposit_amount)
        .bind(quote_id)
        .execute(&mut *tx)
        .await
    {
        tracing::error!("Failed to update quote total: {}", e);
        return StatusCode::INTERNAL_SERVER_ERROR.into_response();
    }

    if let Err(e) = tx.commit().await {
        tracing::error!("Failed to commit transaction: {}", e);
        return StatusCode::INTERNAL_SERVER_ERROR.into_response();
    }

    // Fetch the updated quote to return
    let quote_res = sqlx::query_as::<_, Quote>("SELECT * FROM quotes WHERE id = $1")
        .bind(quote_id)
        .fetch_optional(&pool)
        .await;

    let quote = match quote_res {
        Ok(Some(q)) => q,
        _ => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    };

    let line_items = sqlx::query_as::<_, QuoteLineItem>("SELECT * FROM quote_line_items WHERE quote_id = $1")
        .bind(quote_id)
        .fetch_all(&pool)
        .await
        .unwrap_or_default();

    (StatusCode::OK, Json(QuoteResponse { quote, line_items })).into_response()
}

async fn accept_quote(
    State(pool): State<PgPool>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    let quote_id = match Uuid::parse_str(&id) {
        Ok(uid) => uid,
        Err(_) => return StatusCode::BAD_REQUEST.into_response(),
    };

    match sqlx::query("UPDATE quotes SET status = 'ACCEPTED', updated_at = NOW() WHERE id = $1")
        .bind(quote_id)
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
