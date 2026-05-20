use crate::orchestration::departments::orchestrator::{BaseAgent, AgentTriggerType, DepartmentOrchestrator, Department};
use crate::orchestration::departments::types::{DepartmentType, DepartmentEvent, DepartmentConfig, ApprovalRequest, ActionRisk};
use serde_json::Value;

pub struct BookingAgent {
    orchestrator: std::sync::Arc<DepartmentOrchestrator>,
}

impl BookingAgent {
    pub fn new(orchestrator: std::sync::Arc<DepartmentOrchestrator>) -> Self {
        Self { orchestrator }
    }
}

#[async_trait::async_trait]
impl Department for BookingAgent {
    fn department_type(&self) -> DepartmentType {
        DepartmentType::Booking
    }

    fn subscribed_events(&self) -> Vec<String> {
        vec!["tenant.booking.requested".to_string()]
    }

    async fn handle_event(&self, event: &DepartmentEvent) -> Result<(), String> {
        let tenant_id = event.tenant_id.clone();

        let requested_start_str = event.payload.get("start_time").and_then(|v| v.as_str()).unwrap_or("");
        let requested_end_str = event.payload.get("end_time").and_then(|v| v.as_str()).unwrap_or("");

        let start_time = chrono::DateTime::parse_from_rfc3339(requested_start_str)
            .map(|dt| dt.with_timezone(&chrono::Utc))
            .unwrap_or_else(|_| chrono::Utc::now() + chrono::Duration::days(1));

        let end_time = chrono::DateTime::parse_from_rfc3339(requested_end_str)
            .map(|dt| dt.with_timezone(&chrono::Utc))
            .unwrap_or_else(|_| start_time + chrono::Duration::hours(1));

        let new_slot = crate::services::booking::BookingTimeSlot {
            start_time,
            end_time,
        };

        let existing_bookings_res = sqlx::query!("SELECT start_time, end_time FROM bookings WHERE tenant_id = $1", tenant_id)
            .fetch_all(&self.orchestrator.get_db().pool)
            .await;

        let mut existing_bookings = vec![];
        if let Ok(records) = existing_bookings_res {
            for rec in records {
                if let (Some(st_str), Some(et_str)) = (rec.start_time, rec.end_time) {
                    if let (Ok(st), Ok(et)) = (chrono::DateTime::parse_from_rfc3339(&st_str), chrono::DateTime::parse_from_rfc3339(&et_str)) {
                        existing_bookings.push(crate::services::booking::BookingTimeSlot {
                            start_time: st.with_timezone(&chrono::Utc),
                            end_time: et.with_timezone(&chrono::Utc),
                        });
                    }
                }
            }
        }

        if let Err(e) = crate::services::booking::BookingService::prevent_double_booking(&existing_bookings, &new_slot) {
             return self.orchestrator.execute_action(
                DepartmentType::Booking,
                "Propose alternate meeting time".to_string(),
                event.tenant_id.clone(),
                ActionRisk::AutoExecute,
                serde_json::json!({"error": e}),
            ).await.map(|_| ());
        }

        self.orchestrator.execute_action(
            DepartmentType::Booking,
            "Confirm meeting time".to_string(),
            event.tenant_id.clone(),
            ActionRisk::AutoExecute,
            event.payload.clone(),
        ).await.map(|_| ())
    }

    fn get_config(&self, _tenant_id: &str) -> Option<DepartmentConfig> {
        None
    }

    fn set_config(&mut self, _tenant_id: String, _config: DepartmentConfig) {
    }

    async fn query_memory(&self, _query: &str) -> Result<Vec<String>, String> {
        Ok(vec![])
    }

    async fn request_approval(&self, description: String, tenant_id: String, risk: ActionRisk) -> Result<ApprovalRequest, String> {
        self.orchestrator.execute_action(self.department_type(), description.clone(), tenant_id.clone(), risk, serde_json::json!({})).await
    }
}

#[async_trait::async_trait]
impl BaseAgent for BookingAgent {
    fn agent_id(&self) -> String {
        "booking_agent".to_string()
    }

    fn trigger_type(&self) -> AgentTriggerType {
        AgentTriggerType::EventDriven
    }

    async fn execute(&self, _payload: Value) -> Result<(), String> {
        Ok(())
    }
}
