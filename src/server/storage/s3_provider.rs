use async_trait::async_trait;
use chrono::Utc;
use std::io;
use super::provider::{BlobMetadata, Provider};
use crate::billing::Tracker;

pub struct S3Provider {
    bucket_name: String,
    tracker: Tracker,
}

impl S3Provider {
    pub fn new(bucket_name: String) -> Self {
        S3Provider { bucket_name, tracker: Tracker::new() }
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
        // STUB: Return a fake presigned URL
        Ok(format!("https://s3.amazonaws.com/{}/{}?X-Amz-Signature=stub", self.bucket_name, key))
    }

    async fn read_blob(&self, _key: &str) -> io::Result<Vec<u8>> {
        // STUB
        Ok(vec![])
    }

    async fn write_blob(&self, key: &str, data: &[u8]) -> io::Result<()> {
        let mut key_str = key.to_string();

        // Auto-optimization for images: Resize and convert to WebP
        let extension = std::path::Path::new(key).extension().and_then(|e| e.to_str()).unwrap_or("");
        let reported_size = if ::server_pricing::compression::is_image_extension(extension) && data.len() > 1024 {
            let original_size = data.len();
            match ::server_pricing::compression::optimize_image(data, 1024) {
                Ok((optimized_data, _)) => {
                    let final_data = optimized_data;
                    key_str = ::server_pricing::compression::get_optimized_key(key);
                    let compressed_size = final_data.len();
                    tracing::info!(
                        key = %key_str,
                        original = original_size,
                        actual_compressed = compressed_size,
                        saved = original_size - compressed_size,
                        "S3Provider: Auto-optimized image to WebP via compression utility"
                    );
                    compressed_size
                }
                Err(e) => {
                    tracing::warn!("S3Provider: Image optimization failed for {}: {}. Saving original.", key, e);
                    original_size
                }
            }
        } else {
            data.len()
        };

        let t_id = key_str.split('/').next().unwrap_or("default");
        let agent_id = key_str.split('/').nth(1);
        if let Ok(status) = self.tracker.track_storage_usage(t_id, reported_size as i64, agent_id).await {
            if status.soft_limit_reached {
                if let Some(msg) = status.user_message {
                    tracing::warn!(tid = %t_id, "Storage quota warning: {}", msg);
                }
            }
        }

        let _ = ::server_telemetry::record_storage_rw_cost(
            &crate::db::get_pool(),
            t_id,
            "write",
            reported_size as i64
        ).await;

        Ok(())
    }
}
