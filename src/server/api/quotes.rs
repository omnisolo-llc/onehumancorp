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
        .route("/:id", get(get_quote))
        .route("/:id/accept", post(accept_quote))
}

#[derive(Serialize)]
pub struct QuoteResponse {
    pub quote: Quote,
    pub line_items: Vec<QuoteLineItem>,
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
