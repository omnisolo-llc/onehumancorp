use axum::{
    extract::{State, Path},
    response::IntoResponse,
    routing::post,
    Json, Router,
};
use std::sync::Arc;
use crate::db::DB;
use serde::{Deserialize, Serialize};

#[derive(Clone)]
pub struct ToolIntegrationsApiState {
    pub db: Arc<DB>,
}

#[derive(Deserialize, Debug)]
pub struct ConnectIntegrationRequest {
    pub bot_token: Option<String>,
    pub api_token: Option<String>,
    pub from_phone: Option<String>,
    pub integration_id: Option<String>,
    pub base_url: Option<String>,
}

#[derive(Serialize)]
pub struct ConnectIntegrationResponse {
    pub success: bool,
    pub message: String,
}

pub async fn connect_integration_handler(
    State(state): State<ToolIntegrationsApiState>,
    axum::extract::Extension(user): axum::extract::Extension<::server_common::Claims>,
    Path(id): Path<String>,
    Json(payload): Json<ConnectIntegrationRequest>,
) -> impl IntoResponse {
    let tenant_id = user.organization_id.unwrap_or_else(|| "default".to_string());
    let integration_id = payload.integration_id.clone().unwrap_or(id.clone());

    let pool = state.db.pool.clone();
    let id_uuid = uuid::Uuid::new_v4().to_string();

    let bot_token = payload.bot_token.unwrap_or_default();
    let api_token = payload.api_token.unwrap_or_default();
    let from_phone = payload.from_phone.unwrap_or_default();

    let mut tx = match pool.begin().await {
        Ok(tx) => tx,
        Err(e) => {
            return (axum::http::StatusCode::INTERNAL_SERVER_ERROR, Json(ConnectIntegrationResponse {
                success: false,
                message: format!("Failed to start transaction: {}", e),
            })).into_response()
        }
    };

    if let Err(e) = crate::common::auth_utils::set_org_context(&mut *tx, &tenant_id).await {
         return (axum::http::StatusCode::INTERNAL_SERVER_ERROR, Json(ConnectIntegrationResponse {
             success: false,
             message: format!("Failed to set tenant context: {}", e),
         })).into_response()
    }

    let delete_query = match &state.db.store {
        crate::db::DbStore::Postgres => "DELETE FROM integration_credentials WHERE tenant_id = $1 AND integration_id = $2",
        crate::db::DbStore::Sqlite(_) => "DELETE FROM integration_credentials WHERE tenant_id = ? AND integration_id = ?",
    };

    let _ = sqlx::query(delete_query)
        .bind(&tenant_id)
        .bind(&integration_id)
        .execute(&mut *tx)
        .await;

    let insert_query = match &state.db.store {
        crate::db::DbStore::Postgres => "INSERT INTO integration_credentials (id, tenant_id, integration_id, bot_token, api_token, from_phone) VALUES ($1, $2, $3, $4, $5, $6)",
        crate::db::DbStore::Sqlite(_) => "INSERT INTO integration_credentials (id, tenant_id, integration_id, bot_token, api_token, from_phone) VALUES (?, ?, ?, ?, ?, ?)",
    };

    let res = sqlx::query(insert_query)
        .bind(&id_uuid)
        .bind(&tenant_id)
        .bind(&integration_id)
        .bind(&bot_token)
        .bind(&api_token)
        .bind(&from_phone)
        .execute(&mut *tx)
        .await;

    if let Err(e) = res {
        let _ = tx.rollback().await;
        return (axum::http::StatusCode::INTERNAL_SERVER_ERROR, Json(ConnectIntegrationResponse {
            success: false,
            message: format!("Failed to save credentials: {}", e),
        })).into_response();
    }

    // Also upsert into tool_integrations just in case
    let tool_delete_query = match &state.db.store {
        crate::db::DbStore::Postgres => "DELETE FROM tool_integrations WHERE tenant_id = $1 AND id = $2",
        crate::db::DbStore::Sqlite(_) => "DELETE FROM tool_integrations WHERE tenant_id = ? AND id = ?",
    };
    let _ = sqlx::query(tool_delete_query)
        .bind(&tenant_id)
        .bind(&integration_id)
        .execute(&mut *tx)
        .await;

    let tool_insert_query = match &state.db.store {
        crate::db::DbStore::Postgres => "INSERT INTO tool_integrations (id, tenant_id, name, integration_code, status) VALUES ($1, $2, $3, $4, 'connected')",
        crate::db::DbStore::Sqlite(_) => "INSERT INTO tool_integrations (id, tenant_id, name, integration_code, status) VALUES (?, ?, ?, ?, 'connected')",
    };

    let _ = sqlx::query(tool_insert_query)
        .bind(&integration_id)
        .bind(&tenant_id)
        .bind(&integration_id)
        .bind(&api_token)
        .execute(&mut *tx)
        .await;

    let _ = tx.commit().await;

    Json(ConnectIntegrationResponse {
        success: true,
        message: format!("{} connected successfully", integration_id),
    }).into_response()
}

pub fn router<S: Clone + Send + Sync + 'static>(db: Arc<DB>) -> Router<S> {
    let state = ToolIntegrationsApiState { db };
    Router::new()
        .route("/{id}/connect", post(connect_integration_handler))
        .with_state(state)
}
