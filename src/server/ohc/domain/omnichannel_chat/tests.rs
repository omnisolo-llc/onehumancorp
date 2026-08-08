#[cfg(test)]
mod tests {
    use crate::domain::omnichannel_chat::models::*;
    use crate::domain::omnichannel_chat::engine::*;
    use crate::domain::omnichannel_chat::handlers::*;
    use serde_json::json;

    #[test]
    fn test_identity_resolution() {
        let mut engine = CustomerIdentityResolutionEngine::new();
        let customer = Customer {
            id: "c1".to_string(),
            tenant_id: "t1".to_string(),
            primary_email: Some("test@test.com".to_string()),
            instagram_handle: Some("test_insta".to_string()),
            whatsapp_number: None,
        };
        engine.add_customer(customer);

        let resolved = engine.resolve_identity("instagram", "test_insta");
        assert!(resolved.is_some());
        assert_eq!(resolved.unwrap().id, "c1");

        let unresolved = engine.resolve_identity("whatsapp", "123");
        assert!(unresolved.is_none());
    }

    #[test]
    fn test_omnichannel_gateway() {
        let mut identity_engine = CustomerIdentityResolutionEngine::new();
        identity_engine.add_customer(Customer {
            id: "c1".to_string(),
            tenant_id: "t1".to_string(),
            primary_email: None,
            instagram_handle: Some("sarah_bakes".to_string()),
            whatsapp_number: None,
        });

        let agent = AmbassadorAgent::new();
        let mut gateway = OmnichannelGateway::new(identity_engine, agent);

        let payload = json!({
            "channel": "instagram",
            "handle": "sarah_bakes",
            "content": "Do you have vegan cake?",
            "tenant_id": "t1"
        });

        let result = gateway.receive_webhook(&payload);
        assert!(result.is_ok());
    }
}
