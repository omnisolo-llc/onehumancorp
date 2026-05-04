use sqlx::Row;
use std::sync::Arc;
use tonic::{Request, Response, Status};

use crate::ohc::journey::journey_service_server::JourneyService;
use crate::ohc::journey::{
    GetJourneyStateRequest, GetJourneyStateResponse, JourneyPhase, JourneyState,
    TransitionJourneyRequest, TransitionJourneyResponse,
};
use crate::domain::journey::JourneyStateMachine;
use crate::hub::Hub;
use sqlx::PgPool;

pub struct MyJourneyService {
    pool: PgPool,
    hub: Arc<Hub>,
}

impl MyJourneyService {
    pub fn new(pool: PgPool, hub: Arc<Hub>) -> Self {
        Self { pool, hub }
    }
}

#[tonic::async_trait]
impl JourneyService for MyJourneyService {
    async fn get_journey_state(
        &self,
        request: Request<GetJourneyStateRequest>,
    ) -> Result<Response<GetJourneyStateResponse>, Status> {
        let req = request.into_inner();
        let tenant_id = req.tenant_id;

        if tenant_id.is_empty() {
            return Err(Status::invalid_argument("tenant_id is required"));
        }

        let mut conn = self.pool.acquire().await.map_err(|e| Status::internal(format!("Database error: {}", e)))?;
        sqlx::query("SET SESSION app.current_tenant = $1").bind(&tenant_id).execute(&mut *conn).await.map_err(|e| Status::internal(format!("Database error: {}", e)))?;
        let row = sqlx::query("SELECT phase, updated_at FROM journey_states WHERE tenant_id = $1")
        .bind(&tenant_id)
        .fetch_optional(&mut *conn)
        .await
        .map_err(|e| Status::internal(format!("Database error: {}", e)))?;

        let state = match row {
            Some(r) => JourneyState {
                tenant_id,
                phase: JourneyStateMachine::parse_phase(&r.try_get::<String, _>("phase").unwrap_or_else(|_| "NEW".to_string())).into(),
                updated_at_unix: r.try_get::<i64, _>("updated_at").unwrap_or(0),
            },
            None => JourneyState {
                tenant_id,
                phase: JourneyPhase::New.into(),
                updated_at_unix: 0,
            },
        };

        Ok(Response::new(GetJourneyStateResponse {
            state: Some(state),
        }))
    }

    async fn transition_journey(
        &self,
        request: Request<TransitionJourneyRequest>,
    ) -> Result<Response<TransitionJourneyResponse>, Status> {
        let req = request.into_inner();
        let tenant_id = req.tenant_id;
        let target_phase = JourneyPhase::try_from(req.target_phase).unwrap_or(JourneyPhase::New);

        if tenant_id.is_empty() {
            return Err(Status::invalid_argument("tenant_id is required"));
        }

        let mut tx = self.pool.begin().await.map_err(|e| Status::internal(format!("Failed to start transaction: {}", e)))?;

        sqlx::query("SET LOCAL app.current_tenant = $1").bind(&tenant_id).execute(&mut *tx).await.map_err(|e| Status::internal(format!("Database error: {}", e)))?;

        // Lock the row for update if it exists
        let row = sqlx::query("SELECT phase FROM journey_states WHERE tenant_id = $1 FOR UPDATE")
        .bind(&tenant_id)
        .fetch_optional(&mut *tx)
        .await
        .map_err(|e| Status::internal(format!("Database error: {}", e)))?;

        let current_phase = match row {
            Some(r) => JourneyStateMachine::parse_phase(&r.try_get::<String, _>("phase").unwrap_or_else(|_| "NEW".to_string())),
            None => JourneyPhase::New,
        };

        if !JourneyStateMachine::is_valid_transition(current_phase, target_phase) {
            let _ = tx.rollback().await;
            return Err(Status::failed_precondition("Invalid journey state transition"));
        }

        let target_phase_str = JourneyStateMachine::phase_to_string(target_phase);
        let updated_at = chrono::Utc::now().timestamp();

        sqlx::query(
            "INSERT INTO journey_states (tenant_id, phase, updated_at)
             VALUES ($1, $2, $3)
             ON CONFLICT (tenant_id) DO UPDATE
             SET phase = EXCLUDED.phase, updated_at = EXCLUDED.updated_at")
        .bind(&tenant_id)
        .bind(&target_phase_str)
        .bind(updated_at)
        .execute(&mut *tx)
        .await
        .map_err(|e| Status::internal(format!("Failed to update state: {}", e)))?;

        tx.commit().await.map_err(|e| Status::internal(format!("Failed to commit transaction: {}", e)))?;

        let new_state = JourneyState {
            tenant_id: tenant_id.clone(),
            phase: target_phase.into(),
            updated_at_unix: updated_at,
        };

        // Publish event to AI Job Queue via Hub
        let payload = format!(r#"{{"tenant_id":"{}","phase":"{}"}}"#, tenant_id, target_phase_str);
                let msg = crate::ohc::orchestration::Message {
            id: format!("msg-{}", uuid::Uuid::new_v4()),
            from_agent: "SYSTEM".to_string(),
            to_agent: "all".to_string(),
            r#type: "journey_transition".to_string(),
            content: payload,
            meeting_id: "".to_string(),
            occurred_at_unix: chrono::Utc::now().timestamp(),
        };
        if let Err(e) = self.hub.clone().publish(msg) {
            eprintln!("Failed to publish journey transition event: {}", e);
        }

        Ok(Response::new(TransitionJourneyResponse {
            success: true,
            new_state: Some(new_state),
        }))
    }
}
