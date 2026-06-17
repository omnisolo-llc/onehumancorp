use axum::{
    extract::{State, Json},
    routing::{post},
    Router,
    http::StatusCode,
    response::IntoResponse,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use crate::db::DB;

#[derive(Deserialize)]
pub struct SetupAdminRequest {
    pub username: String,
    pub password: String,
    pub role: String,
}

#[derive(Serialize)]
pub struct SetupAdminResponse {
    pub success: bool,
    pub message: String,
}

pub fn router<S: Clone + Send + Sync + 'static>(db: Arc<DB>) -> Router<S> {
    Router::new()
        .route("/admin", post(create_initial_admin))
        .with_state(db)
}

async fn create_initial_admin(
    State(db): State<Arc<DB>>,
    Json(req): Json<SetupAdminRequest>,
) -> impl IntoResponse {
    // Basic validation
    if req.username.is_empty() || req.password.is_empty() {
        return (
            StatusCode::BAD_REQUEST,
            Json(SetupAdminResponse {
                success: false,
                message: "Username and password are required".to_string(),
            }),
        );
    }

    // Check if an admin already exists.
    // Assuming the bootstrap process uses a fixed system tenant or default tenant
    let tenant_id = "system";

    // Check if admin user exists in DB directly
    let admin_count: Result<i64, _> = match &db.store {
        crate::db::DbStore::Postgres => {
            sqlx::query_scalar("SELECT COUNT(*) FROM users WHERE 'admin' = ANY(roles)")
                .fetch_one(&db.pool)
                .await
        },
        crate::db::DbStore::Sqlite(pool) => {
            // For sqlite roles might be stored as JSON
            sqlx::query_scalar("SELECT COUNT(*) FROM users WHERE roles LIKE '%\"admin\"%'")
                .fetch_one(pool)
                .await
        }
    };

    match admin_count {
        Ok(count) if count > 0 => {
            return (
                StatusCode::CONFLICT, // bootstrap script expects 409 if already exists
                Json(SetupAdminResponse {
                    success: true,
                    message: "Admin already exists".to_string(),
                }),
            );
        }
        Err(e) => {
            tracing::error!("Failed to check for existing admin: {}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(SetupAdminResponse {
                    success: false,
                    message: "Database error".to_string(),
                }),
            );
        }
        _ => {} // No admin exists, proceed
    }

    let id = uuid::Uuid::new_v4().to_string();
    let hashed_pw = match bcrypt::hash(&req.password, 4) {
        Ok(hash) => hash,
        Err(e) => {
            tracing::error!("Failed to hash password: {}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(SetupAdminResponse {
                    success: false,
                    message: "Internal error processing password".to_string(),
                }),
            );
        }
    };
    let roles_json = serde_json::to_string(&vec![req.role.clone()]).unwrap_or_default();
    let now = chrono::Utc::now();

    let res: Result<(), String> = match &db.store {
        crate::db::DbStore::Postgres => {
            sqlx::query(
                r#"
                INSERT INTO users (id, username, email, password_hash, roles, active, tenant_id, created_at, updated_at)
                VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
                "#
            )
            .bind(&id)
            .bind(&req.username)
            .bind(&req.username) // Use username as email for admin setup if email not provided
            .bind(&hashed_pw)
            .bind(&roles_json)
            .bind(true)
            .bind(tenant_id)
            .bind(now)
            .bind(now)
            .execute(&db.pool)
            .await
            .map(|_| ())
            .map_err(|e| e.to_string())
        },
        crate::db::DbStore::Sqlite(pool) => {
            sqlx::query(
                r#"
                INSERT INTO users (id, username, email, password_hash, roles, active, tenant_id, created_at, updated_at)
                VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)
                "#
            )
            .bind(&id)
            .bind(&req.username)
            .bind(&req.username)
            .bind(&hashed_pw)
            .bind(&roles_json)
            .bind(true)
            .bind(tenant_id)
            .bind(now)
            .bind(now)
            .execute(pool)
            .await
            .map(|_| ())
            .map_err(|e| e.to_string())
        }
    };

    match res {
        Ok(_) => {
            (
                StatusCode::CREATED,
                Json(SetupAdminResponse {
                    success: true,
                    message: "Admin account created successfully".to_string(),
                }),
            )
        }
        Err(e) => {
            tracing::error!("Failed to create admin account: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(SetupAdminResponse {
                    success: false,
                    message: "Failed to create account".to_string(),
                }),
            )
        }
    }
}
