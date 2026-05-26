use std::sync::Arc;
use crate::msgbus::{Bus, Message};
use crate::orchestration::departments::orchestrator::DepartmentOrchestrator;
use crate::orchestration::departments::types::DepartmentEvent;

pub struct DepartmentService {
    bus: Arc<dyn Bus>,
    orchestrator: Arc<DepartmentOrchestrator>,
}

impl DepartmentService {
    pub fn new(bus: Arc<dyn Bus>, orchestrator: Arc<DepartmentOrchestrator>) -> Self {
        DepartmentService {
            bus,
            orchestrator,
        }
    }

    pub async fn start(&self) -> Result<(), String> {
        let orchestrator_clone = self.orchestrator.clone();

        let handler = Box::new(move |msg: Message| {
            if msg.topic == "system:order_received" {
                let orchestrator = orchestrator_clone.clone();
                let payload_str = String::from_utf8_lossy(&msg.payload).to_string();

                // Try to parse the payload to extract tenant_id, default to e2e-tenant
                let tenant_id = if let Ok(json) = serde_json::from_str::<serde_json::Value>(&payload_str) {
                    json.get("tenant_id").and_then(|v| v.as_str()).unwrap_or("e2e-tenant").to_string()
                } else {
                    "e2e-tenant".to_string()
                };

                let event = DepartmentEvent {
                    id: uuid::Uuid::new_v4().to_string(),
                    tenant_id,
                    event_type: "tenant.order.created".to_string(),
                    payload: serde_json::json!({"source": "system_bus"}),
                };

                tokio::spawn(async move {
                    let _ = orchestrator.dispatch_event(event).await;
                });
            }
        });

        let _ = self.bus.subscribe("system:order_received".to_string(), handler).await?;

        Ok(())
    }
}
