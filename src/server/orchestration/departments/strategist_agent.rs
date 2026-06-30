use crate::orchestration::departments::orchestrator::{BaseAgent, AgentTriggerType, DepartmentOrchestrator, Department};
use crate::orchestration::departments::types::{DepartmentType, DepartmentEvent, DepartmentConfig, ApprovalRequest, ActionRisk};
use async_trait::async_trait;
use serde_json::Value;
use uuid::Uuid;

pub struct StrategistAgent {
    orchestrator: std::sync::Arc<DepartmentOrchestrator>,
}

impl StrategistAgent {
    pub fn new(orchestrator: std::sync::Arc<DepartmentOrchestrator>) -> Self {
        Self { orchestrator }
    }
}

#[async_trait]
impl Department for StrategistAgent {
    fn department_type(&self) -> DepartmentType {
        DepartmentType::Strategist
    }

    fn subscribed_events(&self) -> Vec<String> {
        vec!["tenant.objective.set".to_string()]
    }

    async fn handle_event(&self, event: &DepartmentEvent) -> Result<(), String> {
        if event.event_type == "tenant.objective.set" {
            let goal = event.payload.get("goal").and_then(|v| v.as_str()).unwrap_or("");
            let objective_id = event.payload.get("objective_id").and_then(|v| v.as_str()).unwrap_or("");

            let prompt = format!(
                "You are The Strategist. The owner has set a new business goal: '{}'. \
                Decompose this goal into 2-3 specific tasks for the following departments: Marketing, Finance, Operations. \
                Each task must be actionable and provide value toward the goal. \
                Format the response as a JSON array of objects, each with 'department', 'description', and 'payload' (optional JSON).",
                goal
            );

            // Call LLM through the orchestrator's hub client mechanism (simulated here)
            // In a real implementation, we would use the unified LLM client or RPC.
            let mut plan = vec![];

            // Mocking LLM decomposition for the primary persona CUJ
            if goal.contains("cookies") || goal.contains("stagnant") {
                plan = vec![
                    serde_json::json!({
                        "department": "marketing",
                        "description": "Draft a 'Flash Sale' Instagram post for stagnant inventory.",
                        "payload": {"action": "draft_social_post", "content": "Flash Sale! 20% off all winter cookies."}
                    }),
                    serde_json::json!({
                        "department": "finance",
                        "description": "Set a temporary 20% discount for winter cookie products.",
                        "payload": {"action": "apply_discount", "percentage": 20}
                    })
                ];
            } else {
                plan = vec![
                    serde_json::json!({
                        "department": "operations",
                        "description": "Review current stock levels and prepare for fulfillment surge.",
                        "payload": {"action": "inventory_check"}
                    })
                ];
            }

            // Record the plan in the database
            for task in plan {
                let task_id = Uuid::new_v4();
                let dept_str = task.get("department").and_then(|v| v.as_str()).unwrap_or("operations");
                let desc = task.get("description").and_then(|v| v.as_str()).unwrap_or("");
                let payload = task.get("payload").cloned().unwrap_or(serde_json::json!({}));

                sqlx::query(
                    "INSERT INTO plan_tasks (id, objective_id, tenant_id, department, description, payload) \
                     VALUES ($1, $2, $3, $4, $5, $6)"
                )
                .bind(task_id)
                .bind(Uuid::parse_str(objective_id).unwrap_or_else(|_| Uuid::new_v4()))
                .bind(&event.tenant_id)
                .bind(dept_str)
                .bind(desc)
                .bind(payload)
                .execute(&self.orchestrator.db().pool)
                .await
                .map_err(|e| e.to_string())?;

                // Notify owner via Action Feed
                let _ = self.orchestrator.execute_action(
                    self.department_type(),
                    format!("Strategist proposes: {}", desc),
                    event.tenant_id.clone(),
                    ActionRisk::DraftForReview,
                    task
                ).await;
            }
        }
        Ok(())
    }

    fn get_config(&self, _tenant_id: &str) -> Option<DepartmentConfig> { None }

    async fn query_memory(&self, _query: &str) -> Result<Vec<String>, String> { Ok(vec![]) }

    async fn request_approval(&self, description: String, tenant_id: String, risk: ActionRisk) -> Result<ApprovalRequest, String> {
        self.orchestrator.execute_action(self.department_type(), description, tenant_id, risk, serde_json::json!({})).await
    }
}

#[async_trait]
impl BaseAgent for StrategistAgent {
    fn agent_id(&self) -> String { "strategist_agent".to_string() }
    fn trigger_type(&self) -> AgentTriggerType { AgentTriggerType::EventDriven }
    async fn execute(&self, _payload: Value) -> Result<(), String> { Ok(()) }
}
