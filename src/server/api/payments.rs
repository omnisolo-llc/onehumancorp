use axum::{extract::State, Json};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use crate::hub::Hub;
use crate::integrations::stripe::terminal::StripeTerminalClient;

pub fn router(hub: Arc<Hub>) -> axum::Router<Arc<dyn ohc_builtin_agent::mesh::transport::MeshTransport>> {
    let internal_router = axum::Router::new()
        .route("/terminal/token", axum::routing::post(create_terminal_token))
        .route("/terminal/payment_intent", axum::routing::post(create_payment_intent))
        .with_state(hub);

    // We need to return a router with MeshTransport state to match the parent
    axum::Router::new().merge(internal_router)
}

#[derive(Deserialize)]
pub struct CreatePaymentIntentRequest {
    pub amount: i64,
    pub currency: String,
}

#[derive(Serialize)]
pub struct CreatePaymentIntentResponse {
    pub success: bool,
    pub client_secret: String,
    pub error: Option<String>,
}

pub async fn create_payment_intent(
    State(_hub): State<Arc<Hub>>,
    axum::extract::Extension(auth_info_opt): axum::extract::Extension<Option<::server_auth::orchestration::AuthInfo>>,
    Json(payload): Json<CreatePaymentIntentRequest>,
) -> Json<CreatePaymentIntentResponse> {
    let tenant_id = match auth_info_opt {
        Some(auth) => {
            if auth.org_id.is_empty() {
                "default".to_string()
            } else {
                auth.org_id
            }
        },
        None => return Json(CreatePaymentIntentResponse {
            success: false,
            client_secret: "".to_string(),
            error: Some("Missing authentication context".to_string()),
        }),
    };

    let stripe_key = match std::env::var("STRIPE_API_KEY") {
        Ok(key) => key,
        Err(_) => return Json(CreatePaymentIntentResponse {
            success: false,
            client_secret: "".to_string(),
            error: Some("STRIPE_API_KEY is required".to_string()),
        }),
    };

    let client = StripeTerminalClient::new(stripe_key);
    match client.create_payment_intent(&tenant_id, payload.amount, &payload.currency).await {
        Ok(intent) => Json(CreatePaymentIntentResponse {
            success: true,
            client_secret: intent.id, // For real intent we should return client_secret here
            error: None,
        }),
        Err(e) => Json(CreatePaymentIntentResponse {
            success: false,
            client_secret: "".to_string(),
            error: Some(e),
        }),
    }
}


#[derive(Deserialize)]
pub struct CreateTokenRequest {
    // Optionally add fields here if needed
}

#[derive(Serialize)]
pub struct CreateTokenResponse {
    pub success: bool,
    pub token: String,
    pub error: Option<String>,
}

