//! Authentication for the documented provider webhook protocols.
use axum::http::HeaderMap;
use hmac::{Hmac, Mac};
use sha2::Sha256;

pub const MAX_BODY_BYTES: usize = 1024 * 1024;
pub const MAX_TIMESTAMP_SKEW_SECONDS: i64 = 300;

/// Verify the official Shippo signature over timestamp, dot, and exact raw body.
/// This proves transport authenticity only. Tenant/account binding and replay
/// admission must occur separately in a durable database transaction.
pub fn verify_shippo_signature(
    secret: &str,
    headers: &HeaderMap,
    body: &[u8],
    now_seconds: i64,
) -> Result<(), &'static str> {
    if secret.trim().is_empty() || body.len() > MAX_BODY_BYTES {
        return Err("unverifiable Shippo webhook");
    }
    let mut headers = headers.get_all("shippo-auth-signature").iter();
    let header = headers.next().ok_or("missing Shippo signature")?;
    if headers.next().is_some() {
        return Err("duplicate Shippo signature");
    }
    let header = header.to_str().map_err(|_| "malformed Shippo signature")?;
    if header.len() > 128 {
        return Err("oversized Shippo signature");
    }
    let mut timestamp = None;
    let mut signature = None;
    for field in header.split(',') {
        let (key, value) = field
            .trim()
            .split_once('=')
            .ok_or("malformed signature field")?;
        match key {
            "t" if timestamp.is_none() => timestamp = Some(value),
            "v1" if signature.is_none() => signature = Some(value),
            _ => return Err("unknown or duplicate signature field"),
        }
    }
    let timestamp_text = timestamp.ok_or("missing signature timestamp")?;
    let timestamp: i64 = timestamp_text
        .parse()
        .map_err(|_| "invalid signature timestamp")?;
    if timestamp <= 0
        || timestamp.to_string() != timestamp_text
        || now_seconds.abs_diff(timestamp) > MAX_TIMESTAMP_SKEW_SECONDS as u64
    {
        return Err("signature timestamp outside accepted interval");
    }
    let signature = signature.ok_or("missing v1 signature")?;
    if signature.len() != 64 || !signature.bytes().all(|b| b.is_ascii_hexdigit()) {
        return Err("invalid v1 signature");
    }
    let signature = hex::decode(signature).map_err(|_| "invalid v1 signature")?;
    let mut mac =
        Hmac::<Sha256>::new_from_slice(secret.as_bytes()).map_err(|_| "invalid webhook secret")?;
    mac.update(timestamp_text.as_bytes());
    mac.update(b".");
    mac.update(body);
    mac.verify_slice(&signature)
        .map_err(|_| "invalid Shippo signature")
}

#[cfg(test)]
mod tests {
    use super::*;
    fn signed(secret: &str, timestamp: i64, body: &[u8]) -> HeaderMap {
        let mut mac = Hmac::<Sha256>::new_from_slice(secret.as_bytes()).unwrap();
        mac.update(timestamp.to_string().as_bytes());
        mac.update(b".");
        mac.update(body);
        let mut headers = HeaderMap::new();
        headers.insert(
            "shippo-auth-signature",
            format!(
                "t={timestamp},v1={}",
                hex::encode(mac.finalize().into_bytes())
            )
            .parse()
            .unwrap(),
        );
        headers
    }
    #[test]
    fn verifies_official_timestamp_dot_raw_body_and_rejects_mutation() {
        let body = br#"{"event":"track_updated"}"#;
        let headers = signed("test-secret", 1000, body);
        assert!(verify_shippo_signature("test-secret", &headers, body, 1000).is_ok());
        assert!(verify_shippo_signature("test-secret", &headers, b"{}", 1000).is_err());
        assert!(verify_shippo_signature("wrong", &headers, body, 1000).is_err());
        assert!(verify_shippo_signature("", &headers, body, 1000).is_err());
        assert!(verify_shippo_signature("test-secret", &headers, body, 1301).is_err());
        assert!(verify_shippo_signature("test-secret", &headers, body, 699).is_err());
    }
    #[test]
    fn duplicate_headers_and_fields_are_rejected() {
        let body = b"{}";
        let mut headers = signed("secret", 1000, body);
        let value = headers["shippo-auth-signature"].clone();
        headers.append("shippo-auth-signature", value);
        assert!(verify_shippo_signature("secret", &headers, body, 1000).is_err());
        let mut headers = signed("secret", 1000, body);
        headers.insert(
            "shippo-auth-signature",
            format!(
                "{},t=1000",
                headers["shippo-auth-signature"].to_str().unwrap()
            )
            .parse()
            .unwrap(),
        );
        assert!(verify_shippo_signature("secret", &headers, body, 1000).is_err());
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProviderScope {
    pub tenant_id: String,
    pub account_namespace: String,
    pub is_test: bool,
}

impl ProviderScope {
    pub fn from_environment(provider: &str) -> Result<Self, &'static str> {
        let read = |suffix: &str| -> Result<String, &'static str> {
            let value = std::env::var(format!("{provider}_{suffix}"))
                .map_err(|_| "provider account binding is not configured")?;
            let value = value.trim();
            if value.is_empty() || value.len() > 128 || value.chars().any(char::is_control) {
                return Err("provider account binding is invalid");
            }
            Ok(value.to_owned())
        };
        let tenant_id = read("TENANT_ID")?;
        if tenant_id.eq_ignore_ascii_case("system") {
            return Err("a specific provider tenant is required");
        }
        let account_namespace = read("ACCOUNT_NAMESPACE")?;
        let is_test = match read("WEBHOOK_MODE")?.as_str() {
            "test" => true,
            "live" => false,
            _ => return Err("provider webhook mode must be test or live"),
        };
        Ok(Self {
            tenant_id,
            account_namespace,
            is_test,
        })
    }
}

pub fn verify_doordash_authorization(
    expected: &str,
    headers: &HeaderMap,
) -> Result<(), &'static str> {
    if expected.trim().is_empty() {
        return Err("DoorDash webhook authentication is not configured");
    }
    let mut values = headers.get_all("authorization").iter();
    let actual = values.next().ok_or("missing DoorDash authorization")?;
    if values.next().is_some() {
        return Err("duplicate DoorDash authorization");
    }
    let actual = actual
        .to_str()
        .map_err(|_| "invalid DoorDash authorization")?;
    // HMAC verification provides a constant-time comparison without logging either header.
    let mut expected_mac = Hmac::<Sha256>::new_from_slice(b"ohc-webhook-header-comparison")
        .map_err(|_| "invalid header comparison")?;
    expected_mac.update(expected.as_bytes());
    let mut actual_mac = Hmac::<Sha256>::new_from_slice(b"ohc-webhook-header-comparison")
        .map_err(|_| "invalid header comparison")?;
    actual_mac.update(actual.as_bytes());
    expected_mac
        .verify_slice(&actual_mac.finalize().into_bytes())
        .map_err(|_| "invalid DoorDash authorization")
}
