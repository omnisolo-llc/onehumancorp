use crate::orchestration::departments::orchestrator::{BaseAgent, AgentTriggerType, DepartmentOrchestrator, Department};
use crate::orchestration::departments::types::{DepartmentType, DepartmentEvent, DepartmentConfig, ApprovalRequest, ActionRisk};
use serde_json::Value;

pub struct MarketingAgent {
    orchestrator: std::sync::Arc<DepartmentOrchestrator>,
}

impl MarketingAgent {
    pub fn new(orchestrator: std::sync::Arc<DepartmentOrchestrator>) -> Self {
        Self { orchestrator }
    }
}

#[async_trait::async_trait]
impl Department for MarketingAgent {
    fn department_type(&self) -> DepartmentType {
        DepartmentType::Marketing
    }

    fn subscribed_events(&self) -> Vec<String> {
        vec![
            "tenant.insight.trending".to_string(),
            "tenant.job.completed".to_string(),
            "tenant.product.created".to_string(),
        ]
    }


    async fn handle_event(&self, event: &DepartmentEvent) -> Result<(), String> {
        let risk = ActionRisk::DraftForReview;

        if event.event_type == "tenant.product.created" {
            let product_id = event.payload.get("product_id").and_then(|v| v.as_str()).unwrap_or("");
            let product_info = self.orchestrator.get_product(&event.tenant_id, product_id).await?;

            let (title, desc) = if let Some((t, d)) = product_info {
                (t, d)
            } else {
                (
                    event.payload.get("name").and_then(|v| v.as_str()).unwrap_or("New Product").to_string(),
                    event.payload.get("description").and_then(|v| v.as_str()).unwrap_or("Check out our newest arrival!").to_string()
                )
            };

            let prompt = format!("Draft an engaging, brief Instagram-ready caption (with hashtags) for a new product we just launched. Product name: {}. Description: {}.", title, desc);

            let mut draft_caption = format!("We just added {} to our catalog! 🚀 Check it out today.", title);

            let llm = crate::minimax::LocalLLMClient::new();
            if let Ok(generated) = llm.reason(&prompt).await {
                draft_caption = generated;
            }

            let payload = serde_json::json!({
                "feature_type": "social_post",
                "product_name": title,
                "draft_copy": draft_caption,
                "media_url": event.payload.get("media_url").and_then(|v| v.as_str()).unwrap_or(""),
            });

            let description = format!("Draft social media post for new product: {}", title);

            return self.orchestrator.execute_action(
                DepartmentType::Marketing,
                description,
                event.tenant_id.clone(),
                risk,
                payload,
            ).await.map(|_| ());
        }

        if event.event_type == "tenant.job.completed" {

            let service_name = event.payload.get("service_name").and_then(|v| v.as_str()).unwrap_or("Service");
            let media = event.payload.get("media").and_then(|v| v.as_array());

            if let Some(media_array) = media {
                if !media_array.is_empty() {
                    let media_url = media_array[0].as_str().unwrap_or("");

                    let draft_copy = format!("Beautiful new {} completed recently. Completed on time and on budget.", service_name.to_lowercase());

                    let payload = serde_json::json!({
                        "feature_type": "case_study",
                        "service_name": service_name,
                        "media_url": media_url,
                        "draft_copy": draft_copy
                    });

                    let description = format!("Draft portfolio case study for {}", service_name);

                    return self.orchestrator.execute_action(
                        DepartmentType::Marketing,
                        description,
                        event.tenant_id.clone(),
                        risk,
                        payload,
                    ).await.map(|_| ());
                }
            }
        }

        self.orchestrator.execute_action(
            DepartmentType::Marketing,
            "Draft social media campaign for trending item".to_string(),
            event.tenant_id.clone(),
            risk,
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
impl BaseAgent for MarketingAgent {
    fn agent_id(&self) -> String {
        "marketing_agent".to_string()
    }

    fn trigger_type(&self) -> AgentTriggerType {
        AgentTriggerType::EventDriven
    }

    async fn execute(&self, _payload: Value) -> Result<(), String> {
        Ok(())
    }
}
