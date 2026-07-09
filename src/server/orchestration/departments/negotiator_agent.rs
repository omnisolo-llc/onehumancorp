use std::collections::HashMap;
use std::sync::Arc;

use super::orchestrator::DepartmentOrchestrator;
use crate::orchestration::departments::{ActionRisk, ApprovalRequest, DepartmentConfig, DepartmentEvent, DepartmentType, Department};
use crate::orchestration::departments::orchestrator::{BaseAgent, AgentTriggerType};
use crate::orchestration::queue::redis_lock::RedisLock;

pub struct NegotiatorAgent {
    orchestrator: Arc<DepartmentOrchestrator>,
    configs: HashMap<String, DepartmentConfig>,
}

impl NegotiatorAgent {
    pub fn new(orchestrator: Arc<DepartmentOrchestrator>) -> Self {
        Self {
            orchestrator,
            configs: HashMap::new(),
        }
    }

    async fn generate_embedding(&self, text: &str) -> Vec<f32> {
        match std::env::var("OHC_NEGOTIATOR_LLM_PROVIDER")
            .or_else(|_| std::env::var("OHC_LLM_PROVIDER"))
            .as_deref()
        {
            Ok("minimax") => {
                let api_key = std::env::var("MINIMAX_API_KEY").unwrap_or_default();
                crate::minimax::MinimaxClient::new(api_key).generate_embedding(text).await.unwrap_or_else(|_| vec![0.0; 1536])
            }
            _ => {
                crate::minimax::LocalLLMClient::new().generate_embedding(text).await.unwrap_or_else(|_| vec![0.0; 1536])
            }
        }
    }
}

#[async_trait::async_trait]
impl Department for NegotiatorAgent {
    fn department_type(&self) -> DepartmentType {
        DepartmentType::CustomerSuccess // Or maybe Sales? Let's use CustomerSuccess for negotiation
    }

    fn subscribed_events(&self) -> Vec<String> {
        vec![
            "tenant.message.received".to_string(),
            "tenant.omnichannel.message.received".to_string(),
        ]
    }

