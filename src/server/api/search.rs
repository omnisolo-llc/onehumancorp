use axum::{extract::{Query, State}, Json};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::db::DB;

#[derive(Deserialize)]
pub struct SearchQuery {
    pub q: String,
}

#[derive(Serialize)]
pub struct SearchResultItem {
    pub id: String,
    pub entity_type: String,
    pub title: String,
    pub subtitle: String,
    pub url: String,
    pub created_at_unix: i64,
}

#[derive(Serialize)]
pub struct SearchResponse {
    pub results: Vec<SearchResultItem>,
}

pub async fn search_handler(
    State(db): State<Arc<DB>>,
    axum::extract::Extension(claims): axum::extract::Extension<::server_common::Claims>,
    Query(query): Query<SearchQuery>,
) -> Result<Json<SearchResponse>, (axum::http::StatusCode, Json<serde_json::Value>)> {
    let tenant_id = claims.organization_id.clone().unwrap_or_default();
    if tenant_id.is_empty() {
        return Err((axum::http::StatusCode::UNAUTHORIZED, Json(serde_json::json!({"error": "Unauthorized"}))));
    }
    let search_term = format!("%{}%", query.q);

    let mut results = Vec::new();

    // 1. Search Customers
    let customers: Vec<(String, String, String, chrono::NaiveDateTime)> = sqlx::query_as(
        "SELECT id, name, email, created_at FROM customers WHERE tenant_id = ? AND (name LIKE ? OR email LIKE ?) LIMIT 5"
    )
    .bind(&tenant_id)
    .bind(&search_term)
    .bind(&search_term)
    .fetch_all(&db.pool)
    .await
    .unwrap_or_default();

    for (id, name, email, created_at) in customers {
        results.push(SearchResultItem {
            id: id.clone(),
            entity_type: "customer".to_string(),
            title: name,
            subtitle: email,
            url: format!("/customers/{}", id),
            created_at_unix: created_at.timestamp(),
        });
    }

    // 2. Search Orders
    let orders: Vec<(String, String, String, chrono::NaiveDateTime)> = sqlx::query_as(
        "SELECT id, customer_id, status, created_at FROM orders WHERE tenant_id = ? AND (id LIKE ? OR customer_id LIKE ?) LIMIT 5"
    )
    .bind(&tenant_id)
    .bind(&search_term)
    .bind(&search_term)
    .fetch_all(&db.pool)
    .await
    .unwrap_or_default();

    for (id, customer_id, status, created_at) in orders {
        results.push(SearchResultItem {
            id: id.clone(),
            entity_type: "order".to_string(),
            title: format!("Order {}", id),
            subtitle: format!("Customer: {} - Status: {}", customer_id, status),
            url: format!("/orders/{}", id),
            created_at_unix: created_at.timestamp(),
        });
    }

    // 3. Search Messages
    let messages: Vec<(String, String, String, chrono::NaiveDateTime)> = sqlx::query_as(
        "SELECT id, source, content, created_at FROM inbox_messages WHERE tenant_id = ? AND content LIKE ? LIMIT 5"
    )
    .bind(&tenant_id)
    .bind(&search_term)
    .fetch_all(&db.pool)
    .await
    .unwrap_or_default();

    for (id, source, content, created_at) in messages {
        results.push(SearchResultItem {
            id: id.clone(),
            entity_type: "message".to_string(),
            title: format!("Message from {}", source),
            subtitle: if content.len() > 50 { format!("{}...", &content[..47]) } else { content },
            url: format!("/inbox/{}", id),
            created_at_unix: created_at.timestamp(),
        });
    }

    // Sort by created_at descending
    results.sort_by(|a, b| b.created_at_unix.cmp(&a.created_at_unix));

    Ok(Json(SearchResponse { results }))
}

pub fn router(db: Arc<DB>) -> axum::Router<Arc<dyn ohc_builtin_agent::mesh::transport::MeshTransport>> {
    axum::Router::new()
        .route("/", axum::routing::get(search_handler))
        .with_state(db)
}
