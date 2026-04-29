
use super::pubsub::PubSubManager;
use std::env;

#[tokio::test]
async fn test_standalone_pubsub() {
    env::set_var("OHC_MULTITENANT", "false");

    let manager = PubSubManager::new();
    let topic = "test_topic";

    let mut rx = manager.subscribe(topic).await.unwrap();

    manager.publish(topic, b"hello world".to_vec()).await.unwrap();

    let msg = rx.recv().await.unwrap();
    assert_eq!(msg, b"hello world");
}

#[tokio::test]
async fn test_cloud_pubsub_no_redis() {
    env::set_var("OHC_MULTITENANT", "true");
    env::remove_var("REDIS_URL");

    let manager = PubSubManager::new();
    let topic = "test_topic";

    let err = manager.subscribe(topic).await.unwrap_err();
    assert_eq!(err, "Redis client not initialized");

    let err = manager.publish(topic, b"hello world".to_vec()).await.unwrap_err();
    assert_eq!(err, "Redis client not initialized");
}
