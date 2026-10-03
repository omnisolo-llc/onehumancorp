use axum::{
    Json, Router,
    extract::{Extension, Path, State},
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
    routing::{get, post},
};
use hmac::{Hmac, Mac};
use serde::{Deserialize, Serialize};
use sha2::Sha256;
use std::sync::Arc;

use super::fulfillment::authentication::ProviderScope;
#[path = "shipping/authority.rs"]
pub mod authority;
use authority::{AuthorizedOwner, ShippingAccess};
#[path = "shipping/labels.rs"]
pub(crate) mod labels;

const MAX_PARCEL_VALUE: f64 = 100_000.0;
type HmacSha256 = Hmac<Sha256>;

fn safe_id(value: &str) -> bool {
    !value.is_empty()
        && value.len() <= 128
        && value
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'_' | b'-'))
}

fn parse_weight(value: &str) -> Option<f64> {
    let weight = value.trim().parse::<f64>().ok()?;
    (weight.is_finite() && weight > 0.0 && weight <= MAX_PARCEL_VALUE).then_some(weight)
}

fn parse_dimensions(value: &str) -> Option<(f64, f64, f64)> {
    let values = value
        .split(['x', 'X'])
        .map(str::trim)
        .map(str::parse::<f64>)
        .collect::<Result<Vec<_>, _>>()
        .ok()?;
    let [length, width, height] = values.as_slice() else {
        return None;
    };
    values
        .iter()
        .all(|value| value.is_finite() && *value > 0.0 && *value <= MAX_PARCEL_VALUE)
        .then_some((*length, *width, *height))
}

fn signed_rate_id(secret: &str, tenant_id: &str, order_id: &str, rate_id: &str) -> Option<String> {
    if secret.trim().is_empty() || !safe_id(order_id) || !safe_id(rate_id) {
        return None;
    }
    let mut mac = HmacSha256::new_from_slice(secret.as_bytes()).ok()?;
    mac.update(tenant_id.as_bytes());
    mac.update(&[0]);
    mac.update(order_id.as_bytes());
    mac.update(&[0]);
    mac.update(rate_id.as_bytes());
    Some(format!(
        "{rate_id}.{}",
        hex::encode(mac.finalize().into_bytes())
    ))
}

fn verified_rate_id<'a>(
    secret: &str,
    tenant_id: &str,
    order_id: &str,
    signed_id: &'a str,
) -> Option<&'a str> {
    let (rate_id, signature) = signed_id.rsplit_once('.')?;
    if !safe_id(rate_id) || signature.len() != 64 {
        return None;
    }
    let signature = hex::decode(signature).ok()?;
    let mut mac = HmacSha256::new_from_slice(secret.as_bytes()).ok()?;
    mac.update(tenant_id.as_bytes());
    mac.update(&[0]);
    mac.update(order_id.as_bytes());
    mac.update(&[0]);
    mac.update(rate_id.as_bytes());
    mac.verify_slice(&signature).ok()?;
    Some(rate_id)
}

#[derive(Deserialize)]
#[allow(non_snake_case)]
pub struct FetchRatesRequest {
    pub orderId: String,
    pub weight: String,
    pub dimensions: String,
}

#[derive(Deserialize)]
#[allow(non_snake_case)]
pub struct PurchaseLabelRequest {
    pub orderId: String,
    pub rateId: String,
}

#[derive(Serialize)]
pub struct RatesResponse {
    pub rates: Vec<crate::integrations::shippo::client::ShippoRate>,
}

pub fn router<S: Clone + Send + Sync + 'static>(access: Arc<ShippingAccess>) -> Router<S> {
    Router::new()
        .route("/rates", post(fetch_rates))
        .route("/label", post(purchase_label))
        .route("/label/{transaction_id}", get(read_label))
        .with_state(access)
}

fn authenticated_tenant(claims: &::server_common::Claims) -> Option<String> {
    ::server_common::auth_utils::signed_tenant_id(claims)
}

async fn tenant_owns_order(
    owner: AuthorizedOwner,
    order_id: &str,
) -> Result<bool, authority::Error> {
    let tenant_id = owner.tenant_id().to_owned();
    match owner {
        AuthorizedOwner::Postgres(owner) => {
            let mut tx = owner.begin().await?;
            let exists=sqlx::query_scalar::<_,bool>("SELECT EXISTS(SELECT 1 FROM orders WHERE tenant_id=$1 AND id=$2 AND lower(COALESCE(status,'')) NOT IN ('canceled','cancelled','fulfilled','returned'))")
                .bind(&tenant_id).bind(order_id).fetch_one(tx.connection()).await?;
            tx.commit().await?;
            Ok(exists)
        }
        AuthorizedOwner::Sqlite(owner) => {
            let mut tx = owner.begin().await?;
            let exists=sqlx::query_scalar::<_,bool>("SELECT EXISTS(SELECT 1 FROM orders WHERE tenant_id=? AND id=? AND lower(COALESCE(status,'')) NOT IN ('canceled','cancelled','fulfilled','returned'))")
                .bind(&tenant_id).bind(order_id).fetch_one(tx.connection()).await?;
            tx.commit().await?;
            Ok(exists)
        }
    }
}