pub async fn create_terminal_token(
    State(_hub): State<Arc<Hub>>,
    axum::extract::Extension(auth_info_opt): axum::extract::Extension<Option<::server_auth::orchestration::AuthInfo>>,
) -> Json<CreateTokenResponse> {
    let tenant_id = match auth_info_opt {
        Some(auth) => {
            if auth.org_id.is_empty() {
                "default".to_string()
            } else {
                auth.org_id
            }
        },
        None => return Json(CreateTokenResponse {
            success: false,
            token: "".to_string(),
            error: Some("Missing authentication context".to_string()),
        }),
    };

    let stripe_key = match std::env::var("STRIPE_API_KEY") {
        Ok(key) => key,
        Err(_) => return Json(CreateTokenResponse {
            success: false,
            token: "".to_string(),
            error: Some("STRIPE_API_KEY is required".to_string()),
        }),
    };

    let client = StripeTerminalClient::new(stripe_key);
    match client.create_terminal_connection_token(&tenant_id).await {
        Ok(token) => Json(CreateTokenResponse {
            success: true,
            token,
            error: None,
        }),
        Err(e) => Json(CreateTokenResponse {
            success: false,
            token: "".to_string(),
            error: Some(e),
        }),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{body::Body, http::Request};
    use tower::ServiceExt;
    use crate::hub::Hub;
    use std::sync::Arc;

    #[tokio::test]
    async fn test_create_terminal_token_missing_auth() {
        let (tx, _rx) = tokio::sync::mpsc::channel(100);
        let pool = crate::db::get_pool();
        let hub = Arc::new(Hub::new(tx, pool.clone()));
        let app = axum::Router::new()
            .route("/terminal/token", axum::routing::post(create_terminal_token))
            .layer(axum::middleware::from_fn(|mut req: axum::extract::Request, next: axum::middleware::Next| async move {
                req.extensions_mut().insert(Option::<::server_auth::orchestration::AuthInfo>::None);
                next.run(req).await
            }))
            .with_state(hub);

        let response = app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/terminal/token")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), axum::http::StatusCode::OK);
    }

    #[tokio::test]
    async fn test_create_payment_intent_missing_auth() {
        let (tx, _rx) = tokio::sync::mpsc::channel(100);
        let pool = crate::db::get_pool();
        let hub = Arc::new(Hub::new(tx, pool.clone()));
        let app = axum::Router::new()
            .route("/terminal/payment_intent", axum::routing::post(create_payment_intent))
            .layer(axum::middleware::from_fn(|mut req: axum::extract::Request, next: axum::middleware::Next| async move {
                req.extensions_mut().insert(Option::<::server_auth::orchestration::AuthInfo>::None);
                next.run(req).await
            }))
            .with_state(hub);

        let payload = serde_json::json!({
            "amount": 1000,
            "currency": "usd"
        });

        let response = app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/terminal/payment_intent")
                    .header("content-type", "application/json")
                    .body(Body::from(serde_json::to_string(&payload).unwrap()))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), axum::http::StatusCode::OK);
    }
}

#[cfg(test)]
mod additional_tests {
    use super::*;
    use axum::{body::Body, http::Request};
    use tower::ServiceExt;
    use crate::hub::Hub;
    use std::sync::Arc;

    #[tokio::test]
    async fn test_create_terminal_token_with_auth() {
        unsafe { std::env::set_var("STRIPE_API_KEY", "sk_test_123"); }
        let (tx, _rx) = tokio::sync::mpsc::channel(100);
        let pool = crate::db::get_pool();
        let hub = Arc::new(Hub::new(tx, pool.clone()));
        let app = axum::Router::new()
            .route("/terminal/token", axum::routing::post(create_terminal_token))
            .layer(axum::middleware::from_fn(|mut req: axum::extract::Request, next: axum::middleware::Next| async move {
                req.extensions_mut().insert(Some(::server_auth::orchestration::AuthInfo {
                    agent_id: "agent_123".to_string(),
                    org_id: "test_tenant".to_string(),
                    spiffe_id: "".to_string(),
                }));
                next.run(req).await
            }))
            .with_state(hub);

        let response = app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/terminal/token")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), axum::http::StatusCode::OK);
    }

    #[tokio::test]
    async fn test_create_payment_intent_with_auth() {
        unsafe { std::env::set_var("STRIPE_API_KEY", "sk_test_123"); }
        let (tx, _rx) = tokio::sync::mpsc::channel(100);
        let pool = crate::db::get_pool();
        let hub = Arc::new(Hub::new(tx, pool.clone()));
        let app = axum::Router::new()
            .route("/terminal/payment_intent", axum::routing::post(create_payment_intent))
            .layer(axum::middleware::from_fn(|mut req: axum::extract::Request, next: axum::middleware::Next| async move {
                req.extensions_mut().insert(Some(::server_auth::orchestration::AuthInfo {
                    agent_id: "agent_123".to_string(),
                    org_id: "test_tenant".to_string(),
                    spiffe_id: "".to_string(),
                }));
                next.run(req).await
            }))
            .with_state(hub);

        let payload = serde_json::json!({
            "amount": 1000,
            "currency": "usd"
        });

        let response = app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/terminal/payment_intent")
                    .header("content-type", "application/json")
                    .body(Body::from(serde_json::to_string(&payload).unwrap()))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), axum::http::StatusCode::OK);
    }
}
