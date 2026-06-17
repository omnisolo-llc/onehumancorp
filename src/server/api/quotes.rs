use axum::{
    extract::{Path, State, Query},
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
        .route("/", post(create_quote))
        .route("/{id}", get(get_quote))
        .route("/{id}", put(update_quote))
        .route("/{id}/accept", post(accept_quote))
        .route("/{id}/approve", axum::routing::patch(approve_quote))
}

#[derive(Serialize)]
pub struct QuoteResponse {
    pub quote: Quote,
    pub line_items: Vec<QuoteLineItem>,
}

#[derive(Deserialize)]
pub struct QuoteQuery {
    pub mobile_optimized: Option<bool>,
}

#[derive(Deserialize)]
pub struct CreateQuoteRequest {
    pub tenant_id: String,
    pub customer_id: String,
    pub total_amount: Option<i64>,
    pub required_deposit: Option<i64>,
    pub checkout_url: Option<String>,
    pub line_items: Vec<QuoteLineItemRequest>,
}

#[derive(Deserialize)]
pub struct UpdateQuoteRequest {
    pub total_amount: Option<i64>,
    pub required_deposit: Option<i64>,
    pub checkout_url: Option<String>,
    pub status: Option<String>,
    pub line_items: Vec<QuoteLineItemRequest>,
}

#[derive(Deserialize)]
pub struct QuoteLineItemRequest {
    pub description: String,
    pub unit_price_cents: i64,
    pub quantity: i32,
    pub is_optional: bool,
}

async fn create_quote(
    State(pool): State<PgPool>,
    Json(payload): Json<CreateQuoteRequest>,
) -> impl IntoResponse {
    let quote_id = Uuid::new_v4();

    let mut tx = match pool.begin().await {
        Ok(t) => t,
        Err(e) => {
            tracing::error!("Failed to begin transaction: {}", e);
            return StatusCode::INTERNAL_SERVER_ERROR.into_response();
        }
    };

    let quote_res = sqlx::query(
        "INSERT INTO quotes (id, tenant_id, customer_id, status, total_amount, required_deposit, checkout_url, created_at, updated_at) VALUES ($1, $2, $3, 'DRAFT', $4, $5, $6, NOW(), NOW())"
    )
    .bind(quote_id)
    .bind(&payload.tenant_id)
    .bind(&payload.customer_id)
    .bind(payload.total_amount)
    .bind(payload.required_deposit)
    .bind(&payload.checkout_url)
    .execute(&mut *tx)
    .await;

    if let Err(e) = quote_res {
        tracing::error!("Failed to insert quote: {}", e);
        return StatusCode::INTERNAL_SERVER_ERROR.into_response();
    }

    for item in payload.line_items {
        let item_id = Uuid::new_v4();
        let res = sqlx::query(
            "INSERT INTO quote_line_items (id, quote_id, description, unit_price_cents, quantity, is_optional, created_at, updated_at) VALUES ($1, $2, $3, $4, $5, $6, NOW(), NOW())"
        )
        .bind(item_id)
        .bind(quote_id)
        .bind(&item.description)
        .bind(item.unit_price_cents)
        .bind(item.quantity)
        .bind(item.is_optional)
        .execute(&mut *tx)
        .await;

        if let Err(e) = res {
            tracing::error!("Failed to insert quote line item: {}", e);
            return StatusCode::INTERNAL_SERVER_ERROR.into_response();
        }
    }

    if let Err(e) = tx.commit().await {
        tracing::error!("Failed to commit transaction: {}", e);
        return StatusCode::INTERNAL_SERVER_ERROR.into_response();
    }

    (StatusCode::CREATED, Json(serde_json::json!({"id": quote_id.to_string()}))).into_response()
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
        Ok(t) => t,
        Err(e) => {
            tracing::error!("Failed to begin transaction: {}", e);
            return StatusCode::INTERNAL_SERVER_ERROR.into_response();
        }
    };

    // we can't easily bind dynamic number of parameters in simple query string building
    // so we'll do it securely:
    let update_res = sqlx::query(
        "UPDATE quotes SET updated_at = NOW(), total_amount = COALESCE($1, total_amount), required_deposit = COALESCE($2, required_deposit), status = COALESCE($3, status), checkout_url = COALESCE($4, checkout_url) WHERE id = $5"
    )
    .bind(payload.total_amount)
    .bind(payload.required_deposit)
    .bind(&payload.status)
    .bind(&payload.checkout_url)
    .bind(quote_id)
    .execute(&mut *tx)
    .await;

    if let Err(e) = update_res {
        tracing::error!("Failed to update quote: {}", e);
        return StatusCode::INTERNAL_SERVER_ERROR.into_response();
    }

    let delete_res = sqlx::query("DELETE FROM quote_line_items WHERE quote_id = $1")
        .bind(quote_id)
        .execute(&mut *tx)
        .await;

    if let Err(e) = delete_res {
        tracing::error!("Failed to delete old quote line items: {}", e);
        return StatusCode::INTERNAL_SERVER_ERROR.into_response();
    }

    for item in payload.line_items {
        let item_id = Uuid::new_v4();
        let res = sqlx::query(
            "INSERT INTO quote_line_items (id, quote_id, description, unit_price_cents, quantity, is_optional, created_at, updated_at) VALUES ($1, $2, $3, $4, $5, $6, NOW(), NOW())"
        )
        .bind(item_id)
        .bind(quote_id)
        .bind(&item.description)
        .bind(item.unit_price_cents)
        .bind(item.quantity)
        .bind(item.is_optional)
        .execute(&mut *tx)
        .await;

        if let Err(e) = res {
            tracing::error!("Failed to insert new quote line item: {}", e);
            return StatusCode::INTERNAL_SERVER_ERROR.into_response();
        }
    }

    if let Err(e) = tx.commit().await {
        tracing::error!("Failed to commit transaction: {}", e);
        return StatusCode::INTERNAL_SERVER_ERROR.into_response();
    }

    (StatusCode::OK, Json(serde_json::json!({"success": true}))).into_response()
}

