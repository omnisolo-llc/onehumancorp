use axum::{extract::{Query, State}, Json};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Deserialize)]
pub struct SearchQuery {
    pub q: String,
}

#[derive(Serialize, Debug)]
pub struct CustomerResult {
    pub id: String,
    pub name: String,
    pub email: Option<String>,
}

#[derive(Serialize, Debug)]
pub struct OrderResult {
    pub id: String,
    pub status: String,
    pub total_amount: Option<f64>,
}

#[derive(Serialize, Debug)]
pub struct MessageResult {
    pub id: String,
    pub content: String,
}

#[derive(Serialize, Debug)]
pub struct SearchResponse {
    pub customers: Vec<CustomerResult>,
    pub orders: Vec<OrderResult>,
    pub messages: Vec<MessageResult>,
}

pub async fn global_search(
    State(pool): State<PgPool>,
    Query(query): Query<SearchQuery>,
) -> Result<Json<SearchResponse>, axum::http::StatusCode> {
    let q = format!("%{}%", query.q);

    let customers = sqlx::query!(
        r#"
        SELECT id, COALESCE(preferences->>'name', email, id) as name, email
        FROM customers
        WHERE email ILIKE $1 OR preferences::text ILIKE $1
        LIMIT 5
        "#,
        q
    )
    .fetch_all(&pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to search customers: {}", e);
        axum::http::StatusCode::INTERNAL_SERVER_ERROR
    })?
    .into_iter()
    .map(|row| CustomerResult {
        id: row.id,
        name: row.name.unwrap_or_else(|| "Unknown".to_string()),
        email: row.email,
    })
    .collect();

    let orders = sqlx::query!(
        r#"
        SELECT id, status, total_amount::float8 as total_amount
        FROM orders
        WHERE id ILIKE $1 OR status ILIKE $1
        LIMIT 5
        "#,
        q
    )
    .fetch_all(&pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to search orders: {}", e);
        axum::http::StatusCode::INTERNAL_SERVER_ERROR
    })?
    .into_iter()
    .map(|row| OrderResult {
        id: row.id,
        status: row.status.unwrap_or_else(|| "pending".to_string()),
        total_amount: row.total_amount,
    })
    .collect();

    let messages = sqlx::query!(
        r#"
        SELECT id, content
        FROM inbox_messages
        WHERE content ILIKE $1
        LIMIT 5
        "#,
        q
    )
    .fetch_all(&pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to search messages: {}", e);
        axum::http::StatusCode::INTERNAL_SERVER_ERROR
    })?
    .into_iter()
    .map(|row| MessageResult {
        id: row.id,
        content: row.content.unwrap_or_else(|| "".to_string()),
    })
    .collect();

    Ok(Json(SearchResponse {
        customers,
        orders,
        messages,
    }))
}

pub fn router(pool: PgPool) -> axum::Router {
    axum::Router::new()
        .route("/search", axum::routing::get(global_search))
        .with_state(pool)
}
