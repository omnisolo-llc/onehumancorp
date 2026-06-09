use axum::{
    extract::{Extension, Query, State},
    response::IntoResponse,
    Json,
    http::StatusCode,
};
use serde::{Deserialize, Serialize};
use sqlx::{PgPool, Row};
use ::server_common::Claims;

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
    Extension(user): Extension<Claims>,
    Query(query): Query<SearchQuery>,
) -> impl IntoResponse {
    let q = query.q.trim();
    if q.is_empty() {
        return Json(UnifiedSearchResponse { results: vec![] }).into_response();
    }

    let tenant_id = match user.organization_id {
        Some(id) if !id.is_empty() => id,
        _ => return (StatusCode::UNAUTHORIZED, "Missing tenant scope").into_response(),
    };

    let ts_query_str = q.split_whitespace()
        .map(|term| format!("{}:*", term.replace(|c: char| !c.is_alphanumeric(), "")))
        .filter(|term| term.len() > 2)
        .collect::<Vec<_>>()
        .join(" & ");

    if ts_query_str.is_empty() {
        return Json(UnifiedSearchResponse { results: vec![] }).into_response();
    }

    let search_pattern = format!("%{}%", q);

    let customers_fut = sqlx::query(
        r#"
        SELECT id, name as title, email as subtitle
        FROM customers
        WHERE tenant_id = $1 AND (
            to_tsvector('english', coalesce(name, '') || ' ' || coalesce(email, '')) @@ to_tsquery('english', $2)
            OR name ILIKE $3
            OR email ILIKE $3
        )
        LIMIT 10
        "#
    )
    .bind(&tenant_id)
    .bind(&ts_query_str)
    .bind(&search_pattern)
    .fetch_all(&pool);

    let orders_fut = sqlx::query(
        r#"
        SELECT id, id as title, status as subtitle
        FROM orders
        WHERE tenant_id = $1 AND (
            to_tsvector('english', id) @@ to_tsquery('english', $2)
            OR id ILIKE $3
        )
        LIMIT 10
        "#
    )
    .bind(&tenant_id)
    .bind(&ts_query_str)
    .bind(&search_pattern)
    .fetch_all(&pool);

    let messages_fut = sqlx::query(
        r#"
        SELECT id, content as title, source as subtitle
        FROM inbox_messages
        WHERE tenant_id = $1 AND (
            to_tsvector('english', coalesce(content, '')) @@ to_tsquery('english', $2)
            OR content ILIKE $3
        )
        LIMIT 10
        "#
    )
    .bind(&tenant_id)
    .bind(&ts_query_str)
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

    Json(UnifiedSearchResponse { results }).into_response()
}
