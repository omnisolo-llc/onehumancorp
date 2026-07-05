use crate::orchestration::departments::orchestrator::{
    AgentTriggerType, BaseAgent, Department, DepartmentOrchestrator,
};
use crate::orchestration::departments::types::{
    ActionRisk, ApprovalRequest, DepartmentConfig, DepartmentEvent, DepartmentType,
};
use std::collections::HashMap;
use std::sync::Arc;

pub struct SalesAgent {
    orchestrator: Arc<DepartmentOrchestrator>,
    configs: HashMap<String, DepartmentConfig>,
    quote_intent_planner: Arc<dyn SalesQuoteIntentPlanner>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct QuoteIntent {
    pub original_message: String,
    pub service_name: String,
    pub preferred_start_time: Option<String>,
    pub preferred_end_time: Option<String>,
}

#[async_trait::async_trait]
pub trait SalesQuoteIntentPlanner: Send + Sync {
    async fn plan_quote_intent(
        &self,
        tenant_id: &str,
        payload: &serde_json::Value,
    ) -> Result<Option<QuoteIntent>, String>;
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SalesIntentBackend {
    Local,
    Minimax { api_key: String },
}

impl SalesIntentBackend {
    pub fn from_env() -> Self {
        match std::env::var("OHC_SALES_LLM_PROVIDER")
            .or_else(|_| std::env::var("OHC_LLM_PROVIDER"))
            .as_deref()
        {
            Ok("minimax") => {
                let api_key = std::env::var("MINIMAX_API_KEY").unwrap_or_default();
                if api_key.trim().is_empty() {
                    Self::Local
                } else {
                    Self::Minimax { api_key }
                }
            }
            _ => Self::Local,
        }
    }
}

struct RuntimeSalesQuoteIntentPlanner {
    backend: SalesIntentBackend,
}

impl RuntimeSalesQuoteIntentPlanner {
    fn from_env() -> Self {
        Self {
            backend: SalesIntentBackend::from_env(),
        }
    }
}

#[async_trait::async_trait]
impl SalesQuoteIntentPlanner for RuntimeSalesQuoteIntentPlanner {
    async fn plan_quote_intent(
        &self,
        tenant_id: &str,
        payload: &serde_json::Value,
    ) -> Result<Option<QuoteIntent>, String> {
        if let Some(intent) = extract_quote_intent(payload) {
            return Ok(Some(intent));
        }

        let original_message = payload
            .get("message")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .trim();
        if original_message.is_empty() {
            return Ok(None);
        }

        let payload_json = serde_json::to_string(payload).map_err(|e| e.to_string())?;
        let prompt = format!(
            "You are the OneHumanCorp sales intent planner. Decide whether an inbound tenant message is asking for a service quote. Return strict JSON only with keys intent, service_name, confidence, original_message, preferred_start_time, and preferred_end_time. intent must be quote or no_quote. confidence is 0.0 to 1.0. service_name must be the concrete service the customer wants only when intent is quote. preferred_start_time and preferred_end_time are optional ISO8601 strings if the customer mentioned specific times. Do not use keyword rules; infer the customer's request from context. Tenant: {tenant_id}. Payload: {payload_json}"
        );

        let mut attempts = 0;
        let max_attempts = 3;
        let mut backoff = std::time::Duration::from_millis(50);
        #[allow(unused_assignments)]
        let mut raw_opt = None;

        loop {
            let ai_op = async {
                match &self.backend {
                    SalesIntentBackend::Minimax { api_key } => {
                        crate::minimax::MinimaxClient::new(api_key.clone())
                            .reason(&crate::pricing::compression::reduce_tokens(&prompt))
                            .await
                    }
                    SalesIntentBackend::Local => crate::minimax::LocalLLMClient::new()
                        .reason(&crate::pricing::compression::reduce_tokens(&prompt))
                        .await,
                }
            };

            match tokio::time::timeout(std::time::Duration::from_secs(60), ai_op).await {
                Ok(Ok(res)) => {
                    raw_opt = Some(res);
                    break;
                },
                _ => {
                    attempts += 1;
                    if attempts == max_attempts {
                        return Err("AI service paused or unavailable".to_string());
                    }
                    tokio::time::sleep(backoff).await;
                    backoff *= 2;
                }
            }
        }
        let raw = raw_opt.ok_or_else(|| "AI service paused or unavailable".to_string())?;

        parse_quote_intent_plan(&raw, original_message)
    }
}

impl SalesAgent {
    async fn generate_embedding(&self, text: &str) -> Vec<f32> {
        let mut attempts = 0;
        let max_attempts = 3;
        let mut backoff = std::time::Duration::from_millis(50);
        #[allow(unused_assignments)]
        let mut vec_opt = None;

        loop {
            let ai_op = async {
                match std::env::var("OHC_SALES_LLM_PROVIDER")
                    .or_else(|_| std::env::var("OHC_LLM_PROVIDER"))
                    .as_deref()
                {
                    Ok("minimax") => {
                        let api_key = std::env::var("MINIMAX_API_KEY").unwrap_or_default();
                        crate::minimax::MinimaxClient::new(api_key).generate_embedding(text).await
                    }
                    _ => {
                        crate::minimax::LocalLLMClient::new().generate_embedding(text).await
                    }
                }
            };

            match tokio::time::timeout(std::time::Duration::from_secs(60), ai_op).await {
                Ok(Ok(res)) => {
                    vec_opt = Some(res);
                    break;
                },
                _ => {
                    attempts += 1;
                    if attempts == max_attempts {
                        return vec![0.0; 1536];
                    }
                    tokio::time::sleep(backoff).await;
                    backoff *= 2;
                }
            }
        }
        vec_opt.unwrap_or_else(|| vec![0.0; 1536])
    }
    pub fn new(orchestrator: Arc<DepartmentOrchestrator>) -> Self {
        Self::with_quote_intent_planner(
            orchestrator,
            Arc::new(RuntimeSalesQuoteIntentPlanner::from_env()),
        )
    }

