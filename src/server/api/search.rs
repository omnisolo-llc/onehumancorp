use std::sync::Arc;
use axum::{
    extract::{Query, Extension},
    response::IntoResponse,
    routing::get,
    Json, Router,
};
use serde::{Deserialize, Serialize};
use crate::hub::Hub;
use sqlx::Row;

#[derive(Debug, Deserialize)]
pub struct SearchQuery {
    pub q: String,
}

#[derive(Debug, Serialize)]
pub struct SearchResultItem {
    pub id: String,
    pub entity_type: String, // "customer", "order", "message", "invoice"
    pub title: String,
    pub subtitle: Option<String>,
    pub url: String,
}

#[derive(Debug, Serialize)]
pub struct SearchResponse {
    pub results: Vec<SearchResultItem>,
}

pub async fn search_handler(
    Extension(claims): Extension<::server_common::Claims>,
    Extension(hub): Extension<Arc<Hub>>,
    Query(query): Query<SearchQuery>,
) -> impl IntoResponse {
    let tenant_id = claims.organization_id;
    let pool = hub.pool.clone();

    // Use Postgres text search (ILIKE or tsvector depending on current setup, ILIKE is simpler if tsvector isn't explicitly defined for all columns)
    // We'll search across customers, orders, inbox_messages, invoices.

    let search_term = format!("%{}%", query.q);

    // Using a parallel or sequential async query.
    let mut results = vec![];

    // Search Customers
    if let Ok(rows) = sqlx::query(
        r#"
        SELECT id, name, email
        FROM customers
        WHERE tenant_id = $1 AND (name ILIKE $2 OR email ILIKE $2)
        LIMIT 10
        "#
    ).bind(&tenant_id).bind(&search_term)
    .fetch_all(&pool)
    .await
    {
        for row in rows {
            let id: String = row.try_get("id").unwrap_or_default();
            let name: String = row.try_get("name").unwrap_or_default();
            let email: Option<String> = row.try_get("email").unwrap_or_default();

            results.push(SearchResultItem {
                id: id.clone(),
                entity_type: "customer".to_string(),
                title: name,
                subtitle: email,
                url: format!("/customers/{}", id),
            });
        }
    }

    // Search Orders
    if let Ok(rows) = sqlx::query(
        r#"
        SELECT id, status, total_amount
        FROM orders
        WHERE tenant_id = $1 AND id ILIKE $2
        LIMIT 10
        "#
    ).bind(&tenant_id).bind(&search_term)
    .fetch_all(&pool)
    .await
    {
        for row in rows {
            let id: String = row.try_get("id").unwrap_or_default();
            let status: Option<String> = row.try_get("status").unwrap_or_default();

            results.push(SearchResultItem {
                id: id.clone(),
                entity_type: "order".to_string(),
                title: format!("Order #{}", id),
                subtitle: Some(format!("Status: {}", status.unwrap_or_default())),
                url: format!("/orders/{}", id),
            });
        }
    }

    // Search Messages
    if let Ok(rows) = sqlx::query(
        r#"
        SELECT id, content, source
        FROM inbox_messages
        WHERE tenant_id = $1 AND content ILIKE $2
        LIMIT 10
        "#
    ).bind(&tenant_id).bind(&search_term)
    .fetch_all(&pool)
    .await
    {
        for row in rows {
            let id: String = row.try_get("id").unwrap_or_default();
            let content: Option<String> = row.try_get("content").unwrap_or_default();
            let source: Option<String> = row.try_get("source").unwrap_or_default();

            let content_preview = if content.clone().unwrap_or_default().len() > 50 {
                format!("{}...", &content.unwrap_or_default()[..47])
            } else {
                content.unwrap_or_default()
            };
            results.push(SearchResultItem {
                id: id.clone(),
                entity_type: "message".to_string(),
                title: content_preview,
                subtitle: source,
                url: format!("/inbox/{}", id),
            });
        }
    }

    // Search Invoices
    if let Ok(rows) = sqlx::query(
        r#"
        SELECT id, type, amount
        FROM invoices
        WHERE tenant_id = $1 AND id ILIKE $2
        LIMIT 10
        "#
    ).bind(&tenant_id).bind(&search_term)
    .fetch_all(&pool)
    .await
    {
        for row in rows {
            let id: String = row.try_get("id").unwrap_or_default();
            let type_: Option<String> = row.try_get("type").unwrap_or_default();

            results.push(SearchResultItem {
                id: id.clone(),
                entity_type: "invoice".to_string(),
                title: format!("Invoice #{}", id),
                subtitle: Some(format!("Type: {}", type_.unwrap_or_default())),
                url: format!("/invoices/{}", id),
            });
        }
    }

    Json(SearchResponse { results })
}

pub fn router<S: Clone + Send + Sync + 'static>() -> Router<S> {
    Router::new().route("/", get(search_handler))
}
