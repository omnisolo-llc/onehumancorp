use axum::{
    extract::{State, Query, Form, Path},
    response::IntoResponse,
    http::StatusCode,
    routing::{post, get},
    Router,
    Json,
};
use std::sync::Arc;
use serde::{Deserialize, Serialize};
use crate::orchestration::departments::orchestrator::DepartmentOrchestrator;
use crate::orchestration::departments::types::{DepartmentType, ActionRisk};
use uuid::Uuid;
use crate::db::DB;

#[derive(Deserialize)]
pub struct TenantQuery {
    pub tenant: Option<String>,
}

#[derive(Deserialize)]
pub struct ClientIntakeRequest {
    pub name: String,
    pub email: String,
    pub details: String,
}

#[derive(Serialize, Deserialize)]
pub struct ClientIntakeResponse {
    pub success: bool,
    pub proposal_drafted: bool,
}

#[derive(Deserialize)]
pub struct CreateSessionRequest {
    pub customer_info: serde_json::Value,
}

#[derive(Deserialize)]
pub struct SendMessageRequest {
    pub content: String,
}

#[derive(Clone)]
pub struct ClientIntakeState {
    pub orchestrator: Arc<DepartmentOrchestrator>,
    pub db: Arc<DB>,
}

pub fn router<S>(orchestrator: Arc<DepartmentOrchestrator>, db: Arc<DB>) -> Router<S>
where
    S: Clone + Send + Sync + 'static,
{
    let state = ClientIntakeState {
        orchestrator,
        db,
    };
    Router::new()
        .route("/", post(handle_client_intake))
        .route("/session", post(create_session))
        .route("/session/{id}", get(get_session))
        .route("/session/{id}/message", post(send_message))
        .route("/session/{id}/history", get(get_history))
        .with_state(state)
}

async fn handle_client_intake(
    State(state): State<ClientIntakeState>,
    Query(query): Query<TenantQuery>,
    Form(payload): Form<ClientIntakeRequest>,
) -> impl IntoResponse {
    let tenant_id = query.tenant.unwrap_or_else(|| "default".to_string());

    // Analyze the unstructured inquiry and extract parameters (mock logic for AI generation)
    // Create a drafted proposal
    let suggested_price = 1500.00;
    let service_name = "Custom Project Scope";

    let drafted_message = format!(
        "Hi there! Based on your request for '{}', I've put together a drafted proposal. The estimated scope will cost around ${}, including standard services.",
        payload.details, suggested_price
    );

    let action_payload = serde_json::json!({
        "feature_type": "quote_draft",
        "customer_inquiry": payload.details,
        "client_name": payload.name,
        "client_email": payload.email,
        "suggested_price": suggested_price,
        "scope": format!("{} with custom requirements.", service_name),
        "suggested_time": "Next Week",
        "generated_response": drafted_message,
        "service": service_name,
        "price": suggested_price,
    });

    match state.orchestrator.execute_action(
        DepartmentType::Sales,
        format!("Draft proposal for new intake: {}", service_name),
        tenant_id,
        ActionRisk::DraftForReview,
        action_payload,
    ).await {
        Ok(_) => (StatusCode::OK, Json(ClientIntakeResponse { success: true, proposal_drafted: true })).into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, Json(ClientIntakeResponse { success: false, proposal_drafted: false })).into_response(),
    }
}

