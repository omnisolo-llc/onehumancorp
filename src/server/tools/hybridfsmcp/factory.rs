use std::env;
use std::path::PathBuf;
use std::sync::Arc;
use super::provider::{S3BlobProvider, BlobProvider, LocalBlobProvider};

pub struct FactoryConfig {
    pub is_multitenant: bool,
    pub is_standalone: bool,
    pub mount_point: String,
    pub workspace: String,
}

impl Default for FactoryConfig {
    fn default() -> Self {
        Self {
            is_multitenant: env::var("OHC_MULTITENANT").unwrap_or_else(|_| "false".to_string()) == "true",
            is_standalone: env::var("OHC_STANDALONE").unwrap_or_else(|_| "false".to_string()) == "true",
            mount_point: env::var("OHC_CLOUD_FS_MOUNT").unwrap_or_else(|_| "ohc-multi-tenant-blobs".to_string()),
            workspace: env::var("OHC_LOCAL_WORKSPACE").unwrap_or_else(|_| "/var/tmp/ohc/blobs".to_string()),
        }
    }
}

pub fn create_fs_provider_with_config(config: &FactoryConfig, tenant_id: Option<String>) -> Arc<dyn BlobProvider> {
    if config.is_multitenant && !config.is_standalone {
        let tenant = tenant_id.unwrap_or_else(|| "system".to_string());
        Arc::new(S3BlobProvider::new(tenant, config.mount_point.clone()))
    } else {
        Arc::new(LocalBlobProvider::new(PathBuf::from(&config.workspace)))
    }
}

pub fn create_fs_provider(tenant_id: Option<String>) -> Arc<dyn BlobProvider> {
    create_fs_provider_with_config(&FactoryConfig::default(), tenant_id)
}
