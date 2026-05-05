use aes_gcm::{
    aead::{Aead, KeyInit},
    Aes256Gcm, Nonce
};
use hmac::{Hmac, Mac};
use sha2::{Sha256, Digest};
use base64::{Engine as _, engine::general_purpose};

fn get_crypto_key() -> [u8; 32] {
    let key = std::env::var("OHC_SQLITE_KEY")
        .unwrap_or_else(|_| std::env::var("OHC_SQLITE_ENCRYPTION_KEY").unwrap_or_else(|_| {
            if std::env::var("STANDALONE_MODE").unwrap_or_default() == "true" {
                "standalone_ephemeral_key".to_string()
            } else {
                "transient_memory_key".to_string()
            }
        }));
    
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
        // SAFETY: This is only called in test context and the key is a constant
        unsafe { std::env::set_var("OHC_SQLITE_KEY", "test_key") };
        let plaintext = "hello world";
        let ciphertext1 = encrypt_deterministic(plaintext);
        let ciphertext2 = encrypt_deterministic(plaintext);
        
        assert_eq!(ciphertext1, ciphertext2, "Encryption must be deterministic");
        
        let decrypted = decrypt_deterministic(&ciphertext1);
        assert_eq!(decrypted, plaintext, "Decryption must match plaintext");
    }
}
