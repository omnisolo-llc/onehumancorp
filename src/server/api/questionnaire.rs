use axum::{
    extract::{Path, State, Query},
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use sqlx::PgPool;

pub fn router(pool: PgPool) -> Router {
    Router::new()
        .route("/", post(create_questionnaire))
        .route("/:id", get(get_questionnaire))
        .route("/:id/submit", post(submit_intake))
        .route("/submissions/:id", get(get_submission))
        .route("/submissions", get(list_submissions))
        .with_state(pool)
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Question {
    pub id: String,
    pub template_id: String,
    pub r#type: String,
    pub text: String,
    pub is_required: bool,
}

#[derive(Serialize, Deserialize)]
pub struct QuestionnaireTemplate {
    pub id: String,
    pub tenant_id: String,
    pub service_id: Option<String>,
    pub title: String,
    pub status: String,
    pub questions: Vec<Question>,
}

#[derive(Deserialize)]
pub struct CreateQuestionnaireRequest {
    pub title: String,
    pub service_id: Option<String>,
}

async fn create_questionnaire(
    State(pool): State<PgPool>,
    headers: axum::http::HeaderMap,
    Json(payload): Json<CreateQuestionnaireRequest>,
) -> Result<Json<QuestionnaireTemplate>, axum::http::StatusCode> {
    let tenant_id = headers.get("x-tenant-id")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("default")
        .to_string();

    let template_id = uuid::Uuid::new_v4().to_string();

    let mut tx = pool.begin().await.map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;
    sqlx::query("SELECT set_config('app.current_tenant', $1, true)")
        .bind(&tenant_id)
        .execute(&mut *tx)
        .await
        .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;

    sqlx::query(
        "INSERT INTO questionnaire_templates (id, tenant_id, service_id, title, status) VALUES ($1, $2, $3, $4, $5)"
    )
    .bind(&template_id)
    .bind(&tenant_id)
    .bind(&payload.service_id)
    .bind(&payload.title)
    .bind("draft")
    .execute(&mut *tx)
    .await
    .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;

    let questions = vec![
        Question {
            id: uuid::Uuid::new_v4().to_string(),
            template_id: template_id.clone(),
            r#type: "text".to_string(),
            text: "What are the dimensions of the room?".to_string(),
            is_required: true,
        },
        Question {
            id: uuid::Uuid::new_v4().to_string(),
            template_id: template_id.clone(),
            r#type: "multiple_choice".to_string(),
            text: "What type of material do you prefer? (e.g. Hardwood, Laminate, Carpet)".to_string(),
            is_required: true,
        },
        Question {
            id: uuid::Uuid::new_v4().to_string(),
            template_id: template_id.clone(),
            r#type: "photo_upload".to_string(),
            text: "Please upload a photo of the current space.".to_string(),
            is_required: false,
        },
    ];

    for q in &questions {
        sqlx::query(
            "INSERT INTO questions (id, template_id, type, text, is_required) VALUES ($1, $2, $3, $4, $5)"
        )
        .bind(&q.id)
        .bind(&q.template_id)
        .bind(&q.r#type)
        .bind(&q.text)
        .bind(q.is_required)
        .execute(&mut *tx)
        .await
        .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;
    }

    tx.commit().await.map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(QuestionnaireTemplate {
        id: template_id,
        tenant_id,
        service_id: payload.service_id,
        title: payload.title,
        status: "draft".to_string(),
        questions,
    }))
}

async fn get_questionnaire(
    State(pool): State<PgPool>,
    Path(id): Path<String>,
) -> Result<Json<QuestionnaireTemplate>, axum::http::StatusCode> {
    use sqlx::Row;
    let template = sqlx::query(
        "SELECT id, tenant_id, service_id, title, status FROM questionnaire_templates WHERE id = $1"
    )
    .bind(&id)
    .fetch_optional(&pool)
    .await
    .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?
    .ok_or(axum::http::StatusCode::NOT_FOUND)?;

    let questions_db = sqlx::query(
        "SELECT id, template_id, type, text, is_required FROM questions WHERE template_id = $1 ORDER BY created_at ASC"
    )
    .bind(&id)
    .fetch_all(&pool)
    .await
    .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;

    let questions = questions_db.into_iter().map(|q| Question {
        id: q.try_get("id").unwrap_or_default(),
        template_id: q.try_get("template_id").unwrap_or_default(),
        r#type: q.try_get("type").unwrap_or_default(),
        text: q.try_get("text").unwrap_or_default(),
        is_required: q.try_get("is_required").unwrap_or_default(),
    }).collect();

    Ok(Json(QuestionnaireTemplate {
        id: template.try_get("id").unwrap_or_default(),
        tenant_id: template.try_get("tenant_id").unwrap_or_default(),
        service_id: template.try_get("service_id").unwrap_or_default(),
        title: template.try_get("title").unwrap_or_default(),
        status: template.try_get("status").unwrap_or_default(),
        questions,
    }))
}

#[derive(Deserialize)]
pub struct SubmissionAnswerRequest {
    pub question_id: String,
    pub answer_text: Option<String>,
    pub photo_url: Option<String>,
}

#[derive(Deserialize)]
pub struct SubmitIntakeRequest {
    pub customer_id: Option<String>,
    pub answers: Vec<SubmissionAnswerRequest>,
}

#[derive(Serialize)]
pub struct SubmitIntakeResponse {
    pub submission_id: String,
}

async fn submit_intake(
    State(pool): State<PgPool>,
    headers: axum::http::HeaderMap,
    Path(id): Path<String>,
    Json(payload): Json<SubmitIntakeRequest>,
) -> Result<Json<SubmitIntakeResponse>, axum::http::StatusCode> {
    let tenant_id = headers.get("x-tenant-id")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("default")
        .to_string();

    let submission_id = uuid::Uuid::new_v4().to_string();

    let mut tx = pool.begin().await.map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;
    sqlx::query("SELECT set_config('app.current_tenant', $1, true)")
        .bind(&tenant_id)
        .execute(&mut *tx)
        .await
        .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;

    let mut parsed_entities = serde_json::json!({});
    for answer in &payload.answers {
        if let Some(text) = &answer.answer_text {
            if text.contains("sq ft") || text.contains("square feet") {
                parsed_entities["dimensions"] = serde_json::Value::String(text.clone());
            } else if text.to_lowercase().contains("hardwood") || text.to_lowercase().contains("laminate") {
                parsed_entities["material"] = serde_json::Value::String(text.clone());
            }
        }
    }

    sqlx::query(
        "INSERT INTO intake_submissions (id, tenant_id, template_id, customer_id, status, parsed_entities) VALUES ($1, $2, $3, $4, $5, $6)"
    )
    .bind(&submission_id)
    .bind(&tenant_id)
    .bind(&id)
    .bind(&payload.customer_id)
    .bind("submitted")
    .bind(&parsed_entities)
    .execute(&mut *tx)
    .await
    .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;

    for answer in payload.answers {
        let answer_id = uuid::Uuid::new_v4().to_string();
        sqlx::query(
            "INSERT INTO submission_answers (id, submission_id, question_id, answer_text, photo_url) VALUES ($1, $2, $3, $4, $5)"
        )
        .bind(&answer_id)
        .bind(&submission_id)
        .bind(&answer.question_id)
        .bind(&answer.answer_text)
        .bind(&answer.photo_url)
        .execute(&mut *tx)
        .await
        .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;
    }

    tx.commit().await.map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(SubmitIntakeResponse {
        submission_id,
    }))
}

#[derive(Serialize)]
pub struct SubmissionAnswer {
    pub question_id: String,
    pub question_text: String,
    pub answer_text: Option<String>,
    pub photo_url: Option<String>,
}

#[derive(Serialize)]
pub struct IntakeSubmissionView {
    pub id: String,
    pub tenant_id: String,
    pub template_id: String,
    pub customer_id: Option<String>,
    pub status: String,
    pub parsed_entities: serde_json::Value,
    pub answers: Vec<SubmissionAnswer>,
    pub summary: String,
    pub draft_quote: String,
}

async fn get_submission(
    State(pool): State<PgPool>,
    headers: axum::http::HeaderMap,
    Path(id): Path<String>,
) -> Result<Json<IntakeSubmissionView>, axum::http::StatusCode> {
    let tenant_id = headers.get("x-tenant-id")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("default")
        .to_string();

    let mut tx = pool.begin().await.map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;
    sqlx::query("SELECT set_config('app.current_tenant', $1, true)")
        .bind(&tenant_id)
        .execute(&mut *tx)
        .await
        .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;

    use sqlx::Row;
    let submission = sqlx::query(
        "SELECT id, tenant_id, template_id, customer_id, status, parsed_entities FROM intake_submissions WHERE id = $1"
    )
    .bind(&id)
    .fetch_optional(&mut *tx)
    .await
    .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?
    .ok_or(axum::http::StatusCode::NOT_FOUND)?;

    let answers_db = sqlx::query(
        "SELECT sa.question_id, q.text as question_text, sa.answer_text, sa.photo_url
         FROM submission_answers sa
         JOIN questions q ON sa.question_id = q.id
         WHERE sa.submission_id = $1"
    )
    .bind(&id)
    .fetch_all(&mut *tx)
    .await
    .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;

    let answers: Vec<SubmissionAnswer> = answers_db.into_iter().map(|a| SubmissionAnswer {
        question_id: a.try_get("question_id").unwrap_or_default(),
        question_text: a.try_get("question_text").unwrap_or_default(),
        answer_text: a.try_get("answer_text").unwrap_or_default(),
        photo_url: a.try_get("photo_url").unwrap_or_default(),
    }).collect();

    let parsed: serde_json::Value = submission.try_get("parsed_entities").unwrap_or(serde_json::json!({}));
    let summary = format!("Customer wants {} flooring for a {} room.",
        parsed.get("material").and_then(|v| v.as_str()).unwrap_or("unspecified"),
        parsed.get("dimensions").and_then(|v| v.as_str()).unwrap_or("unspecified")
    );
    let draft_quote = "$1,200".to_string();

    tx.commit().await.map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(IntakeSubmissionView {
        id: submission.try_get("id").unwrap_or_default(),
        tenant_id: submission.try_get("tenant_id").unwrap_or_default(),
        template_id: submission.try_get("template_id").unwrap_or_default(),
        customer_id: submission.try_get("customer_id").unwrap_or_default(),
        status: submission.try_get("status").unwrap_or_default(),
        parsed_entities: parsed,
        answers,
        summary,
        draft_quote,
    }))
}

