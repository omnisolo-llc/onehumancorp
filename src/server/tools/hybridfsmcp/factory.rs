use std::env;
use std::path::PathBuf;
use std::sync::Arc;
use super::provider::{CloudFSProvider, FileSystemProvider, LocalFSProvider};

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
            is_standalone: crate::is_standalone_runtime(),
            mount_point: env::var("OHC_CLOUD_FS_MOUNT").unwrap_or_else(|_| "/mnt/data/tenant_volumes".to_string()),
            workspace: env::var("OHC_LOCAL_WORKSPACE").unwrap_or_else(|_| "./workspace".to_string()),
        }
    }
}

pub fn create_fs_provider_with_config(config: &FactoryConfig, tenant_id: Option<String>) -> Arc<dyn FileSystemProvider> {
    if config.is_multitenant && !config.is_standalone {
        let tenant = tenant_id.unwrap_or_else(|| ::server_common::auth_utils::get_default_tenant());
        if tenant == "system" || tenant.trim().is_empty() {
            ::server_telemetry::record_error_signal("[bug] Invalid tenant_id for cloud fs provider.");
            tracing::error!("Invalid tenant_id for cloud fs provider."); // pii-safe
            return Arc::new(LocalFSProvider::new(PathBuf::from("/dev/null")));
        }
        Arc::new(CloudFSProvider::new(tenant, PathBuf::from(&config.mount_point)))
    } else {
        Arc::new(LocalFSProvider::new(PathBuf::from(&config.workspace)))
    }
}

pub fn create_fs_provider(tenant_id: Option<String>) -> Arc<dyn FileSystemProvider> {
    create_fs_provider_with_config(&FactoryConfig::default(), tenant_id)
}
