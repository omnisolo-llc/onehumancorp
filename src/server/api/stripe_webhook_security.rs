//! HTTP adapter for Stripe's endpoint-specific raw-body authentication.
use crate::integrations::stripe::webhook_signature::{self, SignatureError};
use axum::{
    body::Body,
    extract::{Request, State},
    http::{HeaderMap, StatusCode},
    middleware::Next,
    response::{IntoResponse, Response},
};

#[derive(Clone, Copy)]
pub enum Endpoint {
    Billing,
    Ledger,
}
impl Endpoint {
    fn signing_secret(self) -> Result<Vec<u8>, StatusCode> {
        let (value, file) = match self {
            Self::Billing => ("STRIPE_WEBHOOK_SECRET", "STRIPE_WEBHOOK_SECRET_FILE"),
            Self::Ledger => (
                "STRIPE_LEDGER_WEBHOOK_SECRET",
                "STRIPE_LEDGER_WEBHOOK_SECRET_FILE",
            ),
        };
        // Different endpoint secrets never fall back to an API key or another
        // endpoint. Reading existing configuration does not provision a key.
        ::server_common::secret_source::load_optional_secret(value, file)
            .map_err(|_| StatusCode::SERVICE_UNAVAILABLE)?
            .ok_or(StatusCode::SERVICE_UNAVAILABLE)
    }
}
fn signature_header(headers: &HeaderMap) -> Result<&str, StatusCode> {
    let mut values = headers.get_all("Stripe-Signature").iter();
    let value = values.next().ok_or(StatusCode::UNAUTHORIZED)?;
    if values.next().is_some() {
        return Err(StatusCode::UNAUTHORIZED);
    }
    value.to_str().map_err(|_| StatusCode::UNAUTHORIZED)
}

pub async fn verify_request(request: Request, endpoint: Endpoint) -> Result<Request, StatusCode> {
    let secret = endpoint.signing_secret()?;
    let signature = signature_header(request.headers())?.to_owned();
    let (parts, body) = request.into_parts();
    let bytes = axum::body::to_bytes(body, webhook_signature::MAX_BODY_BYTES)
        .await
        .map_err(|_| StatusCode::PAYLOAD_TOO_LARGE)?;
    webhook_signature::verify_now(&bytes, &signature, &secret).map_err(|error| match error {
        SignatureError::MissingSecret => StatusCode::SERVICE_UNAVAILABLE,
        SignatureError::BodyTooLarge => StatusCode::PAYLOAD_TOO_LARGE,
        _ => StatusCode::UNAUTHORIZED,
    })?;
    Ok(Request::from_parts(parts, Body::from(bytes)))
}

pub async fn require_verified_stripe(
    State(endpoint): State<Endpoint>,
    request: Request,
    next: Next,
) -> Response {
    match verify_request(request, endpoint).await {
        Ok(request) => next.run(request).await,
        Err(status) => status.into_response(),
    }
}