    pub fn with_quote_intent_planner(
        orchestrator: Arc<DepartmentOrchestrator>,
        quote_intent_planner: Arc<dyn SalesQuoteIntentPlanner>,
    ) -> Self {
        Self {
            orchestrator,
            configs: HashMap::new(),
            quote_intent_planner,
        }
    }
}

pub fn extract_quote_intent(payload: &serde_json::Value) -> Option<QuoteIntent> {
    let original_message = payload
        .get("message")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .trim();

    if let Some(llm_intent) = payload.get("llm_intent") {
        let intent = llm_intent.get("intent").and_then(|v| v.as_str()).unwrap_or("");
        let confidence = llm_intent.get("confidence").and_then(|v| v.as_f64()).unwrap_or(0.0);
        if intent.eq_ignore_ascii_case("quote") && confidence >= 0.7 {
            let service_name = llm_intent
                .get("service_name")
                .and_then(|v| v.as_str())
                .map(str::trim)
                .filter(|name| !name.is_empty())?;
            let preferred_start_time = llm_intent
                .get("preferred_start_time")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string());
            let preferred_end_time = llm_intent
                .get("preferred_end_time")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string());
            return Some(QuoteIntent {
                original_message: original_message.to_string(),
                service_name: service_name.to_string(),
                preferred_start_time,
                preferred_end_time,
            });
        }
    }

    None
}

pub fn parse_quote_intent_plan(
    raw: &str,
    fallback_original_message: &str,
) -> Result<Option<QuoteIntent>, String> {
    let value: serde_json::Value = match serde_json::from_str(raw) {
        Ok(value) => value,
        Err(_) => {
            let start = raw
                .find('{')
                .ok_or_else(|| "sales intent response missing JSON object".to_string())?;
            let end = raw
                .rfind('}')
                .ok_or_else(|| "sales intent response missing JSON object".to_string())?;
            serde_json::from_str(&raw[start..=end])
                .map_err(|e| format!("failed to parse sales intent JSON: {e}"))?
        }
    };

    let intent = value.get("intent").and_then(|v| v.as_str()).unwrap_or("");
    let confidence = value.get("confidence").and_then(|v| v.as_f64()).unwrap_or(0.0);
    if !intent.eq_ignore_ascii_case("quote") || confidence < 0.7 {
        return Ok(None);
    }

    let service_name = value
        .get("service_name")
        .and_then(|v| v.as_str())
        .map(str::trim)
        .filter(|name| !name.is_empty())
        .ok_or_else(|| "sales quote intent missing service_name".to_string())?;

    let original_message = value
        .get("original_message")
        .and_then(|v| v.as_str())
        .map(str::trim)
        .filter(|message| !message.is_empty())
        .unwrap_or(fallback_original_message);

    let preferred_start_time = value
        .get("preferred_start_time")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string());
    let preferred_end_time = value
        .get("preferred_end_time")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string());

    Ok(Some(QuoteIntent {
        original_message: original_message.to_string(),
        service_name: service_name.to_string(),
        preferred_start_time,
        preferred_end_time,
    }))
}

