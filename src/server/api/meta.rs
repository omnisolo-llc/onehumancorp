use axum::{
    extract::State,
    Json,
};
use axum::extract::Query;
use axum::response::IntoResponse;
use std::collections::HashMap;
use std::sync::Arc;
use crate::hub::Hub;
use crate::ohc::orchestration::Message;
use chrono::Utc;
use axum::http::HeaderMap;
use hmac::{Hmac, Mac};
use sha2::Sha256;

type HmacSha256 = Hmac<Sha256>;

pub async fn meta_webhook_verify(
    Query(params): Query<HashMap<String, String>>,
) -> impl IntoResponse {
    let mode = params.get("hub.mode");
    let token = params.get("hub.verify_token");
    let challenge = params.get("hub.challenge");

    // Retrieve secret from environment
    let verify_token = match std::env::var("META_VERIFY_TOKEN") {
        Ok(t) => t,
        Err(_) => {
            tracing::error!("META_VERIFY_TOKEN not set");
            return axum::response::Response::builder()
                .status(axum::http::StatusCode::INTERNAL_SERVER_ERROR)
                .body(axum::body::Body::empty())
                .unwrap();
        }
    };

    if let (Some(m), Some(t), Some(c)) = (mode, token, challenge) {
        if m == "subscribe" && t == &verify_token {
            tracing::info!("Meta webhook verified");
            return axum::response::Response::builder()
                .status(axum::http::StatusCode::OK)
                .body(axum::body::Body::from(c.clone()))
                .unwrap();
        }
    }
    tracing::warn!("Meta webhook verification failed");
    axum::response::Response::builder()
        .status(axum::http::StatusCode::FORBIDDEN)
        .body(axum::body::Body::empty())
        .unwrap()
}

pub fn verify_meta_signature(
    headers: &HeaderMap,
    body_bytes: &[u8],
    app_secret: &str,
) -> Result<(), axum::http::StatusCode> {
    if app_secret.is_empty() {
        tracing::error!("META_APP_SECRET cannot be empty");
        return Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR);
    }

    let signature = headers.get("x-hub-signature-256")
        .and_then(|h| h.to_str().ok())
        .unwrap_or("");

    if !signature.starts_with("sha256=") {
        tracing::warn!("Meta webhook signature format invalid");
        return Err(axum::http::StatusCode::UNAUTHORIZED);
    }

    let sig_hex = &signature["sha256=".len()..];
    let decoded_sig = hex::decode(sig_hex).unwrap_or_default();

    let mut mac = HmacSha256::new_from_slice(app_secret.as_bytes())
        .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;
    mac.update(body_bytes);

    if mac.verify_slice(&decoded_sig).is_err() {
        tracing::warn!("Meta webhook signature mismatch");
        return Err(axum::http::StatusCode::UNAUTHORIZED);
    }

    Ok(())
}

pub async fn meta_webhook_handler(
    State(hub): State<Arc<Hub>>,
    headers: HeaderMap,
    body_bytes: axum::body::Bytes,
) -> axum::response::Result<Json<serde_json::Value>, axum::http::StatusCode> {
    tracing::info!("Received Meta webhook");

    let app_secret = match std::env::var("META_APP_SECRET") {
        Ok(s) => s,
        Err(_) => {
            tracing::error!("META_APP_SECRET not set");
            return Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    verify_meta_signature(&headers, &body_bytes, &app_secret)?;

    let payload: serde_json::Value = serde_json::from_slice(&body_bytes).map_err(|_| axum::http::StatusCode::BAD_REQUEST)?;

    let provider = crate::integrations::meta::provider::MetaProvider::new(hub);

    if let Err(e) = provider.handle_incoming_message(payload).await {
        tracing::error!("MetaProvider failed to handle message: {}", e);
    }

    Ok(Json(serde_json::json!({ "status": "received" })))
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::extract::State;
    use axum::http::HeaderMap;
    use crate::hub::Hub;
    use std::sync::Arc;
    use axum::response::IntoResponse;

    #[test]
    fn test_meta_webhook_verify_missing_secret() {
        let rt = tokio::runtime::Runtime::new().unwrap();
        std::thread::spawn(move || {
            temp_env::with_var("META_VERIFY_TOKEN", None::<String>, || {
                rt.block_on(async {
                    let mut params = HashMap::new();
                    params.insert("hub.mode".to_string(), "subscribe".to_string());
                    params.insert("hub.verify_token".to_string(), "any_token".to_string());
                    params.insert("hub.challenge".to_string(), "12345".to_string());

                    let query = axum::extract::Query(params);
                    let response = meta_webhook_verify(query).await.into_response();

                    assert_eq!(response.status(), axum::http::StatusCode::INTERNAL_SERVER_ERROR);
                });
            });
        }).join().unwrap();
    }

    #[test]
    fn test_meta_webhook_verify_success() {
        let rt = tokio::runtime::Runtime::new().unwrap();
        std::thread::spawn(move || {
            temp_env::with_var("META_VERIFY_TOKEN", Some("my-secret-token"), || {
                rt.block_on(async {
                    let mut params = HashMap::new();
                    params.insert("hub.mode".to_string(), "subscribe".to_string());
                    params.insert("hub.verify_token".to_string(), "my-secret-token".to_string());
                    params.insert("hub.challenge".to_string(), "12345".to_string());

                    let query = axum::extract::Query(params);
                    let response = meta_webhook_verify(query).await.into_response();

                    assert_eq!(response.status(), axum::http::StatusCode::OK);
                });
            });
        }).join().unwrap();
    }

    #[test]
    fn test_meta_webhook_handler_missing_secret() {
        let headers = HeaderMap::new();
        let body = b"{}";

        let response = verify_meta_signature(&headers, body, "");
        assert!(response.is_err());
        assert_eq!(response.unwrap_err(), axum::http::StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[test]
    fn test_meta_webhook_handler_invalid_signature() {
        let mut headers = HeaderMap::new();
        headers.insert("x-hub-signature-256", "sha256=invalid".parse().unwrap());
        let body = b"{}";

        let response = verify_meta_signature(&headers, body, "my-secret-app");
        assert!(response.is_err());
        assert_eq!(response.unwrap_err(), axum::http::StatusCode::UNAUTHORIZED);
    }

    #[test]
    fn test_meta_webhook_handler_valid_signature() {
        let mut headers = HeaderMap::new();
        let body = b"{\"test\":\"data\"}";

        let mut mac = Hmac::<Sha256>::new_from_slice(b"my-secret-app").unwrap();
        mac.update(body);
        let valid_signature = format!("sha256={}", hex::encode(mac.finalize().into_bytes()));

        headers.insert("x-hub-signature-256", valid_signature.parse().unwrap());

        let response = verify_meta_signature(&headers, body, "my-secret-app");
        assert!(response.is_ok());
    }
}
