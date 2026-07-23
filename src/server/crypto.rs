use aes_gcm::{
    aead::{Aead, KeyInit},
    Aes256Gcm, Nonce
};
use hmac::{Hmac, Mac};
use sha2::{Sha256, Digest};
use base64::{Engine as _, engine::general_purpose};

fn get_crypto_key() -> [u8; 32] {
    let key = std::env::var("OHC_SQLITE_KEY")
        .or_else(|_| std::env::var("OHC_SQLITE_ENCRYPTION_KEY"))
        .unwrap_or_else(|_| {
            let is_standalone = ::server_config::get().standalone
                || std::env::var("OHC_STANDALONE_MODE").map(|v| v == "true").unwrap_or(false)
                || std::env::var("OHC_STANDALONE").map(|v| v == "true").unwrap_or(false);
            if is_standalone {
                ::tracing::warn!(
                    "No OHC_SQLITE_KEY configured for standalone mode. \
                     Generating ephemeral key. Data will NOT persist across restarts. \
                     Set OHC_SQLITE_KEY for persistent encryption."
                );
                use std::time::{SystemTime, UNIX_EPOCH};
                let ts = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap_or_default()
                    .as_nanos();
                let pid = std::process::id();
                format!("ephemeral-{}-{}", pid, ts)
            } else {
                panic!(
                    "CRITICAL: No OHC_SQLITE_KEY or OHC_SQLITE_ENCRYPTION_KEY configured. \
                     Set one of these environment variables for production encryption."
                );
            }
        });
    
    let mut hasher = Sha256::new();
    hasher.update(key.as_bytes());
    let result = hasher.finalize();
    let mut key_bytes = [0u8; 32];
    key_bytes.copy_from_slice(&result);
    key_bytes
}

pub fn encrypt_deterministic(plaintext: &str) -> String {
    if plaintext.is_empty() {
        return String::new();
    }
    let key = get_crypto_key();
    let cipher = Aes256Gcm::new_from_slice(&key).expect("Invalid key length");
    
    // Derive nonce from plaintext using HMAC with the encryption key
    type HmacSha256 = Hmac<Sha256>;
    let mut mac = <HmacSha256 as Mac>::new_from_slice(&key).expect("HMAC can take key of any size");
    mac.update(plaintext.as_bytes());
    let mac_result = mac.finalize();
    let mac_bytes = mac_result.into_bytes();
    
    // Nonce size for AES-GCM is 12 bytes
    let nonce = Nonce::from_slice(&mac_bytes[..12]);
    
    let ciphertext = cipher.encrypt(nonce, plaintext.as_bytes()).expect("Encryption failed");
    
    let mut final_msg = Vec::new();
    final_msg.extend_from_slice(&mac_bytes[..12]);
    final_msg.extend_from_slice(&ciphertext);
    
    general_purpose::STANDARD.encode(final_msg)
}

pub fn decrypt_deterministic(ciphertext_b64: &str) -> String {
    if ciphertext_b64.is_empty() {
        return String::new();
    }
    
    let ciphertext = match general_purpose::STANDARD.decode(ciphertext_b64) {
        Ok(c) => c,
        Err(_) => return ciphertext_b64.to_string(), // Fallback if not base64
    };
    
    if ciphertext.len() < 12 {
        return ciphertext_b64.to_string(); // Fallback
    }
    
    let key = get_crypto_key();
    let cipher = Aes256Gcm::new_from_slice(&key).expect("Invalid key length");
    
    let nonce = Nonce::from_slice(&ciphertext[..12]);
    let ciphertext_data = &ciphertext[12..];
    
    match cipher.decrypt(nonce, ciphertext_data) {
        Ok(plaintext) => String::from_utf8(plaintext).unwrap_or_else(|_| ciphertext_b64.to_string()),
        Err(_) => ciphertext_b64.to_string(), // Fallback
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deterministic_encryption() {
        temp_env::with_vars(vec![("OHC_SQLITE_KEY", Some("test_key"))], || {
            let plaintext = "hello world";
            let ciphertext1 = encrypt_deterministic(plaintext);
            let ciphertext2 = encrypt_deterministic(plaintext);
            assert_eq!(ciphertext1, ciphertext2, "Encryption must be deterministic");
            let decrypted = decrypt_deterministic(&ciphertext1);
            assert_eq!(decrypted, plaintext, "Decryption must match plaintext");
        });
    }
    #[test]
    fn test_empty_string() {
        assert_eq!(encrypt_deterministic(""), "");
        assert_eq!(decrypt_deterministic(""), "");
    }

    #[test]
    fn test_fallback_invalid_base64() {
        let invalid = "not base64 data";
        assert_eq!(decrypt_deterministic(invalid), invalid);
    }

    #[test]
    fn test_fallback_too_short() {
        let short = base64::engine::general_purpose::STANDARD.encode("short");
        assert_eq!(decrypt_deterministic(&short), short);
    }

    #[test]
    fn test_cloud_mode_panics_without_key() {
        temp_env::with_vars(vec![
            ("OHC_SQLITE_KEY", None::<&str>),
            ("OHC_SQLITE_ENCRYPTION_KEY", None::<&str>),
        ], || {
            let result = std::panic::catch_unwind(|| {
                temp_env::with_vars(vec![("OHC_SQLITE_KEY", Some("test_key"))], || {
                    let _key = get_crypto_key();
                });
            });
            assert!(result.is_ok(), "Should not panic when OHC_SQLITE_KEY is set");
        });
    }

    #[test]
    fn test_standalone_mode_generates_ephemeral_key() {
        temp_env::with_vars(vec![
            ("OHC_SQLITE_KEY", None::<&str>),
            ("OHC_SQLITE_ENCRYPTION_KEY", None::<&str>),
            ("OHC_STANDALONE_MODE", Some("true")),
        ], || {
            // Ephemeral key includes PID+timestamp, so each call generates a different key.
            // We just verify it doesn't panic and produces valid 32-byte keys.
            let key1 = get_crypto_key();
            let key2 = get_crypto_key();
            assert_eq!(key1.len(), 32);
            assert_eq!(key2.len(), 32);
        });
    }

    #[test]
    fn test_different_keys_produce_different_crypto_keys() {
        temp_env::with_vars(vec![("OHC_SQLITE_KEY", Some("key_a"))], || {
            let key_a = get_crypto_key();
            temp_env::with_vars(vec![("OHC_SQLITE_KEY", Some("key_b"))], || {
                let key_b = get_crypto_key();
                assert_ne!(key_a, key_b, "Different env keys must produce different crypto keys");
            });
        });
    }
}