pub fn risk_for_service_price(config: Option<&DepartmentConfig>, price: f64) -> ActionRisk {
    let Some(config) = config else {
        return ActionRisk::DraftForReview;
    };

    if config.auto_approve_limits > 0.0 && price <= config.auto_approve_limits {
        ActionRisk::AutoExecute
    } else {
        ActionRisk::DraftForReview
    }
}

#[async_trait::async_trait]
impl Department for SalesAgent {
    fn department_type(&self) -> DepartmentType {
        DepartmentType::Sales
    }

    fn subscribed_events(&self) -> Vec<String> {
        vec![
            "tenant.quote.requested".to_string(),
            "tenant.message.received".to_string(),
            "tenant.omnichannel.message.received".to_string(),
            "tenant.work_intake.received".to_string(),
            "agent:sales:approved".to_string(),
            "pos_sales".to_string(),
            "tenant.order.created".to_string(),
        ]
    }

    async fn handle_event(&self, event: &DepartmentEvent) -> Result<(), String> {
        if event.event_type == "tenant.order.created" {
            let customer_id = event.payload.get("customer_id").and_then(|v| v.as_str()).unwrap_or("");
            let order_id = event.payload.get("order_id").and_then(|v| v.as_str()).unwrap_or("");
            tracing::info!("Sales Agent received order created for customer: {}, order: {}", customer_id, order_id);

            // Promoter Logic: Draft an upsell/subscription reply.
            let proposed_action = serde_json::json!({
                "action_type": "Draft Reply",
                "customer_id": customer_id,
                "draft_reply": format!("Hi! Thanks for your order {}. Would you like to set up a subscription for 10% off your next deliveries? Reply YES to subscribe.", order_id),
                "order_id": order_id
            });

            self.orchestrator.execute_action(
                DepartmentType::Sales,
                "Promote Subscription".to_string(),
                event.tenant_id.clone(),
                ActionRisk::DraftForReview,
                proposed_action,
            ).await.map_err(|e| e.to_string())?;

            return Ok(());
        }

        if event.event_type == "POS_SALE_COMPLETED" {
            let amount = event.payload.get("amount").and_then(|v| v.as_f64()).unwrap_or(0.0);
            let customer_id = event.payload.get("customer_id").and_then(|v| v.as_str()).unwrap_or("Unknown");
            tracing::info!("Sales Agent: Recorded POS sale of ${} for customer {}", amount, customer_id);
            return Ok(());
        }

        if event.event_type == "agent:sales:approved" {
            if let Some(payload) = event.payload.get("original_payload") {
                if let Some(feature_type) = payload.get("feature_type").and_then(|v| v.as_str()) {
                    if feature_type == "quote_draft" {
                        let suggested_price = payload.get("suggested_price").and_then(|v| v.as_f64()).unwrap_or(0.0);
                        let customer_inquiry = payload.get("customer_inquiry").and_then(|v| v.as_str()).unwrap_or("Unknown");
                        let deposit_amount = suggested_price * 0.20; // 20% deposit

                        tracing::info!(
                            "Executing approved quote draft for inquiry: '{}'. Suggested price: ${}. Deposit: ${}",
                            customer_inquiry,
                            suggested_price,
                            deposit_amount
                        );

                        // Simulate creating a Stripe checkout session for the deposit
                        let stripe_client = crate::integrations::stripe::client::StripeClient::new(
                            std::env::var("STRIPE_API_KEY").unwrap_or_else(|_| "sk_test_123".to_string()),
                        );

                        match stripe_client.create_checkout_session("price_dummy", "cus_dummy", deposit_amount, None, None).await {
                            Ok(url) => {
                                tracing::info!("Generated deposit link: {}", url);
                                // Update proposals table to persist Stripe URL
                                if let Some(eid) = payload.get("estimate_id").and_then(|v| v.as_str()) {
                                    let db = crate::db::get_pool();
                                    let _ = sqlx::query("UPDATE estimates SET status = 'sent', updated_at = NOW() WHERE id = $1")
                                        .bind(eid)
                                        .execute(&db)
                                        .await;
                                }
                            }
                            Err(e) => {
                                tracing::error!("Failed to create checkout session for quote deposit: {}", e);
                            }
                        }

                        if let Some(lid) = payload.get("service_lead_id").and_then(|v| v.as_str()) {
                            let db = crate::db::get_pool();
                            let _ = sqlx::query("UPDATE service_leads SET status = 'estimated', updated_at = NOW() WHERE id = $1")
                                .bind(lid)
                                .execute(&db)
                                .await;
                        }

                        // NOTE: booking_slot is NOT confirmed here. It is kept soft-locked until payment webhook.
                    }
                }
            }
            return Ok(());
        }
        if event.event_type == "tenant.omnichannel.message.received" || event.event_type == "tenant.work_intake.received" {
            let source = event.payload.get("source").and_then(|v| v.as_str()).unwrap_or("");
            if source == "booking_form" || source == "sms" {
                let preferred_time = event.payload.get("timestamp").and_then(|v| v.as_str()).unwrap_or("");
                let message = event.payload.get("message").and_then(|v| v.as_str()).unwrap_or("");
                let service_name = "Field Service Repair";
                let price = 150.0;

                let sched_event = DepartmentEvent {
                    id: uuid::Uuid::new_v4().to_string(),
                    tenant_id: event.tenant_id.clone(),
                    event_type: "tenant.quote.requires_scheduling".to_string(),
                    payload: serde_json::json!({
                        "source": source,
                        "message": message,
                        "service_name": service_name,
                        "price": price,
                        "preferred_time": preferred_time,
                        "customer_id": event.payload.get("customer_id").and_then(|v| v.as_str()).unwrap_or(""),
                    }),
                };
                let _ = self.orchestrator.dispatch_event(sched_event).await;
            }
        }


                if event.event_type == "tenant.message.received" || event.event_type == "tenant.omnichannel.message.received" || event.event_type == "tenant.work_intake.received" {
            let planned_intent = match self
                .quote_intent_planner
                .plan_quote_intent(&event.tenant_id, &event.payload)
                .await
            {
                Ok(intent) => intent,
                Err(err) => {
                    ::server_telemetry::record_error_signal("[bug] SalesAgent LLM planning failed");
                    tracing::error!("SalesAgent LLM planning failed: {}", err);
                    None
                }
            };

            if let Some(intent) = planned_intent {
                let query_embedding = self.generate_embedding(&intent.original_message).await;

                let context_records = self
                    .orchestrator
                    .query_long_term_memory(&event.tenant_id, &query_embedding, 3)
                    .await.unwrap_or_default();

                let service = self
                    .orchestrator
                    .get_service_by_name_like(&event.tenant_id, &intent.service_name)
                    .await?;

                let (service_name, mut price, service_id) = service.unwrap_or((intent.service_name, 75.0, uuid::Uuid::new_v4().to_string()));

                let mut context_summary = String::new();
                for r in context_records {
                    context_summary.push_str(&r);
                    context_summary.push_str(" ");
                }

                let prompt = format!(
                    "You are a sales agent drafting a project proposal for a new inquiry. Inquiry: {}. Service: {}. Base Price: {}. Past context: {}. Output strict JSON with keys: scope, suggested_time, suggested_price, drafted_message. Do not use markdown.",
                    intent.original_message, service_name, price, context_summary
                );

                let raw_response = match std::env::var("OHC_SALES_LLM_PROVIDER")
                    .or_else(|_| std::env::var("OHC_LLM_PROVIDER"))
                    .as_deref()
                {
                    Ok("minimax") => {
                        let api_key = std::env::var("MINIMAX_API_KEY").unwrap_or_default();
                        crate::minimax::MinimaxClient::new(api_key).reason(&crate::pricing::compression::reduce_tokens(&prompt)).await.unwrap_or_default()
                    }
                    _ => {
                        crate::minimax::LocalLLMClient::new().reason(&crate::pricing::compression::reduce_tokens(&prompt)).await.unwrap_or_default()
                    }
                };

                let mut scope = format!("{} including labor and standard materials.", service_name);
                let mut suggested_time = "Tomorrow at 2 PM".to_string();
                let mut drafted_message = format!("Based on our past projects, I can offer {} starting at ${:.2}. Should I send over the formal agreement?", service_name, price);

                if let Ok(parsed) = serde_json::from_str::<serde_json::Value>(&raw_response) {
                    if let Some(s) = parsed.get("scope").and_then(|v| v.as_str()) {
                        scope = s.to_string();
                    }
                    if let Some(t) = parsed.get("suggested_time").and_then(|v| v.as_str()) {
                        suggested_time = t.to_string();
                    }
                    if let Some(p) = parsed.get("suggested_price").and_then(|v| v.as_f64()) {
                        price = p;
                    }
                    if let Some(m) = parsed.get("drafted_message").and_then(|v| v.as_str()) {
                        drafted_message = m.to_string();
                    }
                }

                let risk =
                    risk_for_service_price(self.get_config(&event.tenant_id).as_ref(), price);

                ::server_telemetry::record_business_event(
                    &event.tenant_id,
                    ::server_telemetry::get_deployment_mode(),
                    "quote_generated_from_message",
                );

                let mut proposed_slot_id = None;
                if !suggested_time.is_empty() {
                    proposed_slot_id = self.orchestrator.soft_lock_booking_slot(
                        &event.tenant_id,
                        &service_name,
                        &suggested_time,
                        std::time::Duration::from_secs(600) // 10 mins
                    ).await.ok().flatten();
                }

                // Create estimates (quotes) and deposit requirements
                let estimate_id = uuid::Uuid::new_v4().to_string();
                let deposit_requirement_id = uuid::Uuid::new_v4().to_string();
                let service_lead_id = event.payload.get("service_lead_id").and_then(|v| v.as_str()).unwrap_or("");
                let customer_id_val = event.payload.get("customer_id").and_then(|v| v.as_str()).unwrap_or("");

                let db = crate::db::get_pool();
                let deposit_amount_cents = (price * 0.20 * 100.0) as i64;
                let price_cents = (price * 100.0) as i64;

                if !service_lead_id.is_empty() {
                    let _ = sqlx::query("INSERT INTO estimates (id, tenant_id, service_lead_id, customer_id, description, min_price_cents, max_price_cents, status) VALUES ($1, $2, $3, $4, $5, $6, $7, 'draft')")
                        .bind(&estimate_id)
                        .bind(&event.tenant_id)
                        .bind(service_lead_id)
                        .bind(uuid::Uuid::parse_str(customer_id_val).ok())
                        .bind(&scope)
                        .bind(price_cents)
                        .bind(price_cents)
                        .execute(&db)
                        .await;

                    let _ = sqlx::query("INSERT INTO deposit_requirements (id, tenant_id, estimate_id, amount_cents, percentage, status) VALUES ($1, $2, $3, $4, 20.00, 'pending')")
                        .bind(&deposit_requirement_id)
                        .bind(&event.tenant_id)
                        .bind(&estimate_id)
                        .bind(deposit_amount_cents)
                        .execute(&db)
                        .await;
                }

                let action_payload = serde_json::json!({
                    "inbox_message_id": event.payload.get("inbox_message_id").and_then(|v| v.as_str()).unwrap_or(""),
                    "customer_id": customer_id_val,
                    "service_lead_id": service_lead_id,
                    "estimate_id": estimate_id,
                    "deposit_requirement_id": deposit_requirement_id,
                    "feature_type": "quote_draft",
                    "customer_inquiry": intent.original_message,
                    "suggested_price": price,
                    "scope": scope,
                    "suggested_time": suggested_time,
                    "proposed_slot_id": proposed_slot_id,
                    "generated_response": drafted_message,
                    "service": service_name.clone(),
                    "service_id": service_id.clone(),
                    "price": price,
                    "preferred_start_time": intent.preferred_start_time,
                    "preferred_end_time": intent.preferred_end_time,
                });

                self.orchestrator
                    .execute_action(
                        DepartmentType::Sales,
                        format!("Draft quote for {}", service_name),
                        event.tenant_id.clone(),
                        risk,
                        action_payload,
                    )
                    .await
                    .map(|_| ())?;
                return Ok(());
            }

            return Ok(());
        }

        // Query memory context
        let message = event.payload.get("message").and_then(|v| v.as_str()).unwrap_or("");
        let query_embedding = self.generate_embedding(message).await;

        let _context = self
            .orchestrator
            .query_long_term_memory(&event.tenant_id, &query_embedding, 5)
            .await?;

        ::server_telemetry::record_business_event(
            &event.tenant_id,
            ::server_telemetry::get_deployment_mode(),
            "quote_generated",
        );

        self.orchestrator
            .execute_action(
                DepartmentType::Sales,
                "Quote generated for review".to_string(),
                event.tenant_id.clone(),
                ActionRisk::DraftForReview,
                event.payload.clone(),
            )
            .await
            .map(|_| ())
    }

