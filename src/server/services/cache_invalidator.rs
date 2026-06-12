
use futures::StreamExt;
use serde::Deserialize;
use tracing::{info, error, warn};

#[derive(Deserialize, Debug)]
pub struct InvalidationEvent {
    pub event: String,
    pub tags: Vec<String>,
}

pub async fn start_cache_invalidator() {
    let redis_url = match std::env::var("REDIS_URL") {
        Ok(url) => url,
        Err(_) => {
            warn!("REDIS_URL not set, Cache Invalidator Service will not start.");
            return;
        }
    };

    let client = match redis::Client::open(redis_url.clone()) {
        Ok(c) => c,
        Err(e) => {
            error!("Failed to create Redis client for Cache Invalidator: {}", e);
            return;
        }
    };

    let mut pubsub_conn = match client.get_async_pubsub().await {
        Ok(conn) => conn,
        Err(e) => {
            error!("Failed to get Redis pubsub connection: {}", e);
            return;
        }
    };

    if let Err(e) = pubsub_conn.subscribe("cache_invalidation_events").await {
        error!("Failed to subscribe to cache_invalidation_events: {}", e);
        return;
    }

    info!("Cache Invalidator Service started, listening on cache_invalidation_events");

    let mut stream = pubsub_conn.on_message();

    let edge_cache = crate::builder::edge::get_edge_cache();

    while let Some(msg) = stream.next().await {
        let payload: String = match msg.get_payload() {
            Ok(p) => p,
            Err(e) => {
                error!("Failed to get message payload: {}", e);
                continue;
            }
        };

        match serde_json::from_str::<InvalidationEvent>(&payload) {
            Ok(event) => {
                info!("Received invalidation event: {}", event.event);
                for tag in event.tags {
                    info!("Invalidating cache for tag: {}", tag);
                    edge_cache.invalidate_by_tag(&tag).await;
                }
            }
            Err(e) => {
                error!("Failed to parse invalidation event: {}", e);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[tokio::test]
    async fn test_cache_invalidator_event_parsing() {
        let payload = r#"{"event": "inventory.updated", "tags": ["tenant-id:123", "entity:product:456"]}"#;
        let event: Result<InvalidationEvent, _> = serde_json::from_str(payload);
        assert!(event.is_ok());
        let event = event.unwrap();
        assert_eq!(event.event, "inventory.updated");
        assert_eq!(event.tags.len(), 2);
        assert_eq!(event.tags[0], "tenant-id:123");
        assert_eq!(event.tags[1], "entity:product:456");
    }

    #[tokio::test]
    async fn test_cache_invalidator_cache_invalidation() {
        // Here we test just the logic applied in the stream loop.
        let edge_cache = crate::builder::edge::get_edge_cache();

        let tag = "test_tag_123";
        edge_cache.set_with_tags("test_key", "test_val".to_string(), vec![tag.to_string()], Duration::from_secs(60)).await;

        assert_eq!(edge_cache.get("test_key").await, Some("test_val".to_string()));

        let event = InvalidationEvent {
            event: "test.event".to_string(),
            tags: vec![tag.to_string()],
        };

        for tag in event.tags {
            edge_cache.invalidate_by_tag(&tag).await;
        }

        assert_eq!(edge_cache.get("test_key").await, None);
    }
}
