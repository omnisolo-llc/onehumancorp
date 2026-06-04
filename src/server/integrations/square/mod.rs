pub mod provider;
pub mod client;

#[cfg(test)]
mod tests {
    use super::provider::SquareProvider;
    use serde_json::json;

    #[tokio::test]
    async fn test_square_provider_initialization() {
        let provider = SquareProvider::new("test_token".to_string());
        let integration = provider.to_integration_provider();
        assert_eq!(integration.metadata.id, "square");
        assert_eq!(integration.metadata.category, "pos");
    }
}
