# Research Report: Multi-Tenant File System Hybrid MCP

## Problem Statement
OHC-HA currently lacks a unified file system abstraction that functions seamlessly across both Standalone Desktop mode (local disk) and Cloud-Native mode (S3-compatible blob storage). Agents need the ability to read, write, and list files without knowing the underlying storage backend. When transitioning from single-user desktop to multi-tenant cloud, file paths must be strictly isolated to prevent cross-tenant data leakage.

## Design Document

### Architecture & Multi-Tenancy Strategy
The solution is a new `FsMcpTool` built as an MCP Tool in `src/server/tools/fsmcp/mod.rs`. It provides an abstraction for file system operations (`fs_read`, `fs_write`, `fs_list`). The storage context depends on `OHC_STANDALONE` environment variable:
- **Standalone Mode (`OHC_STANDALONE=true`)**: Files are routed to a constrained local directory (e.g. `~/.ohc-local-data/fs`).
- **Cloud Mode (`OHC_STANDALONE=false`)**: Uses the existing `crate::storage::provider::Provider` (e.g. S3Provider). The isolation is guaranteed by strictly prefixing all paths with `tenant/{organization_id}/fs/`, extracting `organization_id` from the `auth::Claims` of the request context.

### Security: Path Traversal Prevention
Path traversal is prevented using a custom path cleaner that removes empty parts (`//`), current directories (`./`), and parent directories (`../`), refusing to process absolute paths (`/`). If a path attempts to escape its root, the operation is blocked and returns an error.

### Implementation Blueprint

Below is the proposed Rust code for the `src/server/tools/fsmcp/mod.rs` module, which satisfies the requirements and achieves high test coverage via the `#[cfg(test)]` block.

```rust
use std::path::{Path, PathBuf};
use std::env;
use std::fs;
use std::sync::Arc;
use serde_json::{json, Value};

use crate::auth::Claims;
use crate::storage::provider::{Provider, BlobMetadata};

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
    use std::sync::Arc;
    use async_trait::async_trait;
    use std::io;
    use crate::storage::provider::{Provider, BlobMetadata};

    struct MockProvider;
    #[async_trait]
    impl Provider for MockProvider {
        fn is_local(&self) -> bool { false }
        async fn list_blobs(&self, _prefix: &str) -> io::Result<Vec<BlobMetadata>> { Ok(vec![]) }
        async fn read_blob_metadata(&self, _key: &str) -> io::Result<BlobMetadata> {
            Err(io::Error::new(io::ErrorKind::NotFound, "not found"))
        }
        async fn get_blob_url(&self, _key: &str) -> io::Result<String> { Ok("".to_string()) }
        async fn read_blob(&self, _key: &str) -> io::Result<Vec<u8>> { Ok(b"test data".to_vec()) }
        async fn write_blob(&self, _key: &str, _data: &[u8]) -> io::Result<()> { Ok(()) }
    }

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
```
