use std::path::{Path, PathBuf};
use std::env;
use std::fs;
use std::sync::Arc;

use crate::auth::Claims;
use crate::storage::provider::Provider;

/// Cleans a relative path string and resolves it securely against the base path.
/// Blocks absolute paths and traversing above the root directory using `..`.
pub fn clean_path_str(base: &Path, rel_path: &str) -> Option<PathBuf> {
    if rel_path.starts_with("/") {
        return None;
    }
    let mut parts = Vec::new();
    for part in rel_path.split('/') {
        if part.is_empty() || part == "." {
            continue;
        }
        if part == ".." {
            if parts.is_empty() {
                return None; // Traversal beyond base
            }
            parts.pop();
        } else {
            parts.push(part);
        }
    }

    let mut resolved = base.to_path_buf();
    for part in parts {
        resolved.push(part);
    }
    Some(resolved)
}

/// Generates a clean S3 key for cloud multi-tenant storage.
/// Ensures no `..` traversal can escape the tenant boundary.
pub fn clean_s3_key(org_id: &str, rel_path: &str) -> Option<String> {
    if rel_path.starts_with("/") {
        return None;
    }
    let mut parts = Vec::new();
    for part in rel_path.split('/') {
        if part.is_empty() || part == "." {
            continue;
        }
        if part == ".." {
            if parts.is_empty() {
                return None;
            }
            parts.pop();
        } else {
            parts.push(part);
        }
    }

    let key = parts.join("/");
    Some(format!("tenant/{}/fs/{}", org_id, key))
}

pub struct FsMcpTool {
    is_standalone: bool,
    local_base_dir: PathBuf,
    storage_provider: Option<Arc<dyn Provider>>,
}

impl FsMcpTool {
    pub fn new(storage_provider: Option<Arc<dyn Provider>>) -> Self {
        let is_standalone = env::var("OHC_STANDALONE").unwrap_or_else(|_| "false".to_string()) == "true";
        let local_base_dir = if let Ok(home) = env::var("HOME") {
            PathBuf::from(home).join(".ohc-local-data/fs")
        } else {
            PathBuf::from("/tmp/.ohc-local-data/fs")
        };

        Self {
            is_standalone,
            local_base_dir,
            storage_provider,
        }
    }

    pub async fn read(&self, claims: &Claims, path: &str) -> Result<String, String> {
        if self.is_standalone {
            let full_path = clean_path_str(&self.local_base_dir, path).ok_or("Invalid path")?;
            fs::read_to_string(&full_path).map_err(|e| e.to_string())
        } else {
            let org_id = claims.organization_id.as_ref().ok_or("Organization ID required for cloud mode")?;
            let key = clean_s3_key(org_id, path).ok_or("Invalid path")?;
            if let Some(ref provider) = self.storage_provider {
                let data = provider.read_blob(&key).await.map_err(|e| e.to_string())?;
                String::from_utf8(data).map_err(|e| e.to_string())
            } else {
                Err("Storage provider not configured".to_string())
            }
        }
    }

    pub async fn write(&self, claims: &Claims, path: &str, content: &str) -> Result<(), String> {
        if self.is_standalone {
            let full_path = clean_path_str(&self.local_base_dir, path).ok_or("Invalid path")?;
            if let Some(parent) = full_path.parent() {
                let _ = fs::create_dir_all(parent);
            }
            fs::write(&full_path, content).map_err(|e| e.to_string())
        } else {
            let org_id = claims.organization_id.as_ref().ok_or("Organization ID required for cloud mode")?;
            let key = clean_s3_key(org_id, path).ok_or("Invalid path")?;
            if let Some(ref provider) = self.storage_provider {
                provider.write_blob(&key, content.as_bytes()).await.map_err(|e| e.to_string())
            } else {
                Err("Storage provider not configured".to_string())
            }
        }
    }

