use sqlx::{PgPool, FromRow};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use axum::{
    extract::{State, Path},
    routing::{get, post, patch},
    Json, Router,
};
use serde::{Deserialize, Serialize};

pub fn router(pool: PgPool) -> Router {
    Router::new()
        .route("/quotes", post(create_quote))
        .route("/quotes/:id", get(get_quote))
        .route("/quotes/:id/items", get(get_quote_items).post(add_quote_item))
        .route("/quotes/:id/items/:item_id", patch(patch_quote_item))
        .route("/quotes/:id/approve", patch(approve_quote))
        .route("/quotes/:id/public", get(get_quote_public))
        .with_state(pool)
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Quote {
    pub id: Uuid,
    pub tenant_id: String,
    pub customer_id: Uuid,
    pub status: String,
    pub valid_until: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct QuoteLineItem {
    pub id: Uuid,
    pub quote_id: Uuid,
    pub description: String,
    pub unit_price_cents: i64,
    pub quantity: i32,
    pub is_optional: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateQuoteReq {
    pub customer_id: Uuid,
    pub status: String,
    pub line_items: Vec<QuoteLineItemReq>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QuoteLineItemReq {
    pub description: String,
    pub unit_price_cents: i64,
    pub quantity: i32,
    pub is_optional: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateQuoteItemReq {
    pub unit_price_cents: Option<i64>,
    pub is_optional: Option<bool>,
}

async fn create_quote(
    State(pool): State<PgPool>,
    Json(payload): Json<CreateQuoteReq>,
) -> Result<Json<Quote>, axum::http::StatusCode> {
    let quote_id = Uuid::new_v4();
    let tenant_id = "test_tenant".to_string(); // In reality, get from context

    let mut tx = pool.begin().await.map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;

    let quote = sqlx::query_as::<_, Quote>(
        "INSERT INTO quotes (id, tenant_id, customer_id, status) VALUES ($1, $2, $3, $4) RETURNING *"
    )
    .bind(quote_id)
    .bind(&tenant_id)
    .bind(payload.customer_id)
    .bind(&payload.status)
    .fetch_one(&mut *tx)
    .await
    .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;

    for item in payload.line_items {
        sqlx::query(
            "INSERT INTO quote_line_items (id, quote_id, description, unit_price_cents, quantity, is_optional) VALUES ($1, $2, $3, $4, $5, $6)"
        )
        .bind(Uuid::new_v4())
        .bind(quote_id)
        .bind(item.description)
        .bind(item.unit_price_cents)
        .bind(item.quantity)
        .bind(item.is_optional)
        .execute(&mut *tx)
        .await
        .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;
    }

    tx.commit().await.map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(quote))
}

async fn get_quote(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
) -> Result<Json<Quote>, axum::http::StatusCode> {
    let quote = sqlx::query_as::<_, Quote>("SELECT * FROM quotes WHERE id = $1")
        .bind(id)
        .fetch_optional(&pool)
        .await
        .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;

    match quote {
        Some(q) => Ok(Json(q)),
        None => Err(axum::http::StatusCode::NOT_FOUND),
    }
}

async fn get_quote_items(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
) -> Result<Json<Vec<QuoteLineItem>>, axum::http::StatusCode> {
    let items = sqlx::query_as::<_, QuoteLineItem>("SELECT * FROM quote_line_items WHERE quote_id = $1")
        .bind(id)
        .fetch_all(&pool)
        .await
        .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(items))
}

async fn add_quote_item(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
    Json(payload): Json<QuoteLineItemReq>,
) -> Result<Json<QuoteLineItem>, axum::http::StatusCode> {
    let item = sqlx::query_as::<_, QuoteLineItem>(
        "INSERT INTO quote_line_items (id, quote_id, description, unit_price_cents, quantity, is_optional) VALUES ($1, $2, $3, $4, $5, $6) RETURNING *"
    )
    .bind(Uuid::new_v4())
    .bind(id)
    .bind(payload.description)
    .bind(payload.unit_price_cents)
    .bind(payload.quantity)
    .bind(payload.is_optional)
    .fetch_one(&pool)
    .await
    .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(item))
}

async fn patch_quote_item(
    State(pool): State<PgPool>,
    Path((_id, item_id)): Path<(Uuid, Uuid)>,
    Json(payload): Json<UpdateQuoteItemReq>,
) -> Result<Json<QuoteLineItem>, axum::http::StatusCode> {
    let mut item = sqlx::query_as::<_, QuoteLineItem>("SELECT * FROM quote_line_items WHERE id = $1")
        .bind(item_id)
        .fetch_optional(&pool)
        .await
        .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(axum::http::StatusCode::NOT_FOUND)?;

    if let Some(price) = payload.unit_price_cents {
        item.unit_price_cents = price;
    }
    if let Some(optional) = payload.is_optional {
        item.is_optional = optional;
    }

    let updated = sqlx::query_as::<_, QuoteLineItem>(
        "UPDATE quote_line_items SET unit_price_cents = $1, is_optional = $2, updated_at = NOW() WHERE id = $3 RETURNING *"
    )
    .bind(item.unit_price_cents)
    .bind(item.is_optional)
    .bind(item_id)
    .fetch_one(&pool)
    .await
    .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(updated))
}

async fn approve_quote(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
) -> Result<Json<Quote>, axum::http::StatusCode> {
    let current_quote = sqlx::query_as::<_, Quote>("SELECT * FROM quotes WHERE id = $1")
        .bind(id)
        .fetch_optional(&pool)
        .await
        .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(axum::http::StatusCode::NOT_FOUND)?;

    let next_status = match current_quote.status.as_str() {
        "DRAFT" | "PENDING_APPROVAL" => "SENT",
        "SENT" => "ACCEPTED",
        _ => return Err(axum::http::StatusCode::BAD_REQUEST),
    };

    let quote = sqlx::query_as::<_, Quote>(
        "UPDATE quotes SET status = $1, updated_at = NOW() WHERE id = $2 RETURNING *"
    )
    .bind(next_status)
    .bind(id)
    .fetch_one(&pool)
    .await
    .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;

    // Integrate Stripe deposit logic here if next_status is ACCEPTED...

    Ok(Json(quote))
}

async fn get_quote_public(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
) -> Result<Json<serde_json::Value>, axum::http::StatusCode> {
    let quote = sqlx::query_as::<_, Quote>("SELECT * FROM quotes WHERE id = $1")
        .bind(id)
        .fetch_optional(&pool)
        .await
        .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(axum::http::StatusCode::NOT_FOUND)?;

    let items = sqlx::query_as::<_, QuoteLineItem>("SELECT * FROM quote_line_items WHERE quote_id = $1")
        .bind(id)
        .fetch_all(&pool)
        .await
        .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(serde_json::json!({
        "quote": quote,
        "items": items
    })))
}

#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::PgPool;

    async fn setup_db() -> PgPool {
        let db_url = std::env::var("OHC_DATABASE_URL").unwrap_or_else(|_| "postgres://postgres:postgres@localhost:5432/test".to_string());
        PgPool::connect(&db_url).await.unwrap()
    }

    #[tokio::test]
    async fn test_quote_status_transitions() {
        if std::env::var("OHC_DATABASE_URL").is_err() { return; }
        let pool = setup_db().await;

        let customer_id = Uuid::new_v4();
        let quote_id = Uuid::new_v4();

        // Seed a draft quote
        sqlx::query("INSERT INTO quotes (id, tenant_id, customer_id, status) VALUES ($1, 'test_tenant', $2, 'DRAFT')")
            .bind(quote_id)
            .bind(customer_id)
            .execute(&pool)
            .await
            .unwrap();

        // 1. Approve DRAFT -> SENT
        let res = approve_quote(State(pool.clone()), Path(quote_id)).await.unwrap();
        assert_eq!(res.0.status, "SENT");

        // 2. Approve SENT -> ACCEPTED
        let res = approve_quote(State(pool.clone()), Path(quote_id)).await.unwrap();
        assert_eq!(res.0.status, "ACCEPTED");
    }
}
