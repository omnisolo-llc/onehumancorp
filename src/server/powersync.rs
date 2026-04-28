use std::sync::RwLock;
use serde_json::json;
use ed25519_dalek::{SigningKey, Signer};
use base64::{Engine as _, engine::general_purpose};
use chrono::{Utc, Duration};

static POWER_SYNC_KEY: RwLock<Option<(SigningKey, ed25519_dalek::VerifyingKey)>> = RwLock::new(None);

fn get_powersync_keys() -> (SigningKey, ed25519_dalek::VerifyingKey) {
    let mut cache = POWER_SYNC_KEY.write().unwrap();
    if let Some(keys) = &*cache {
        return keys.clone();
    }

    let keys = if let Ok(seed_b64) = std::env::var("OHC_POWERSYNC_PRIV_KEY") {
        if let Ok(seed) = general_purpose::STANDARD.decode(seed_b64) {
            if seed.len() == 32 {
                let mut seed_arr = [0u8; 32];
                seed_arr.copy_from_slice(&seed);
                let signing_key = SigningKey::from_bytes(&seed_arr);
                let verifying_key = signing_key.verifying_key();
                (signing_key, verifying_key)
            } else {
                generate_random_keys()
            }
        } else {
            generate_random_keys()
        }
    } else {
        generate_random_keys()
    };

    *cache = Some(keys.clone());
    keys
}

fn generate_random_keys() -> (SigningKey, ed25519_dalek::VerifyingKey) {
    let mut cspring = rand::rngs::OsRng;
    let signing_key = SigningKey::generate(&mut cspring);
    let verifying_key = signing_key.verifying_key();
    (signing_key, verifying_key)
}

pub fn get_powersync_jwks() -> serde_json::Value {
    let (_, pub_key) = get_powersync_keys();
    let x = general_purpose::URL_SAFE_NO_PAD.encode(pub_key.as_bytes());

    json!({
        "keys": [
            {
                "kty": "OKP",
                "crv": "Ed25519",
                "use": "sig",
                "kid": "powersync-key-1",
                "x": x,
            }
        ]
    })
}

pub fn generate_powersync_token(sub: &str, tenant_id: &str) -> Result<String, String> {
    let (priv_key, _) = get_powersync_keys();

    let now = Utc::now();
    let exp = (now + Duration::hours(24)).timestamp();

    let claims = json!({
        "iss": "ohc-backend",
        "sub": sub,
        "aud": "powersync",
        "iat": now.timestamp(),
        "exp": exp,
        "tenant_id": tenant_id,
    });

    let hdr = json!({
        "alg": "EdDSA",
        "typ": "JWT",
        "kid": "powersync-key-1",
    });

    let hdr_bytes = serde_json::to_vec(&hdr).map_err(|e| e.to_string())?;
    let claims_bytes = serde_json::to_vec(&claims).map_err(|e| e.to_string())?;

    let sig_input = format!("{}.{}", 
        general_purpose::URL_SAFE_NO_PAD.encode(hdr_bytes),
        general_purpose::URL_SAFE_NO_PAD.encode(claims_bytes)
    );

    let signature = priv_key.sign(sig_input.as_bytes());

    let token = format!("{}.{}", sig_input, general_purpose::URL_SAFE_NO_PAD.encode(signature.to_bytes()));

    Ok(token)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_powersync_jwks() {
        let jwks = get_powersync_jwks();
        let keys = jwks.get("keys").and_then(|k| k.as_array()).expect("expected keys array");
        assert_eq!(keys.len(), 1);
        let key = keys[0].as_object().expect("expected key object");
        assert_eq!(key.get("kty").and_then(|v| v.as_str()), Some("OKP"));
        assert_eq!(key.get("crv").and_then(|v| v.as_str()), Some("Ed25519"));
        assert_eq!(key.get("kid").and_then(|v| v.as_str()), Some("powersync-key-1"));
        assert!(key.contains_key("x"));
    }

    #[test]
    fn test_generate_powersync_token() {
        let token = generate_powersync_token("user-1", "org-1").unwrap();
        assert!(!token.is_empty());
        
        let parts: Vec<&str> = token.split('.').collect();
        assert_eq!(parts.len(), 3);
    }
}