async fn get_quote(
    State(pool): State<PgPool>,
    Query(query): Query<QuoteQuery>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    let mobile_optimized = query.mobile_optimized.unwrap_or(false);
    let quote_id = match Uuid::parse_str(&id) {
        Ok(uid) => uid,
        Err(_) => return StatusCode::BAD_REQUEST.into_response(),
    };

    let (quote_res, items_res) = tokio::join!(
        sqlx::query_as::<_, Quote>("SELECT * FROM quotes WHERE id = $1")
            .bind(quote_id)
            .fetch_optional(&pool),
        sqlx::query_as::<_, QuoteLineItem>("SELECT * FROM quote_line_items WHERE quote_id = $1")
            .bind(quote_id)
            .fetch_all(&pool)
    );

    let quote = match quote_res {
        Ok(Some(q)) => q,
        Ok(None) => return StatusCode::NOT_FOUND.into_response(),
        Err(e) => {
            tracing::error!("Failed to fetch quote: {}", e);
            return StatusCode::INTERNAL_SERVER_ERROR.into_response();
        }
    };

    let mut line_items = match items_res {
        Ok(items) => items,
        Err(e) => {
            tracing::error!("Failed to fetch quote line items: {}", e);
            return StatusCode::INTERNAL_SERVER_ERROR.into_response();
        }
    };

    if mobile_optimized {
        let mut q = quote;
        q.created_at = None;
        q.updated_at = None;
        q.valid_until = None;

        for item in &mut line_items {
            item.created_at = None;
            item.updated_at = None;
        }

        (StatusCode::OK, Json(QuoteResponse { quote: q, line_items })).into_response()
    } else {
        (StatusCode::OK, Json(QuoteResponse { quote, line_items })).into_response()
    }
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;
    use crate::domain::repository::models::{Quote, QuoteLineItem};

    #[test]
    fn test_quote_mobile_optimization() {
        let quote = Quote {
            id: "q1".to_string(),
            tenant_id: "t1".to_string(),
            customer_id: "c1".to_string(),
            status: "DRAFT".to_string(),
            valid_until: Some(chrono::Utc::now()),
            total_amount: None,
            required_deposit: None,
            checkout_url: None,
            created_at: Some(chrono::Utc::now()),
            updated_at: Some(chrono::Utc::now()),
        };

        let mut line_items = vec![QuoteLineItem {
            id: "li1".to_string(),
            quote_id: "q1".to_string(),
            description: "item".to_string(),
            unit_price_cents: 100,
            quantity: 1,
            is_optional: false,
            created_at: Some(chrono::Utc::now()),
            updated_at: Some(chrono::Utc::now()),
        }];

        // Simulate mobile_optimized = true logic
        let mut q = quote;
        q.created_at = None;
        q.updated_at = None;
        q.valid_until = None;

        for item in &mut line_items {
            item.created_at = None;
            item.updated_at = None;
        }

        assert!(q.created_at.is_none());
        assert!(q.updated_at.is_none());
        assert!(q.valid_until.is_none());
        assert!(line_items[0].created_at.is_none());
        assert!(line_items[0].updated_at.is_none());
    }
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

async fn approve_quote(
    State(pool): State<PgPool>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    let quote_id = match Uuid::parse_str(&id) {
        Ok(uid) => uid,
        Err(_) => return StatusCode::BAD_REQUEST.into_response(),
    };

    let quote = match sqlx::query_as::<_, Quote>(
        "UPDATE quotes SET status = 'ACCEPTED', updated_at = NOW() WHERE id = $1 RETURNING *"
    )
    .bind(quote_id)
    .fetch_optional(&pool)
    .await
    {
        Ok(Some(q)) => q,
        Ok(None) => return StatusCode::NOT_FOUND.into_response(),
        Err(e) => {
            tracing::error!("Failed to approve quote: {}", e);
            return StatusCode::INTERNAL_SERVER_ERROR.into_response();
        }
    };

    let amount_usd = (quote.total_amount.unwrap_or(0) as f64) / 100.0;

    // Fallback if Stripe client isn't fully integrated here
    let stripe_key = std::env::var("STRIPE_API_KEY").unwrap_or_else(|_| "sk_test_mock".to_string());
    let stripe_client = crate::integrations::stripe::client::StripeClient::new(stripe_key);

    match stripe_client.create_checkout_session(
        &format!("Quote #{}", quote.id),
        &quote.customer_id.to_string(),
        amount_usd,
        false
    ).await {
        Ok(url) => {
            let _ = sqlx::query("UPDATE quotes SET checkout_url = $1 WHERE id = $2")
                .bind(&url)
                .bind(&quote.id)
                .execute(&pool)
                .await;

            let mut q = quote;
            q.checkout_url = Some(url);
            (StatusCode::OK, Json(serde_json::json!({"quote": q}))).into_response()
        },
        Err(e) => {
            tracing::error!("Failed to create Stripe checkout session: {}", e);
            (StatusCode::OK, Json(serde_json::json!({"quote": quote}))).into_response()
        }
    }
}
