use axum::{
    extract::{Path, State},
    routing::{get, patch, post},
    Json, Router,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool};
use uuid::Uuid;

pub fn router(pool: PgPool) -> Router {
    Router::new()
        .route("/quotes", post(create_quote))
        .route("/quotes/:id", get(get_quote))
        .route("/quotes/:id/approve", patch(approve_quote))
        .with_state(pool)
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Quote {
    pub id: Uuid,
    pub tenant_id: String,
    pub customer_id: Uuid,
    pub status: String,
    pub valid_until: Option<DateTime<Utc>>,
    pub checkout_url: Option<String>,
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

async fn create_quote(
    State(pool): State<PgPool>,
    Json(payload): Json<CreateQuoteReq>,
) -> Result<Json<Quote>, axum::http::StatusCode> {
    // Basic implementation
    let quote_id = Uuid::new_v4();
    let tenant_id = "test_tenant".to_string(); // In reality, get from context

    let mut tx = pool
        .begin()
        .await
        .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;

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

    tx.commit()
        .await
        .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;

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

async fn approve_quote(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
) -> Result<Json<Quote>, axum::http::StatusCode> {
    let payment_intent_id = format!("pi_test_{}", Uuid::new_v4().to_string().replace("-", ""));
    let session_id = Uuid::new_v4().to_string();
    let checkout_url = format!(
        "https://checkout.stripe.com/pay/cs_test_{}",
        session_id.replace("-", "")
    );

    let mut tx = pool
        .begin()
        .await
        .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;

    let quote = sqlx::query_as::<_, Quote>(
        "UPDATE quotes SET status = 'SENT', checkout_url = $2, updated_at = NOW() WHERE id = $1 RETURNING *"
    )
    .bind(id)
    .bind(&checkout_url)
    .fetch_optional(&mut *tx)
    .await
    .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;

    if let Some(ref q) = quote {
        // Create a pending_payment booking linked to the quote
        let booking_id = Uuid::new_v4().to_string();
        let product_id = "quote_service"; // Placeholder product ID for quote-based bookings
        let start_time = Utc::now() + chrono::Duration::days(1);
        let end_time = start_time + chrono::Duration::hours(2);

        let result = sqlx::query(
            "INSERT INTO bookings (id, tenant_id, customer_id, product_id, start_time, end_time, status, payment_intent_id, quote_id) \
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)"
        )
        .bind(&booking_id)
        .bind(&q.tenant_id)
        .bind(q.customer_id.to_string())
        .bind(product_id)
        .bind(start_time)
        .bind(end_time)
        .bind("pending_payment")
        .bind(&payment_intent_id)
        .bind(q.id.to_string())
        .execute(&mut *tx)
        .await;

        if result.is_err() {
            let _ = tx.rollback().await;
            return Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR);
        }
    }

    tx.commit()
        .await
        .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;

    match quote {
        Some(q) => Ok(Json(q)),
        None => Err(axum::http::StatusCode::NOT_FOUND),
    }
}
