use axum::{extract::Query, Json};
use serde::{Deserialize, Serialize};
use crate::db::get_pool;
use ::server_common::Claims;
use axum::response::IntoResponse;

#[derive(Deserialize)]
pub struct SearchQuery {
    pub q: String,
}

#[derive(Serialize)]
pub struct SearchResultItem {
    pub id: String,
    pub entity_type: String, // "customer", "order", "message"
    pub title: String,
    pub subtitle: String,
}

#[derive(Serialize)]
pub struct SearchResponse {
    pub results: Vec<SearchResultItem>,
}

pub async fn search_handler(
    axum::extract::Extension(user): axum::extract::Extension<Claims>,
    Query(query): Query<SearchQuery>,
) -> impl IntoResponse {
    let pool = get_pool();
    let tenant_id = user.organization_id.unwrap_or_default();

    // We use websearch_to_tsquery for safe parsing of user input
    let q = query.q.clone();

    let mut results = Vec::new();

    if q.is_empty() {
        return (axum::http::StatusCode::OK, Json(SearchResponse { results })).into_response();
    }

    // Search Customers using tsvector
    match sqlx::query(
        r#"
        SELECT id, name, email
        FROM customers
        WHERE tenant_id = $1
          AND to_tsvector('english', coalesce(name, '') || ' ' || coalesce(email, '')) @@ websearch_to_tsquery('english', $2)
        LIMIT 5
        "#
    )
    .bind(&tenant_id)
    .bind(&q)
    .fetch_all(&pool)
    .await {
        Ok(rows) => {
            for row in rows {
                use sqlx::Row;
                let id: String = row.get("id");
                let name: String = row.get("name");
                let email: String = row.try_get("email").unwrap_or_default();
                results.push(SearchResultItem {
                    id,
                    entity_type: "customer".to_string(),
                    title: name,
                    subtitle: email,
                });
            }
        }
        Err(e) => tracing::error!("Error searching customers: {}", e),
    }

    // Search Orders using tsvector
    match sqlx::query(
        r#"
        SELECT id, status
        FROM orders
        WHERE tenant_id = $1
          AND to_tsvector('english', coalesce(id, '') || ' ' || coalesce(status, '')) @@ websearch_to_tsquery('english', $2)
        LIMIT 5
        "#
    )
    .bind(&tenant_id)
    .bind(&q)
    .fetch_all(&pool)
    .await {
        Ok(rows) => {
            for row in rows {
                use sqlx::Row;
                let id: String = row.get("id");
                let status: String = row.try_get("status").unwrap_or_default();
                results.push(SearchResultItem {
                    id: id.clone(),
                    entity_type: "order".to_string(),
                    title: format!("Order {}", id),
                    subtitle: format!("Status: {}", status),
                });
            }
        }
        Err(e) => tracing::error!("Error searching orders: {}", e),
    }

    // Search Messages using tsvector
    match sqlx::query(
        r#"
        SELECT id, content
        FROM inbox_messages
        WHERE tenant_id = $1
          AND to_tsvector('english', coalesce(content, '')) @@ websearch_to_tsquery('english', $2)
        LIMIT 5
        "#
    )
    .bind(&tenant_id)
    .bind(&q)
    .fetch_all(&pool)
    .await {
        Ok(rows) => {
            for row in rows {
                use sqlx::Row;
                let id: String = row.get("id");
                let content: String = row.try_get("content").unwrap_or_default();
                let subtitle = if content.len() > 50 {
                    format!("{}...", &content[0..47])
                } else {
                    content
                };
                results.push(SearchResultItem {
                    id,
                    entity_type: "message".to_string(),
                    title: "Message".to_string(),
                    subtitle,
                });
            }
        }
        Err(e) => tracing::error!("Error searching messages: {}", e),
    }

    (axum::http::StatusCode::OK, Json(SearchResponse { results }))
}
