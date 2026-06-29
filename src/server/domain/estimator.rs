use sqlx::PgPool;
use serde_json::Value;
use uuid::Uuid;

pub async fn handle_proposal_action(tenant_id: &str, payload: &Value, pool: &PgPool) -> Result<(), sqlx::Error> {
    if let Some(action) = payload.get("action").and_then(|v| v.as_str()) {
        if action == "approve" {
            if let Some(proposal_id) = payload.get("proposal_id").and_then(|v| v.as_str()) {
                tracing::info!("Approved quote draft: {}", proposal_id);
                sqlx::query("UPDATE quotes SET status = 'SENT', updated_at = NOW() WHERE id = $1 AND tenant_id = $2")
                    .bind(Uuid::parse_str(proposal_id).unwrap_or_default())
                    .bind(tenant_id)
                    .execute(pool)
                    .await?;
            }
        }
    }
    Ok(())
}


use ohc_builtin_agent::llm::LlmClient;
use std::sync::Arc;

fn build_estimator_llm_client() -> Option<Arc<dyn LlmClient>> {
    let key = std::env::var("GEMINI_API_KEY")
        .or_else(|_| std::env::var("OPENAI_API_KEY"))
        .ok()?;
    let model = std::env::var("OHC_LLM_MODEL").unwrap_or_else(|_| "gemini-pro".to_string());
    let endpoint = std::env::var("OHC_LLM_ENDPOINT").ok();

    let mut config = if let Some(endpoint) = endpoint {
        ohc_builtin_agent::llm::openai::OpenAIClientConfig::openai_compatible(key, endpoint, Some(model.clone()))
    } else {
        ohc_builtin_agent::llm::openai::OpenAIClientConfig::openai(key)
    };
    config.default_model = Some(model);
    Some(Arc::new(ohc_builtin_agent::llm::openai::OpenAIClient::from_config(config)))
}

pub async fn parse_inquiry_to_proposal(tenant_id: &str, customer_id: Uuid, inquiry_text: &str, pool: &PgPool) -> Result<Uuid, sqlx::Error> {
    // Parse inquiry to proposal using services as the ServiceCatalog

    let mut matched_service_name = "Custom Service Base Fee".to_string();
    let mut matched_price_cents: i64 = 15000;

    // Find all services for the tenant
    #[derive(sqlx::FromRow, serde::Serialize)]
    struct Service {
        title: String,
        price_cents: i64,
    }

    let services = sqlx::query_as::<_, Service>("SELECT title, price_cents FROM services WHERE tenant_id = $1")
        .bind(tenant_id)
        .fetch_all(pool)
        .await?;

    let catalog_json = serde_json::to_string(&services).unwrap_or_default();
    let llm_client = build_estimator_llm_client();

    if let Some(llm) = llm_client {
        let prompt = format!(
            "You are the Estimator Agent for a service business. You have the following service catalog:
{0}

Customer Inquiry: '{1}'

Task: Extract the scope of work and identify the closest matching service from the catalog based on the inquiry. Respond ONLY in valid JSON format: {{ \"matched_service_title\": \"string\", \"matched_price_cents\": 15000 }}",
            catalog_json, inquiry_text
        );


        let req = ohc_builtin_agent::types::ChatRequest {
            messages: vec![ohc_builtin_agent::types::Message::user(&prompt)],
            model: std::env::var("OHC_LLM_MODEL").unwrap_or_else(|_| "gemini-pro".to_string()),
            temperature: 0.0,
            max_tokens: 256,
            system: "You are an Estimator Agent. Parse scopes of work and match with service catalog. Output pure JSON.".to_string(),
            tools: vec![],
        };

        if let Ok(resp) = llm.chat(req).await {
            if let Ok(parsed) = serde_json::from_str::<serde_json::Value>(&resp.message.content) {
                if let Some(title) = parsed.get("matched_service_title").and_then(|t| t.as_str()) {
                    matched_service_name = title.to_string();
                }
                if let Some(price) = parsed.get("matched_price_cents").and_then(|p| p.as_i64()) {
                    matched_price_cents = price;
                }
            }
        }
    } else {
        // Fallback naive matching
        let inquiry_lower = inquiry_text.to_lowercase();
        for service in services {
            if inquiry_lower.contains(&service.title.to_lowercase()) {
                matched_service_name = service.title;
                matched_price_cents = service.price_cents;
                break;
            }
        }
    }

    let proposal_id = Uuid::new_v4();
    let required_deposit_cents: i64 = matched_price_cents / 3; // 33% deposit

    // Create Draft Quote
    sqlx::query(
        "INSERT INTO quotes (id, tenant_id, customer_id, status, total_amount_cents, required_deposit_cents, stripe_payment_link, created_at, updated_at) VALUES ($1, $2, $3, 'DRAFT', $4, $5, NULL, NOW(), NOW())"
    )
    .bind(proposal_id)
    .bind(tenant_id)
    .bind(customer_id)
    .bind(matched_price_cents)
    .bind(required_deposit_cents)
    .execute(pool)
    .await?;

    // Create sample line item
    sqlx::query(
        "INSERT INTO quote_line_items (id, quote_id, description, unit_price_cents, quantity, is_optional, created_at, updated_at, tenant_id) VALUES ($1, $2, $3, $4, $5, FALSE, NOW(), NOW(), $6)"
    )
    .bind(Uuid::new_v4())
    .bind(proposal_id)
    .bind(matched_service_name)
    .bind(matched_price_cents)
    .bind(1)
    .bind(tenant_id)
    .execute(pool)
    .await?;

    tracing::info!("Estimator Agent drafted proposal {} for tenant {}", proposal_id, tenant_id); // pii-safe

    Ok(proposal_id)
}
