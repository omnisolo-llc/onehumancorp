#[cfg(test)]
mod tests {
    use crate::integrations::stripe::webhooks::*;
    use sqlx::PgPool;

    #[tokio::test]
    async fn test_verify_signature() {
        assert!(verify_signature("payload", "signature", "secret"));
    }
}
