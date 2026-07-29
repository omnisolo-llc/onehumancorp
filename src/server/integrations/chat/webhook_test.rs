use crate::webhook::WebhookPayload;
use uuid::Uuid;

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_webhook_payload_parsing() {
        let payload = r#"{"tenant_id":"123e4567-e89b-12d3-a456-426614174000","inbox_id":"123e4567-e89b-12d3-a456-426614174000","contact_id":"123e4567-e89b-12d3-a456-426614174000","content":"test message"}"#;
        let parsed: Result<WebhookPayload, _> = serde_json::from_str(payload);
        assert!(parsed.is_ok());
        let p = parsed.unwrap();
        assert_eq!(p.content, "test message");
        assert!(Uuid::parse_str(&p.tenant_id).is_ok());
    }
}
