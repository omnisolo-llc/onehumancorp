use futures::StreamExt;
use serde::Deserialize;
use tracing::{info, error, warn};

#[derive(Deserialize, Debug)]
pub struct InvalidationEvent {
    pub event: String,
    pub tags: Vec<String>,
}

pub async fn start_cache_invalidator(pool: sqlx::PgPool) {
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
    let http_client = reqwest::Client::new();
    // Default to edge-cache:80 which is our docker-compose service name.
    let edge_cache_url = std::env::var("OHC_EDGE_CACHE_URL").unwrap_or_else(|_| "http://edge-cache:80/purge".to_string());

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
                let mut tenant_id_str = None;
                let mut product_id_str = None;

                for tag in &event.tags {
                    info!("Invalidating cache for tag: {}", tag);
                    if tag.starts_with("tenant-id:") {
                        tenant_id_str = Some(tag.trim_start_matches("tenant-id:").to_string());
                    } else if tag.starts_with("entity:product:") {
                        product_id_str = Some(tag.trim_start_matches("entity:product:").to_string());
                    }
                    edge_cache.invalidate_by_tag(tag).await;
                    let cdn_cache = crate::utils::edge_caching_middleware::get_cdn_cache();
                    cdn_cache.invalidate_by_tag(tag).await;

                    // Send HTTP PURGE request to NGINX edge cache
                    let res = http_client.request(reqwest::Method::from_bytes(b"PURGE").unwrap_or(reqwest::Method::POST), &edge_cache_url)
                        .header("X-Cache-Tag", tag)
                        .send()
                        .await;

                    match res {
                        Ok(response) => {
                            if !response.status().is_success() {
                                warn!("Failed to purge edge cache for tag {}: status {}", tag, response.status());
                            } else {
                                info!("Successfully purged edge cache for tag {}", tag);
                            }
                        }
                        Err(e) => {
                            warn!("Failed to send purge request to edge cache for tag {}: {}", tag, e);
                        }
                    }
                }

                if let (Some(t_str), Some(p_str)) = (tenant_id_str, product_id_str) {
                    if let (Ok(tenant_id), Ok(product_id)) = (uuid::Uuid::parse_str(&t_str), uuid::Uuid::parse_str(&p_str)) {
                        let site_id_res = sqlx::query_scalar::<_, uuid::Uuid>(
                            "SELECT id FROM builder_sites WHERE tenant_id = $1 ORDER BY created_at ASC LIMIT 1"
                        )
                        .bind(tenant_id)
                        .fetch_one(&pool)
                        .await;

                        if let Ok(_site_id) = site_id_res {
                            info!("Pre-warming cache for product: {} tenant: {}", product_id, tenant_id);
                            let cache_key = format!("storefront:product:{}:{}", tenant_id, product_id);
                            let _ = crate::builder::edge::regenerate_product_cache(pool.clone(), tenant_id, product_id, cache_key, edge_cache.clone()).await;
                        }
                    }
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
            let cdn_cache = crate::utils::edge_caching_middleware::get_cdn_cache();
            cdn_cache.invalidate_by_tag(&tag).await;
        }

        assert_eq!(edge_cache.get("test_key").await, None);
    }
}
