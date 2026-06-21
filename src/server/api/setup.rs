use serde::{Deserialize};
use crate::db::DB;
use std::sync::Arc;

#[derive(Deserialize, Debug)]
pub struct WhatsAppCloudApiIntegrationRequest {
    pub integration_id: String,
}

pub async fn save_whatsapp_cloud_api_integration_handler(
    axum::extract::State(db): axum::extract::State<Arc<DB>>,
    axum::extract::Extension(user): axum::extract::Extension<::server_common::Claims>,
    axum::Json(_payload): axum::Json<WhatsAppCloudApiIntegrationRequest>,
) -> impl axum::response::IntoResponse {
    let tenant_id = user.organization_id.unwrap_or_else(|| "test_tenant".to_string());

    // In a real OAuth flow this would process the token from Meta.
    // For this requirements we just simulate the connection and store the record.
    let integration_id = uuid::Uuid::new_v4().to_string();
    let name = "WhatsApp Cloud API";
    let integration_code = serde_json::json!({
        "status": "connected",
        "access_token": "simulated_token_from_meta", // Normally we would get this via OAuth
        "phone_number": "15555555555" // Normally from the Meta onboarding
    });

    let status = "connected";

    let res = match &db.store {
        crate::db::DbStore::Postgres => {
            sqlx::query(
                "INSERT INTO tool_integrations (id, tenant_id, name, integration_code, status) VALUES ($1, $2, $3, $4, $5) ON CONFLICT(id) DO UPDATE SET integration_code = $4, status = $5"
            )
            .bind(&integration_id)
            .bind(&tenant_id)
            .bind(name)
            .bind(integration_code.to_string())
            .bind(status)
            .execute(&db.pool)
            .await
        },
        crate::db::DbStore::Sqlite(pool) => {
            sqlx::query(
                "INSERT INTO tool_integrations (id, tenant_id, name, integration_code, status) VALUES (?, ?, ?, ?, ?) ON CONFLICT(id) DO UPDATE SET integration_code = ?, status = ?"
            )
            .bind(&integration_id)
            .bind(&tenant_id)
            .bind(name)
            .bind(integration_code.to_string())
            .bind(status)
            .bind(integration_code.to_string())
            .bind(status)
            .execute(pool)
            .await
        }
    };

    match res {
        Ok(_) => {
            (axum::http::StatusCode::OK, axum::Json(serde_json::json!({"success": true, "message": "WhatsApp Cloud API connected successfully"}))).into_response()
        },
        Err(e) => {
            tracing::error!("Failed to save WhatsApp Cloud API integration: {:?}", e);
            (axum::http::StatusCode::INTERNAL_SERVER_ERROR, axum::Json(serde_json::json!({"error": e.to_string()}))).into_response()
        }
    }
}
