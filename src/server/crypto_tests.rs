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