async fn fetch_rates(
    State(access): State<Arc<ShippingAccess>>,
    Extension(claims): Extension<::server_common::Claims>,
    headers: HeaderMap,
    Json(payload): Json<FetchRatesRequest>,
) -> impl IntoResponse {
    let Some(tenant_id) = authenticated_tenant(&claims) else {
        return StatusCode::UNAUTHORIZED.into_response();
    };
    let owner = match access.authorize(&claims, &headers).await {
        Ok(owner) => owner,
        Err(error) => return authority::response(error),
    };
    let _scope = match ProviderScope::from_environment("SHIPPO") {
        Ok(scope) if scope.tenant_id == tenant_id => scope,
        Ok(_) => return StatusCode::FORBIDDEN.into_response(),
        Err(message) => {
            return (
                StatusCode::SERVICE_UNAVAILABLE,
                Json(serde_json::json!({"error":message})),
            )
                .into_response();
        }
    };
    if !safe_id(payload.orderId.trim()) {
        return (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({ "error": "valid order id is required" })),
        )
            .into_response();
    }
    let Some(weight) = parse_weight(&payload.weight) else {
        return (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({ "error": "weight must be a positive number" })),
        )
            .into_response();
    };
    let Some((length, width, height)) = parse_dimensions(&payload.dimensions) else {
        return (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({ "error": "dimensions must contain three positive numbers" })),
        )
            .into_response();
    };
    match tenant_owns_order(owner, payload.orderId.trim()).await {
        Ok(true) => {}
        Ok(false) => return StatusCode::NOT_FOUND.into_response(),
        Err(error) => {
            return authority::response(error);
        }
    }
    let token = match std::env::var("SHIPPO_API_TOKEN") {
        Ok(token) if !token.trim().is_empty() => token,
        _ => {
            return (
                StatusCode::SERVICE_UNAVAILABLE,
                Json(serde_json::json!({ "error": "Shippo is not configured" })),
            )
                .into_response();
        }
    };
    let client = crate::integrations::shippo::provider::ShippoProvider::new(token.clone());
    let dimensions = format!("{length}x{width}x{height}");
    match client.fetch_rates(weight, &dimensions).await {
        Ok(mut rates) => {
            let current = match access.authorize(&claims, &headers).await {
                Ok(owner) => owner,
                Err(error) => return authority::response(error),
            };
            if let Err(error) = current.confirm().await {
                return authority::response(error);
            }
            for rate in &mut rates {
                let Some(signed_id) =
                    signed_rate_id(&token, &tenant_id, payload.orderId.trim(), &rate.id)
                else {
                    return StatusCode::BAD_GATEWAY.into_response();
                };
                rate.id = signed_id;
            }
            (StatusCode::OK, Json(RatesResponse { rates })).into_response()
        }
        Err(error) => (
            StatusCode::BAD_GATEWAY,
            Json(serde_json::json!({ "error": error })),
        )
            .into_response(),
    }
}

