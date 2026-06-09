use axum::{
    extract::{Query, State},
    response::IntoResponse,
    Json,
};
use serde::{Deserialize, Serialize};
use sqlx::{PgPool, Row};
use std::sync::Arc;

#[derive(Deserialize)]
pub struct SearchQuery {
    pub q: String,
}

#[derive(Serialize)]
pub struct SearchResultItem {
    pub entity_type: String,
    pub id: String,
    pub title: String,
    pub subtitle: String,
    pub url_path: String,
}

#[derive(Serialize)]
pub struct SearchResponse {
    pub results: Vec<SearchResultItem>,
}

pub async fn search_handler(
    State(pool): State<PgPool>,
    axum::extract::Extension(claims): axum::extract::Extension<::server_common::Claims>,
    Query(params): Query<SearchQuery>,
) -> impl IntoResponse {
    let tenant_id = claims.organization_id.unwrap_or_default();
    let query_str = format!("%{}%", params.q);

    // SQL query that searches across customers, orders, and messages
    let query = r#"
        SELECT
            'customer' as entity_type,
            id::text,
            name::text as title,
            COALESCE(email, '')::text as subtitle,
            CONCAT('/customers/', id) as url_path,
            updated_at
        FROM customers
        WHERE tenant_id = $1 AND (name ILIKE $2 OR email ILIKE $2)

        UNION ALL

        SELECT
            'order' as entity_type,
            id::text,
            id::text as title,
            COALESCE(status, '')::text as subtitle,
            CONCAT('/orders/', id) as url_path,
            updated_at
        FROM orders
        WHERE tenant_id = $1 AND id::text ILIKE $2

        UNION ALL

        SELECT
            'message' as entity_type,
            id::text,
            COALESCE(source, 'Message')::text as title,
            SUBSTRING(COALESCE(content, ''), 1, 100)::text as subtitle,
            CONCAT('/triage?message=', id) as url_path,
            created_at as updated_at
        FROM inbox_messages
        WHERE tenant_id = $1 AND (content ILIKE $2 OR draft_reply ILIKE $2)

        ORDER BY updated_at DESC
        LIMIT 20;
    "#;

    let rows_result = sqlx::query(query)
        .bind(&tenant_id)
        .bind(&query_str)
        .fetch_all(&pool)
        .await;

    match rows_result {
        Ok(rows) => {
            let results: Vec<SearchResultItem> = rows
                .into_iter()
                .map(|row| SearchResultItem {
                    entity_type: row.get("entity_type"),
                    id: row.get("id"),
                    title: row.get("title"),
                    subtitle: row.get("subtitle"),
                    url_path: row.get("url_path"),
                })
                .collect();

            Json(SearchResponse { results }).into_response()
        }
        Err(e) => {
            tracing::error!("Search error: {:?}", e);
            (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({ "error": "Internal server error" })),
            )
                .into_response()
        }
    }
}

pub fn router() -> axum::Router<PgPool> {
    axum::Router::new().route("/", axum::routing::get(search_handler))
}

#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::PgPool;
    use axum::{
        body::Body,
        http::{Request, StatusCode},
    };
    use tower::ServiceExt;

    #[tokio::test]
    async fn test_search_handler_success() {
        let database_url = std::env::var("OHC_DATABASE_URL").unwrap_or_else(|_| "postgres://ohc:ohc@localhost:5432/ohc".to_string());
        let pool_res = sqlx::postgres::PgPoolOptions::new()
            .acquire_timeout(std::time::Duration::from_millis(500))
            .connect(&database_url)
            .await;

        let pool = match pool_res {
            Ok(p) => p,
            Err(_) => return, // Skip test if no DB
        };

        if sqlx::query("SELECT 1").execute(&pool).await.is_err() {
            return;
        }

        let tenant_id = "test-tenant-search-123".to_string();

        let req = Request::builder()
            .uri("/?q=john")
            .method("GET")
            .extension(::server_common::Claims {
                sub: "user-123".to_string(),
                username: "user".to_string(),
                iat: 0,
                exp: 9999999999,
                organization_id: Some(tenant_id.clone()),
                email: "user@example.com".to_string(),
                roles: vec![],
                jti: "".to_string(),
                session_id: None,
            })
            .body(Body::empty())
            .unwrap();

        let router = router().with_state(pool);
        let res = router.oneshot(req).await.unwrap();

        assert_eq!(res.status(), StatusCode::OK);
    }
}
