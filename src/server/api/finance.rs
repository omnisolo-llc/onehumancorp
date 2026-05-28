use axum::{
    extract::{State, Json},
    routing::post,
    Router,
};
use std::sync::Arc;
use crate::db::DB;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize)]
pub struct ReceiptUploadRequest {
    pub image_data_base64: String, // Or image url, depends on the client
    pub filename: Option<String>,
}

#[derive(Serialize)]
pub struct ReceiptUploadResponse {
    pub status: String,
    pub job_id: String,
}

pub fn router(db: Arc<DB>) -> Router<Arc<dyn crate::orchestration::mesh::TeammateMesh>> {
    let app = Router::new()
        .route("/receipts/upload", post(upload_receipt_handler))
        .with_state(db);

    Router::new().merge(app)
}

async fn upload_receipt_handler(
    State(db): State<Arc<DB>>,
    request: axum::extract::Request,
) -> Result<Json<ReceiptUploadResponse>, axum::http::StatusCode> {
    let tenant_id = match request.extensions().get::<::server_auth::orchestration::AuthInfo>() {
        Some(auth) => {
            if auth.org_id.is_empty() {
                return Err(axum::http::StatusCode::UNAUTHORIZED);
            } else {
                auth.org_id.clone()
            }
        },
        None => return Err(axum::http::StatusCode::UNAUTHORIZED)
    };

    // Extract payload from request
    let body_bytes = match axum::body::to_bytes(request.into_body(), usize::MAX).await {
        Ok(b) => b,
        Err(_) => return Err(axum::http::StatusCode::BAD_REQUEST),
    };

    let payload: ReceiptUploadRequest = match serde_json::from_slice(&body_bytes) {
        Ok(p) => p,
        Err(_) => return Err(axum::http::StatusCode::BAD_REQUEST),
    };

    let job_id = Uuid::new_v4().to_string();

    // Create a task for the Finance Agent to process this receipt
    let task_payload = serde_json::json!({
        "image_data": payload.image_data_base64,
        "filename": payload.filename,
    });

    let res = sqlx::query(
        "INSERT INTO tasks (id, tenant_id, title, description, status, payload, assigned_agent_id)
         VALUES ($1, $2, 'Process Receipt', 'Extract data from uploaded receipt', 'PENDING', $3, (SELECT id FROM agents WHERE role = 'finance' AND tenant_id = $2 LIMIT 1))"
    )
    .bind(&job_id)
    .bind(tenant_id)
    .bind(task_payload)
    .execute(&db.pool)
    .await;

    match res {
        Ok(_) => Ok(Json(ReceiptUploadResponse {
            status: "Processing".to_string(),
            job_id,
        })),
        Err(_) => Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR),
    }
}