    pub async fn list(&self, claims: &Claims, path: &str) -> Result<Vec<String>, String> {
        if self.is_standalone {
            let full_path = clean_path_str(&self.local_base_dir, path).ok_or("Invalid path")?;
            let mut entries = Vec::new();
            if full_path.is_dir() {
                if let Ok(rd) = fs::read_dir(&full_path) {
                    for entry in rd.flatten() {
                        if let Some(name) = entry.file_name().to_str() {
                            entries.push(name.to_string());
                        }
                    }
                }
            }
            Ok(entries)
        } else {
            let org_id = claims.organization_id.as_ref().ok_or("Organization ID required for cloud mode")?;
            let key_prefix = clean_s3_key(org_id, path).ok_or("Invalid path")?;
            if let Some(ref provider) = self.storage_provider {
                let blobs = provider.list_blobs(&key_prefix).await.map_err(|e| e.to_string())?;
                let mut entries = Vec::new();
                for blob in blobs {
                    entries.push(blob.key.clone());
                }
                Ok(entries)
            } else {
                Err("Storage provider not configured".to_string())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_path_traversal() {
        let base = PathBuf::from("/base/dir");
        assert_eq!(clean_path_str(&base, "test.txt"), Some(PathBuf::from("/base/dir/test.txt")));
        assert_eq!(clean_path_str(&base, "folder/test.txt"), Some(PathBuf::from("/base/dir/folder/test.txt")));
        assert_eq!(clean_path_str(&base, "../test.txt"), None);
        assert_eq!(clean_path_str(&base, "folder/../../test.txt"), None);
        assert_eq!(clean_path_str(&base, "/test.txt"), None);
    }

    #[test]
    fn test_s3_key_clean() {
        assert_eq!(clean_s3_key("org1", "test.txt"), Some("tenant/org1/fs/test.txt".to_string()));
        assert_eq!(clean_s3_key("org1", "../test.txt"), None);
        assert_eq!(clean_s3_key("org1", "folder/test.txt"), Some("tenant/org1/fs/folder/test.txt".to_string()));
    }
}

#[cfg(test)]
mod additional_tests {
    use super::*;
    use async_trait::async_trait;
    use std::io;
    use crate::storage::provider::{Provider, BlobMetadata};
    use std::sync::Arc;
    use chrono::Utc;

    struct MockProvider;
    #[async_trait]
    impl Provider for MockProvider {
        fn is_local(&self) -> bool { false }
        async fn list_blobs(&self, prefix: &str) -> io::Result<Vec<BlobMetadata>> {
            if prefix == "tenant/org1/fs/testdir" {
                Ok(vec![
                   BlobMetadata {
                       key: "test1.txt".to_string(),
                       size: 0,
                       last_modified: Utc::now(),
                       content_type: "".to_string(),
                   }
                ])
            } else {
                Ok(vec![])
            }
        }
        async fn read_blob_metadata(&self, _key: &str) -> io::Result<BlobMetadata> {
            Err(io::Error::new(io::ErrorKind::NotFound, "not found"))
        }
        async fn get_blob_url(&self, _key: &str) -> io::Result<String> { Ok("".to_string()) }
        async fn read_blob(&self, key: &str) -> io::Result<Vec<u8>> {
            if key == "tenant/org1/fs/test.txt" {
                Ok(b"test data".to_vec())
            } else {
                Err(io::Error::new(io::ErrorKind::NotFound, "not found"))
            }
        }
        async fn write_blob(&self, key: &str, _data: &[u8]) -> io::Result<()> {
             if key == "tenant/org1/fs/test.txt" {
                 Ok(())
             } else {
                 Err(io::Error::new(io::ErrorKind::PermissionDenied, "denied"))
             }
        }
    }

    #[tokio::test]
    async fn test_cloud_read() {
        let tool = FsMcpTool {
            is_standalone: false,
            local_base_dir: PathBuf::from("/tmp"),
            storage_provider: Some(Arc::new(MockProvider)),
        };
        let mut claims = Claims {
            sub: "".to_string(),
            username: "".to_string(),
            email: "".to_string(),
            roles: vec![],
            organization_id: Some("org1".to_string()),
            session_id: None,
            iat: 0,
            exp: 0,
            jti: "".to_string(),
        };

        let result = tool.read(&claims, "test.txt").await;
        assert_eq!(result.unwrap(), "test data");

        let err_result = tool.read(&claims, "missing.txt").await;
        assert!(err_result.is_err());
    }

    #[tokio::test]
    async fn test_cloud_write() {
        let tool = FsMcpTool {
            is_standalone: false,
            local_base_dir: PathBuf::from("/tmp"),
            storage_provider: Some(Arc::new(MockProvider)),
        };
        let claims = Claims {
            sub: "".to_string(),
            username: "".to_string(),
            email: "".to_string(),
            roles: vec![],
            organization_id: Some("org1".to_string()),
            session_id: None,
            iat: 0,
            exp: 0,
            jti: "".to_string(),
        };

        let result = tool.write(&claims, "test.txt", "content").await;
        assert!(result.is_ok());

        let err_result = tool.write(&claims, "other.txt", "content").await;
        assert!(err_result.is_err());
    }

    #[tokio::test]
    async fn test_cloud_list() {
        let tool = FsMcpTool {
            is_standalone: false,
            local_base_dir: PathBuf::from("/tmp"),
            storage_provider: Some(Arc::new(MockProvider)),
        };
        let claims = Claims {
            sub: "".to_string(),
            username: "".to_string(),
            email: "".to_string(),
            roles: vec![],
            organization_id: Some("org1".to_string()),
            session_id: None,
            iat: 0,
            exp: 0,
            jti: "".to_string(),
        };

        let result = tool.list(&claims, "testdir").await;
        assert_eq!(result.unwrap(), vec!["test1.txt".to_string()]);
    }
}
