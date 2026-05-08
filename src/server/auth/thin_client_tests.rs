#[cfg(test)]
mod thin_client_auth_tests {
    use crate::auth::Store;
    use chrono::Utc;

    #[tokio::test]
    async fn test_oauth_thin_client_token_revocation() {
        unsafe { std::env::set_var("OHC_SQLITE_KEY", "dummy"); }
        let s = Store::new();
        let u = s.create_user("tc-user".to_string(), "tc@test.com".to_string(), "tcpass1".to_string(), vec![], "tc-org".to_string()).unwrap();
        let token = s.issue_token(&u).unwrap();

        let claims = s.validate_token(&token).await.unwrap();

        // Ensure valid initially
        assert!(s.validate_token(&token).await.is_ok());

        // Revoke token
        s.revoke_token(claims.jti.clone(), Utc::now() + chrono::Duration::hours(24), "tc-org");

        // Ensure invalid after revocation
        assert!(s.validate_token(&token).await.is_err());
    }
}
