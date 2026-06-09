use axum::{
    extract::{Query, State, Extension},
    Json,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use crate::db::DB;
use sqlx::Row;
use server_common::Claims;

#[derive(Deserialize)]
pub struct GlobalSearchQuery {
    pub q: String,
}

#[derive(Serialize)]
pub struct SearchResultItem {
    pub id: String,
    pub entity_type: String,
    pub title: String,
    pub subtitle: Option<String>,
}

#[derive(Serialize)]
pub struct GlobalSearchResponse {
    pub results: Vec<SearchResultItem>,
}

pub async fn global_search_handler(
    State(db): State<Arc<DB>>,
    Extension(user): Extension<Claims>,
    Query(query): Query<GlobalSearchQuery>,
) -> Json<GlobalSearchResponse> {
    let q = format!("%{}%", query.q);
    let tenant_id = user.organization_id;
    let mut results = Vec::new();

    // Search Customers
    if let Ok(rows) = sqlx::query(
        "SELECT id, name, email FROM customers WHERE tenant_id = $1 AND (name ILIKE $2 OR email ILIKE $2) LIMIT 10",
    )
    .bind(&tenant_id)
    .bind(&q)
    .fetch_all(&db.pool)
    .await {
        for row in rows {
            results.push(SearchResultItem {
                id: row.try_get("id").unwrap_or_default(),
                entity_type: "customer".to_string(),
                title: row.try_get("name").unwrap_or_default(),
                subtitle: row.try_get("email").ok(),
            });
        }
    }

    // Search Orders
    if let Ok(rows) = sqlx::query(
        "SELECT id, status FROM orders WHERE tenant_id = $1 AND (id ILIKE $2 OR status ILIKE $2) LIMIT 10",
    )
    .bind(&tenant_id)
    .bind(&q)
    .fetch_all(&db.pool)
    .await {
        for row in rows {
            results.push(SearchResultItem {
                id: row.try_get("id").unwrap_or_default(),
                entity_type: "order".to_string(),
                title: format!("Order #{}", row.try_get::<String, _>("id").unwrap_or_default()),
                subtitle: row.try_get("status").ok(),
            });
        }
    }

    // Search Messages
    if let Ok(rows) = sqlx::query(
        "SELECT id, content FROM inbox_messages WHERE tenant_id = $1 AND content ILIKE $2 LIMIT 10",
    )
    .bind(&tenant_id)
    .bind(&q)
    .fetch_all(&db.pool)
    .await {
        for row in rows {
            results.push(SearchResultItem {
                id: row.try_get("id").unwrap_or_default(),
                entity_type: "message".to_string(),
                title: "Message".to_string(),
                subtitle: row.try_get("content").ok(),
            });
        }
    }

    Json(GlobalSearchResponse { results })
}
