pub mod models;
pub mod gateway;
pub mod dispatcher;

#[cfg(test)]
mod tests {
    use super::gateway::OmnichannelGateway;
    use uuid::Uuid;

    #[tokio::test]
    async fn test_ingest_webhook() {
        let gateway = OmnichannelGateway::new();
        let tenant_id = Uuid::new_v4();
        let msg = gateway.ingest_webhook(tenant_id, "instagram", "sarah123", "Do you have vegan cakes?").await;
        assert!(msg.is_ok());
        let msg = msg.unwrap();
        assert_eq!(msg.message_type, "incoming");
        assert_eq!(msg.content, "Do you have vegan cakes?");
    }
}
