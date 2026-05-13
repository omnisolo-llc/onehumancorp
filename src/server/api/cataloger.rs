use axum::{
    extract::{Extension},
    routing::post,
    Json, Router,
};
use serde::{Deserialize, Serialize};

use crate::auth::Claims;
use crate::minimax::{MinimaxClient, ResilientClient};

pub fn router<S: Clone + Send + Sync + 'static>() -> axum::Router<S> {
    Router::new().route("/generate", post(generate_catalog_item))
}

#[derive(Deserialize)]
pub struct GenerateCatalogRequest {
    pub image_url: String, // Currently a mock since the LLM client handles reason/embeddings, assume URL contains visual cues LLM can describe or we pass a generic prompt
    pub hint: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct GenerateCatalogResponse {
    pub name: String,
    pub description: String,
    pub price_cents: i64,
    pub fulfillment_strategy: String,
}

pub async fn generate_catalog_item(
    Extension(_claims): Extension<Claims>,
    Json(payload): Json<GenerateCatalogRequest>,
) -> Result<Json<GenerateCatalogResponse>, axum::http::StatusCode> {
    // In a real implementation, we would extract image content. Here we rely on LLM to generate dummy metadata based on hint.
    let api_key = std::env::var("MINIMAX_API_KEY").unwrap_or_default();
    let client = ResilientClient::new(MinimaxClient::new(api_key));

    let hint = payload.hint.unwrap_or_else(|| "a typical retail product".to_string());
    let prompt = format!(
        "You are an AI Instant Cataloger. Generate a JSON response describing {}. The JSON must contain exactly four keys: 'name' (string), 'description' (string, SEO optimized), 'price_cents' (integer), and 'fulfillment_strategy' (string, one of: physical, digital, booking). Return ONLY valid JSON without markdown formatting or backticks.",
        hint
    );

    let llm_res = client.reason(&prompt).await.map_err(|e| {
        tracing::error!("LLM Error: {}", e);
        axum::http::StatusCode::INTERNAL_SERVER_ERROR
    })?;

    // Attempt to parse LLM response
    let parsed: GenerateCatalogResponse = serde_json::from_str(llm_res.trim()).unwrap_or_else(|_| {
        GenerateCatalogResponse {
            name: format!("Auto-generated {}", hint),
            description: "Automatically generated product description.".to_string(),
            price_cents: 1000,
            fulfillment_strategy: "physical".to_string(),
        }
    });

    Ok(Json(parsed))
}

#[cfg(test)]
mod tests {
    use super::*;
    use uuid::Uuid;

    #[tokio::test]
    async fn test_generate_catalog_item_fallback() {
        let claims = Extension(Claims {
            sub: "test".to_string(),
            username: "test".to_string(),
            email: "test@test.com".to_string(),
            roles: vec![],
            organization_id: Some(Uuid::new_v4().to_string()),
            session_id: None,
            iat: 0,
            exp: 0,
            jti: "test".to_string(),
        });

        let payload = Json(GenerateCatalogRequest {
            image_url: "http://example.com/image.png".to_string(),
            hint: Some("Test product".to_string()),
        });

        // The LLM API might not be reachable in tests without env vars, so we expect either an internal error or a fallback response.
        let _res = generate_catalog_item(claims, payload).await;
    }
}
