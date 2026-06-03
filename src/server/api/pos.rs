use axum::{extract::State, Json, http::StatusCode};
use std::sync::Arc;
use crate::hub::Hub;
use crate::integrations::stripe::client::StripeClient;
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct ConnectionTokenResponse {
    pub token: String,
}

#[derive(Serialize, Deserialize)]
pub struct CreateIntentRequest {
    pub amount_usd: f64,
}

#[derive(Serialize)]
pub struct CreateIntentResponse {
    pub intent_id: String,
}

#[derive(Serialize, Deserialize)]
pub struct CaptureIntentRequest {
    pub intent_id: String,
}

#[derive(Serialize)]
pub struct CaptureIntentResponse {
    pub charge_id: String,
}

pub fn router(hub: Arc<Hub>) -> axum::Router<Arc<dyn ohc_builtin_agent::mesh::transport::MeshTransport>> {
    axum::Router::new()
        .route("/token", axum::routing::post(get_connection_token))
        .route("/intent", axum::routing::post(create_payment_intent))
        .route("/capture", axum::routing::post(capture_payment_intent))
        .with_state(hub)
}

fn get_stripe_client() -> Result<StripeClient, StatusCode> {
    let stripe_key = std::env::var("STRIPE_API_KEY").unwrap_or_else(|_| "sk_test_123".to_string());
    Ok(StripeClient::new(stripe_key))
}

pub async fn get_connection_token(
    State(_hub): State<Arc<Hub>>,
    request: axum::extract::Request,
) -> Result<Json<ConnectionTokenResponse>, StatusCode> {
    let tenant_id = match request.extensions().get::<::server_auth::orchestration::AuthInfo>() {
        Some(auth) => {
            if auth.org_id.is_empty() {
                "default".to_string()
            } else {
                auth.org_id.clone()
            }
        },
        None => "default".to_string(),
    };

    let client = get_stripe_client()?;
    match client.create_terminal_connection_token(&tenant_id).await {
        Ok(token) => Ok(Json(ConnectionTokenResponse { token })),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn create_payment_intent(
    State(_hub): State<Arc<Hub>>,
    request: axum::extract::Request,
) -> Result<Json<CreateIntentResponse>, StatusCode> {
    let tenant_id = match request.extensions().get::<::server_auth::orchestration::AuthInfo>() {
        Some(auth) => {
            if auth.org_id.is_empty() {
                "default".to_string()
            } else {
                auth.org_id.clone()
            }
        },
        None => "default".to_string(),
    };

    let body_bytes = match axum::body::to_bytes(request.into_body(), 1024 * 1024).await {
        Ok(b) => b,
        Err(_) => return Err(StatusCode::BAD_REQUEST),
    };
    let req: CreateIntentRequest = serde_json::from_slice(&body_bytes).map_err(|_| StatusCode::BAD_REQUEST)?;

    let client = get_stripe_client()?;
    match client.create_payment_intent(req.amount_usd, &tenant_id).await {
        Ok(intent_id) => Ok(Json(CreateIntentResponse { intent_id })),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn capture_payment_intent(
    State(_hub): State<Arc<Hub>>,
    request: axum::extract::Request,
) -> Result<Json<CaptureIntentResponse>, StatusCode> {
    let tenant_id = match request.extensions().get::<::server_auth::orchestration::AuthInfo>() {
        Some(auth) => {
            if auth.org_id.is_empty() {
                "default".to_string()
            } else {
                auth.org_id.clone()
            }
        },
        None => "default".to_string(),
    };

    let body_bytes = match axum::body::to_bytes(request.into_body(), 1024 * 1024).await {
        Ok(b) => b,
        Err(_) => return Err(StatusCode::BAD_REQUEST),
    };
    let req: CaptureIntentRequest = serde_json::from_slice(&body_bytes).map_err(|_| StatusCode::BAD_REQUEST)?;

    let client = get_stripe_client()?;
    match client.capture_payment_intent(&req.intent_id, &tenant_id).await {
        Ok(charge_id) => Ok(Json(CaptureIntentResponse { charge_id })),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::http::Request;
    use axum::body::Body;

    #[tokio::test]
    async fn test_get_connection_token() {
        let tenant_id = "default".to_string();
        let client = StripeClient::new("sk_test_123".to_string());
        let token = client.create_terminal_connection_token(&tenant_id).await.unwrap();
        assert_eq!(token, "tss_mock_token_for_default");
    }

    #[tokio::test]
    async fn test_create_payment_intent() {
        let tenant_id = "default".to_string();
        let client = StripeClient::new("sk_test_123".to_string());
        let intent_id = client.create_payment_intent(15.0, &tenant_id).await.unwrap();
        assert_eq!(intent_id, "pi_mock_intent_default_1500");
    }

    #[tokio::test]
    async fn test_capture_payment_intent() {
        let tenant_id = "default".to_string();
        let client = StripeClient::new("sk_test_123".to_string());
        let charge_id = client.capture_payment_intent("pi_123", &tenant_id).await.unwrap();
        assert_eq!(charge_id, "ch_mock_charge_default_pi_123");
    }
}
