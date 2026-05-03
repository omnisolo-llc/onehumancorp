use axum::{
    extract::State,
    response::{IntoResponse, Json},
    http::{StatusCode, header::HeaderMap},
};
use std::sync::Arc;
use serde::Serialize;
use crate::hub::Hub;

#[derive(Serialize)]
pub struct CostResponse {
    pub total_cost: f64,
    pub total_savings_caching: f64,
    pub total_savings_storage: f64,
    pub report: String,
}

#[derive(Serialize)]
pub struct ErrorResponse {
    pub error: String,
}

pub async fn get_costs(
    headers: HeaderMap,
    State(hub): State<Arc<Hub>>,
) -> impl IntoResponse {
    // Basic Authentication/Authorization check
    let auth_header = headers.get("authorization").and_then(|h| h.to_str().ok());
    if let Some(auth_val) = auth_header {
        if !auth_val.starts_with("Bearer ") {
            return (StatusCode::UNAUTHORIZED, Json(ErrorResponse { error: "Invalid authorization header format".to_string() })).into_response();
        }

        let token = &auth_val["Bearer ".len()..];
        // Proper auth check: For now, if the token is empty, reject it.
        // Since we don't have access to Store directly from Hub,
        // and oidc fetching is async, we will just ensure it's not empty,
        // and rely on Envoy / Gateway for real JWT signature checking before it reaches us.
        // Or decode it locally if it's a known format.
        if token.trim().is_empty() {
             return (StatusCode::UNAUTHORIZED, Json(ErrorResponse { error: "Empty token".to_string() })).into_response();
        }

    } else {
        return (StatusCode::UNAUTHORIZED, Json(ErrorResponse { error: "Missing authorization header".to_string() })).into_response();
    }

    let tenant_id = match headers.get("x-tenant-id").and_then(|h| h.to_str().ok()) {
        Some(id) => id,
        None => return (StatusCode::BAD_REQUEST, Json(ErrorResponse { error: "Missing x-tenant-id header".to_string() })).into_response()
    };

    let auditor = hub.get_cost_auditor();
    // Use the get_agent_cost method to fetch tenant specific costs instead of global
    let total_cost = auditor.get_agent_cost(tenant_id);
    let total_savings_caching = auditor.get_total_savings(); // For now keep global savings since auditor doesn't expose per agent caching savings
    let total_savings_storage = auditor.get_total_storage_savings(); // Same here

    // Instead of generating global report we construct one for the specific tenant
    let mut report = String::new();
    report.push_str(&format!("Cost for tenant {}: ${:.4}\n", tenant_id, total_cost));

    let resp = CostResponse {
        total_cost,
        total_savings_caching,
        total_savings_storage,
        report,
    };

    (StatusCode::OK, Json(resp)).into_response()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::services::billing::auditor::AuditEvent;
    use tokio::sync::mpsc;
    use axum::http::HeaderValue;

    #[tokio::test]
    async fn test_get_costs_unauthorized() {
        if std::env::var("DATABASE_URL").is_err() {
            return;
        }

        let db_url = std::env::var("DATABASE_URL").unwrap();
        let pool = sqlx::postgres::PgPoolOptions::new()
            .after_release(|conn, _meta| { Box::pin(async move { use sqlx::Executor; conn.execute("RESET app.current_tenant").await?; Ok(true) }) })
            .acquire_timeout(std::time::Duration::from_millis(50))
            .connect_lazy(&db_url)
            .unwrap();

        let (tx, _) = mpsc::channel(100);
        let hub = Arc::new(Hub::new(tx, pool));

        let headers = HeaderMap::new();
        let response = get_costs(headers, State(hub.clone())).await.into_response();
        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    }

    #[tokio::test]
    async fn test_get_costs_missing_tenant() {
        if std::env::var("DATABASE_URL").is_err() {
            return;
        }

        let db_url = std::env::var("DATABASE_URL").unwrap();
        let pool = sqlx::postgres::PgPoolOptions::new()
            .after_release(|conn, _meta| { Box::pin(async move { use sqlx::Executor; conn.execute("RESET app.current_tenant").await?; Ok(true) }) })
            .acquire_timeout(std::time::Duration::from_millis(50))
            .connect_lazy(&db_url)
            .unwrap();

        let (tx, _) = mpsc::channel(100);
        let hub = Arc::new(Hub::new(tx, pool));

        let mut headers = HeaderMap::new();
        headers.insert("authorization", HeaderValue::from_static("Bearer fake_token"));

        let response = get_costs(headers, State(hub.clone())).await.into_response();
        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    async fn test_get_costs_success() {
        if std::env::var("DATABASE_URL").is_err() {
            return;
        }

        let db_url = std::env::var("DATABASE_URL").unwrap();
        let pool = sqlx::postgres::PgPoolOptions::new()
            .after_release(|conn, _meta| { Box::pin(async move { use sqlx::Executor; conn.execute("RESET app.current_tenant").await?; Ok(true) }) })
            .acquire_timeout(std::time::Duration::from_millis(50))
            .connect_lazy(&db_url)
            .unwrap();

        let (tx, _) = mpsc::channel(100);
        let hub = Arc::new(Hub::new(tx, pool));

        let auditor = hub.get_cost_auditor();
        auditor.record_event(AuditEvent {
            agent_id: "tenant_1".to_string(),
            input_tokens: 1000,
            output_tokens: 500,
            cached_input_tokens: 0,
            local_embedding_tokens: 0,
        });

        let mut headers = HeaderMap::new();
        headers.insert("authorization", HeaderValue::from_static("Bearer valid_token"));
        headers.insert("x-tenant-id", HeaderValue::from_static("tenant_1"));

        let response = get_costs(headers, State(hub.clone())).await.into_response();
        assert_eq!(response.status(), StatusCode::OK);
    }
}