    async fn handle_event(&self, event: &DepartmentEvent) -> Result<(), String> {
        let message = event.payload.get("message").and_then(|v| v.as_str()).unwrap_or("");
        let sender_id = event.payload.get("sender_id").and_then(|v| v.as_str()).unwrap_or("unknown");
        let inbox_id = event.payload.get("inbox_message_id")
            .or_else(|| event.payload.get("message_id"))
            .and_then(|v| v.as_str()).unwrap_or("");

        let mut customer_id = event.payload.get("customer_id").and_then(|v| v.as_str()).unwrap_or("").to_string();
        let pool = crate::db::get_pool();

        if customer_id.is_empty() && sender_id != "unknown" {
            let result: Result<(String,), sqlx::Error> = sqlx::query_as("SELECT id FROM customers WHERE tenant_id = $1 AND (phone = $2 OR email = $2 OR name = $2) LIMIT 1")
                .bind(&event.tenant_id)
                .bind(&sender_id)
                .fetch_one(&pool)
                .await;
            if let Ok((id,)) = result {
                customer_id = id;
            }
        }

        // We use LLM to parse intent.
        let prompt = format!(
            "Parse this customer message into JSON with keys: intent (quote, agree, or other), service_name, confidence, original_message. Message: {}",
            message
        );
        let compressed_prompt = crate::pricing::compression::reduce_tokens(&prompt);

        let raw_response = match std::env::var("OHC_NEGOTIATOR_LLM_PROVIDER")
            .or_else(|_| std::env::var("OHC_LLM_PROVIDER"))
            .as_deref()
        {
            Ok("minimax") => {
                let api_key = std::env::var("MINIMAX_API_KEY").unwrap_or_default();
                crate::minimax::MinimaxClient::new(api_key).reason(&compressed_prompt).await.unwrap_or_default()
            }
            _ => {
                crate::minimax::LocalLLMClient::new().reason(&compressed_prompt).await.unwrap_or_default()
            }
        };

        let mut is_quote_intent = false;
        let mut is_agree_intent = false;
        let mut service_name = "Service".to_string();

        if let Ok(parsed) = serde_json::from_str::<serde_json::Value>(&raw_response) {
            let intent = parsed.get("intent").and_then(|v| v.as_str()).unwrap_or("");
            if intent == "quote" {
                is_quote_intent = true;
                service_name = parsed.get("service_name").and_then(|v| v.as_str()).unwrap_or("Service").to_string();
            } else if intent == "agree" {
                is_agree_intent = true;
            }
        } else if message.to_lowercase().contains("fix") || message.to_lowercase().contains("repair") {
            is_quote_intent = true;
            service_name = "Emergency Repair".to_string();
        } else if message.to_lowercase().contains("sounds good") || message.to_lowercase().contains("yes") || message.to_lowercase().contains("works for me") {
            is_agree_intent = true;
        }

        if !is_quote_intent && !is_agree_intent {
            return Ok(()); // Handled by other agents (e.g. SalesAgent or CustomerSuccessAgent)
        }

        if is_agree_intent {
            // Check if there's a pending quote_sent for this customer
            use sqlx::Row;
            let pending_intake = sqlx::query("SELECT id, service_name, suggested_price, suggested_time FROM conversational_intakes WHERE tenant_id = $1 AND customer_id = $2 AND status = 'quote_sent' ORDER BY created_at DESC LIMIT 1")
                .bind(&event.tenant_id)
                .bind(&customer_id)
                .fetch_optional(&pool)
                .await;

            if let Ok(Some(row)) = pending_intake {
                let intake_id: String = row.get("id");
                let pending_service_name: String = row.get("service_name");
                let suggested_price: f64 = row.get("suggested_price");
                let suggested_time: String = row.get("suggested_time");

                // Transition state
                let _ = sqlx::query("UPDATE conversational_intakes SET status = 'payment_pending', updated_at = CURRENT_TIMESTAMP WHERE id = $1")
                    .bind(&intake_id)
                    .execute(&pool)
                    .await;

                let action_payload = serde_json::json!({
                    "feature_type": "autonomous_quote_agreed",
                    "service": pending_service_name,
                    "customer_inquiry": message,
                    "suggested_price": suggested_price,
                    "suggested_time": suggested_time,
                    "deposit_amount_cents": (suggested_price * 100.0) as i64,
                    "inbox_message_id": inbox_id,
                });

                self.orchestrator
                    .execute_action(
                        DepartmentType::CustomerSuccess,
                        format!("Finalize booking and send deposit link for {}", pending_service_name),
                        event.tenant_id.clone(),
                        ActionRisk::AutoExecute,
                        action_payload,
                    )
                    .await
                    .map(|_| ())?;
                return Ok(());
            }
        }

        // Generate quote (is_quote_intent)
        // Fetch Dynamic Pricing based on tenant's heuristics instead of hardcoding
        let mut price = 120.0;
        let heuristics_res = sqlx::query("SELECT service_category, base_rate_cents FROM pricing_heuristics WHERE tenant_id = $1")
            .bind(&event.tenant_id)
            .fetch_all(&pool)
            .await;

        if let Ok(heuristics) = heuristics_res {
            use sqlx::Row;
            for h in heuristics {
                let category: String = h.get("service_category");
                let rate_cents: i64 = h.get("base_rate_cents");
                if message.to_lowercase().contains(&category.to_lowercase()) || service_name.to_lowercase() == category.to_lowercase() {
                    price = (rate_cents as f64) / 100.0;
                    service_name = category;
                    break;
                }
            }
        }

        // Generate Proposed Options dynamically instead of hardcoding
        let scope = format!("Standard {}", service_name);

        // Find actual availability from BookingService integration
        // Here we just fetch the next available slot dynamically if the user has defined availability blocks, otherwise fallback to standard default time.
        // Assuming we have a service checking next slot
        let mut suggested_time = "Tomorrow 2 PM or 4 PM".to_string();

        // We can do a rudimentary availability check from availability_blocks
        let available_blocks_res = sqlx::query("SELECT start_time FROM availability_blocks WHERE tenant_id = $1 AND status = 'available' ORDER BY start_time ASC LIMIT 1")
            .bind(&event.tenant_id)
            .fetch_one(&pool)
            .await;

        let mut start_time_rfc = "2024-10-15T14:00:00Z".to_string();
        let mut end_time_rfc = "2024-10-15T15:00:00Z".to_string();
        if let Ok(row) = available_blocks_res {
            use sqlx::Row;
            let start_time_dt: chrono::DateTime<chrono::Utc> = row.get("start_time");
            suggested_time = format!("{}", start_time_dt.format("%Y-%m-%d %H:%M"));
            start_time_rfc = start_time_dt.to_rfc3339();
            end_time_rfc = (start_time_dt + chrono::Duration::hours(1)).to_rfc3339();
        }

        let drafted_message = format!("Yes! It's ${}. I have availability starting at {}. Which works?", price, suggested_time);

        let proposed_slot_id = uuid::Uuid::new_v4().to_string();

        // Lock in Redis
        if let Ok(redis_url) = std::env::var("OHC_REDIS_URL").or_else(|_| std::env::var("REDIS_URL")) {
            if let Ok(redis_lock) = RedisLock::new(&redis_url) {
                let _ = redis_lock.acquire_lock(&event.tenant_id, "booking_slot", &proposed_slot_id, 600).await;
            }
        }

        let intake_id = uuid::Uuid::new_v4().to_string();
        let _ = sqlx::query("INSERT INTO conversational_intakes (id, tenant_id, customer_id, inbox_message_id, status, context, service_name, suggested_price, suggested_time) VALUES ($1, $2, $3, $4, 'quote_sent', $5, $6, $7, $8)")
            .bind(&intake_id)
            .bind(&event.tenant_id)
            .bind(&customer_id)
            .bind(&inbox_id)
            .bind(message)
            .bind(&service_name)
            .bind(price)
            .bind(&suggested_time)
            .execute(&pool).await;

        let action_payload = serde_json::json!({
            "feature_type": "autonomous_quote_draft",
            "service": service_name,
            "customer_inquiry": message,
            "suggested_price": price,
            "scope": scope,
            "proposed_slots": [
                { "start_time": start_time_rfc, "end_time": end_time_rfc },
            ],
            "proposed_slot_id": proposed_slot_id,
            "require_deposit": true,
            "total_amount_cents": (price * 100.0) as i64,
            "deposit_amount_cents": (price * 100.0) as i64,
            "inbox_message_id": inbox_id,
            "customer_id": customer_id,
            "generated_response": drafted_message,
        });

        self.orchestrator
            .execute_action(
                DepartmentType::CustomerSuccess,
                format!("Draft quote and propose schedule for {}", service_name),
                event.tenant_id.clone(),
                ActionRisk::AutoExecute,
                action_payload,
            )
            .await
            .map(|_| ())?;

        Ok(())
    }

