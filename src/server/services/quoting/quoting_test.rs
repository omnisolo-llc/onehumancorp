#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quote_request_serializes() {
        let req = QuoteRequest {
            customer_id: Uuid::new_v4(),
            message: "Need a quote".to_string(),
        };
        let serialized = serde_json::to_string(&req).unwrap();
        assert!(serialized.contains("Need a quote"));
    }
}
