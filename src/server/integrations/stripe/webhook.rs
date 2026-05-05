use hmac::{Hmac, Mac};
use sha2::Sha256;
use hex;

pub fn verify_signature(payload: &str, signature_header: &str, secret: &str) -> bool {
    let mut timestamp = "";
    let mut v1_sig = "";

    for part in signature_header.split(',') {
        let mut kv = part.splitn(2, '=');
        let key = kv.next().unwrap_or("");
        let value = kv.next().unwrap_or("");
        if key == "t" {
            timestamp = value;
        } else if key == "v1" {
            v1_sig = value;
        }
    }

    if timestamp.is_empty() || v1_sig.is_empty() {
        return false;
    }

    let signed_payload = format!("{}.{}", timestamp, payload);

    let mut mac = match Hmac::<Sha256>::new_from_slice(secret.as_bytes()) {
        Ok(m) => m,
        Err(_) => return false,
    };

    mac.update(signed_payload.as_bytes());
    let expected_sig = hex::encode(mac.finalize().into_bytes());

    expected_sig == v1_sig
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_verify_signature_valid() {
        let secret = "whsec_test_secret";
        let payload = r#"{"type":"customer.subscription.updated"}"#;
        let timestamp = "1614560000";

        let signed_payload = format!("{}.{}", timestamp, payload);
        let mut mac = Hmac::<Sha256>::new_from_slice(secret.as_bytes()).unwrap();
        mac.update(signed_payload.as_bytes());
        let expected_sig = hex::encode(mac.finalize().into_bytes());

        let signature_header = format!("t={},v1={}", timestamp, expected_sig);

        assert!(verify_signature(payload, &signature_header, secret));
    }

    #[test]
    fn test_verify_signature_invalid() {
        let secret = "whsec_test_secret";
        let payload = r#"{"type":"customer.subscription.updated"}"#;
        let signature_header = "t=1614560000,v1=invalid_signature";

        assert!(!verify_signature(payload, &signature_header, secret));
    }
}
