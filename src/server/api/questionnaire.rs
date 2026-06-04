use axum::{
    extract::{Extension, Json, Path},
    response::IntoResponse,
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use crate::hub::Hub;
use axum::http::StatusCode;

#[derive(Deserialize, Debug)]
pub struct CreateTemplateRequest {
    pub product_id: String,
    pub title: String,
    pub questions: Vec<QuestionRequest>,
}

#[derive(Deserialize, Debug)]
pub struct QuestionRequest {
    pub r#type: String, // 'text', 'multiple_choice', 'photo_upload'
    pub text: String,
    pub is_required: bool,
    pub options: Option<serde_json::Value>,
}

#[derive(Serialize)]
pub struct TemplateResponse {
    pub id: String,
    pub product_id: String,
    pub title: String,
    pub questions: Vec<QuestionResponse>,
}

#[derive(Serialize)]
pub struct QuestionResponse {
    pub id: String,
    pub r#type: String,
    pub text: String,
    pub is_required: bool,
    pub options: Option<serde_json::Value>,
}

#[derive(Deserialize, Debug)]
pub struct SubmitIntakeRequest {
    pub product_id: String,
    pub customer_id: Option<String>,
    pub answers: Vec<AnswerRequest>,
}

#[derive(Deserialize, Debug)]
pub struct AnswerRequest {
    pub question_id: String,
    pub answer_text: Option<String>,
    pub answer_photo_url: Option<String>,
}

#[derive(Serialize)]
pub struct SubmitIntakeResponse {
    pub success: bool,
    pub submission_id: String,
}

#[derive(Serialize)]
pub struct ErrorResponse {
    pub error: String,
    pub message: String,
}

async fn handle_create_template(
    Extension(hub): Extension<Arc<Hub>>,
    Extension(claims): Extension<::server_common::Claims>,
    Json(payload): Json<CreateTemplateRequest>,
) -> impl IntoResponse {
    let tenant_id = claims.organization_id.unwrap_or_else(|| "system".to_string());

    let mut conn = match hub.pool.acquire().await {
        Ok(c) => c,
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, Json(ErrorResponse { error: "DB".into(), message: "Failed to acquire DB connection".into() })).into_response(),
    };

    let template_id = uuid::Uuid::new_v4().to_string();

    let insert_template = sqlx::query(
        "INSERT INTO questionnaire_templates (id, tenant_id, product_id, title) VALUES ($1, $2, $3, $4)"
    )
    .bind(&template_id)
    .bind(&tenant_id)
    .bind(&payload.product_id)
    .bind(&payload.title)
    .execute(&mut *conn)
    .await;

    if insert_template.is_err() {
        return (StatusCode::INTERNAL_SERVER_ERROR, Json(ErrorResponse { error: "DB".into(), message: "Failed to create template".into() })).into_response();
    }

    for q in payload.questions {
        let question_id = uuid::Uuid::new_v4().to_string();
        let _ = sqlx::query(
            "INSERT INTO questionnaire_questions (id, tenant_id, template_id, type, text, is_required, options) VALUES ($1, $2, $3, $4, $5, $6, $7)"
        )
        .bind(&question_id)
        .bind(&tenant_id)
        .bind(&template_id)
        .bind(&q.r#type)
        .bind(&q.text)
        .bind(q.is_required)
        .bind(q.options)
        .execute(&mut *conn)
        .await;
    }

    (StatusCode::OK, Json(serde_json::json!({"success": true, "template_id": template_id}))).into_response()
}

async fn handle_get_template(
    Extension(hub): Extension<Arc<Hub>>,
    Extension(claims): Extension<::server_common::Claims>,
    Path(product_id): Path<String>,
) -> impl IntoResponse {
    let tenant_id = claims.organization_id.unwrap_or_else(|| "system".to_string());

    let mut conn = match hub.pool.acquire().await {
        Ok(c) => c,
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, Json(ErrorResponse { error: "DB".into(), message: "Failed to acquire DB connection".into() })).into_response(),
    };

    use sqlx::Row;

    let template_record = sqlx::query(
        "SELECT id, title FROM questionnaire_templates WHERE tenant_id = $1 AND product_id = $2 LIMIT 1"
    )
    .bind(&tenant_id)
    .bind(&product_id)
    .fetch_optional(&mut *conn)
    .await;

    let template_row = match template_record {
        Ok(Some(t)) => t,
        _ => return (StatusCode::NOT_FOUND, Json(ErrorResponse { error: "NOT_FOUND".into(), message: "Template not found".into() })).into_response(),
    };

    let t_id: String = template_row.get("id");
    let t_title: String = template_row.get("title");

    let questions_records = sqlx::query(
        "SELECT id, type, text, is_required, options FROM questionnaire_questions WHERE tenant_id = $1 AND template_id = $2"
    )
    .bind(&tenant_id)
    .bind(&t_id)
    .fetch_all(&mut *conn)
    .await
    .unwrap_or_default();

    let questions: Vec<QuestionResponse> = questions_records.into_iter().map(|q| QuestionResponse {
        id: q.get("id"),
        r#type: q.get("type"),
        text: q.get("text"),
        is_required: q.try_get("is_required").unwrap_or(false),
        options: q.try_get("options").unwrap_or(None),
    }).collect();

    (StatusCode::OK, Json(TemplateResponse {
        id: t_id,
        product_id,
        title: t_title,
        questions,
    })).into_response()
}

