use axum::{
    extract::{Extension, Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use std::sync::Arc;
use uuid::Uuid;
use ::server_common::Claims;
use crate::orchestration::departments::orchestrator::DepartmentOrchestrator;
use crate::orchestration::departments::types::{DepartmentType, ActionRisk};

#[derive(Clone)]
pub struct IntakeState {
    pub pool: PgPool,
    pub orchestrator: Arc<DepartmentOrchestrator>,
}

#[derive(Deserialize, Serialize)]
pub struct CreateTemplateRequest {
    pub title: String,
    pub questions: Vec<QuestionInput>,
}

#[derive(Deserialize, Serialize)]
pub struct QuestionInput {
    pub type_name: String, // 'text', 'multiple_choice', 'photo_upload'
    pub text: String,
    pub is_required: bool,
    pub options: Option<serde_json::Value>,
}

#[derive(Serialize)]
pub struct CreateTemplateResponse {
    pub template_id: String,
}

#[derive(Serialize)]
pub struct GetTemplateResponse {
    pub id: String,
    pub title: String,
    pub questions: Vec<QuestionModel>,
}

#[derive(Serialize)]
pub struct QuestionModel {
    pub id: String,
    pub type_name: String,
    pub text: String,
    pub is_required: bool,
    pub options: Option<serde_json::Value>,
}

#[derive(Deserialize)]
pub struct SubmitIntakeRequest {
    pub customer_name: String,
    pub customer_email: String,
    pub answers: Vec<AnswerInput>,
}

#[derive(Deserialize)]
pub struct AnswerInput {
    pub question_id: String,
    pub raw_response: Option<String>,
    pub media_url: Option<String>,
}

#[derive(Serialize)]
pub struct SubmitIntakeResponse {
    pub submission_id: String,
    pub success: bool,
}

pub fn router<S>(pool: PgPool, orchestrator: Arc<DepartmentOrchestrator>) -> Router<S>
where
    S: Clone + Send + Sync + 'static,
{
    let state = IntakeState { pool, orchestrator };
    Router::new()
        .route("/templates", post(create_template))
        .route("/templates/:id", get(get_template))
        .route("/submit/:template_id", post(submit_intake))
        .with_state(state)
}

async fn create_template(
    State(state): State<IntakeState>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<CreateTemplateRequest>,
) -> impl IntoResponse {
    let tenant_id = match claims.organization_id.as_deref() {
        Some(org_id) => org_id.to_string(),
        None => return (StatusCode::UNAUTHORIZED, Json(CreateTemplateResponse { template_id: "".to_string() })).into_response(),
    };

    let template_id = Uuid::new_v4().to_string();

    let mut tx = match state.pool.begin().await {
        Ok(tx) => tx,
        Err(e) => {
            tracing::error!("Failed to begin transaction: {}", e);
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(CreateTemplateResponse { template_id: "".to_string() })).into_response();
        }
    };

    // SET LOCAL app.current_tenant
    if let Err(e) = sqlx::query(&format!("SET LOCAL app.current_tenant = '{}'", tenant_id))
        .execute(&mut *tx)
        .await
    {
         tracing::error!("Failed to set tenant context: {}", e);
         return (StatusCode::INTERNAL_SERVER_ERROR, Json(CreateTemplateResponse { template_id: "".to_string() })).into_response();
    }

    if let Err(e) = sqlx::query("INSERT INTO questionnaire_templates (id, tenant_id, title) VALUES ($1, $2, $3)")
        .bind(&template_id)
        .bind(&tenant_id)
        .bind(&payload.title)
        .execute(&mut *tx)
        .await
    {
         tracing::error!("Failed to insert template: {}", e);
         return (StatusCode::INTERNAL_SERVER_ERROR, Json(CreateTemplateResponse { template_id: "".to_string() })).into_response();
    }

    for q in payload.questions {
        let q_id = Uuid::new_v4().to_string();
        if let Err(e) = sqlx::query("INSERT INTO questions (id, tenant_id, template_id, type, text, is_required, options) VALUES ($1, $2, $3, $4, $5, $6, $7)")
            .bind(&q_id)
            .bind(&tenant_id)
            .bind(&template_id)
            .bind(&q.type_name)
            .bind(&q.text)
            .bind(&q.is_required)
            .bind(&q.options)
            .execute(&mut *tx)
            .await
        {
             tracing::error!("Failed to insert question: {}", e);
             return (StatusCode::INTERNAL_SERVER_ERROR, Json(CreateTemplateResponse { template_id: "".to_string() })).into_response();
        }
    }

    if let Err(e) = tx.commit().await {
         tracing::error!("Failed to commit transaction: {}", e);
         return (StatusCode::INTERNAL_SERVER_ERROR, Json(CreateTemplateResponse { template_id: "".to_string() })).into_response();
    }

    (StatusCode::CREATED, Json(CreateTemplateResponse { template_id })).into_response()
}

async fn get_template(
    State(state): State<IntakeState>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    // Note: GET might be called unauthenticated from the storefront (widget), so we skip claims check here
    // or pass tenant_id via query parameter. For now, assuming we don't enforce RLS reading publicly if we want storefront to see it,
    // but the schema enforces RLS. We'll use a service role or bypass for public forms,
    // OR we can fetch it explicitly. Let's just do a direct fetch since it's a public form.

    // Fetch template
    let template_record: Option<(String, String)> = match sqlx::query_as("SELECT id, title FROM questionnaire_templates WHERE id = $1")
        .bind(&id)
        .fetch_optional(&state.pool)
        .await
    {
        Ok(r) => r,
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, "Error fetching template").into_response(),
    };

    let template = match template_record {
        Some(t) => t,
        None => return (StatusCode::NOT_FOUND, "Template not found").into_response(),
    };

    // Fetch questions
    let questions_records: Vec<(String, String, String, bool, Option<serde_json::Value>)> = match sqlx::query_as(
        "SELECT id, type, text, is_required, options FROM questions WHERE template_id = $1 ORDER BY created_at ASC"
    )
        .bind(&id)
        .fetch_all(&state.pool)
        .await
    {
        Ok(r) => r,
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, "Error fetching questions").into_response(),
    };

    let questions = questions_records.into_iter().map(|(q_id, q_type, q_text, q_req, q_opt)| {
        QuestionModel {
            id: q_id,
            type_name: q_type,
            text: q_text,
            is_required: q_req,
            options: q_opt,
        }
    }).collect();

    (StatusCode::OK, Json(GetTemplateResponse {
        id: template.0,
        title: template.1,
        questions,
    })).into_response()
}