#[derive(Deserialize)]
pub struct ListSubmissionsQuery {
    pub tenant_id: Option<String>,
}

async fn list_submissions(
    State(pool): State<PgPool>,
    Query(query): Query<ListSubmissionsQuery>,
    headers: axum::http::HeaderMap,
) -> Result<Json<Vec<IntakeSubmissionView>>, axum::http::StatusCode> {
    let tenant_id = query.tenant_id.or_else(|| {
        headers.get("x-tenant-id").and_then(|v| v.to_str().ok()).map(|s| s.to_string())
    }).unwrap_or_else(|| "default".to_string());

    let mut tx = pool.begin().await.map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;
    sqlx::query("SELECT set_config('app.current_tenant', $1, true)")
        .bind(&tenant_id)
        .execute(&mut *tx)
        .await
        .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;

    use sqlx::Row;
    let submissions_db = sqlx::query(
        "SELECT id FROM intake_submissions ORDER BY created_at DESC"
    )
    .fetch_all(&mut *tx)
    .await
    .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;

    tx.commit().await.map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;

    let mut results = Vec::new();
    for row in submissions_db {
        let mut header_map = axum::http::HeaderMap::new();
        header_map.insert("x-tenant-id", tenant_id.parse().unwrap());
        let id: String = row.try_get("id").unwrap_or_default();
        if let Ok(Json(sub)) = get_submission(State(pool.clone()), header_map, Path(id)).await {
            results.push(sub);
        }
    }

    Ok(Json(results))
}
