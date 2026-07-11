use axum::{
    extract::{State, Json},
    response::IntoResponse,
    http::StatusCode,
    routing::post,
    Router,
};
use std::sync::Arc;
use serde::{Deserialize, Serialize};
use crate::orchestration::departments::orchestrator::DepartmentOrchestrator;
use crate::orchestration::departments::types::{DepartmentType, ActionRisk};
use uuid::Uuid;
use crate::db::get_pool;
use crate::minimax::{LocalLLMClient, MinimaxClient};

#[derive(Deserialize)]
pub struct WebhookPayload {
    pub tenant_id: String,
    pub message: String,
    pub source: String,
    #[serde(default)]
    pub target_language: Option<String>,
    pub customer_name: Option<String>,
    pub customer_email: Option<String>,
}

#[derive(Serialize)]
pub struct WebhookResponse {
    pub success: bool,
    pub request_id: Option<String>,
}

pub fn router<S>(orchestrator: Arc<DepartmentOrchestrator>) -> Router<S>
where
    S: Clone + Send + Sync + 'static,
{
    Router::new()
        .route("/", post(handle_webhook))
        .with_state(orchestrator)
}

async fn analyze_intake_inquiry(inquiry: &str) -> Result<(f64, String, String), String> {
    let prompt = format!(
        "Analyze the following client intake inquiry and extract the key project parameters. Provide a suggested price (as a number), a short service name (e.g., 'Logo Refresh'), and a brief scope description.\n\nInquiry: {}\n\nRespond with a JSON object containing keys: 'suggested_price' (number), 'service_name' (string), and 'scope' (string).",
        inquiry
    );

    let raw_response = match std::env::var("OHC_SALES_LLM_PROVIDER").or_else(|_| std::env::var("OHC_LLM_PROVIDER")).as_deref() {
        Ok("minimax") => {
            let api_key = std::env::var("MINIMAX_API_KEY").unwrap_or_default();
            if api_key.trim().is_empty() {
                LocalLLMClient::new().reason(&prompt).await
            } else {
                MinimaxClient::new(api_key).reason(&prompt).await
            }
        },
        _ => LocalLLMClient::new().reason(&prompt).await,
    }?;

    let parsed: serde_json::Value = serde_json::from_str(&raw_response).unwrap_or(serde_json::json!({}));

    let price = parsed.get("suggested_price").and_then(|v| v.as_f64()).unwrap_or(1500.0);
    let name = parsed.get("service_name").and_then(|v| v.as_str()).unwrap_or("Custom Project Scope").to_string();
    let scope = parsed.get("scope").and_then(|v| v.as_str()).unwrap_or("Custom requirements based on inquiry.").to_string();

    Ok((price, name, scope))
}

