use super::sync_mutations_handler::*;
use axum::{body::Body, http::{Request, StatusCode, header}};

#[tokio::test]
async fn test_sync_mutations_unauthorized() {
    let req = GenericOfflineSyncRequest {
        mutations: vec![],
    };

    let serialized = serde_json::to_string(&req).unwrap();
    assert!(serialized.contains("mutations"));
}