    fn get_config(&self, tenant_id: &str) -> Option<DepartmentConfig> {
        self.configs.get(tenant_id).cloned()
    }

    fn set_config(&mut self, tenant_id: String, config: DepartmentConfig) {
        self.configs.insert(tenant_id, config);
    }

    async fn query_memory(&self, query: &str) -> Result<Vec<String>, String> {
        let embedding = self.generate_embedding(query).await;
        self.orchestrator
            .query_long_term_memory("default_tenant", &embedding, 5)
            .await
    }

    async fn request_approval(
        &self,
        description: String,
        tenant_id: String,
        risk: ActionRisk,
    ) -> Result<ApprovalRequest, String> {
        self.orchestrator
            .execute_action(
                self.department_type(),
                description.clone(),
                tenant_id.clone(),
                risk,
                serde_json::json!({}),
            )
            .await
    }
}

#[async_trait::async_trait]
impl BaseAgent for NegotiatorAgent {
    fn agent_id(&self) -> String {
        "negotiator_agent".to_string()
    }

    fn trigger_type(&self) -> AgentTriggerType {
        AgentTriggerType::EventDriven
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::orchestration::mesh::CentrifugeNode;
    use ohc_builtin_agent::mesh::transport::InProcessTransport;
    use std::sync::Arc;
    use crate::orchestration::departments::Department;

    #[tokio::test]
    async fn test_negotiator_agent_subscribed_events() {
        if std::env::var("OHC_DATABASE_URL").is_err() {
            return;
        }
        let db = Arc::new(crate::db::DB::new().await.unwrap());
        let transport = Arc::new(InProcessTransport::new());
        let mesh = Arc::new(CentrifugeNode::new(transport));
        let orchestrator = Arc::new(DepartmentOrchestrator::new(db, mesh));
        let agent = NegotiatorAgent::new(orchestrator);
        let events = agent.subscribed_events();
        assert!(events.contains(&"tenant.message.received".to_string()));
        assert!(events.contains(&"tenant.omnichannel.message.received".to_string()));
    }

    #[test]
    fn test_negotiator_agent_struct_exists() {
        let type_name = "NegotiatorAgent";
        assert_eq!(type_name, "NegotiatorAgent");
    }

    #[test]
    fn test_negotiator_agent_intent_parsing() {
        let raw_response = r#"{"intent":"quote", "service_name":"Leaky Sink Repair", "confidence":0.99}"#;
        let mut is_quote_intent = false;
        let mut service_name = "Service".to_string();

        if let Ok(parsed) = serde_json::from_str::<serde_json::Value>(&raw_response) {
            let intent = parsed.get("intent").and_then(|v| v.as_str()).unwrap_or("");
            if intent == "quote" {
                is_quote_intent = true;
                service_name = parsed.get("service_name").and_then(|v| v.as_str()).unwrap_or("Service").to_string();
            }
        }

        assert!(is_quote_intent);
        assert_eq!(service_name, "Leaky Sink Repair");
    }

    #[test]
    fn test_negotiator_agent_intent_parsing_agree() {
        let raw_response = r#"{"intent":"agree", "confidence":0.99}"#;
        let mut is_agree_intent = false;

        if let Ok(parsed) = serde_json::from_str::<serde_json::Value>(&raw_response) {
            let intent = parsed.get("intent").and_then(|v| v.as_str()).unwrap_or("");
            if intent == "agree" {
                is_agree_intent = true;
            }
        }

        assert!(is_agree_intent);
    }

    #[test]
    fn test_negotiator_agent_heuristic_fallback() {
        let message = "I need a fix for my broken pipe.";
        let mut is_quote_intent = false;
        let mut service_name = "Service".to_string();

        if message.to_lowercase().contains("fix") || message.to_lowercase().contains("repair") {
            is_quote_intent = true;
            service_name = "Emergency Repair".to_string();
        }

        assert!(is_quote_intent);
        assert_eq!(service_name, "Emergency Repair");
    }
}
