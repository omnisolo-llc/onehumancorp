use std::sync::Arc;
use crate::db::DB;

pub struct VisionWorker {
    pub db: Arc<DB>,
    pub hub: Arc<crate::hub::Hub>,
}

impl VisionWorker {
    pub fn new(db: Arc<DB>, hub: Arc<crate::hub::Hub>) -> Self {
        Self { db, hub }
    }

    pub fn start(&self) {
        let hub = self.hub.clone();
        let mut vision_rx = hub.subscribe_teammate_mesh("vision_inbox".to_string());

        tokio::spawn(async move {
            while let Ok(event) = vision_rx.recv().await {
                if event.action == "VisualIntake" {
                    // In a real scenario, this would deserialize to interop::IntakePayload
                    // and call the LLM to get a visual estimate.
                    // For now, we mock the extraction.

                    if let Ok(payload_str) = String::from_utf8(event.payload.clone()) {
                        if let Ok(payload_json) = serde_json::from_str::<serde_json::Value>(&payload_str) {
                            let tenant_id = payload_json.get("tenant_id").and_then(|t| t.as_str()).unwrap_or("default_tenant");
                            let user_text = payload_json.get("user_text_context").and_then(|t| t.as_str()).unwrap_or("Unknown item");

                            // Mocking the result
                            let estimate = serde_json::json!({
                                "tenant_id": tenant_id,
                                "item_type": user_text,
                                "visible_damage": "minor scratches",
                                "estimated_size": "medium",
                                "complexity_score": 3,
                            });

                            let out_payload = serde_json::to_vec(&estimate).unwrap_or_default();

                            let out_event = ::server_ohc::orchestration::TeammateMeshEvent {
                                agent_id: "vision_sidecar".to_string(),
                                action: "tenant.quote.requested".to_string(),
                                status: "completed".to_string(),
                                payload: out_payload,
                                msg_id: uuid::Uuid::new_v4().to_string(),
                            };

                            let _ = hub.publish_teammate_event("sales_inbox".to_string(), out_event);
                        }
                    }
                }
            }
        });
    }
}
