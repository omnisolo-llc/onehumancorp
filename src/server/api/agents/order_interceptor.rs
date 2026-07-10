use axum::{
    extract::{Extension, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::{Deserialize, Serialize};
use ::server_common::Claims;
use sqlx::{PgPool, FromRow};


#[derive(Debug, Deserialize)]
pub struct InterceptOrderRequest {
    pub raw_input: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InterceptedOrder {
    pub intent: String,
    pub items: Vec<InterceptedOrderItem>,
    pub language: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InterceptedOrderItem {
    pub item: String,
    pub quantity: i32,
}

#[derive(FromRow)]
struct TenantLanguage {
    language_preference: String,
}

pub async fn intercept_order_handler(
    State(pool): State<PgPool>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<InterceptOrderRequest>,
) -> impl IntoResponse {
    let tenant_id = match claims.organization_id.as_deref() {
        Some(org_id) => org_id,
        None => return (StatusCode::UNAUTHORIZED, "Unauthorized").into_response(),
    };

    // Get tenant language preference
    let language_preference = match get_tenant_language(&pool, tenant_id).await {
        Ok(lang) => lang,
        Err(_) => "en".to_string(), // Default fallback
    };

    match intercept_order(tenant_id, &language_preference, &payload.raw_input).await {
        Ok(order) => (StatusCode::OK, Json(order)).into_response(),
        Err(e) => {
            tracing::error!("Failed to intercept order: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, e).into_response()
        }
    }
}

async fn get_tenant_language(pool: &PgPool, tenant_id: &str) -> Result<String, sqlx::Error> {
    let row = sqlx::query_as::<_, TenantLanguage>("SELECT language_preference FROM tenants WHERE id = $1")
        .bind(tenant_id)
        .fetch_one(pool)
        .await?;
    Ok(row.language_preference)
}

pub async fn intercept_order(
    tenant_id: &str,
    tenant_language: &str,
    raw_input: &str,
) -> Result<InterceptedOrder, String> {
    let prompt = format!(
        "Return strict JSON representing an order extracted from the following multilingual input. \
        The output language for item names must be translated to {tenant_language}. \
        The JSON must have this structure: {{ \"intent\": \"Order\", \"items\": [{{ \"item\": \"item name\", \"quantity\": 1 }}], \"language\": \"detected source language\" }} \
        Tenant: {tenant_id}. Input: {raw_input}"
    );

    let raw = match std::env::var("OHC_LLM_PROVIDER").as_deref() {
        Ok("gemini") => crate::minimax::LocalLLMClient::new().reason(&prompt).await,
        Ok("minimax") => {
            let api_key = std::env::var("MINIMAX_API_KEY")
                .map_err(|_| "MINIMAX_API_KEY is required".to_string())?;
            crate::minimax::MinimaxClient::new(api_key).reason(&prompt).await
        }
        _ => crate::minimax::LocalLLMClient::new().reason(&prompt).await,
    }?;

    parse_intercepted_order(&raw)
}

fn parse_intercepted_order(raw: &str) -> Result<InterceptedOrder, String> {
    let value: serde_json::Value = match serde_json::from_str(raw) {
        Ok(value) => value,
        Err(_) => {
            let start = raw.find('{').ok_or_else(|| "missing JSON object".to_string())?;
            let end = raw.rfind('}').ok_or_else(|| "missing JSON object".to_string())?;
            serde_json::from_str(&raw[start..=end])
                .map_err(|e| format!("failed to parse JSON: {e}"))?
        }
    };

    serde_json::from_value(value).map_err(|e| format!("failed to parse struct: {e}"))
}
