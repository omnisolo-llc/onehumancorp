use axum::{
    extract::{State, Path},
    Json,
    response::IntoResponse,
    http::StatusCode,
};
use std::sync::Arc;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::orchestration::departments::types::{DepartmentEvent};
use crate::orchestration::departments::orchestrator::DepartmentOrchestrator;
use crate::db::DbStore;

#[derive(Clone)]
pub struct StrategistApiState {
    pub db: Arc<crate::db::DB>,
    pub orchestrator: Arc<DepartmentOrchestrator>,
}

#[derive(Deserialize)]
pub struct CreateObjectivePayload {
    pub tenant_id: String,
    pub goal: String,
    pub target_date: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Serialize)]
pub struct ObjectiveResponse {
    pub id: String,
    pub goal: String,
    pub status: String,
}

pub async fn create_objective_handler(
    State(state): State<StrategistApiState>,
    Json(payload): Json<CreateObjectivePayload>,
) -> impl IntoResponse {
    let id = Uuid::new_v4();
    let tenant_id = payload.tenant_id;
    let goal = payload.goal;

    let res = match &state.db.store {
        DbStore::Postgres => {
            sqlx::query(
                "INSERT INTO strategic_objectives (id, tenant_id, goal, target_date) VALUES ($1, $2, $3, $4)"
            )
            .bind(id)
            .bind(&tenant_id)
            .bind(&goal)
            .bind(payload.target_date)
            .execute(&state.db.pool)
            .await
        },
        _ => return (StatusCode::NOT_IMPLEMENTED, "Only Postgres supported for Strategist").into_response(),
    };

    if let Err(e) = res {
        return (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response();
    }

    // Trigger Strategist Agent
    let event = DepartmentEvent {
        id: Uuid::new_v4().to_string(),
        tenant_id: tenant_id.clone(),
        event_type: "tenant.objective.set".to_string(),
        payload: serde_json::json!({
            "objective_id": id.to_string(),
            "goal": goal,
        }),
    };

    let orch = state.orchestrator.clone();
    tokio::spawn(async move {
        let _ = orch.dispatch_event(event).await;
    });

    (StatusCode::CREATED, Json(ObjectiveResponse {
        id: id.to_string(),
        goal,
        status: "PENDING".to_string(),
    })).into_response()
}

pub async fn get_objectives_handler(
    State(state): State<StrategistApiState>,
    Path(tenant_id): Path<String>,
) -> impl IntoResponse {
    let rows = match &state.db.store {
        DbStore::Postgres => {
            sqlx::query_as::<_, (String, String, String)>(
                "SELECT id::text, goal, status FROM strategic_objectives WHERE tenant_id = $1"
            )
            .bind(&tenant_id)
            .fetch_all(&state.db.pool)
            .await
        },
        _ => return (StatusCode::NOT_IMPLEMENTED, "Only Postgres supported").into_response(),
    };

    match rows {
        Ok(items) => {
            let resp: Vec<ObjectiveResponse> = items.into_iter().map(|(id, goal, status)| {
                ObjectiveResponse { id, goal, status }
            }).collect();
            (StatusCode::OK, Json(resp)).into_response()
        },
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}
