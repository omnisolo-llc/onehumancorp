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

async fn approve_quote(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
) -> Result<Json<Quote>, axum::http::StatusCode> {
    let quote = sqlx::query_as::<_, Quote>(
        "UPDATE quotes SET status = 'ACCEPTED', updated_at = NOW() WHERE id = $1 RETURNING *"
    )
    .bind(id)
    .fetch_optional(&pool)
    .await
    .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;

    // Integrate Stripe deposit logic here...

    match quote {
        Some(q) => Ok(Json(q)),
        None => Err(axum::http::StatusCode::NOT_FOUND),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db;
    use sqlx::Row;
    use sqlx::Executor;
    use std::env;

    #[tokio::test]
    async fn test_tenant_isolation_quotes() {
        let database_url = env::var("OHC_DATABASE_URL")
            .unwrap_or_else(|_| "postgres://postgres:postgres@localhost:5432/test".to_string());

        // Ensure this only runs for Postgres, as SQLite handling is different and test might fail without proper migrations.
        if !database_url.starts_with("postgres") {
            return;
        }

        let db = match db::DB::new().await {
            Ok(d) => d,
            Err(_) => return, // Environment doesn't support full Postgres initialization (e.g. read-only filesystem)
        };
        let pool = db.pool.clone();

        // If DB isn't migrated/available, just return
        if sqlx::query("SELECT 1 FROM quotes LIMIT 1").execute(&pool).await.is_err() {
            return;
        }

        let tenant_1 = "tenant_A_quote_test";
        let tenant_2 = "tenant_B_quote_test";
        let quote_id = Uuid::new_v4();
        let customer_id = Uuid::new_v4();

        // Tenant 1 inserts a quote
        {
            let mut tx = pool.begin().await.unwrap();
            tx.execute(format!("SET LOCAL app.current_tenant = '{}'", tenant_1).as_str()).await.unwrap();

            sqlx::query("INSERT INTO quotes (id, tenant_id, customer_id, status) VALUES ($1, $2, $3, $4)")
                .bind(quote_id)
                .bind(tenant_1)
                .bind(customer_id)
                .bind("DRAFT")
                .execute(&mut *tx)
                .await
                .unwrap();

            tx.commit().await.unwrap();
        }

        // Tenant 2 tries to read the quote, should not be found
        {
            let mut tx = pool.begin().await.unwrap();
            tx.execute(format!("SET LOCAL app.current_tenant = '{}'", tenant_2).as_str()).await.unwrap();

            let quote: Option<Quote> = sqlx::query_as("SELECT * FROM quotes WHERE id = $1")
                .bind(quote_id)
                .fetch_optional(&mut *tx)
                .await
                .unwrap();

            assert!(quote.is_none(), "Tenant 2 should not be able to see Tenant 1's quote");

            tx.commit().await.unwrap();
        }

        // Tenant 1 tries to read their own quote, should be found
        {
            let mut tx = pool.begin().await.unwrap();
            tx.execute(format!("SET LOCAL app.current_tenant = '{}'", tenant_1).as_str()).await.unwrap();

            let quote: Option<Quote> = sqlx::query_as("SELECT * FROM quotes WHERE id = $1")
                .bind(quote_id)
                .fetch_optional(&mut *tx)
                .await
                .unwrap();

            assert!(quote.is_some(), "Tenant 1 should be able to see their own quote");
            assert_eq!(quote.unwrap().id, quote_id);

            tx.commit().await.unwrap();
        }
    }
}
