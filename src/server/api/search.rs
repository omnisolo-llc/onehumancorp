use axum::{
    extract::{Query, State},
    response::IntoResponse,
    Json, Extension,
};
use serde::{Deserialize, Serialize};
use sqlx::{PgPool, Row};
use crate::db::DB;
use ::server_common::Claims;

#[derive(Deserialize)]
pub struct SearchQuery {
    pub q: String,
}

#[derive(Serialize)]
pub struct SearchResultItem {
    pub id: String,
    pub title: String,
    pub subtitle: String,
    pub entity_type: String,
}

#[derive(Serialize)]
pub struct GlobalSearchResponse {
    pub results: Vec<SearchResultItem>,
}

pub async fn global_search_handler(
    State(db): State<DB>,
    Extension(user): Extension<Claims>,
    Query(query): Query<SearchQuery>,
) -> impl IntoResponse {
    let tenant_id = user.organization_id.unwrap_or_default();
    let q_term = format!("%{}%", query.q);

    let pool = &db.pool;

    let customers_fut = sqlx::query(
        "SELECT id, name, email FROM customers WHERE tenant_id = $1 AND (name ILIKE $2 OR email ILIKE $2 OR phone ILIKE $2) LIMIT 5"
    )
    .bind(&tenant_id)
    .bind(&q_term)
    .fetch_all(pool);

    let orders_fut = sqlx::query(
        "SELECT id, status FROM orders WHERE tenant_id = $1 AND (id ILIKE $2 OR status ILIKE $2) LIMIT 5"
    )
    .bind(&tenant_id)
    .bind(&q_term)
    .fetch_all(pool);

    let invoices_fut = sqlx::query(
        "SELECT id, status, total_amount FROM invoices WHERE tenant_id = $1 AND (id ILIKE $2 OR status ILIKE $2) LIMIT 5"
    )
    .bind(&tenant_id)
    .bind(&q_term)
    .fetch_all(pool);

    let messages_fut = sqlx::query(
        "SELECT id, content, source FROM inbox_messages WHERE tenant_id = $1 AND (content ILIKE $2 OR source ILIKE $2) LIMIT 5"
    )
    .bind(&tenant_id)
    .bind(&q_term)
    .fetch_all(pool);

    let (customers_res, orders_res, invoices_res, messages_res) = tokio::join!(customers_fut, orders_fut, invoices_fut, messages_fut);

    let mut results = Vec::new();

    if let Ok(rows) = customers_res {
        for row in rows {
            let name: String = row.get("name");
            let email: String = row.try_get("email").unwrap_or_default();
            results.push(SearchResultItem {
                id: row.get("id"),
                title: name,
                subtitle: email,
                entity_type: "customer".to_string(),
            });
        }
    }

    if let Ok(rows) = orders_res {
        for row in rows {
            let status: String = row.get("status");
            results.push(SearchResultItem {
                id: row.get("id"),
                title: format!("Order {}", row.get::<String, _>("id")),
                subtitle: format!("Status: {}", status),
                entity_type: "order".to_string(),
            });
        }
    }

    if let Ok(rows) = invoices_res {
        for row in rows {
            let status: String = row.get("status");
            let amount: f64 = row.try_get("total_amount").unwrap_or(0.0);
            results.push(SearchResultItem {
                id: row.get("id"),
                title: format!("Invoice {}", row.get::<String, _>("id")),
                subtitle: format!("Status: {} - Amount: ${:.2}", status, amount),
                entity_type: "invoice".to_string(),
            });
        }
    }

    if let Ok(rows) = messages_res {
        for row in rows {
            let content: String = row.get("content");
            let source: String = row.try_get("source").unwrap_or_default();
            let snippet = if content.len() > 50 {
                format!("{}...", &content[..47])
            } else {
                content
            };
            results.push(SearchResultItem {
                id: row.get("id"),
                title: format!("Message from {}", source),
                subtitle: snippet,
                entity_type: "message".to_string(),
            });
        }
    }

    axum::response::Json(GlobalSearchResponse { results })
}
