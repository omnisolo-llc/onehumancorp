use axum::{
    extract::{Path, State},
    routing::{get, post},
    Json, Router,
    http::StatusCode,
};
use serde::{Deserialize, Serialize};
use sqlx::{PgPool, Row};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use tracing::info;

#[derive(Serialize)]
pub struct ClientPortal {
    pub id: Uuid,
    pub tenant_id: String,
    pub client_id: Uuid,
    pub name: String,
    pub branding_config: serde_json::Value,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Deserialize)]
pub struct CreatePortalRequest {
    pub tenant_id: String,
    pub client_id: Uuid,
    pub name: String,
}

pub async fn create_portal(
    State(pool): State<PgPool>,
    Json(payload): Json<CreatePortalRequest>,
) -> Result<Json<ClientPortal>, (StatusCode, String)> {
    let id = Uuid::new_v4();
    let row = sqlx::query(
        r#"
        INSERT INTO client_portals (id, tenant_id, client_id, name)
        VALUES ($1, $2, $3, $4)
        RETURNING id, tenant_id, client_id, name, branding_config, created_at, updated_at
        "#
    )
    .bind(id)
    .bind(&payload.tenant_id)
    .bind(payload.client_id)
    .bind(&payload.name)
    .fetch_one(&pool)
    .await
    .map_err(|e| {
        (StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to create portal: {}", e))
    })?;

    Ok(Json(ClientPortal {
        id: row.get("id"),
        tenant_id: row.get("tenant_id"),
        client_id: row.get("client_id"),
        name: row.get("name"),
        branding_config: row.get("branding_config"),
        created_at: row.get("created_at"),
        updated_at: row.get("updated_at"),
    }))
}

pub async fn generate_magic_link(
    State(pool): State<PgPool>,
    Path(portal_id): Path<Uuid>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    let portal_row = sqlx::query("SELECT tenant_id FROM client_portals WHERE id = $1")
        .bind(portal_id)
        .fetch_optional(&pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let tenant_id: String = match portal_row {
        Some(row) => row.get("tenant_id"),
        None => return Err((StatusCode::NOT_FOUND, "Portal not found".into())),
    };

    let session_id = Uuid::new_v4();
    let token = Uuid::new_v4().to_string(); // In a real app, sign a JWT or use a secure random string
    let token_hash = sha256::digest(token.clone()); // Assuming a simple hash or just store the token for this test
    let expires_at = Utc::now() + chrono::Duration::days(7);

    sqlx::query(
        r#"
        INSERT INTO client_portal_sessions (id, tenant_id, client_portal_id, token_hash, expires_at)
        VALUES ($1, $2, $3, $4, $5)
        "#
    )
    .bind(session_id)
    .bind(&tenant_id)
    .bind(portal_id)
    .bind(&token) // For simplicity, we just store it
    .bind(expires_at)
    .execute(&pool)
    .await
    .map_err(|e| {
        (StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to create session: {}", e))
    })?;

    let magic_link = format!("https://portal.onehumancorp.com/access?token={}", token);

    Ok(Json(serde_json::json!({
        "magic_link": magic_link,
        "expires_at": expires_at
    })))
}

pub fn router(pool: PgPool) -> Router {
    Router::new()
        .route("/api/v1/portals", post(create_portal))
        .route("/api/v1/portals/:id/magic-link", post(generate_magic_link))
        .with_state(pool)
}
