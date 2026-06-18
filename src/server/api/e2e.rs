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
pub struct SetupRequest {
    pub query: String,
}

#[derive(Serialize)]
pub struct SetupResponse {
    pub success: bool,
    pub message: String,
}

pub fn router<S: Clone + Send + Sync + 'static>(db: Arc<DB>) -> Router<S> {
    Router::new()
        .route("/setup", post(run_setup_query))
        .with_state(db)
}

async fn run_setup_query(
    State(db): State<Arc<DB>>,
    Json(req): Json<SetupRequest>,
) -> impl IntoResponse {
    let res: Result<(), String> = match &db.store {
        crate::db::DbStore::Postgres => {
            sqlx::query(&req.query)
                .execute(&db.pool)
                .await
                .map(|_| ())
                .map_err(|e| e.to_string())
        },
        crate::db::DbStore::Sqlite(pool) => {
            sqlx::query(&req.query)
                .execute(pool)
                .await
                .map(|_| ())
                .map_err(|e| e.to_string())
        }
    };

    match res {
        Ok(_) => {
            (
                StatusCode::OK,
                Json(SetupResponse {
                    success: true,
                    message: "Query executed successfully".to_string(),
                }),
            )
        }
        Err(e) => {
            tracing::error!("Failed to execute query: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(SetupResponse {
                    success: false,
                    message: e,
                }),
            )
        }
    }
}
