use crate::orchestration::departments::orchestrator::{BaseAgent, AgentTriggerType, DepartmentOrchestrator, Department};
use base64::{Engine as _, engine::general_purpose};
use crate::orchestration::departments::types::{DepartmentType, DepartmentEvent, DepartmentConfig, ApprovalRequest, ActionRisk};
use serde_json::Value;

pub struct OperationsAgent {
    orchestrator: std::sync::Arc<DepartmentOrchestrator>,
    configs: std::sync::RwLock<std::collections::HashMap<String, DepartmentConfig>>,
}

impl OperationsAgent {
    pub fn new(orchestrator: std::sync::Arc<DepartmentOrchestrator>) -> Self {
        Self { orchestrator, configs: std::sync::RwLock::new(std::collections::HashMap::new()) }
    }
}

#[async_trait::async_trait]
impl Department for OperationsAgent {
    fn department_type(&self) -> DepartmentType {
        DepartmentType::Operations
    }

    fn subscribed_events(&self) -> Vec<String> {
        vec![
            "tenant.quote.accepted".to_string(),
            "tenant.order.created".to_string(),
        ]
    }

    async fn handle_event(&self, event: &DepartmentEvent) -> Result<(), String> {
        let config = self.get_config(&event.tenant_id);
        let risk = if let Some(cfg) = config {
            if cfg.auto_approve_limits > 0.0 {
                ActionRisk::AutoExecute
            } else {
                ActionRisk::DraftForReview
            }
        } else {
            ActionRisk::DraftForReview
        };

        let action_description = if event.event_type == "tenant.order.created" {
            "Process Order & Update Inventory".to_string()
        } else {
            "Create order and booking".to_string()
        };

        self.orchestrator.execute_action(
            DepartmentType::Operations,
            action_description,
            event.tenant_id.clone(),
            risk,
            event.payload.clone(),
        ).await?;

        // Dispatch event for customer success agent
        let cs_event = DepartmentEvent {
            id: uuid::Uuid::new_v4().to_string(),
            tenant_id: event.tenant_id.clone(),
            event_type: "tenant.order.fulfillment_ready".to_string(),
            payload: event.payload.clone(),
        };
        self.orchestrator.dispatch_event(cs_event).await
    }

    fn get_config(&self, _tenant_id: &str) -> Option<DepartmentConfig> {
        self.configs.read().unwrap().get(_tenant_id).cloned().or_else(|| {
            Some(DepartmentConfig { tone_of_voice: "professional".to_string(), auto_approve_limits: 10.0, printnode_api_key: None, printnode_printer_id: None })
        })
    }

    fn set_config(&mut self, tenant_id: String, config: DepartmentConfig) {
        self.configs.write().unwrap().insert(tenant_id, config);
    }

    async fn query_memory(&self, _query: &str) -> Result<Vec<String>, String> {
        Ok(vec![])
    }

    async fn request_approval(&self, description: String, tenant_id: String, risk: ActionRisk) -> Result<ApprovalRequest, String> {
        self.orchestrator.execute_action(self.department_type(), description.clone(), tenant_id.clone(), risk, serde_json::json!({})).await
    }
}

#[async_trait::async_trait]
impl BaseAgent for OperationsAgent {
    fn agent_id(&self) -> String {
        "operations_agent".to_string()
    }

    fn trigger_type(&self) -> AgentTriggerType {
        AgentTriggerType::EventDriven
    }

    async fn execute(&self, payload: Value) -> Result<(), String> {
        if let Some(action) = payload.get("action").and_then(|v| v.as_str()) {
            if action == "print_receipt" {
                let tenant_id = payload.get("tenant_id").and_then(|v| v.as_str()).unwrap_or_default();
                let config = self.get_config(tenant_id);

                if let Some(cfg) = config {
                    if let (Some(api_key), Some(printer_id_str)) = (cfg.printnode_api_key, cfg.printnode_printer_id) {
                        if let Ok(printer_id) = printer_id_str.parse::<i32>() {
                            let provider = crate::integrations::printnode::provider::PrintNodeProvider::new(api_key);
                            let content = format!("Receipt for Order {}", payload.get("order_id").and_then(|v| v.as_str()).unwrap_or("Unknown"));
                            let content_b64 = general_purpose::STANDARD.encode(content);
                            let req = crate::integrations::printnode::provider::PrintJobRequest {
                                printer_id: printer_id,
                                title: "Receipt".to_string(),
                                content_type: "raw_base64".to_string(),
                                content: content_b64,
                                source: "OHC Swarm".to_string(),
                            };
                            let _ = provider.print_job(req).await;
                        }
                    }
                }
            } else if action == "check_printnode" {
                let configs = self.configs.read().unwrap();
                for (tenant_id, cfg) in configs.iter() {
                    if cfg.printnode_api_key.is_some() && cfg.printnode_printer_id.is_some() {
                        tracing::info!("Checked PrintNode printers for tenant");
                    }
                }
            }
        }
        Ok(())
    }
}
