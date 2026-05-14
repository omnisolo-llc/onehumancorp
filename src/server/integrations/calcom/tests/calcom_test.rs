use crate::integrations::calcom::client::{CalComClientWrapper, RealCalComClient};
use crate::integrations::calcom::provider::CalComProvider;
use std::sync::Arc;

#[tokio::test]
async fn test_calcom_sync() {
    let client = RealCalComClient::new("test_key".to_string());
    let result = client.sync_calendar("user_123", "available").await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_calcom_provider_initialization() {
    let provider = CalComProvider::new("test_key".to_string());
    assert_eq!(provider.metadata.id, "calcom");
    assert_eq!(provider.metadata.name, "Cal.com Integration");
}