async fn create_session(
    State(state): State<ClientIntakeState>,
    Query(query): Query<TenantQuery>,
    Json(payload): Json<CreateSessionRequest>,
) -> impl IntoResponse {
    let tenant_id = query.tenant.unwrap_or_else(|| "default".to_string());
    let session_id = Uuid::new_v4().to_string();

    let mut tx = match state.db.pool.begin().await {
        Ok(tx) => tx,
        Err(e) => {
            tracing::error!("Failed to begin transaction: {:?}", e);
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"error": "internal error"}))).into_response();
        }
    };
    if let Err(e) = ::server_common::auth_utils::set_org_context(&mut *tx, &tenant_id).await {
        tracing::error!("Failed to set org context: {:?}", e);
        return (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"error": "internal error"}))).into_response();
    }

    // Upsert a default flow for the tenant if it doesn't exist.
    let flow_id = "default_flow".to_string();
    match sqlx::query("INSERT INTO agentic_intake_flows (id, tenant_id, name, required_fields, initial_prompt) VALUES ($1, $2, $3, $4, $5) ON CONFLICT (id) DO NOTHING")
        .bind(&flow_id)
        .bind(&tenant_id)
        .bind("Default Intake Flow")
        .bind(serde_json::json!(["service_type", "date", "budget"]))
        .bind("What do you need help with?")
        .execute(&mut *tx)
        .await
    {
        Ok(_) => {},
        Err(e) => {
            tracing::error!("Failed to insert default flow: {:?}", e);
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"error": "Failed to create default flow"}))).into_response();
        }
    }

    match sqlx::query("INSERT INTO intake_sessions (id, tenant_id, flow_id, customer_info, status) VALUES ($1, $2, $3, $4, $5)")
        .bind(&session_id)
        .bind(&tenant_id)
        .bind(&flow_id)
        .bind(&payload.customer_info)
        .bind("active")
        .execute(&mut *tx)
        .await
    {
        Ok(_) => {
            // Also insert the initial message from the agent.
            let msg_id = Uuid::new_v4().to_string();
            let _ = sqlx::query("INSERT INTO intake_messages (id, tenant_id, session_id, role, content) VALUES ($1, $2, $3, $4, $5)")
                .bind(&msg_id)
                .bind(&tenant_id)
                .bind(&session_id)
                .bind("agent")
                .bind("What do you need help with?")
                .execute(&mut *tx)
                .await;

            if let Err(e) = tx.commit().await {
                tracing::error!("Failed to commit transaction: {:?}", e);
                return (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"error": "failed to commit"}))).into_response();
            }
            (StatusCode::OK, Json(serde_json::json!({
                "session_id": session_id,
                "status": "active",
                "initial_message": "What do you need help with?"
            }))).into_response()
        },
        Err(e) => {
            tracing::error!("Failed to create intake session: {:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"error": "Failed to create session"}))).into_response()
        }
    }
}

async fn get_session(
    State(state): State<ClientIntakeState>,
    Path(id): Path<String>,
    Query(query): Query<TenantQuery>,
) -> impl IntoResponse {
    let tenant_id = query.tenant.unwrap_or_else(|| "default".to_string());

    let mut tx = match state.db.pool.begin().await {
        Ok(tx) => tx,
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"error": "internal error"}))).into_response(),
    };
    if let Err(_) = ::server_common::auth_utils::set_org_context(&mut *tx, &tenant_id).await {
        return (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"error": "internal error"}))).into_response();
    }

    match sqlx::query_as::<_, crate::domain::repository::models::IntakeSession>(
        "SELECT id, tenant_id, flow_id, customer_info, collected_data, status, created_at, updated_at FROM intake_sessions WHERE id = $1 AND tenant_id = $2"
    )
    .bind(&id)
    .bind(&tenant_id)
    .fetch_optional(&mut *tx)
    .await {
        Ok(Some(session)) => {
            let _ = tx.commit().await;
            (StatusCode::OK, Json(session)).into_response()
        },
        Ok(None) => {
            let _ = tx.commit().await;
            (StatusCode::NOT_FOUND, Json(serde_json::json!({"error": "Session not found"}))).into_response()
        },
        Err(e) => {
            tracing::error!("Failed to fetch intake session: {:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"error": "internal error"}))).into_response()
        }
    }
}