async fn purchase_label(
    State(access): State<Arc<ShippingAccess>>,
    Extension(claims): Extension<::server_common::Claims>,
    headers: HeaderMap,
    Json(payload): Json<PurchaseLabelRequest>,
) -> impl IntoResponse {
    let Some(tenant_id) = authenticated_tenant(&claims) else {
        return StatusCode::UNAUTHORIZED.into_response();
    };
    let owner = match access.authorize(&claims, &headers).await {
        Ok(owner) => owner,
        Err(error) => return authority::response(error),
    };
    let scope = match ProviderScope::from_environment("SHIPPO") {
        Ok(scope) if scope.tenant_id == tenant_id => scope,
        Ok(_) => return StatusCode::FORBIDDEN.into_response(),
        Err(message) => {
            return (
                StatusCode::SERVICE_UNAVAILABLE,
                Json(serde_json::json!({"error":message})),
            )
                .into_response();
        }
    };
    let order_id = payload.orderId.trim();
    if !safe_id(order_id) || payload.rateId.len() > 256 {
        return (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({ "error": "valid order and rate ids are required" })),
        )
            .into_response();
    }
    let token = match std::env::var("SHIPPO_API_TOKEN") {
        Ok(token) if !token.trim().is_empty() => token,
        _ => {
            return (
                StatusCode::SERVICE_UNAVAILABLE,
                Json(serde_json::json!({ "error": "Shippo is not configured" })),
            )
                .into_response();
        }
    };
    let Some(rate_id) = verified_rate_id(&token, &tenant_id, order_id, payload.rateId.trim())
    else {
        return (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({ "error": "rate does not belong to this order" })),
        )
            .into_response();
    };
    match tenant_owns_order(owner, order_id).await {
        Ok(true) => {}
        Ok(false) => return StatusCode::NOT_FOUND.into_response(),
        Err(error) => {
            return authority::response(error);
        }
    }
    let client = crate::integrations::shippo::provider::ShippoProvider::new(token);
    match client.purchase_label(rate_id).await {
        Ok(response) => {
            let recorded=match access.authorize(&claims,&headers).await {
                Ok(owner)=>labels::record(owner,&scope,order_id,&response).await,
                Err(error)=>Err(error),
            };
            match recorded {
                Ok(()) => (StatusCode::OK, Json(response)).into_response(),
                Err(error) => {
                    tracing::error!("failed to persist purchased shipping label: {error}");
                    (StatusCode::ACCEPTED, Json(serde_json::json!({
                        "success":false,"status":"outcome_unknown","reconciliationRequired":true,
                        "transactionId":response.transaction_id,
                        "error":"Provider returned a label but local recording failed; reconcile before retrying"
                    }))).into_response()
                }
            }
        }
        Err(error) => (
            StatusCode::ACCEPTED,
            Json(serde_json::json!({ "success":false,"status":"outcome_unknown","reconciliationRequired":true,"error": error })),
        )
            .into_response(),
    }
}

async fn read_label(
    State(access): State<Arc<ShippingAccess>>,
    Extension(claims): Extension<::server_common::Claims>,
    Path(transaction): Path<String>,
    headers: HeaderMap,
) -> axum::response::Response {
    let Some(tenant_id) = authenticated_tenant(&claims) else {
        return StatusCode::UNAUTHORIZED.into_response();
    };
    let owner = match access.authorize(&claims, &headers).await {
        Ok(owner) => owner,
        Err(error) => return authority::response(error),
    };
    let scope = match ProviderScope::from_environment("SHIPPO") {
        Ok(scope) if scope.tenant_id == tenant_id => scope,
        Ok(_) => return StatusCode::FORBIDDEN.into_response(),
        Err(message) => {
            return (
                StatusCode::SERVICE_UNAVAILABLE,
                Json(serde_json::json!({"error":message})),
            )
                .into_response();
        }
    };
    if !safe_id(&transaction) {
        return StatusCode::BAD_REQUEST.into_response();
    }
    match labels::read(owner,&scope,&transaction).await {
        Ok(Some(receipt)) => (StatusCode::OK,Json(receipt)).into_response(),
        Ok(None) => (StatusCode::NOT_FOUND,Json(serde_json::json!({"error":"No local receipt; this does not establish whether a provider purchase occurred"}))).into_response(),
        Err(error) => authority::response(error)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_and_normalizes_three_positive_package_dimensions() {
        assert_eq!(parse_dimensions("10x8x6"), Some((10.0, 8.0, 6.0)));
        assert_eq!(parse_dimensions(" 10.5 X 8 X 6 "), Some((10.5, 8.0, 6.0)));
    }

    #[test]
    fn rejects_incomplete_non_finite_and_non_positive_dimensions() {
        for dimensions in ["10", "10x8", "10x8x6x4", "NaNx8x6", "0x8x6", "-1x8x6"] {
            assert_eq!(parse_dimensions(dimensions), None, "{dimensions}");
        }
    }

    #[test]
    fn rejects_invalid_shipping_weight_instead_of_substituting_one_ounce() {
        assert_eq!(parse_weight(""), None);
        assert_eq!(parse_weight("not-a-number"), None);
        assert_eq!(parse_weight("0"), None);
        assert_eq!(parse_weight("-1"), None);
        assert_eq!(parse_weight("16"), Some(16.0));
    }

    #[test]
    fn rate_tokens_are_bound_to_the_tenant_and_order() {
        let signed = signed_rate_id("secret", "tenant-a", "order-1", "rate_123").unwrap();
        assert_eq!(
            verified_rate_id("secret", "tenant-a", "order-1", &signed),
            Some("rate_123")
        );
        assert!(verified_rate_id("secret", "tenant-b", "order-1", &signed).is_none());
        assert!(verified_rate_id("secret", "tenant-a", "order-2", &signed).is_none());
        assert!(verified_rate_id("wrong", "tenant-a", "order-1", &signed).is_none());
    }
}
