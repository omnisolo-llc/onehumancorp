use axum::{
    extract::{Query, State},
    response::IntoResponse,
    Json,
};
use serde::{Deserialize, Serialize};
use sqlx::{PgPool, Row};

#[derive(Deserialize)]
pub struct SearchQuery {
    pub q: String,
}

#[derive(Serialize)]
pub struct SearchResult {
    pub id: String,
    pub entity_type: String,
    pub title: String,
    pub subtitle: Option<String>,
}

#[derive(Serialize)]
pub struct UnifiedSearchResponse {
    pub results: Vec<SearchResult>,
}

pub async fn search_handler(
    State(pool): State<PgPool>,
    Query(query): Query<SearchQuery>,
) -> impl IntoResponse {
    let q = query.q.trim();
    if q.is_empty() {
        return Json(UnifiedSearchResponse { results: vec![] });
    }

    let search_pattern = format!("%{}%", q);

    let customers_fut = sqlx::query(
        r#"
        SELECT id, name as title, email as subtitle
        FROM customers
        WHERE name ILIKE $1 OR email ILIKE $1
        LIMIT 10
        "#
    )
    .bind(&search_pattern)
    .fetch_all(&pool);

    let orders_fut = sqlx::query(
        r#"
        SELECT id, id as title, status as subtitle
        FROM orders
        WHERE id ILIKE $1
        LIMIT 10
        "#
    )
    .bind(&search_pattern)
    .fetch_all(&pool);

    let messages_fut = sqlx::query(
        r#"
        SELECT id, content as title, source as subtitle
        FROM inbox_messages
        WHERE content ILIKE $1
        LIMIT 10
        "#
    )
    .bind(&search_pattern)
    .fetch_all(&pool);

    let (customers_res, orders_res, messages_res) =
        tokio::join!(customers_fut, orders_fut, messages_fut);

    let mut results = Vec::new();

    if let Ok(customers) = customers_res {
        for c in customers {
            results.push(SearchResult {
                id: c.get("id"),
                entity_type: "customer".to_string(),
                title: c.try_get("title").unwrap_or_default(),
                subtitle: c.try_get("subtitle").ok(),
            });
        }
    }

    if let Ok(orders) = orders_res {
        for o in orders {
            let title: String = o.try_get("title").unwrap_or_default();
            results.push(SearchResult {
                id: o.get("id"),
                entity_type: "order".to_string(),
                title: format!("Order #{}", title.chars().take(8).collect::<String>()),
                subtitle: o.try_get("subtitle").ok(),
            });
        }
    }

    if let Ok(messages) = messages_res {
        for m in messages {
            results.push(SearchResult {
                id: m.get("id"),
                entity_type: "message".to_string(),
                title: m.try_get("title").unwrap_or_default(),
                subtitle: m.try_get("subtitle").ok(),
            });
        }
    }

    Json(UnifiedSearchResponse { results })
}
