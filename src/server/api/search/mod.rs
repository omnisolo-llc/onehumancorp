use axum::{
    extract::{Query, State},
    response::IntoResponse,
    Json,
};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct OmniboxSearchQuery {
    pub q: String,
}

#[derive(Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct OmniboxSearchResult {
    pub entity_type: String, // "customer", "order", "message", "invoice"
    pub id: String,
    pub title: String,
    pub subtitle: String,
    pub url: String,
}

pub async fn search_handler(
    State(db): State<crate::db::DB>,
    Query(query): Query<OmniboxSearchQuery>,
) -> impl IntoResponse {
    let tenant_id = std::env::var("OHC_DEFAULT_TENANT_ID").unwrap_or_else(|_| "e2e-tenant".to_string());

    let mut results = Vec::new();

    // Fallback if sqlite or something
    if query.q.is_empty() {
        return Json(results);
    }

    // We will do a parallel query against postgres full text search if store is postgres
    match db.store {
        crate::db::DbStore::Postgres => {
            // customers
            let customers_result = sqlx::query!(
                r#"
                SELECT id, email, name
                FROM customers
                WHERE tenant_id = $1 AND (
                    to_tsvector('english', coalesce(name, '') || ' ' || coalesce(email, '')) @@ plainto_tsquery('english', $2)
                    OR name ILIKE $3
                    OR email ILIKE $3
                )
                LIMIT 5
                "#,
                tenant_id,
                query.q,
                format!("%{}%", query.q)
            ).fetch_all(&db.pool).await;

            if let Ok(records) = customers_result {
                for r in records {
                    results.push(OmniboxSearchResult {
                        entity_type: "customer".to_string(),
                        id: r.id.clone(),
                        title: r.name.unwrap_or_else(|| "Unknown".to_string()),
                        subtitle: r.email.unwrap_or_else(|| "".to_string()),
                        url: format!("/customers/{}", r.id),
                    });
                }
            }

            // orders
            let orders_result = sqlx::query!(
                r#"
                SELECT id, status
                FROM orders
                WHERE tenant_id = $1 AND (
                    to_tsvector('english', coalesce(id, '') || ' ' || coalesce(status, '')) @@ plainto_tsquery('english', $2)
                    OR id ILIKE $3
                )
                LIMIT 5
                "#,
                tenant_id,
                query.q,
                format!("%{}%", query.q)
            ).fetch_all(&db.pool).await;

            if let Ok(records) = orders_result {
                for r in records {
                    results.push(OmniboxSearchResult {
                        entity_type: "order".to_string(),
                        id: r.id.clone(),
                        title: format!("Order #{}", r.id),
                        subtitle: r.status.unwrap_or_else(|| "Unknown status".to_string()),
                        url: format!("/orders/{}", r.id),
                    });
                }
            }

            // inbox_messages
            let messages_result = sqlx::query!(
                r#"
                SELECT id, source, content
                FROM inbox_messages
                WHERE tenant_id = $1 AND (
                    to_tsvector('english', coalesce(source, '') || ' ' || coalesce(content, '')) @@ plainto_tsquery('english', $2)
                    OR content ILIKE $3
                )
                LIMIT 5
                "#,
                tenant_id,
                query.q,
                format!("%{}%", query.q)
            ).fetch_all(&db.pool).await;

            if let Ok(records) = messages_result {
                for r in records {
                    results.push(OmniboxSearchResult {
                        entity_type: "message".to_string(),
                        id: r.id.clone(),
                        title: format!("Message from {}", r.source.unwrap_or_else(|| "Unknown".to_string())),
                        subtitle: r.content.unwrap_or_else(|| "".to_string()).chars().take(50).collect::<String>(),
                        url: format!("/inbox/messages/{}", r.id),
                    });
                }
            }

        },
        crate::db::DbStore::Sqlite(_) => {
            // Basic fallback for sqlite
        }
    }

    Json(results)
}