async fn send_message(
    State(state): State<ClientIntakeState>,
    Path(id): Path<String>,
    Query(query): Query<TenantQuery>,
    Json(payload): Json<SendMessageRequest>,
) -> impl IntoResponse {
    let tenant_id = query.tenant.unwrap_or_else(|| "default".to_string());
    let mut tx = match state.db.pool.begin().await {
        Ok(tx) => tx,
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"error": "internal error"}))).into_response(),
    };
    if let Err(_) = ::server_common::auth_utils::set_org_context(&mut *tx, &tenant_id).await {
        return (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"error": "internal error"}))).into_response();
    }

    // 1. Insert user message
    let user_msg_id = Uuid::new_v4().to_string();
    if let Err(e) = sqlx::query("INSERT INTO intake_messages (id, tenant_id, session_id, role, content) VALUES ($1, $2, $3, $4, $5)")
        .bind(&user_msg_id)
        .bind(&tenant_id)
        .bind(&id)
        .bind("user")
        .bind(&payload.content)
        .execute(&mut *tx)
        .await {
        tracing::error!("Failed to insert user message: {:?}", e);
        return (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"error": "Failed to save message"}))).into_response();
    }

    // 2. Simple logic: determine response
    // A full implementation would use llm_service to generate response and parse JSON.
    // For now, simple mock logic
    let content_lower = payload.content.to_lowercase();
    let (agent_reply, status) = if content_lower.contains("repair") {
        ("Could you please provide a photo of the item to be repaired?", "active")
    } else if content_lower.contains("photo") || content_lower.contains("pic") || content_lower.contains("image") {
        ("Thank you. We have all the information we need. An agent will get back to you shortly.", "completed")
    } else if content_lower.contains("cake") {
        ("What flavor and dietary restrictions do you have?", "active")
    } else if content_lower.contains("chocolate") || content_lower.contains("vegan") || content_lower.contains("gluten") {
        ("Got it. We will send you a quote shortly.", "completed")
    } else {
        ("Could you provide more details about the date and your budget?", "active")
    };

    // Update status if completed
    if status == "completed" {
         if let Err(e) = sqlx::query("UPDATE intake_sessions SET status = 'completed', updated_at = CURRENT_TIMESTAMP WHERE id = $1 AND tenant_id = $2")
             .bind(&id)
             .bind(&tenant_id)
             .execute(&mut *tx)
             .await {
             tracing::error!("Failed to update session status: {:?}", e);
         }

        // Generate Lead & Opportunity
        let lead_id = Uuid::new_v4().to_string();
        let opp_id = Uuid::new_v4().to_string();

        let _ = sqlx::query("INSERT INTO leads (id, tenant_id, source, context) VALUES ($1, $2, $3, $4)")
            .bind(&lead_id)
            .bind(&tenant_id)
            .bind("agentic_intake")
            .bind(serde_json::json!({"session_id": id}).to_string())
            .execute(&mut *tx)
            .await;

        let _ = sqlx::query("INSERT INTO opportunities (id, tenant_id, lead_id, title, stage) VALUES ($1, $2, $3, $4, $5)")
            .bind(&opp_id)
            .bind(&tenant_id)
            .bind(&lead_id)
            .bind("New Agentic Intake")
            .bind("Qualified")
            .execute(&mut *tx)
            .await;
    }

    // 3. Insert agent message
    let agent_msg_id = Uuid::new_v4().to_string();
    if let Err(e) = sqlx::query("INSERT INTO intake_messages (id, tenant_id, session_id, role, content) VALUES ($1, $2, $3, $4, $5)")
        .bind(&agent_msg_id)
        .bind(&tenant_id)
        .bind(&id)
        .bind("agent")
        .bind(agent_reply)
        .execute(&mut *tx)
        .await {
        tracing::error!("Failed to insert agent message: {:?}", e);
        return (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"error": "Failed to save agent message"}))).into_response();
    }

    if let Err(e) = tx.commit().await {
        tracing::error!("Failed to commit transaction: {:?}", e);
        return (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"error": "failed to commit"}))).into_response();
    }

    (StatusCode::OK, Json(serde_json::json!({
        "reply": agent_reply,
        "status": status
    }))).into_response()
}

async fn get_history(
    State(state): State<ClientIntakeState>,
    Path(id): Path<String>,
    Query(query): Query<TenantQuery>,
) -> impl IntoResponse {
    let tenant_id = query.tenant.unwrap_or_else(|| "default".to_string());
    let mut tx = match state.db.pool.begin().await {
        Ok(tx) => tx,
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"error": "internal error"}))).into_response(),
    };
    if let Err(_) = ::server_common::auth_utils::set_org_context(&mut *tx, &tenant_id).await {
        return (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"error": "internal error"}))).into_response();
    }

    match sqlx::query_as::<_, crate::domain::repository::models::IntakeMessage>(
        "SELECT id, tenant_id, session_id, role, content, created_at FROM intake_messages WHERE session_id = $1 AND tenant_id = $2 ORDER BY created_at ASC"
    )
    .bind(&id)
    .bind(&tenant_id)
    .fetch_all(&mut *tx)
    .await {
        Ok(messages) => {
            let _ = tx.commit().await;
            (StatusCode::OK, Json(messages)).into_response()
        },
        Err(e) => {
            let _ = tx.commit().await;
            tracing::error!("Failed to fetch messages: {:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"error": "internal error"}))).into_response()
        }
    }
}