async fn submit_intake(
    State(state): State<IntakeState>,
    Path(template_id): Path<String>,
    axum::extract::Query(query): axum::extract::Query<std::collections::HashMap<String, String>>,
    Json(payload): Json<SubmitIntakeRequest>,
) -> impl IntoResponse {
    // Determine tenant. Might be passed via query `?tenant=xxx` if unauthenticated storefront
    let tenant_id = query.get("tenant").cloned().unwrap_or_else(|| "my-business".to_string());

    let submission_id = Uuid::new_v4().to_string();

    let mut tx = match state.pool.begin().await {
        Ok(tx) => tx,
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, Json(SubmitIntakeResponse { submission_id: "".to_string(), success: false })).into_response(),
    };

    // SET LOCAL app.current_tenant
    if let Err(e) = sqlx::query(&format!("SET LOCAL app.current_tenant = '{}'", tenant_id))
        .execute(&mut *tx)
        .await
    {
         tracing::error!("Failed to set tenant context: {}", e);
         return (StatusCode::INTERNAL_SERVER_ERROR, Json(SubmitIntakeResponse { submission_id: "".to_string(), success: false })).into_response();
    }

    if let Err(e) = sqlx::query("INSERT INTO intake_submissions (id, tenant_id, template_id, customer_name, customer_email) VALUES ($1, $2, $3, $4, $5)")
        .bind(&submission_id)
        .bind(&tenant_id)
        .bind(&template_id)
        .bind(&payload.customer_name)
        .bind(&payload.customer_email)
        .execute(&mut *tx)
        .await
    {
         tracing::error!("Failed to insert submission: {}", e);
         return (StatusCode::INTERNAL_SERVER_ERROR, Json(SubmitIntakeResponse { submission_id: "".to_string(), success: false })).into_response();
    }

    for ans in payload.answers {
        let ans_id = Uuid::new_v4().to_string();
        if let Err(e) = sqlx::query("INSERT INTO submission_answers (id, tenant_id, submission_id, question_id, raw_response, media_url) VALUES ($1, $2, $3, $4, $5, $6)")
            .bind(&ans_id)
            .bind(&tenant_id)
            .bind(&submission_id)
            .bind(&ans.question_id)
            .bind(&ans.raw_response)
            .bind(&ans.media_url)
            .execute(&mut *tx)
            .await
        {
             tracing::error!("Failed to insert answer: {}", e);
             return (StatusCode::INTERNAL_SERVER_ERROR, Json(SubmitIntakeResponse { submission_id: "".to_string(), success: false })).into_response();
        }
    }

    if let Err(e) = tx.commit().await {
         tracing::error!("Failed to commit transaction: {}", e);
         return (StatusCode::INTERNAL_SERVER_ERROR, Json(SubmitIntakeResponse { submission_id: "".to_string(), success: false })).into_response();
    }

    // Trigger AI Sales Agent logic (Simulated here using orchestrator execute_action to put it in Agent Feed)
    let ai_payload = serde_json::json!({
        "feature_type": "proposal_draft",
        "customer_name": payload.customer_name,
        "customer_email": payload.customer_email,
        "context": {
            "summary": format!("{} wants a logo refresh and 3-page site", payload.customer_name),
            "suggested_price": 1200.0,
            "timeline": "2 weeks starting next Monday",
            "scope": "Custom Branding & Web Design",
            "weekly_health_report": false,
            "smart_pricing": false
        }
    });

    let _ = state.orchestrator.execute_action(
        DepartmentType::Sales,
        format!("New Intake: {} Branding. Proposal Drafted.", payload.customer_name),
        tenant_id,
        ActionRisk::DraftForReview,
        ai_payload,
    ).await;

    (StatusCode::OK, Json(SubmitIntakeResponse { submission_id, success: true })).into_response()
}
