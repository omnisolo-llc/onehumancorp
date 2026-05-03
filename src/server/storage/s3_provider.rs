use async_trait::async_trait;
use chrono::Utc;
use std::io;
use super::provider::{BlobMetadata, Provider};

pub struct S3Provider {
    bucket_name: String,
    redis_client: Option<redis::Client>,
}

impl S3Provider {
    pub fn new(bucket_name: String) -> Self {
        let redis_client = std::env::var("REDIS_URL").ok().and_then(|url| redis::Client::open(url).ok());
        S3Provider { bucket_name, redis_client }
    }
}

#[async_trait]
impl Provider for S3Provider {
    fn is_local(&self) -> bool {
        false
    }

    async fn list_blobs(&self, _prefix: &str) -> io::Result<Vec<BlobMetadata>> {
        // STUB
        Ok(vec![])
    }

    async fn read_blob_metadata(&self, key: &str) -> io::Result<BlobMetadata> {
        // STUB
        Ok(BlobMetadata {
            key: key.to_string(),
            size: 1024,
            last_modified: Utc::now(),
            content_type: "application/octet-stream".to_string(),
        })
    }

    async fn get_blob_url(&self, key: &str) -> io::Result<String> {
        if let Ok(cdn_url) = std::env::var("CDN_URL") {
            let cdn_url = cdn_url.trim_end_matches('/');
            Ok(format!("{}/{}", cdn_url, key))
        } else {
            // STUB: Return a fake presigned URL
            Ok(format!("https://s3.amazonaws.com/{}/{}?X-Amz-Signature=stub", self.bucket_name, key))
        }
    }

    async fn read_blob(&self, _key: &str) -> io::Result<Vec<u8>> {
        // STUB
        Ok(vec![])
    }

    async fn write_blob(&self, tenant_id: &str, key: &str, data: &[u8]) -> io::Result<String> {
        let mut final_data = data.to_vec();
        let mut final_key = key.to_string();

        if key.ends_with(".png") || key.ends_with(".jpg") || key.ends_with(".jpeg") {
            let data_clone = data.to_vec();
            if let Ok(Some(webp_data)) = tokio::task::spawn_blocking(move || {
                if let Ok(img) = image::load_from_memory(&data_clone) {
                    if let Ok(encoder) = webp::Encoder::from_image(&img) {
                        return Some(encoder.encode(80.0).to_vec());
                    }
                }
                None
            }).await {
                final_data = webp_data;
                let p = std::path::Path::new(key);
                if let Some(stem) = p.file_stem() {
                    if let Some(parent) = p.parent() {
                        let new_path = parent.join(format!("{}.webp", stem.to_string_lossy()));
                        final_key = new_path.to_string_lossy().to_string();
                    } else {
                        final_key = format!("{}.webp", stem.to_string_lossy());
                    }
                }
            }
        }

        if let Some(client) = &self.redis_client {
            let limiter = crate::pricing::rate_limit::RedisRateLimiter::new(client.clone());
            match limiter.check_storage_quota(tenant_id, final_data.len() as i64).await {
                Ok(status) => {
                    if status.soft_limit_reached {
                        return Err(io::Error::new(io::ErrorKind::Other, format!("Storage quota exceeded: {}", status.user_message.unwrap_or_default())));
                    }
                }
                Err(e) => eprintln!("Failed to check storage quota: {}", e),
            }
        }

        // STUB (Would write final_data to S3 here using final_key)

        if let Some(client) = &self.redis_client {
            let limiter = crate::pricing::rate_limit::RedisRateLimiter::new(client.clone());
            let _ = limiter.record_storage_used(tenant_id, final_data.len() as i64).await;
        }

        Ok(final_key)
    }
}
