use axum::{
    extract::{Extension, Multipart},
    response::IntoResponse,
    routing::post,
    Json, Router,
};
use reqwest::StatusCode;
use serde::Serialize;
use std::sync::Arc;
use crate::Hub;

#[derive(Serialize)]
pub struct AiIngestResponse {
    pub success: bool,
    pub job_id: Option<String>,
    pub message: Option<String>,
}

#[derive(Serialize)]
pub struct ErrorResponse {
    pub error: String,
    pub message: String,
}

pub async fn handle_ai_ingest(
    Extension(hub): Extension<Arc<Hub>>,
    Extension(claims): Extension<::server_common::Claims>,
    mut multipart: Multipart,
) -> impl IntoResponse {
    let tenant_id = claims
        .organization_id
        .unwrap_or_else(|| ::server_common::auth_utils::get_default_tenant());

    let mut image_data = Vec::new();
    while let Some(field) = multipart.next_field().await.unwrap_or(None) {
        if field.name() == Some("image") {
            image_data = field.bytes().await.unwrap_or_default().to_vec();
            break;
        }
    }

    if image_data.is_empty() {
        return (
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                error: "BAD_REQUEST".to_string(),
                message: "No image data provided".to_string(),
            }),
        )
            .into_response();
    }

    let base64_image = base64::Engine::encode(&base64::engine::general_purpose::STANDARD, &image_data);

    let payload = serde_json::json!({
        "image": base64_image,
        "tenant_id": tenant_id,
    });

    let queue = crate::orchestration::queue::OHCJobQueue::new(std::sync::Arc::new(hub.pool.clone()));
    let enqueue_res = queue.enqueue(&tenant_id, "AutoDreamVisionAgent", &payload).await;

    let job_id = match enqueue_res {
        Ok(id) => id,
        Err(e) => {
            tracing::error!("Failed to enqueue AI ingest job: {}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    error: "QUEUE_ERROR".to_string(),
                    message: "Failed to queue job".to_string(),
                }),
            )
                .into_response();
        }
    };

    (
        StatusCode::OK,
        Json(AiIngestResponse {
            success: true,
            job_id: Some(job_id),
            message: Some("Job queued successfully".to_string()),
        }),
    )
        .into_response()
}

pub fn router<S: Clone + Send + Sync + 'static>(hub: Arc<Hub>) -> Router<S> {
    Router::new()
        .route("/ai-ingest", post(handle_ai_ingest))
        .layer(Extension(hub))
}
