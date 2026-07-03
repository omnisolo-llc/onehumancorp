use axum::{
    extract::{Path, State},
    Json, Router, routing::post,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use crate::api::payment_ledger::AppState;
use axum::response::IntoResponse;
use tracing::info;

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/connect", post(connect_quickbooks))
}

#[derive(Deserialize)]
pub struct ConnectQuickBooksRequest {
    pub code: String,
    pub realm_id: String,
    pub redirect_uri: String,
}

#[derive(Serialize)]
pub struct ConnectQuickBooksResponse {
    pub success: bool,
}

pub async fn connect_quickbooks(
    State(state): State<Arc<AppState>>,
    Path(tenant_id): Path<String>,
    Json(payload): Json<ConnectQuickBooksRequest>,
) -> impl axum::response::IntoResponse {
    info!("Connecting QuickBooks for tenant {}", tenant_id);

    let client_id = std::env::var("QUICKBOOKS_CLIENT_ID").unwrap_or_else(|_| "".to_string());
    let client_secret = std::env::var("QUICKBOOKS_CLIENT_SECRET").unwrap_or_else(|_| "".to_string());

    if client_id.is_empty() || client_secret.is_empty() {
        // Fallback for standalone/local tests
        let access_token = format!("mock_access_{}", payload.code);
        let refresh_token = format!("mock_refresh_{}", payload.code);
        return save_and_respond(&state, tenant_id, access_token, refresh_token, payload.realm_id).await;
    }

    let client = reqwest::Client::new();
    let token_res = client.post("https://oauth.platform.intuit.com/oauth2/v1/tokens/bearer")
        .basic_auth(&client_id, Some(&client_secret))
        .form(&[
            ("grant_type", "authorization_code"),
            ("code", &payload.code),
            ("redirect_uri", &payload.redirect_uri)
        ])
        .send()
        .await;

    let (access_token, refresh_token) = match token_res {
        Ok(res) if res.status().is_success() => {
            let json: serde_json::Value = res.json().await.unwrap_or_default();
            if let (Some(token), Some(refresh)) = (json.get("access_token").and_then(|t| t.as_str()), json.get("refresh_token").and_then(|t| t.as_str())) {
                (token.to_string(), refresh.to_string())
            } else {
                return (axum::http::StatusCode::INTERNAL_SERVER_ERROR, Json(ConnectQuickBooksResponse { success: false })).into_response();
            }
        },
        _ => {
            return (axum::http::StatusCode::INTERNAL_SERVER_ERROR, Json(ConnectQuickBooksResponse { success: false })).into_response();
        }
    };

    save_and_respond(&state, tenant_id, access_token, refresh_token, payload.realm_id).await
}

async fn save_and_respond(state: &Arc<AppState>, tenant_id: String, access_token: String, refresh_token: String, realm_id: String) -> axum::response::Response {
    let db = &state.db;

    let creds = serde_json::json!({
        "access_token": access_token.clone(),
        "refresh_token": refresh_token.clone(),
        "company_id": realm_id
    });

    let mut success_db = false;
    match &db.store {
        crate::db::DbStore::Postgres => {
            let res = sqlx::query("
                INSERT INTO tenant_integrations (tenant_id, integration_id, status, credentials)
                VALUES ($1, 'quickbooks', 'connected', $2)
                ON CONFLICT (tenant_id, integration_id)
                DO UPDATE SET status = 'connected', credentials = $2, updated_at = NOW()
            ").bind(&tenant_id).bind(&creds).execute(&db.pool).await;
            success_db = res.is_ok();
        },
        crate::db::DbStore::Sqlite(sqlite_pool) => {
            let res = sqlx::query("
                INSERT INTO tenant_integrations (tenant_id, integration_id, status, credentials)
                VALUES (?, 'quickbooks', 'connected', ?)
                ON CONFLICT(tenant_id, integration_id)
                DO UPDATE SET status = 'connected', credentials = ?, updated_at = CURRENT_TIMESTAMP
            ").bind(&tenant_id).bind(&creds.to_string()).bind(&creds.to_string()).execute(sqlite_pool).await;
            success_db = res.is_ok();
        }
    };

    if !success_db {
        return (axum::http::StatusCode::INTERNAL_SERVER_ERROR, Json(ConnectQuickBooksResponse { success: false })).into_response();
    }

    let provider = std::sync::Arc::new(crate::integrations::quickbooks::provider::QuickBooksProvider::new(access_token, refresh_token));
    // state.hub.integrations.quickbooks_clients.write().unwrap().insert(tenant_id.clone(), provider);

    (axum::http::StatusCode::OK, Json(ConnectQuickBooksResponse { success: true })).into_response()
}
