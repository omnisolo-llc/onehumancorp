use crate::orchestration::departments::orchestrator::DepartmentOrchestrator;
use crate::orchestration::departments::types::DepartmentEvent;
use sqlx::PgPool;
use std::sync::Arc;
use uuid::Uuid;

#[derive(Clone)]
pub struct LoyaltyService {
    pool: PgPool,
    orchestrator: Option<Arc<DepartmentOrchestrator>>,
}

impl LoyaltyService {
    pub fn new(pool: PgPool, orchestrator: Option<Arc<DepartmentOrchestrator>>) -> Self {
        Self { pool, orchestrator }
    }

    pub async fn trigger_points_awarded(&self, tenant_id: &str, customer_id: &str, points: i32, total_points: i32) -> Result<(), String> {
        if let Some(orch) = &self.orchestrator {
            let event = DepartmentEvent {
                event_type: "loyalty.points_awarded".to_string(),
                tenant_id: tenant_id.to_string(),
                payload: serde_json::json!({
                    "customer_id": customer_id,
                    "points": points,
                    "total_points": total_points
                }),
                id: Uuid::new_v4().to_string(),
            };

            // Dispatch event to AI orchestration
            if let Err(e) = orch.dispatch_event(event).await {
                tracing::error!("Failed to dispatch loyalty.points_awarded: {:?}", e);
            }
        }
        Ok(())
    }
}