    fn get_config(&self, _tenant_id: &str) -> Option<DepartmentConfig> {
        self.configs.get(_tenant_id).cloned()
    }

    fn set_config(&mut self, _tenant_id: String, _config: DepartmentConfig) {
        self.configs.insert(_tenant_id, _config);
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
impl BaseAgent for SalesAgent {
    fn agent_id(&self) -> String {
        "sales_agent".to_string()
    }

    fn trigger_type(&self) -> AgentTriggerType {
        AgentTriggerType::EventDriven
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::orchestration::departments::types::DepartmentConfig;

    struct ScriptedSalesQuoteIntentPlanner {
        intent: Option<QuoteIntent>,
    }

    #[async_trait::async_trait]
    impl SalesQuoteIntentPlanner for ScriptedSalesQuoteIntentPlanner {
        async fn plan_quote_intent(
            &self,
            _tenant_id: &str,
            _payload: &serde_json::Value,
        ) -> Result<Option<QuoteIntent>, String> {
            Ok(self.intent.clone())
        }
    }

    #[test]
    fn llm_quote_intent_detects_service_without_keyword_fallback() {
        let payload = serde_json::json!({
            "message": "Can someone come by around 2 PM?",
            "llm_intent": {
                "intent": "quote",
                "service_name": "Plumbing Fix",
                "confidence": 0.91
            }
        });

        let intent = extract_quote_intent(&payload).expect("LLM intent should request a quote");

        assert_eq!(intent.service_name, "Plumbing Fix");
        assert_eq!(intent.original_message, "Can someone come by around 2 PM?");
    }

    #[test]
    fn keyword_only_message_does_not_create_quote_intent() {
        let payload = serde_json::json!({
            "message": "Can you fix it tomorrow?"
        });

        assert_eq!(extract_quote_intent(&payload), None);
    }

    #[test]
    fn parses_quote_intent_plan_from_llm_json() {
        let intent = parse_quote_intent_plan(
            r#"{"intent":"quote","service_name":"Emergency Plumbing","confidence":0.91,"original_message":"Water is coming through the ceiling"}"#,
            "fallback message",
        )
        .expect("LLM JSON should parse")
        .expect("high confidence quote intent should be returned");

        assert_eq!(intent.service_name, "Emergency Plumbing");
        assert_eq!(intent.original_message, "Water is coming through the ceiling");
    }

    #[test]
    fn low_confidence_llm_plan_does_not_create_quote_intent() {
        let intent = parse_quote_intent_plan(
            r#"{"intent":"quote","service_name":"Emergency Plumbing","confidence":0.42,"original_message":"Maybe later"}"#,
            "Maybe later",
        )
        .expect("LLM JSON should parse");

        assert_eq!(intent, None);
    }

    #[tokio::test]
    async fn deterministic_quote_intent_planner_is_injectable() {
        let planner: Arc<dyn SalesQuoteIntentPlanner> = Arc::new(ScriptedSalesQuoteIntentPlanner {
            intent: Some(QuoteIntent {
                original_message: "The drain backed up after closing".to_string(),
                service_name: "Drain Cleaning".to_string(),
                preferred_start_time: None,
                preferred_end_time: None,
            }),
        });

        let intent = planner
            .plan_quote_intent(
                "tenant-sales",
                &serde_json::json!({"message": "The drain backed up after closing"}),
            )
            .await
            .expect("scripted planner should not fail")
            .expect("scripted planner should return quote intent");

        assert_eq!(intent.service_name, "Drain Cleaning");
        assert_eq!(intent.original_message, "The drain backed up after closing");
    }

    #[test]
    fn dynamic_service_price_controls_action_risk() {
        let config = DepartmentConfig {
            tone_of_voice: "friendly".to_string(),
            auto_approve_limits: 100.0,
        };

        assert_eq!(
            risk_for_service_price(Some(&config), 75.0),
            ActionRisk::AutoExecute
        );
        assert_eq!(
            risk_for_service_price(Some(&config), 175.0),
            ActionRisk::DraftForReview
        );
        assert_eq!(
            risk_for_service_price(None, 1.0),
            ActionRisk::DraftForReview
        );
    }
}
