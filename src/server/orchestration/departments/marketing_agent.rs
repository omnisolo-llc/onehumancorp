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
            "tenant.product.created".to_string(),
            "tenant.job.completed".to_string(),
            "tenant.inventory.updated".to_string(),
            "tenant.pricing.discount_applied".to_string(),
        ]
    }

    async fn handle_event(&self, event: &DepartmentEvent) -> Result<(), String> {
        let risk = ActionRisk::DraftForReview;


        if event.event_type == "tenant.product.created" {
            let product_name = event.payload.get("name").and_then(|v| v.as_str()).unwrap_or("a new product");

            let draft_copy = format!("Check out our new product: {}! 🚀 #newarrival #ohc", product_name);
            let payload = serde_json::json!({
                "feature_type": "social_post",
                "product_name": product_name,
                "draft_copy": draft_copy
            });
            let description = format!("7-Day Social Calendar: {}", product_name);

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

        if event.event_type == "tenant.product.created" || event.event_type == "tenant.inventory.updated" {
            let product_name = event.payload.get("name").and_then(|v| v.as_str()).unwrap_or("New Product");
            let description = event.payload.get("description").and_then(|v| v.as_str()).unwrap_or("");
            let images = event.payload.get("images").and_then(|v| v.as_array());

            let image_url = if let Some(imgs) = images {
                if !imgs.is_empty() {
                    imgs[0].as_str().unwrap_or("")
                } else {
                    ""
                }
            } else {
                ""
            };

            // Vision API: Optimize/Crop Image
            // In a real implementation this would call a Vision API or image processing service.
            // For now, we simulate the optimized image URL.
            let optimized_image_url = if !image_url.is_empty() {
                format!("{}_optimized.jpg", image_url.trim_end_matches(".jpg"))
            } else {
                "".to_string()
            };

            let prompt = format!("Draft a short, engaging Instagram caption for a new or restocked product named '{}'. Description: '{}'. Keep it energetic and include 3 relevant hashtags.", product_name, description);

            let draft_copy = if let Ok(provider) = std::env::var("OHC_LLM_PROVIDER") {
                if provider == "minimax" {
                    let minimax_key = std::env::var("MINIMAX_API_KEY").unwrap_or_default();
                    crate::minimax::MinimaxClient::new(minimax_key).reason(&prompt).await.unwrap_or_else(|_| format!("Check out our new {}!", product_name))
                } else {
                    crate::minimax::LocalLLMClient::new().reason(&prompt).await.unwrap_or_else(|_| format!("Check out our new {}!", product_name))
                }
            } else {
                crate::minimax::LocalLLMClient::new().reason(&prompt).await.unwrap_or_else(|_| format!("Check out our new {}!", product_name))
            };

            let payload = serde_json::json!({
                "feature_type": "social_post",
                "product_name": product_name,
                "image_url": optimized_image_url,
                "draft_copy": draft_copy
            });

            let action_desc = format!("Draft Instagram post for {}", product_name);
            return self.orchestrator.execute_action(DepartmentType::Marketing, action_desc, event.tenant_id.clone(), risk, payload).await.map(|_| ());
        }

        if event.event_type == "tenant.pricing.discount_applied" {
            let product_name = event.payload.get("product_name").and_then(|v| v.as_str()).unwrap_or("Product");
            let new_price = event.payload.get("new_price").and_then(|v| v.as_f64()).unwrap_or(0.0);

            let draft_copy = format!("🚨 FLASH SALE! 🚨 Grab our {} for just ${:.2} while supplies last! Link in bio. 🛍️✨ #Sale #FlashSale", product_name, new_price);
            let payload = serde_json::json!({
                "feature_type": "social_post",
                "product_name": product_name,
                "draft_copy": draft_copy
            });
            let description = format!("Draft Flash Sale Post: {}", product_name);

            return self.orchestrator.execute_action(
                DepartmentType::Marketing,
                description,
                event.tenant_id.clone(),
                ActionRisk::DraftForReview,
                payload,
            ).await.map(|_| ());
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
