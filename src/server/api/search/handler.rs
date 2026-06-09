use axum::{
    extract::{Query, State},
    Json,
};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Deserialize)]
pub struct SearchQuery {
    pub q: String,
    pub tenant_id: String,
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

pub async fn global_search(
    State(pool): State<PgPool>,
    axum::extract::Extension(auth_info): axum::extract::Extension<::server_auth::orchestration::AuthInfo>,
    Query(query): Query<SearchQuery>,
) -> Json<SearchResponse> {
    let mut results = Vec::new();

    let search_term = format!("%{}%", query.q);

    // Extract tenant_id securely from auth context
    let tenant_id = auth_info.org_id;

    // Search Customers
    let customers_query = "SELECT id, name, email, phone FROM customers WHERE tenant_id = $1 AND (name ILIKE $2 OR email ILIKE $2 OR phone ILIKE $2) LIMIT 5";
    let customers: Vec<sqlx::postgres::PgRow> = sqlx::query(customers_query)
        .bind(&tenant_id)
        .bind(&search_term)
        .fetch_all(&pool)
        .await
        .unwrap_or_default();

    for row in customers {
        use sqlx::Row;
        results.push(SearchResultItem {
            id: row.try_get("id").unwrap_or_default(),
            entity_type: "customer".to_string(),
            title: row.try_get("name").unwrap_or_default(),
            subtitle: row.try_get("email").unwrap_or_default(),
        });
    }

    // Search Orders
    let orders_query = "SELECT id, status, total_amount FROM orders WHERE tenant_id = $1 AND (id ILIKE $2 OR status ILIKE $2) LIMIT 5";
    let orders: Vec<sqlx::postgres::PgRow> = sqlx::query(orders_query)
        .bind(&tenant_id)
        .bind(&search_term)
        .fetch_all(&pool)
        .await
        .unwrap_or_default();

    for row in orders {
        use sqlx::Row;
        let id: String = row.try_get("id").unwrap_or_default();
        let status: String = row.try_get("status").unwrap_or_default();
        results.push(SearchResultItem {
            id: id.clone(),
            entity_type: "order".to_string(),
            title: format!("Order {}", id),
            subtitle: format!("Status: {}", status),
        });
    }

    // Search Messages
    let messages_query = "SELECT id, content FROM inbox_messages WHERE tenant_id = $1 AND content ILIKE $2 LIMIT 5";
    let messages: Vec<sqlx::postgres::PgRow> = sqlx::query(messages_query)
        .bind(&tenant_id)
        .bind(&search_term)
        .fetch_all(&pool)
        .await
        .unwrap_or_default();

    for row in messages {
        use sqlx::Row;
        let content: String = row.try_get("content").unwrap_or_default();
        results.push(SearchResultItem {
            id: row.try_get("id").unwrap_or_default(),
            entity_type: "message".to_string(),
            title: content.chars().take(50).collect(),
            subtitle: "Message".to_string(),
        });
    }

    Json(SearchResponse { results })
}