async fn handle_webhook(
    State(orchestrator): State<Arc<DepartmentOrchestrator>>,
    Json(payload): Json<WebhookPayload>,
) -> impl IntoResponse {
    // For incoming Stripe webhooks for new orders, route to Operations to process the order
    if payload.source == "stripe" && payload.message == "order_placed" {
        // Trigger SMS notification for new orders
        tokio::spawn(async move {
            let _ = crate::dispatch_critical_sms("new_order", "You have received a new order!").await;
        });

        let event = crate::orchestration::departments::types::DepartmentEvent {
            id: uuid::Uuid::new_v4().to_string(),
            tenant_id: payload.tenant_id.clone(),
            event_type: "tenant.order.created".to_string(),
            payload: serde_json::json!({"source": payload.source, "message": payload.message}),
        };

        match orchestrator.dispatch_event(event).await {
            Ok(_) => return (StatusCode::OK, Json(WebhookResponse { success: true, request_id: None })).into_response(),
            Err(e) => {
                if e.contains("AI Budget exhausted") {
                    return (StatusCode::TOO_MANY_REQUESTS, Json(WebhookResponse { success: false, request_id: None })).into_response();
                } else {
                    return (StatusCode::INTERNAL_SERVER_ERROR, Json(WebhookResponse { success: false, request_id: None })).into_response();
                }
            }
        }
    }

    if payload.source == "mercadopago" {
        if payload.message == "approved" {
            tokio::spawn(async move {
                let _ = crate::dispatch_critical_sms("new_order", "You have received a new order!").await;
            });

            let event = crate::orchestration::departments::types::DepartmentEvent {
                id: uuid::Uuid::new_v4().to_string(),
                tenant_id: payload.tenant_id.clone(),
                event_type: "tenant.order.created".to_string(),
                payload: serde_json::json!({"source": payload.source, "message": payload.message}),
            };
            let _ = orchestrator.dispatch_event(event).await;
        }
        return (StatusCode::OK, Json(WebhookResponse { success: true, request_id: None })).into_response();
    }

    // Client intake flow via email or form webhook
    if payload.source == "intake_form" || payload.source == "email_inquiry" || payload.source == "work_intake" {
        let tenant_id = payload.tenant_id.clone();
        let inquiry = payload.message.clone();
        let pool = crate::db::get_pool();

        let customer_id = uuid::Uuid::new_v4().to_string();
        if let Some(name) = &payload.customer_name {
            let _ = sqlx::query(
                "INSERT INTO customers (id, tenant_id, name, email, created_at, updated_at) VALUES ($1, $2, $3, $4, NOW(), NOW()) ON CONFLICT DO NOTHING"
            )
            .bind(uuid::Uuid::parse_str(&customer_id).unwrap_or_default())
            .bind(&tenant_id)
            .bind(name)
            .bind(&payload.customer_email)
            .execute(&pool)
            .await;
        }

        let service_lead_id = uuid::Uuid::new_v4().to_string();
        let _ = sqlx::query("INSERT INTO service_leads (id, tenant_id, customer_id, description, source, status, created_at, updated_at) VALUES ($1, $2, $3, $4, $5, 'new', NOW(), NOW())")
            .bind(&service_lead_id)
            .bind(&tenant_id)
            .bind(uuid::Uuid::parse_str(&customer_id).unwrap_or_default())
            .bind(&inquiry)
            .bind(&payload.source)
            .execute(&pool)
            .await;

        let (suggested_price, service_name, scope) = match analyze_intake_inquiry(&inquiry).await {
            Ok(res) => res,
            Err(_) => (1500.00, "Custom Project Scope".to_string(), "Custom requirements based on inquiry.".to_string()),
        };

        // Generate embedding for memory query
        let prompt_for_embedding = format!("Past proposals for {}", service_name);
        let query_embedding = match std::env::var("OHC_SALES_LLM_PROVIDER").or_else(|_| std::env::var("OHC_LLM_PROVIDER")).as_deref() {
            Ok("minimax") => {
                let api_key = std::env::var("MINIMAX_API_KEY").unwrap_or_default();
                if api_key.trim().is_empty() {
                    LocalLLMClient::new().generate_embedding(&prompt_for_embedding).await
                } else {
                    MinimaxClient::new(api_key).generate_embedding(&prompt_for_embedding).await
                }
            },
            _ => LocalLLMClient::new().generate_embedding(&prompt_for_embedding).await,
        }.unwrap_or_else(|_| vec![0.0; 1536]);

        let context = match orchestrator.query_long_term_memory(&tenant_id, &query_embedding, 5).await {
            Ok(c) => c,
            Err(_) => vec![],
        };

        let drafted_message = format!(
            "Hi there! Based on your request for '{}', I've put together a drafted proposal. The estimated scope will cost around ${}, including standard services.",
            inquiry, suggested_price
        );

        // Customer record created above

        let quote_id = uuid::Uuid::new_v4().to_string();
        let quote_line_item_id = uuid::Uuid::new_v4().to_string();

        let price_cents = (suggested_price * 100.0) as i64;

        let _ = sqlx::query(
            "INSERT INTO quotes (id, tenant_id, customer_id, status, created_at, updated_at) VALUES ($1, $2, $3, 'DRAFT', NOW(), NOW())"
        )
        .bind(uuid::Uuid::parse_str(&quote_id).unwrap_or_default())
        .bind(&tenant_id)
        .bind(uuid::Uuid::parse_str(&customer_id).unwrap_or_default())
        .execute(&pool)
        .await;

        let _ = sqlx::query(
            "INSERT INTO quote_line_items (id, quote_id, description, unit_price_cents, quantity, is_optional, created_at, updated_at, tenant_id) VALUES ($1, $2, $3, $4, 1, false, NOW(), NOW(), $5)"
        )
        .bind(uuid::Uuid::parse_str(&quote_line_item_id).unwrap_or_default())
        .bind(uuid::Uuid::parse_str(&quote_id).unwrap_or_default())
        .bind(&scope)
        .bind(price_cents)
        .bind(tenant_id.clone())
        .execute(&pool)
        .await;

        let action_payload = serde_json::json!({
            "feature_type": "quote_draft",
            "service_lead_id": service_lead_id,
            "customer_inquiry": inquiry,
            "suggested_price": suggested_price,
            "scope": scope,
            "suggested_time": "Next Week",
            "generated_response": drafted_message,
            "service": service_name,
            "price": suggested_price,
            "context": context,
            "quote_id": quote_id,
            "customer_name": payload.customer_name.unwrap_or_else(|| "Unknown".to_string()),
        });

        match orchestrator.execute_action(
            DepartmentType::Sales,
            format!("Action Required: Approve Estimate for {}", service_name),
            tenant_id,
            ActionRisk::DraftForReview,
            action_payload,
        ).await {
            Ok(_) => return (StatusCode::OK, Json(WebhookResponse { success: true, request_id: None })).into_response(),
            Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, Json(WebhookResponse { success: false, request_id: None })).into_response(),
        }
    }

    let pool = get_pool();
    let id = Uuid::new_v4().to_string();
    // Insert the raw message initially; the TranslationAgent will update it with translated content and a draft reply.
    let _ = sqlx::query(
        r#"
        INSERT INTO inbox_messages (id, tenant_id, source, original_content, content, translated_from_language, draft_reply, status, created_at)
        VALUES ($1, $2, $3, $4, $5, $6, $7, 'unread', NOW())
        "#
    )
    .bind(&id)
    .bind(&payload.tenant_id)
    .bind(&payload.source)
    .bind(&payload.message)
    .bind(&payload.message) // content starts as original until translated
    .bind(None::<String>) // translated_from_language starts empty
    .bind(None::<String>) // draft_reply starts empty
    .execute(&pool)
    .await;

    let target_language = payload.target_language.unwrap_or_else(|| "English".to_string());

    let event = crate::orchestration::departments::types::DepartmentEvent {
        id: uuid::Uuid::new_v4().to_string(),
        tenant_id: payload.tenant_id.clone(),
        event_type: "tenant.omnichannel.message.received".to_string(),
        payload: serde_json::json!({
            "source": payload.source,
            "original_message": payload.message,
            "target_language": target_language,
            "inbox_message_id": id,
        }),
    };

    match orchestrator.dispatch_event(event).await {
        Ok(_) => (StatusCode::OK, Json(WebhookResponse { success: true, request_id: Some(id) })).into_response(),
        Err(e) => {
            if e.contains("AI Budget exhausted") {
                (StatusCode::TOO_MANY_REQUESTS, Json(WebhookResponse { success: false, request_id: None })).into_response()
            } else {
                (StatusCode::INTERNAL_SERVER_ERROR, Json(WebhookResponse { success: false, request_id: None })).into_response()
            }
        }
    }
}

