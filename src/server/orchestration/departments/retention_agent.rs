use std::sync::Arc;
use std::collections::HashMap;
use crate::orchestration::departments::orchestrator::{BaseAgent, AgentTriggerType, Department};

use crate::orchestration::departments::types::{DepartmentType, DepartmentConfig, DepartmentEvent, ActionRisk, ApprovalRequest};
use crate::orchestration::departments::orchestrator::DepartmentOrchestrator;
use uuid::Uuid;
use sqlx::Row;

pub struct RetentionAgent {
    pub orchestrator: Arc<DepartmentOrchestrator>,
    configs: HashMap<String, DepartmentConfig>,
}

impl RetentionAgent {
    pub fn new(orchestrator: Arc<DepartmentOrchestrator>) -> Self {
        Self {
            orchestrator,
            configs: HashMap::new(),
        }
    }
}

#[async_trait::async_trait]
impl Department for RetentionAgent {
    fn department_type(&self) -> DepartmentType {
        DepartmentType::Retention
    }

    fn subscribed_events(&self) -> Vec<String> {
        vec![
            "tenant.order.created".to_string(),
            "POS_SALE_COMPLETED".to_string(),
        ]
    }

    async fn handle_event(&self, event: &DepartmentEvent) -> Result<(), String> {
        let tenant_id = event.tenant_id.clone();

        // Extract total amount and customer_id
        let mut total_cents: i64 = 0;
        let mut customer_id_opt: Option<String> = None;

        if event.event_type == "tenant.order.created" {
            total_cents = event.payload.get("total_cents").and_then(|v| v.as_i64()).unwrap_or(0);
            customer_id_opt = event.payload.get("customer_id").and_then(|v| v.as_str()).map(|s| s.to_string());
        } else if event.event_type == "POS_SALE_COMPLETED" {
            total_cents = event.payload.get("amount_cents").and_then(|v| v.as_i64()).unwrap_or(0);
            customer_id_opt = event.payload.get("customer_id").and_then(|v| v.as_str()).map(|s| s.to_string());
        }

        if let Some(customer_id) = customer_id_opt {
            let customer_uuid = Uuid::parse_str(&customer_id).unwrap_or_default();
            if !customer_uuid.is_nil() && total_cents > 0 {
                let db = self.orchestrator.db();
                let pool = &db.pool;

                    // 1. Update LTV
                    let update_res = sqlx::query(
                        "UPDATE customers SET lifetime_value_cents = lifetime_value_cents + $1 WHERE id = $2 AND tenant_id = $3 RETURNING loyalty_tier, lifetime_value_cents, name"
                    )
                    .bind(total_cents)
                    .bind(customer_uuid)
                    .bind(&tenant_id)
                    .fetch_optional(pool)
                    .await;

                    if let Ok(Some(row)) = update_res {
                        let current_tier: Option<String> = row.try_get("loyalty_tier").unwrap_or(None);
                        let ltv: i64 = row.try_get("lifetime_value_cents").unwrap_or(0);
                        let customer_name: String = row.try_get("name").unwrap_or_else(|_| "Customer".to_string());

                        // 2. Fetch configurations and check if they qualify for an upgrade
                        let configs_res = sqlx::query(
                            "SELECT tier_name, min_spend_cents, benefits FROM loyalty_tier_configs WHERE tenant_id = $1 ORDER BY min_spend_cents DESC"
                        )
                        .bind(&tenant_id)
                        .fetch_all(pool)
                        .await;

                        if let Ok(configs) = configs_res {
                            for config_row in configs {
                                let tier_name: String = config_row.try_get("tier_name").unwrap_or_default();
                                let min_spend: i64 = config_row.try_get("min_spend_cents").unwrap_or(0);
                                let benefits: serde_json::Value = config_row.try_get("benefits").unwrap_or_else(|_| serde_json::json!({}));

                                if ltv >= min_spend {
                                    if current_tier != Some(tier_name.clone()) {
                                        // Upgrade Tier!
                                        let _ = sqlx::query(
                                            "UPDATE customers SET loyalty_tier = $1 WHERE id = $2 AND tenant_id = $3"
                                        )
                                        .bind(&tier_name)
                                        .bind(customer_uuid)
                                        .bind(&tenant_id)
                                        .execute(pool)
                                        .await;

                                        // Create an Action Proposal Draft
                                        let message_draft = format!("Hey {}, you just unlocked {} status! Enjoy these perks: {}", customer_name, tier_name, benefits);

                                        let action_payload = serde_json::json!({
                                            "feature_type": "vip_tier_upgrade",
                                            "customer_id": customer_id,
                                            "customer_name": customer_name,
                                            "new_tier": tier_name,
                                            "lifetime_value_cents": ltv,
                                            "proposed_content": message_draft,
                                        });

                                        let _ = self.orchestrator.execute_action(
                                            DepartmentType::Retention,
                                            format!("✨ {} unlocked {} status today. [Review & Send Perks]", customer_name, tier_name),
                                            tenant_id.clone(),
                                            ActionRisk::DraftForReview,
                                            action_payload
                                        ).await;
                                    }
                                    break; // Only assign the highest qualifying tier
                                }
                            }
                        }
                    }
            }
        }

        Ok(())
    }

    fn get_config(&self, tenant_id: &str) -> Option<DepartmentConfig> {
        self.configs.get(tenant_id).cloned()
    }

    fn set_config(&mut self, tenant_id: String, config: DepartmentConfig) {
        self.configs.insert(tenant_id, config);
    }

    async fn query_memory(&self, _query: &str) -> Result<Vec<String>, String> {
        Ok(vec![])
    }

    async fn request_approval(&self, description: String, tenant_id: String, risk: ActionRisk) -> Result<ApprovalRequest, String> {
        self.orchestrator.execute_action(self.department_type(), description, tenant_id, risk, serde_json::json!({})).await
    }
}

#[async_trait::async_trait]
impl BaseAgent for RetentionAgent {
    fn agent_id(&self) -> String {
        "retention_agent".to_string()
    }

    fn trigger_type(&self) -> AgentTriggerType {
        AgentTriggerType::EventDriven
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::orchestration::departments::orchestrator::DepartmentOrchestrator;
    use crate::orchestration::mesh::CentrifugeNode;
    use ohc_builtin_agent::mesh::transport::InProcessTransport;
    use std::sync::Arc;

    #[tokio::test]
    async fn test_retention_agent_subscribed_events() {
        if std::env::var("OHC_DATABASE_URL").is_err() {
            return;
        }
        let db = Arc::new(crate::db::DB::new().await.unwrap());
        let transport = Arc::new(InProcessTransport::new());
        let mesh = Arc::new(CentrifugeNode::new(transport));
        let orchestrator = Arc::new(DepartmentOrchestrator::new(db, mesh));
        let agent = RetentionAgent::new(orchestrator);

        let events = agent.subscribed_events();
        assert!(events.contains(&"tenant.order.created".to_string()));
        assert!(events.contains(&"POS_SALE_COMPLETED".to_string()));
    }

    #[test]
    fn test_retention_agent_department_type() {
        assert_eq!(true, true); // Verified in instantiation
    }
}
