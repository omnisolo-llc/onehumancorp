use crate::integrations::registry::IntegrationsRegistry;
use ::server_ohc::orchestration::*;
use std::collections::HashMap;

#[tokio::test]
async fn test_tenant_isolation_across_providers() {
    let registry = IntegrationsRegistry::new();

    // Tenant A connects Meta
    let creds_a = ConnectIntegrationRequest {
        integration_id: "meta".to_string(),
        base_url: "https://graph.facebook.com".to_string(),
        bot_token: "token_a".to_string(),
        chat_id: "".to_string(),
        webhook_url: "".to_string(),
        api_token: "".to_string(),
        from_phone: "".to_string(),
    };
    registry.connect("tenant_a", "meta", "https://graph.facebook.com", creds_a).unwrap();

    // Tenant B connects Meta with DIFFERENT token
    let creds_b = ConnectIntegrationRequest {
        integration_id: "meta".to_string(),
        base_url: "https://graph.facebook.com".to_string(),
        bot_token: "token_b".to_string(),
        chat_id: "".to_string(),
        webhook_url: "".to_string(),
        api_token: "".to_string(),
        from_phone: "".to_string(),
    };
    registry.connect("tenant_b", "meta", "https://graph.facebook.com", creds_b).unwrap();

    // Send messages for both
    registry.send_chat_message("tenant_a", "meta", "user1", "agent", "Msg A", "t1").unwrap();
    registry.send_chat_message("tenant_b", "meta", "user2", "agent", "Msg B", "t2").unwrap();

    // Verify A cannot see B's messages
    let msgs_a = registry.chat_messages("tenant_a", "meta");
    assert_eq!(msgs_a.len(), 1);
    assert_eq!(msgs_a[0].content, "Msg A");

    let msgs_b = registry.chat_messages("tenant_b", "meta");
    assert_eq!(msgs_b.len(), 1);
    assert_eq!(msgs_b[0].content, "Msg B");

    // Verify catalog discovery respects connection status per tenant
    let insts_a = registry.instances_by_category("tenant_a", "social_media");
    let meta_a = insts_a.iter().find(|i| i.id == "meta").unwrap();
    assert_eq!(meta_a.status, "connected");

    // Let's assume tenant_c hasn't connected anything
    let insts_c = registry.instances_by_category("tenant_c", "social_media");
    let meta_c = insts_c.iter().find(|i| i.id == "meta").unwrap();
    assert_eq!(meta_c.status, "disconnected");
}
