use axum::{extract::State, extract::Json, extract::Path, routing::post, Router};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use crate::db::DB;
use axum::http::StatusCode;
use axum::response::IntoResponse;

#[derive(Deserialize)]
pub struct SubmitFeedbackReq {
    pub rating: i32,
    pub review_text: Option<String>,
}

#[derive(Serialize)]
pub struct SubmitFeedbackResp {
    pub success: bool,
    pub action: String, // e.g. "redirect_google", "triaged"
}

pub async fn submit_feedback(
    State(db): State<Arc<DB>>,
    Path(tenant_id): Path<String>,
    Path(customer_id): Path<String>,
    Json(payload): Json<SubmitFeedbackReq>,
) -> impl IntoResponse {
    let pool = &db.pool;

    let review_id = uuid::Uuid::new_v4().to_string();
    let mut status = "published";
    let mut action = "redirect_google";

    if payload.rating <= 3 {
        status = "triaged";
        action = "triaged";
        // Here we'd also generate a Triage Action card in the Owner Feed via Agent
        tracing::info!("Intercepted negative review: {} stars", payload.rating);
    }

    let result = sqlx::query(
        r#"
        INSERT INTO customer_reviews (id, tenant_id, customer_id, rating, review_text, status)
        VALUES ($1, $2, $3, $4, $5, $6)
        "#
    )
    .bind(&review_id)
    .bind(&tenant_id)
    .bind(&customer_id)
    .bind(payload.rating)
    .bind(&payload.review_text)
    .bind(status)
    .execute(pool)
    .await;

    match result {
        Ok(_) => {
            (StatusCode::OK, Json(SubmitFeedbackResp { success: true, action: action.to_string() })).into_response()
        },
        Err(e) => {
            tracing::error!("Failed to submit feedback: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, "Internal Error").into_response()
        }
    }
}

pub fn reputation_routes(db: Arc<DB>) -> Router {
    Router::new()
        .route("/api/reputation/:tenant_id/feedback/:customer_id", post(submit_feedback))
        .with_state(db)
}
