use std::sync::Arc;
use crate::hub::Hub;
use ::server_pricing::compression::optimize_image;

pub struct UploadService {
    hub: Arc<Hub>,
}

impl UploadService {
    pub fn new(hub: Arc<Hub>) -> Self {
        Self { hub }
    }

    pub async fn upload_product_photo(&self, tenant_id: &str, image_data: &[u8]) -> Result<String, String> {
        // 1. Optimize image (Resize to 800x800 max and convert to WebP)
        let optimized = optimize_image(image_data, 800, 800)?;

        let original_size = image_data.len() as i64;
        let optimized_size = optimized.len() as i64;

        // 2. Check storage quota
        let tracker = self.hub.tracker();
        let quota_status = tracker.track_storage_usage(tenant_id, optimized_size, None).await?;

        if !quota_status.is_allowed {
            return Err(quota_status.user_message.unwrap_or_else(|| "Storage quota exceeded".to_string()));
        }

        // 3. Record savings in auditor
        if let Some(auditor) = &tracker.auditor {
            auditor.record_storage_compression(original_size, optimized_size);
        }

        // 4. In a real implementation, we would upload to S3/CDN here.
        // For this task, we return a mock URL.
        let photo_id = uuid::Uuid::new_v4().to_string();
        let mock_url = format!("https://cdn.onehumancorp.com/products/{}.webp", photo_id);

        tracing::info!(
            "Product photo uploaded for tenant {}: {} bytes -> {} bytes (saved {}%)",
            tenant_id,
            original_size,
            optimized_size,
            (original_size - optimized_size) * 100 / original_size.max(1)
        );

        Ok(mock_url)
    }
}
