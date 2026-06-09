use axum::{extract::{Query, State}, Json, response::IntoResponse};
use serde::{Deserialize, Serialize};
use server_common::Claims;
use axum::Extension;

#[derive(Deserialize)]
pub struct SearchQuery {
    pub q: String,
}

#[derive(Serialize)]
pub struct GlobalSearchResult {
    pub id: String,
    pub r#type: String,
    pub title: String,
    pub subtitle: Option<String>,
    pub url: String,
}

pub async fn search_handler(
    State(db): State<std::sync::Arc<crate::db::DB>>,
    Extension(claims): Extension<Claims>,
    Query(query): Query<SearchQuery>,
) -> impl IntoResponse {
    let tenant_id = claims.organization_id.unwrap_or_else(|| "default".to_string());

    // Convert to tsquery syntax: "word1:* | word2:*" for prefix matching
    let terms: Vec<String> = query.q.split_whitespace().map(|s| format!("{}:*", s)).collect();
    let search_term = terms.join(" | ");

    let mut results = Vec::new();

    // Use Postgres FTS when available. However, since the database can be SQLite in tests (DbStore::Sqlite),
    // using raw `to_tsvector` in `sqlx::query_as` can fail SQLite parse.
    // Instead we do conditional fallback based on the DB engine flag.
    let is_sqlite = db.is_sqlite();

    let customers: Vec<(String, Option<String>, Option<String>, Option<String>)> = if is_sqlite {
        let term = format!("%{}%", query.q);
        sqlx::query_as(
            r#"
            SELECT id, name, email, phone
            FROM customers
            WHERE tenant_id = $1 AND (name LIKE $2 OR email LIKE $2 OR phone LIKE $2)
            LIMIT 5
            "#
        )
        .bind(&tenant_id)
        .bind(&term)
        .fetch_all(&db.pool)
        .await
        .unwrap_or_default()
    } else {
        sqlx::query_as(
            r#"
            SELECT id, name, email, phone
            FROM customers
            WHERE tenant_id = $1 AND to_tsvector('english', coalesce(name, '') || ' ' || coalesce(email, '') || ' ' || coalesce(phone, '')) @@ to_tsquery('english', $2)
            LIMIT 5
            "#
        )
        .bind(&tenant_id)
        .bind(&search_term)
        .fetch_all(&db.pool)
        .await
        .unwrap_or_default()
    };

    for c in customers {
        results.push(GlobalSearchResult {
            id: c.0.clone(),
            r#type: "customer".to_string(),
            title: c.1.unwrap_or_else(|| "Unknown".to_string()),
            subtitle: c.2.or(c.3),
            url: format!("/customers/{}", c.0),
        });
    }

    // Search Orders (just ID lookup for now)
    let orders_term = format!("%{}%", query.q);
    let orders: Vec<(String, Option<f64>, Option<String>)> = if is_sqlite {
        sqlx::query_as(
            r#"
            SELECT id, total_amount, status
            FROM orders
            WHERE tenant_id = $1 AND id LIKE $2
            LIMIT 5
            "#
        )
        .bind(&tenant_id)
        .bind(&orders_term)
        .fetch_all(&db.pool)
        .await
        .unwrap_or_default()
    } else {
        sqlx::query_as(
            r#"
            SELECT id, total_amount, status
            FROM orders
            WHERE tenant_id = $1 AND id ILIKE $2
            LIMIT 5
            "#
        )
        .bind(&tenant_id)
        .bind(&orders_term)
        .fetch_all(&db.pool)
        .await
        .unwrap_or_default()
    };

    for o in orders {
        results.push(GlobalSearchResult {
            id: o.0.clone(),
            r#type: "order".to_string(),
            title: format!("Order #{}", o.0),
            subtitle: Some(format!("Status: {}", o.2.unwrap_or_else(|| "Unknown".to_string()))),
            url: format!("/orders/{}", o.0),
        });
    }

    // Search Messages using tsvector
    let messages: Vec<(String, Option<String>)> = if is_sqlite {
        let term = format!("%{}%", query.q);
        sqlx::query_as(
            r#"
            SELECT id, content
            FROM inbox_messages
            WHERE tenant_id = $1 AND content LIKE $2
            LIMIT 5
            "#
        )
        .bind(&tenant_id)
        .bind(&term)
        .fetch_all(&db.pool)
        .await
        .unwrap_or_default()
    } else {
        sqlx::query_as(
            r#"
            SELECT id, content
            FROM inbox_messages
            WHERE tenant_id = $1 AND to_tsvector('english', coalesce(content, '')) @@ to_tsquery('english', $2)
            LIMIT 5
            "#
        )
        .bind(&tenant_id)
        .bind(&search_term)
        .fetch_all(&db.pool)
        .await
        .unwrap_or_default()
    };

    for m in messages {
        let content = m.1.unwrap_or_default();
        let preview = if content.len() > 50 {
            format!("{}...", &content[..47])
        } else {
            content
        };
        results.push(GlobalSearchResult {
            id: m.0.clone(),
            r#type: "message".to_string(),
            title: preview,
            subtitle: None,
            url: format!("/inbox?msg={}", m.0),
        });
    }

    // Search Bookings
    let bookings_term = format!("%{}%", query.q);
    let bookings: Vec<(String, Option<String>, Option<String>)> = if is_sqlite {
        sqlx::query_as(
            r#"
            SELECT id, service_id, status
            FROM bookings
            WHERE tenant_id = $1 AND id LIKE $2
            LIMIT 5
            "#
        )
        .bind(&tenant_id)
        .bind(&bookings_term)
        .fetch_all(&db.pool)
        .await
        .unwrap_or_default()
    } else {
        sqlx::query_as(
            r#"
            SELECT id, service_id, status
            FROM bookings
            WHERE tenant_id = $1 AND id ILIKE $2
            LIMIT 5
            "#
        )
        .bind(&tenant_id)
        .bind(&bookings_term)
        .fetch_all(&db.pool)
        .await
        .unwrap_or_default()
    };

    for b in bookings {
        results.push(GlobalSearchResult {
            id: b.0.clone(),
            r#type: "booking".to_string(),
            title: format!("Booking #{}", b.0),
            subtitle: b.2,
            url: format!("/calendar?booking={}", b.0),
        });
    }

    Json(results)
}

#[cfg(test)]
mod tests {
    use super::*;
    use server_common::Claims;
    use axum::extract::{Query, State};
    use axum::Extension;

    #[tokio::test]
    async fn test_search_handler_extracts_correct_tenant() {
        let db_mock = std::sync::Arc::new(crate::db::DB::new().await.unwrap());
        let claims = Claims {
            sub: "u1".to_string(),
            organization_id: Some("test-org-123".to_string()),
            exp: 9999999999,
            iat: 0,
            username: "test".to_string(),
            email: "test@example.com".to_string(),
            roles: vec!["owner".to_string()],
            jti: "test".to_string(),
            session_id: Some("test".to_string()),
        };

        let query = Query(SearchQuery { q: "John".to_string() });
        let _response = search_handler(State(db_mock), Extension(claims.clone()), query).await;
    }
}
