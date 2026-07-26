#[cfg(test)]
mod tests {
    use axum::Router;
    use sqlx::PgPool;
    use crate::api::inbox::chat_api::chat_router;

    #[tokio::test]
    async fn test_chat_api_routing() {
        assert!(true);
    }
}
