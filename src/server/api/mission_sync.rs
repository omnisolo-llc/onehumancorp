use axum::{
    extract::{Path, State},
    response::IntoResponse,
    Json,
    routing::{post, get},
    Router,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use sqlx::{Pool, Postgres};
use uuid::Uuid;
use tracing::{info, error};

#[derive(Clone)]
pub struct MissionSyncState {
    pub db: Pool<Postgres>,
}

#[derive(Deserialize, Serialize)]
pub struct MissionPayload {
    pub role: String,
    pub task: String,
    pub context: Option<String>,
    pub action_risk: Option<String>,
}

#[derive(Deserialize)]
pub struct EscalateRequest {
    pub local_id: String,
    pub payload: MissionPayload,
}

#[derive(Serialize)]
pub struct EscalateResponse {
    pub cloud_id: String,
    pub status: String,
}

#[derive(Serialize)]
pub struct StatusResponse {
    pub cloud_id: String,
    pub status: String,
    pub result: Option<String>,
}

pub async fn escalate_mission(
    parts: axum::http::request::Parts,
    State(state): State<Arc<MissionSyncState>>,
    Json(payload): Json<EscalateRequest>,
) -> impl IntoResponse {
    let claims = parts.extensions.get::<crate::auth::Claims>();
    let tenant_id = match claims.and_then(|c| c.organization_id.clone()) {
        Some(org_id) => org_id,
        None => return (
            axum::http::StatusCode::UNAUTHORIZED,
            Json(EscalateResponse {
                cloud_id: "".to_string(),
                status: "UNAUTHORIZED".to_string(),
            }),
        ).into_response(),
    };

    let cloud_id = Uuid::new_v4().to_string();
    let payload_str = serde_json::to_string(&payload.payload).unwrap_or_default();

    // In a real system, we'd insert this into a Postgres table, enqueue a k8s pod job, etc.
    // For now we simulate accepting the task and inserting it into cloud agent_missions.

    let res = sqlx::query(
        "INSERT INTO agent_missions (id, status, payload, organization_id) VALUES ($1, 'ACCEPTED', $2, $3)"
    )
    .bind(&cloud_id)
    .bind(&payload_str)
    .bind(&tenant_id)
    .execute(&state.db)
    .await;

    if let Err(e) = res {
        error!("Failed to escalate mission to cloud db: {}", e);
        return (
            axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            Json(EscalateResponse {
                cloud_id: "".to_string(),
                status: "ERROR".to_string(),
            }),
        ).into_response();
    }

    info!("Escalated local mission to cloud mission");

    (
        axum::http::StatusCode::OK,
        Json(EscalateResponse {
            cloud_id,
            status: "ACCEPTED".to_string(),
        }),
    ).into_response()
}

pub async fn get_mission_status(
    parts: axum::http::request::Parts,
    State(state): State<Arc<MissionSyncState>>,
    Path(cloud_id): Path<String>,
) -> impl IntoResponse {
    let claims = parts.extensions.get::<crate::auth::Claims>();
    let tenant_id = match claims.and_then(|c| c.organization_id.clone()) {
        Some(org_id) => org_id,
        None => return (
            axum::http::StatusCode::UNAUTHORIZED,
            Json(StatusResponse {
                cloud_id: "".to_string(),
                status: "UNAUTHORIZED".to_string(),
                result: None,
            }),
        ).into_response(),
    };

    let row: Result<Option<(String,)>, _> = sqlx::query_as(
        "SELECT status FROM agent_missions WHERE id = $1 AND organization_id = $2"
    )
    .bind(&cloud_id)
    .bind(&tenant_id)
    .fetch_optional(&state.db)
    .await;

    match row {
        Ok(Some((status,))) => {
            (
                axum::http::StatusCode::OK,
                Json(StatusResponse {
                    cloud_id,
                    status,
                    result: Some("<computed result from k8s pod>".to_string()),
                }),
            ).into_response()
        }
        Ok(None) => {
            (
                axum::http::StatusCode::NOT_FOUND,
                Json(StatusResponse {
                    cloud_id,
                    status: "NOT_FOUND".to_string(),
                    result: None,
                }),
            ).into_response()
        }
        Err(e) => {
            error!("Failed to get mission status: {}", e);
            (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                Json(StatusResponse {
                    cloud_id,
                    status: "ERROR".to_string(),
                    result: None,
                }),
            ).into_response()
        }
    }
}

pub fn router(state: Arc<MissionSyncState>) -> Router {
    Router::new()
        .route("/api/v1/missions/escalate", post(escalate_mission))
        .route("/api/v1/missions/:cloud_id/status", get(get_mission_status))
        .with_state(state)
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use axum::{
        extract::{Path, State},
        Json,
    };
    use crate::api::mission_sync::{escalate_mission, get_mission_status, EscalateRequest, MissionSyncState};

    #[tokio::test]
    async fn test_mission_sync_endpoints() {
        if std::env::var("DATABASE_URL").is_err() && std::env::var("OHC_DATABASE_URL").is_err() {
            return; // skip if db is unavailable
        }

        let pool = sqlx::postgres::PgPoolOptions::new()
            .connect_lazy("postgres://postgres:postgres@localhost:5432/test")
            .unwrap();
        let state = Arc::new(MissionSyncState { db: pool });

        let claims = crate::auth::Claims {
            sub: "test".to_string(),
            username: "test".to_string(),
            email: "test@test.com".to_string(),
            roles: vec!["admin".to_string()],
            organization_id: Some("test_tenant".to_string()),
            session_id: None,
            iat: 0,
            exp: 0,
            jti: "".to_string(),
        };

        let req = EscalateRequest {
            local_id: "local_123".to_string(),
            payload: crate::api::mission_sync::MissionPayload {
                role: "agent".to_string(),
                task: "task".to_string(),
                context: None,
                action_risk: None,
            },
        };

        let parts = axum::http::request::Request::builder()
            .extension(claims.clone())
            .body(())
            .unwrap()
            .into_parts()
            .0;

        let _f = escalate_mission(parts.clone(), State(state.clone()), Json(req));
        let _g = get_mission_status(parts, State(state), Path("id".to_string()));
    }
}
