use crate::orchestration::departments::orchestrator::{BaseAgent, AgentTriggerType, DepartmentOrchestrator, Department};
use crate::orchestration::departments::types::{DepartmentType, DepartmentEvent, DepartmentConfig, ApprovalRequest, ActionRisk};

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
        vec!["tenant.report.weekly_health".to_string(), "tenant.inventory.analyze_stagnant".to_string(), "tenant.shift.ended".to_string()]
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


        if event.event_type == "tenant.shift.ended" {
            let shift_id = event.payload.get("shift_id").and_then(|v| v.as_str()).unwrap_or("unknown_shift");
            let db_url = std::env::var("DATABASE_URL").unwrap_or_default();
            if !db_url.is_empty() && event.tenant_id != "system" {
                if let Ok(pool) = sqlx::PgPool::connect(&db_url).await {
                    let mut tx = match pool.begin().await { Ok(tx) => tx, Err(_) => return Ok(()) };
                    let _ = ::server_common::auth_utils::set_org_context(&mut *tx, &event.tenant_id).await;

                    let completed_count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM ohc_staff_tasks WHERE tenant_id = $1 AND status = 'completed'")
                        .bind(&event.tenant_id)
                        .fetch_one(&mut *tx).await.unwrap_or((0,));

                    let escalated_count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM ohc_staff_tasks WHERE tenant_id = $1 AND status = 'escalated'")
                        .bind(&event.tenant_id)
                        .fetch_one(&mut *tx).await.unwrap_or((0,));

                    let summary_text = format!("Shift completed. {} tasks finished, {} issues escalated.", completed_count.0, escalated_count.0);
                    let summary_id = format!("summary_{}", uuid::Uuid::new_v4());

                    let _ = sqlx::query("INSERT INTO ohc_shift_summaries (id, tenant_id, shift_id, summary_text, issues_escalated, tasks_completed) VALUES ($1, $2, $3, $4, $5, $6)")
                        .bind(&summary_id)
                        .bind(&event.tenant_id)
                        .bind(&shift_id)
                        .bind(&summary_text)
                        .bind(escalated_count.0 as i32)
                        .bind(completed_count.0 as i32)
                        .execute(&mut *tx).await;

                    let _ = tx.commit().await;
                }
            }
            return Ok(());
        }
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

        if event.event_type == "tenant.report.weekly_health" {
            let gross_sales = event.payload.get("gross_sales").and_then(|v| v.as_f64()).unwrap_or(0.0);
            let orders_count = event.payload.get("orders_count").and_then(|v| v.as_i64()).unwrap_or(0);
            let top_seller_name = event.payload.get("top_seller_name").and_then(|v| v.as_str()).unwrap_or("N/A");

            let prompt = format!(
                "You are The Advisor. The user had {} orders this week. Gross sales: ${}. Most people bought {}. Generate a radically simple, plain-language business health report. Do not use jargon. Format the response as JSON with keys 'summary' and 'actionable_suggestion'.",
                orders_count, gross_sales, top_seller_name
            );

            let mut drafted_msg = r#"{"summary": "Great job this week! You made a good amount of sales.", "actionable_suggestion": "Want me to draft a new promotional post for your website?"}"#.to_string();

            let mut attempts = 0;
            let mut _ai_call_succeeded = false;
            while attempts < 3 {
                let ai_op = async {
                    if let Ok(mut client) = ::server_ohc::orchestration::hub_service_client::HubServiceClient::connect(std::env::var("OHC_HUB_URL").unwrap_or_else(|_| "http://127.0.0.1:8081".to_string())).await {
                        let reason_req = ::server_ohc::orchestration::ReasonRequest {
                            prompt: ::server_pricing::compression::reduce_tokens(&prompt),
                            from_agent_id: "The Advisor".into(),
                        };
                        if let Ok(res) = client.reason(tonic::Request::new(reason_req)).await {
                            return Ok(res.into_inner().content);
                        }
                    }
                    Err("AI call failed".to_string())
                };

                match tokio::time::timeout(std::time::Duration::from_secs(60), ai_op).await {
                    Ok(Ok(content)) => {
                        drafted_msg = content;
                        _ai_call_succeeded = true;
                        break;
                    },
                    _ => {
                        attempts += 1;
                        if attempts == 3 {
                            let _ = self.orchestrator.execute_action(
                                DepartmentType::BusinessAdvisory,
                                "AI Agent Paused: Advisory".to_string(),
                                event.tenant_id.clone(),
                                ActionRisk::DraftForReview,
                                serde_json::json!({"error": "The AI agent responsible for answering questions is paused because the AI service is unavailable."})
                            ).await;
                        } else {
                            tokio::time::sleep(std::time::Duration::from_secs(2u64.pow(attempts))).await;
                        }
                    }
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

    async fn execute(&self, payload: serde_json::Value) -> Result<(), String> {
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
                let mut attempts = 0;
                while attempts < 3 {
                    let ai_op = async {
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
                            if let Ok(_) = client.publish_mesh_event(tonic::Request::new(publish_req)).await {
                                return Ok(());
                            }
                        }
                        Err("Hub call failed".to_string())
                    };

                    match tokio::time::timeout(std::time::Duration::from_secs(60), ai_op).await {
                        Ok(Ok(_)) => {
                            break;
                        },
                        _ => {
                            attempts += 1;
                            tokio::time::sleep(std::time::Duration::from_secs(2u64.pow(attempts))).await;
                        }
                    }
                }
            }
        }

        Ok(())
    }
}
