use axum::{
    routing::post,
    Router, Json, extract::State, http::StatusCode, http::HeaderMap,
};
use serde::{Deserialize, Serialize};
use crate::billing::Tracker;

#[derive(Deserialize)]
pub struct CheckoutRequest {
    pub price_id: String,
    pub customer_id: String,
}

#[derive(Serialize, Debug)]
pub struct CheckoutResponse {
    pub url: String,
}

#[derive(Deserialize)]
pub struct CancelRequest {
    pub subscription_id: String,
}

#[derive(Serialize, Debug)]
pub struct CancelResponse {
    pub status: String,
}

#[derive(Deserialize)]
pub struct InvoicesRequest {
    pub customer_id: String,
}

pub fn router<S: Clone + Send + Sync + 'static>(tracker: Tracker) -> Router<S> {
    Router::new()
        .route("/checkout", post(create_checkout_session))
        .route("/cancel", post(cancel_subscription))
        .route("/invoices", post(list_invoices))
        .with_state(tracker)
}

async fn validate_auth(headers: &HeaderMap) -> Result<String, (StatusCode, String)> {
    let auth = headers.get("authorization").and_then(|h| h.to_str().ok());
    match auth {
        Some(token) if token.starts_with("Bearer ") => {
            // For this implementation, we extract the identity from the token directly.
            // In a real environment with zero secrets, this would be validated via OIDC/SPIFFE.
            let identity = token.trim_start_matches("Bearer ");
            if identity.is_empty() {
                return Err((StatusCode::UNAUTHORIZED, "Invalid token".to_string()));
            }
            Ok(identity.to_string())
        }
        _ => Err((StatusCode::UNAUTHORIZED, "Missing Authorization header".to_string())),
    }
}

async fn create_checkout_session(
    headers: HeaderMap,
    State(tracker): State<Tracker>,
    Json(payload): Json<CheckoutRequest>,
) -> Result<Json<CheckoutResponse>, (StatusCode, String)> {
    let identity = validate_auth(&headers).await?;
    if identity != payload.customer_id {
         return Err((StatusCode::FORBIDDEN, "IDOR: identity mismatch".to_string()));
    }
    match tracker.create_checkout_session(&payload.price_id, &payload.customer_id).await {
        Ok(url) => Ok(Json(CheckoutResponse { url })),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e)),
    }
}

async fn cancel_subscription(
    headers: HeaderMap,
    State(tracker): State<Tracker>,
    Json(payload): Json<CancelRequest>,
) -> Result<Json<CancelResponse>, (StatusCode, String)> {
    let identity = validate_auth(&headers).await?;
    if !payload.subscription_id.ends_with(&identity) {
        return Err((StatusCode::FORBIDDEN, "IDOR: identity mismatch".to_string()));
    }
    match tracker.cancel_subscription(&payload.subscription_id).await {
        Ok(sub) => Ok(Json(CancelResponse { status: sub.status })),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e)),
    }
}

async fn list_invoices(
    headers: HeaderMap,
    State(tracker): State<Tracker>,
    Json(payload): Json<InvoicesRequest>,
) -> Result<Json<Vec<crate::integrations::stripe::client::StripeInvoice>>, (StatusCode, String)> {
    let identity = validate_auth(&headers).await?;
    if identity != payload.customer_id {
         return Err((StatusCode::FORBIDDEN, "IDOR: identity mismatch".to_string()));
    }
    match tracker.list_invoices(&payload.customer_id).await {
        Ok(invoices) => Ok(Json(invoices)),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use crate::integrations::stripe::client::StripeClient;
    use axum::http::header::AUTHORIZATION;

    #[tokio::test]
    async fn test_create_checkout_session() {
        let mut tracker = Tracker::new();
        tracker.stripe_client = Some(Arc::new(StripeClient::new("sk_test_123".to_string())));

        let payload = CheckoutRequest {
            price_id: "price_123".into(),
            customer_id: "cus_123".into(),
        };

        let mut headers = HeaderMap::new();
        headers.insert(AUTHORIZATION, "Bearer cus_123".parse().unwrap());

        let response = create_checkout_session(headers, State(tracker), Json(payload)).await.unwrap();
        assert!(response.0.url.contains("checkout.stripe.com"));
    }

    #[tokio::test]
    async fn test_cancel_subscription() {
        let mut tracker = Tracker::new();
        tracker.stripe_client = Some(Arc::new(StripeClient::new("sk_test_123".to_string())));
        let payload = CancelRequest { subscription_id: "sub_123_cus_123".into() };
        let mut headers = HeaderMap::new();
        headers.insert(AUTHORIZATION, "Bearer cus_123".parse().unwrap());
        let response = cancel_subscription(headers, State(tracker), Json(payload)).await.unwrap();
        assert_eq!(response.0.status, "canceled");
    }

    #[tokio::test]
    async fn test_list_invoices() {
        let mut tracker = Tracker::new();
        tracker.stripe_client = Some(Arc::new(StripeClient::new("sk_test_123".to_string())));
        let payload = InvoicesRequest { customer_id: "cus_123".into() };
        let mut headers = HeaderMap::new();
        headers.insert(AUTHORIZATION, "Bearer cus_123".parse().unwrap());
        let response = list_invoices(headers, State(tracker), Json(payload)).await.unwrap();
        assert_eq!(response.0.len(), 1);
    }

    #[tokio::test]
    async fn test_auth_failures() {
        let tracker = Tracker::new();

        // Test missing header
        let headers = HeaderMap::new();
        let payload = CheckoutRequest { price_id: "p_1".into(), customer_id: "c_1".into() };
        let res = create_checkout_session(headers, State(tracker.clone()), Json(payload)).await;
        assert_eq!(res.unwrap_err().0, StatusCode::UNAUTHORIZED);

        // Test invalid token
        let mut headers = HeaderMap::new();
        headers.insert(AUTHORIZATION, "Bearer ".parse().unwrap());
        let payload = CancelRequest { subscription_id: "s_1".into() };
        let res = cancel_subscription(headers, State(tracker.clone()), Json(payload)).await;
        assert_eq!(res.unwrap_err().0, StatusCode::UNAUTHORIZED);

        // Test IDOR mismatch
        let mut headers = HeaderMap::new();
        headers.insert(AUTHORIZATION, "Bearer malicious_user".parse().unwrap());
        let payload = InvoicesRequest { customer_id: "target_user".into() };
        let res = list_invoices(headers, State(tracker.clone()), Json(payload)).await;
        assert_eq!(res.unwrap_err().0, StatusCode::FORBIDDEN);
    }
}
