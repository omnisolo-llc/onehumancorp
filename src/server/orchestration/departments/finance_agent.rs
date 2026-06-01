use crate::orchestration::departments::orchestrator::{BaseAgent, AgentTriggerType, DepartmentOrchestrator, Department};
use crate::orchestration::departments::types::{DepartmentType, DepartmentEvent, DepartmentConfig, ApprovalRequest, ActionRisk};
use serde_json::Value;

pub struct FinanceAgent {
    orchestrator: std::sync::Arc<DepartmentOrchestrator>,
}

impl FinanceAgent {
    pub fn new(orchestrator: std::sync::Arc<DepartmentOrchestrator>) -> Self {
        Self { orchestrator }
    }
}

#[async_trait::async_trait]
impl Department for FinanceAgent {
    fn department_type(&self) -> DepartmentType {
        DepartmentType::Finance
    }

    fn subscribed_events(&self) -> Vec<String> {
        vec!["tenant.payment.received".to_string()]
    }

    async fn handle_event(&self, event: &DepartmentEvent) -> Result<(), String> {
        if event.event_type == "tenant.payment.received" {
            let product_id = event.payload.get("metadata")
                .and_then(|m| m.get("product_id"))
                .and_then(|v| v.as_str())
                .unwrap_or("");

            let amount_cents = event.payload.get("amount_received")
                .or_else(|| event.payload.get("amount"))
                .and_then(|v| v.as_i64())
                .unwrap_or(0);

            let payment_intent_id = event.payload.get("id")
                .and_then(|v| v.as_str())
                .unwrap_or("");

            if !product_id.is_empty() && amount_cents > 0 {
                // Query split rules
                let db_pool = crate::db::get_pool();
                let rule_query = sqlx::query!(
                    "SELECT partner_id, partner_phone_or_email, split_type, split_value FROM split_payment_rules WHERE tenant_id = $1 AND product_id = $2",
                    event.tenant_id, product_id
                );

                if let Ok(rules) = rule_query.fetch_all(&db_pool).await {
                    for rule in rules {
                        let mut partner_cents: i64 = 0;
                        if rule.split_type == "percentage" {
                            // Ensure precision handling. e.g. "30" means 30%
                            let percentage = sqlx::types::BigDecimal::to_string(&rule.split_value).parse::<f64>().unwrap_or(0.0);
                            partner_cents = ((amount_cents as f64) * (percentage / 100.0)).round() as i64;
                        } else if rule.split_type == "flat" {
                            let flat_amount = sqlx::types::BigDecimal::to_string(&rule.split_value).parse::<f64>().unwrap_or(0.0);
                            partner_cents = (flat_amount * 100.0).round() as i64;
                        }

                        if partner_cents > 0 {
                            let partner_connect_id = if rule.partner_id.starts_with("acct_") {
                                Some(rule.partner_id.clone())
                            } else {
                                // Needs onboarding
                                None
                            };

                            if let Some(connect_id) = partner_connect_id {
                                if let Ok(stripe_key) = std::env::var("STRIPE_API_KEY") {
                                    let stripe_client = crate::integrations::stripe::client::StripeClient::new(stripe_key);
                                    let _ = stripe_client.create_transfer(partner_cents, &connect_id, payment_intent_id).await;
                                    tracing::info!("Executed Stripe Connect transfer to partner {}: {} cents", connect_id, partner_cents);
                                }
                            } else {
                                // Queue SMS onboarding task
                                let sms_event = DepartmentEvent {
                                    id: uuid::Uuid::new_v4().to_string(),
                                    tenant_id: event.tenant_id.clone(),
                                    event_type: "tenant.notification.send_sms".to_string(),
                                    source: "finance_agent".to_string(),
                                    payload: serde_json::json!({
                                        "phone_number": rule.partner_phone_or_email,
                                        "message": format!("You've been added to a job on OHC! Tap here to tell us where to send your {} cents cut: https://ohc.app/onboard/partner/{}", partner_cents, rule.partner_id)
                                    }),
                                };
                                let _ = self.orchestrator.dispatch_event(sms_event).await;
                            }

                            // Trigger briefing
                            let briefing_event = DepartmentEvent {
                                id: uuid::Uuid::new_v4().to_string(),
                                tenant_id: event.tenant_id.clone(),
                                event_type: "tenant.briefing.generated".to_string(),
                                source: "finance_agent".to_string(),
                                payload: serde_json::json!({
                                    "message": format!("A split payment of {} cents was processed for your partner {}.", partner_cents, rule.partner_id)
                                }),
                            };
                            let _ = self.orchestrator.dispatch_event(briefing_event).await;
                        }
                    }
                }
            }
        }

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

        self.orchestrator.execute_action(
            DepartmentType::Finance,
            "Record deposit and track payment".to_string(),
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
impl BaseAgent for FinanceAgent {
    fn agent_id(&self) -> String {
        "finance_agent".to_string()
    }

    fn trigger_type(&self) -> AgentTriggerType {
        AgentTriggerType::Scheduled
    }

    async fn execute(&self, _payload: Value) -> Result<(), String> {
        // Run scheduled worker to aggregate weekly sales data.
        Ok(())
    }
}
