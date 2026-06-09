use crate::orchestration::departments::orchestrator::{BaseAgent, AgentTriggerType, DepartmentOrchestrator, Department};
use crate::orchestration::departments::types::{DepartmentType, DepartmentEvent, DepartmentConfig, ApprovalRequest, ActionRisk};
use serde_json::Value;

pub struct BusinessAdvisoryAgent {
    orchestrator: std::sync::Arc<DepartmentOrchestrator>,
}

impl BusinessAdvisoryAgent {
    pub fn new(orchestrator: std::sync::Arc<DepartmentOrchestrator>) -> Self {
        Self { orchestrator }
    }
}

#[async_trait::async_trait]
impl Department for BusinessAdvisoryAgent {
    fn department_type(&self) -> DepartmentType {
        DepartmentType::BusinessAdvisory
    }

    fn subscribed_events(&self) -> Vec<String> {
        vec![
            "tenant.report.weekly_health".to_string(),
            "tenant.inventory.analyze_stagnant".to_string(),
            "tenant.report.morning_briefing".to_string(),
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

        if event.event_type == "tenant.inventory.analyze_stagnant" {
            // CRON-based inventory analysis pipeline to detect stagnant stock based on sales velocity
            // In a real scenario, this queries inventory and order_history tables.
            // For now, we simulate finding a stagnant product matching the smart pricing policy.

            let payload = serde_json::json!({
                "context": {
                    "smart_pricing": true,
                    "product_id": uuid::Uuid::new_v4().to_string(),
                    "product_name": "Winter Scarf",
                    "old_price": 50.0,
                    "new_price": 42.5,
                    "discount_amount": 7.5,
                    "sales_projection": "+$120",
                    "stagnant_days": 60,
                    "margin_percent": 40
                }
            });

            return self.orchestrator.execute_action(
                DepartmentType::BusinessAdvisory,
                "Smart Price Suggestion: Winter Scarf".to_string(),
                event.tenant_id.clone(),
                ActionRisk::DraftForReview, // Always draft for review for pricing changes initially
                payload
            ).await.map(|_| ());
        }

        if event.event_type == "tenant.report.morning_briefing" {
            // Simulated fetch from internal analytics endpoints:
            // /api/v1/analytics/daily_revenue
            // /api/v1/analytics/pending_bookings
            // /api/v1/analytics/unanswered_messages
            // We simulate the data directly here for the AI summary generation.
            let prompt = format!(
                "You are the Decision Assistant. Generate a plain-language Morning Briefing for the business owner. They have {} bookings today totaling ${}, and {} unanswered DMs. Include action pills like '[Review 2 Unread DMs]' in the summary.",
                3, 450, 2
            );

            let mut drafted_msg = r#"{"summary": "Good morning! You have 3 bookings today totaling $450, and 2 unanswered DMs.", "action_pills": ["[Review 2 Unread DMs]", "[View Bookings]"]}"#.to_string();

            if let Ok(mut client) = ::server_ohc::orchestration::hub_service_client::HubServiceClient::connect(std::env::var("OHC_HUB_URL").unwrap_or_else(|_| "http://127.0.0.1:8081".to_string())).await {
                let reason_req = ::server_ohc::orchestration::ReasonRequest {
                    prompt,
                    from_agent_id: "Decision Assistant".into(),
                };
                if let Ok(res) = client.reason(tonic::Request::new(reason_req)).await {
                    drafted_msg = res.into_inner().content;
                }
            }

            let parsed: serde_json::Value = serde_json::from_str(&drafted_msg).unwrap_or(serde_json::json!({
                "summary": drafted_msg,
                "action_pills": ["[Review 2 Unread DMs]"]
            }));

            let payload = serde_json::json!({
                "tenant_id": event.tenant_id.clone(),
                "context": {
                    "morning_briefing": true,
                    "summary": parsed.get("summary").and_then(|v| v.as_str()).unwrap_or("Good morning! You have 3 custom cake deliveries today totaling $450, and 2 unanswered DMs."),
                    "actionable_suggestion": "Review DMs"
                }
            });

            return self.orchestrator.execute_action(
                DepartmentType::BusinessAdvisory,
                "Draft Morning Briefing".to_string(),
                event.tenant_id.clone(),
                risk,
                payload,
            ).await.map(|_| ());
        }

        if event.event_type == "tenant.report.weekly_health" {
            let gross_sales = event.payload.get("gross_sales").and_then(|v| v.as_f64()).unwrap_or(0.0);
            let orders_count = event.payload.get("orders_count").and_then(|v| v.as_i64()).unwrap_or(0);
            let top_seller_name = event.payload.get("top_seller_name").and_then(|v| v.as_str()).unwrap_or("N/A");

            let prompt = format!(
                "You are The Advisor. The user had {} orders this week. Gross sales: ${}. Most people bought {}. Generate a radically simple, plain-language business health report. Do not use jargon. Format the response as JSON with keys 'summary' and 'actionable_suggestion'.",
                orders_count, gross_sales, top_seller_name
            );

            let mut drafted_msg = r#"{"summary": "Great job this week! You made a good amount of sales.", "actionable_suggestion": "Want me to draft a new promotional post for your website?"}"#.to_string();

            if let Ok(mut client) = ::server_ohc::orchestration::hub_service_client::HubServiceClient::connect(std::env::var("OHC_HUB_URL").unwrap_or_else(|_| "http://127.0.0.1:8081".to_string())).await {
                let reason_req = ::server_ohc::orchestration::ReasonRequest {
                    prompt,
                    from_agent_id: "The Advisor".into(),
                };
                if let Ok(res) = client.reason(tonic::Request::new(reason_req)).await {
                    drafted_msg = res.into_inner().content;
                }
            }

            let parsed: serde_json::Value = serde_json::from_str(&drafted_msg).unwrap_or(serde_json::json!({
                "summary": drafted_msg,
                "actionable_suggestion": "Consider adding a new promotion."
            }));

            let payload = serde_json::json!({
                "tenant_id": event.tenant_id.clone(),
                "context": {
                    "weekly_health_report": true,
                    "summary": parsed.get("summary").and_then(|v| v.as_str()).unwrap_or("Here is your weekly summary."),
                    "actionable_suggestion": parsed.get("actionable_suggestion").and_then(|v| v.as_str()).unwrap_or("Want me to run a promo?")
                }
            });

            return self.orchestrator.execute_action(
                DepartmentType::BusinessAdvisory,
                "Draft weekly business health report".to_string(),
                event.tenant_id.clone(),
                risk,
                payload,
            ).await.map(|_| ());
        }

        // Fallback
        Ok(())
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
impl BaseAgent for BusinessAdvisoryAgent {
    fn agent_id(&self) -> String {
        "business_advisory_agent".to_string()
    }

    fn trigger_type(&self) -> AgentTriggerType {
        AgentTriggerType::Scheduled
    }

    async fn execute(&self, payload: Value) -> Result<(), String> {
        // If triggered as a CRON job for inventory analysis
        if payload.get("action").and_then(|v| v.as_str()) == Some("analyze_stagnant_inventory") {
            let tenant_id = payload.get("tenant_id").and_then(|v| v.as_str()).unwrap_or("system");
            let event = DepartmentEvent {
                id: uuid::Uuid::new_v4().to_string(),
                tenant_id: tenant_id.to_string(),
                event_type: "tenant.inventory.analyze_stagnant".to_string(),
                payload: payload.clone(),
            };
            return self.handle_event(&event).await;
        }

        // Handle execution of the weekly health report action (e.g., if user approves the suggestion)
        if let Some(context) = payload.get("context") {
            if context.get("weekly_health_report").and_then(|v| v.as_bool()) == Some(true) {
                let tenant_id = payload.get("tenant_id").and_then(|v| v.as_str()).unwrap_or("system");

                // Dispatch a job to The Promoter
                if let Ok(mut client) = ::server_ohc::orchestration::hub_service_client::HubServiceClient::connect(std::env::var("OHC_HUB_URL").unwrap_or_else(|_| "http://127.0.0.1:8081".to_string())).await {
                    let publish_req = ::server_ohc::orchestration::PublishMeshEventRequest {
                        event: Some(::server_ohc::orchestration::MeshEvent {
                            event_id: uuid::Uuid::new_v4().to_string(),
                            topic: "tenant.marketing.draft_requested".to_string(),
                            payload: serde_json::to_string(&serde_json::json!({
                                "name": "Social Post from Advisory",
                                "description": context.get("actionable_suggestion").and_then(|v| v.as_str()).unwrap_or(""),
                                "action": "draft_social_post",
                                "tenant_id": tenant_id.to_string()
                            })).unwrap_or_default().into_bytes(),
                            ..Default::default()
                        }),
                    };
                    let _ = client.publish_mesh_event(tonic::Request::new(publish_req)).await;
                }
            }
        }

        Ok(())
    }
}