async fn handle_submit_intake(
    Extension(hub): Extension<Arc<Hub>>,
    Extension(claims): Extension<::server_common::Claims>,
    Json(payload): Json<SubmitIntakeRequest>,
) -> impl IntoResponse {
    let tenant_id = claims.organization_id.unwrap_or_else(|| "system".to_string());

    let mut conn = match hub.pool.acquire().await {
        Ok(c) => c,
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, Json(ErrorResponse { error: "DB".into(), message: "Failed to acquire DB connection".into() })).into_response(),
    };

    let submission_id = uuid::Uuid::new_v4().to_string();

    // Mock AI extraction (as per plan step 4)
    let parsed_entities = serde_json::json!({
        "intent": "service_booking",
        "preferences_extracted": true,
        "mocked_quote": 1200
    });

    let insert_submission = sqlx::query(
        "INSERT INTO intake_submissions (id, tenant_id, customer_id, product_id, status, parsed_entities) VALUES ($1, $2, $3, $4, 'submitted', $5)"
    )
    .bind(&submission_id)
    .bind(&tenant_id)
    .bind(&payload.customer_id)
    .bind(&payload.product_id)
    .bind(&parsed_entities)
    .execute(&mut *conn)
    .await;

    if insert_submission.is_err() {
        return (StatusCode::INTERNAL_SERVER_ERROR, Json(ErrorResponse { error: "DB".into(), message: "Failed to create submission".into() })).into_response();
    }

    for a in payload.answers {
        let answer_id = uuid::Uuid::new_v4().to_string();
        let _ = sqlx::query(
            "INSERT INTO intake_submission_answers (id, tenant_id, submission_id, question_id, answer_text, answer_photo_url) VALUES ($1, $2, $3, $4, $5, $6)"
        )
        .bind(&answer_id)
        .bind(&tenant_id)
        .bind(&submission_id)
        .bind(&a.question_id)
        .bind(&a.answer_text)
        .bind(&a.answer_photo_url)
        .execute(&mut *conn)
        .await;
    }

    (StatusCode::OK, Json(SubmitIntakeResponse {
        success: true,
        submission_id,
    })).into_response()
}

pub fn router<S: Clone + Send + Sync + 'static>(hub: Arc<Hub>) -> Router<S> {
    Router::new()
        .route("/templates/{product_id}", get(handle_get_template))
        .route("/templates", post(handle_create_template))
        .route("/submit", post(handle_submit_intake))
        .layer(Extension(hub))
}
