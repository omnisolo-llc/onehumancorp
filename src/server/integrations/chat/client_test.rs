use super::client::{ChannelAdapter, WebWidgetAdapter};

#[tokio::test]
async fn test_client_adapter() {
    let adapter = WebWidgetAdapter {
        connected_clients: std::sync::Arc::new(std::collections::HashMap::new()),
    };
    let result = adapter.send_message("user_123", "Hello World").await;
    assert!(result.is_ok());
}
