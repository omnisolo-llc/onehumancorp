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
