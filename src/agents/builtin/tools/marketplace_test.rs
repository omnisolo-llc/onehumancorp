use ohc_builtin_agent_tools::marketplace::{HttpMarketplaceProvider, MarketplaceProvider, MarketplaceAgent};
use std::time::Duration;

#[tokio::test]
async fn test_http_marketplace_provider_bad_url_search() {
    let provider = HttpMarketplaceProvider::new("http://127.0.0.1:0"); // Port 0 is invalid
    let err = provider.search("rust").await.unwrap_err();
    assert!(err.contains("Failed to search marketplace"));
}

#[tokio::test]
async fn test_http_marketplace_provider_bad_url_fetch() {
    let provider = HttpMarketplaceProvider::new("http://127.0.0.1:0");
    let err = provider.fetch_agent("agent-1").await.unwrap_err();
    assert!(err.contains("Failed to fetch agent"));
}

#[tokio::test]
async fn test_http_marketplace_provider_bad_url_publish() {
    let provider = HttpMarketplaceProvider::new("http://127.0.0.1:0");
    let agent = MarketplaceAgent {
        id: "mock".to_string(),
        name: "mock".to_string(),
        description: "mock".to_string(),
        author: "mock".to_string(),
        version: "mock".to_string(),
        endpoint: "mock".to_string(),
    };
    let err = provider.publish_agent(agent).await.unwrap_err();
    assert!(err.contains("Failed to publish agent"));
}
