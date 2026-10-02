//! Authenticated search over the same tenant-owned records shown in the UI.
use axum::{
    Json, Router,
    extract::{Extension, Query, State},
    http::{StatusCode, header},
    response::{IntoResponse, Response},
    routing::get,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SearchQuery {
    pub q: String,
    pub limit: Option<i64>,
}

#[derive(Serialize)]
struct SearchResult {
    id: String,
    entity_type: String,
    title: String,
    subtitle: String,
    route: String,
}

#[derive(sqlx::FromRow)]
struct SearchRow {
    id: String,
    entity_type: String,
    title: String,
    subtitle: String,
}

pub fn router(pool: sqlx::PgPool, auth_store: Arc<::server_auth::Store>) -> Router {
    Router::new()
        .route("/api/v1/search", get(search_handler))
        .route_layer(axum::middleware::from_fn_with_state(
            auth_store,
            ::server_auth::strict_bearer_auth_middleware,
        ))
        .with_state(pool)
}

fn failure(status: StatusCode, message: &'static str) -> Response {
    (
        status,
        [(header::CACHE_CONTROL, "private, no-store")],
        Json(serde_json::json!({"success": false, "error": message})),
    )
        .into_response()
}

async fn search_handler(
    State(pool): State<sqlx::PgPool>,
    Extension(claims): Extension<::server_common::Claims>,
    params: Result<Query<SearchQuery>, axum::extract::rejection::QueryRejection>,
) -> Response {
    let Some(tenant_id) = ::server_common::auth_utils::signed_tenant_id(&claims) else {
        return failure(
            StatusCode::FORBIDDEN,
            "A signed business identity is required",
        );
    };
    let Ok(Query(params)) = params else {
        return failure(StatusCode::BAD_REQUEST, "Invalid search query");
    };
    let query = params.q.trim();
    let limit = params.limit.unwrap_or(20);
    if !(2..=200).contains(&query.chars().count()) || !(1..=50).contains(&limit) {
        return failure(StatusCode::BAD_REQUEST, "Invalid search query");
    }
    // Search is literal text, not a user-supplied LIKE pattern or SQL fragment.
    let pattern = format!(
        "%{}%",
        query
            .replace('\\', "\\\\")
            .replace('%', "\\%")
            .replace('_', "\\_")
    );
    match search_records(&pool, &tenant_id, &pattern, limit).await {
        Ok(results) => (
            [(header::CACHE_CONTROL, "private, no-store")],
            Json(serde_json::json!({"success": true, "results": results})),
        )
            .into_response(),
        Err(_) => {
            // SQL errors can contain private search values; do not log them.
            tracing::warn!("Tenant search database read failed");
            failure(
                StatusCode::SERVICE_UNAVAILABLE,
                "Search is temporarily unavailable",
            )
        }
    }
}

async fn search_records(
    pool: &sqlx::PgPool,
    tenant: &str,
    pattern: &str,
    limit: i64,
) -> Result<Vec<SearchResult>, sqlx::Error> {
    let mut tx = pool.begin().await?;
    sqlx::query("SET TRANSACTION READ ONLY")
        .execute(&mut *tx)
        .await?;
    ::server_common::auth_utils::set_org_context(&mut *tx, tenant).await?;
    sqlx::query("SET LOCAL statement_timeout = '2s'")
        .execute(&mut *tx)
        .await?;
    let rows = sqlx::query_as::<_, SearchRow>(
        r#"SELECT id, entity_type, title, subtitle FROM (
            SELECT c.id::text AS id, 'customer' AS entity_type,
                   LEFT(c.name, 200) AS title,
                   LEFT(COALESCE(c.email, c.phone, ''), 240) AS subtitle, 0 AS category
            FROM customers c
            WHERE c.tenant_id::text = $1
              AND (c.name ILIKE $2 ESCAPE '\' OR c.email ILIKE $2 ESCAPE '\'
                   OR c.phone ILIKE $2 ESCAPE '\')
            UNION ALL
            SELECT o.id::text, 'order', LEFT('Order ' || o.id::text, 200),
                   LEFT(concat_ws(' · ', NULLIF(c.name, ''), NULLIF(o.status, '')), 240), 1
            FROM orders o
            LEFT JOIN customers c ON c.id = o.customer_id AND c.tenant_id = o.tenant_id
            WHERE o.tenant_id::text = $1
              AND (o.id::text ILIKE $2 ESCAPE '\' OR c.name ILIKE $2 ESCAPE '\'
                   OR o.status ILIKE $2 ESCAPE '\')
            UNION ALL
            SELECT m.id::text, 'message',
                   LEFT(COALESCE(NULLIF(m.sender_id, ''), m.source), 200),
                   LEFT(COALESCE(NULLIF(m.translated_content, ''), m.original_content), 240), 2
            FROM omni_inbox_messages m
            WHERE m.tenant_id::text = $1
              AND (m.sender_id ILIKE $2 ESCAPE '\' OR m.source ILIKE $2 ESCAPE '\'
                   OR m.original_content ILIKE $2 ESCAPE '\'
                   OR m.translated_content ILIKE $2 ESCAPE '\')
        ) records ORDER BY category, id LIMIT $3"#,
    )
    .bind(tenant)
    .bind(pattern)
    .bind(limit)
    .fetch_all(&mut *tx)
    .await?;
    tx.commit().await?;
    rows.into_iter()
        .map(|row| {
            // The maintained customer destination accepts at most 200 characters.
            // Dot-only path segments are normalized by browsers before routing.
            if row.id.is_empty()
                || row.id.chars().count() > 200
                || (row.entity_type == "order" && matches!(row.id.as_str(), "." | ".."))
            {
                return Err(sqlx::Error::Decode(
                    "Search record identifier cannot be routed".into(),
                ));
            }
            let encoded = urlencoding::encode(&row.id);
            let route = match row.entity_type.as_str() {
                "customer" => format!("/customer/memory-graph?customerId={encoded}"),
                "order" => format!("/orders/{encoded}"),
                // This category is a SQL literal in the query above.
                _ => format!("/inbox?messageId={encoded}"),
            };
            Ok(SearchResult {
                id: row.id,
                entity_type: row.entity_type,
                title: row.title,
                subtitle: row.subtitle,
                route,
            })
        })
        .collect()
}
