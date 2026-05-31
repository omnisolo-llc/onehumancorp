use ohc_builtin_agent_core::types::ToolError;
use serde_json::{json, Value};
use std::sync::Arc;
use std::env;
use std::path::PathBuf;
use tokio::fs;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use super::{Tool, ToolExecutor};

#[async_trait::async_trait]
pub trait BlobManager: Send + Sync {
    async fn write_blob(&self, key: &str, data: &[u8]) -> Result<(), String>;
    async fn read_blob(&self, key: &str) -> Result<Vec<u8>, String>;
}

pub struct HybridBlobManager {
    is_cloud: bool,
    local_dir: Option<PathBuf>,
    cloud_mock_store: dashmap::DashMap<String, Vec<u8>>,
}

impl HybridBlobManager {
    pub async fn new() -> Self {
        let s3_endpoint = env::var("S3_ENDPOINT").unwrap_or_default();
        let is_cloud = !s3_endpoint.is_empty();

        let local_dir = if !is_cloud {
            let tmp_dir = env::var("OHC_BLOB_DIR").unwrap_or_else(|_| "/tmp/ohc_blobs".to_string());
            let path = PathBuf::from(tmp_dir);
            if let Err(e) = fs::create_dir_all(&path).await {
                tracing::error!("Failed to create local blob directory: {}", e);
            }
            Some(path)
        } else {
            None
        };

        let manager = Self {
            is_cloud,
            local_dir: local_dir.clone(),
            cloud_mock_store: dashmap::DashMap::new(),
        };

        if let Some(dir) = local_dir {
            tokio::spawn(async move {
                let mut interval = tokio::time::interval(std::time::Duration::from_secs(3600));
                loop {
                    interval.tick().await;
                    Self::cleanup_old_blobs(&dir, std::time::Duration::from_secs(86400)).await;
                }
            });
        }

        manager
    }

    pub async fn cleanup_old_blobs(dir: &std::path::Path, max_age: std::time::Duration) {
        if let Ok(mut entries) = tokio::fs::read_dir(dir).await {
            while let Ok(Some(entry)) = entries.next_entry().await {
                if let Ok(metadata) = entry.metadata().await {
                    if metadata.is_file() {
                        if let Ok(modified) = metadata.modified() {
                            if let Ok(elapsed) = modified.elapsed() {
                                if elapsed > max_age {
                                    let _ = tokio::fs::remove_file(entry.path()).await;
                                }
                            }
                        }
                    } else if metadata.is_dir() {
                        Box::pin(Self::cleanup_old_blobs(&entry.path(), max_age)).await;
                    }
                }
            }
        }
    }

    // Test injection constructor
    pub async fn new_for_test(local_dir: PathBuf) -> Self {
        if let Err(e) = fs::create_dir_all(&local_dir).await {
            tracing::error!("Failed to create test local blob directory: {}", e);
        }
        Self {
            is_cloud: false,
            local_dir: Some(local_dir),
            cloud_mock_store: dashmap::DashMap::new(),
        }
    }

    // Cloud test injection
    pub fn new_cloud_for_test() -> Self {
        Self {
            is_cloud: true,
            local_dir: None,
            cloud_mock_store: dashmap::DashMap::new(),
        }
    }

    fn sanitize_key(key: &str) -> Result<String, String> {
        if key.contains("..") || key.starts_with('/') {
            return Err("Invalid blob key: path traversal not allowed".to_string());
        }
        Ok(key.to_string())
    }
}

#[async_trait::async_trait]
impl BlobManager for HybridBlobManager {
    async fn write_blob(&self, key: &str, data: &[u8]) -> Result<(), String> {
        let safe_key = Self::sanitize_key(key)?;

        if self.is_cloud {
            // Mock S3 put object
            tracing::debug!("Cloud: Writing to S3 key {}", safe_key);
            self.cloud_mock_store.insert(safe_key, data.to_vec());
            Ok(())
        } else {
            let local_dir = self.local_dir.as_ref().ok_or("Local directory not configured")?;
            let path = local_dir.join(&safe_key);

            if let Some(parent) = path.parent() {
                if let Err(e) = fs::create_dir_all(parent).await {
                    return Err(format!("Failed to create directories for blob: {}", e));
                }
            }

            let mut file = fs::File::create(&path).await.map_err(|e| format!("Failed to create file: {}", e))?;
            file.write_all(data).await.map_err(|e| format!("Failed to write data: {}", e))?;

            Ok(())
        }
    }

    async fn read_blob(&self, key: &str) -> Result<Vec<u8>, String> {
        let safe_key = Self::sanitize_key(key)?;

        if self.is_cloud {
            // Mock S3 get object
            tracing::debug!("Cloud: Reading from S3 key {}", safe_key);
            if let Some(data) = self.cloud_mock_store.get(&safe_key) {
                Ok(data.clone())
            } else {
                Err("Blob not found in cloud mock".to_string())
            }
        } else {
            let local_dir = self.local_dir.as_ref().ok_or("Local directory not configured")?;
            let path = local_dir.join(&safe_key);

            let mut file = fs::File::open(&path).await.map_err(|e| format!("Failed to open file: {}", e))?;
            let mut buffer = Vec::new();
            file.read_to_end(&mut buffer).await.map_err(|e| format!("Failed to read data: {}", e))?;

            Ok(buffer)
        }
    }
}


struct HybridBlobExecutor {
    manager: Arc<dyn BlobManager>,
}

#[async_trait::async_trait]
impl ToolExecutor for HybridBlobExecutor {
    async fn execute(
        &self,
        args: Value,
    ) -> Result<String, ToolError> {
        let action = args["Action"].as_str().ok_or_else(|| ToolError::LlmRecoverable("hybrid_blob: Action is required".to_string()))?;
        let key = args["Key"].as_str().ok_or_else(|| ToolError::LlmRecoverable("hybrid_blob: Key is required".to_string()))?;

        match action {
            "read" => {
                let data = self.manager.read_blob(key).await.map_err(|e| ToolError::LlmRecoverable(e))?;
                // Attempt to return as UTF-8 string, otherwise base64 encode
                let content = if let Ok(s) = String::from_utf8(data.clone()) {
                    s
                } else {
                    use base64::{Engine as _, engine::general_purpose};
                    general_purpose::STANDARD.encode(&data)
                };

                Ok(json!({
                    "status": "read",
                    "key": key,
                    "data": content
                }).to_string())
            }
            "write" => {
                let data_str = args["Data"].as_str().ok_or_else(|| ToolError::LlmRecoverable("hybrid_blob: Data is required for write".to_string()))?;
                let data_bytes = data_str.as_bytes();

                self.manager.write_blob(key, data_bytes).await.map_err(|e| ToolError::LlmRecoverable(e))?;

                Ok(json!({
                    "status": "written",
                    "key": key
                }).to_string())
            }
            _ => Err(ToolError::LlmRecoverable("invalid action".to_string())),
        }
    }
}

pub fn hybrid_blob_tool() -> Tool {
    // Need a blocking way to instantiate if we aren't awaiting
    let s3_endpoint = env::var("S3_ENDPOINT").unwrap_or_default();
    let is_cloud = !s3_endpoint.is_empty();

    let local_dir = if !is_cloud {
        let tmp_dir = env::var("OHC_BLOB_DIR").unwrap_or_else(|_| "/tmp/ohc_blobs".to_string());
        let path = PathBuf::from(tmp_dir);
        // We do synchronous dir creation here since this is tool init
        if let Err(e) = std::fs::create_dir_all(&path) {
            tracing::error!("Failed to create tool local blob directory: {}", e);
        }
        Some(path)
    } else {
        None
    };

    let manager = Arc::new(HybridBlobManager {
        is_cloud,
        local_dir,
        cloud_mock_store: dashmap::DashMap::new(),
    });

    Tool {
        name: "hybrid_blob".to_string(),
        description: "Reads and writes blobs across standalone and cloud environments".to_string(),
        is_read_only: false,
        parameters: json!({
            "type": "object",
            "properties": {
                "Action": {
                    "type": "string",
                    "description": "Action to perform: read, write"
                },
                "Key": {
                    "type": "string",
                    "description": "Blob key"
                },
                "Data": {
                    "type": "string",
                    "description": "Data to write (required for write action)"
                }
            },
            "required": ["Action", "Key"]
        }),
        execute: Arc::new(HybridBlobExecutor { manager }),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[tokio::test]
    async fn test_local_blob_write_read() {
        let dir = tempdir().unwrap();
        let manager = HybridBlobManager::new_for_test(dir.path().to_path_buf()).await;

        let key = "test/image.png";
        let data = b"fake png data";

        manager.write_blob(key, data).await.unwrap();
        let read_data = manager.read_blob(key).await.unwrap();

        assert_eq!(read_data, data);

        let path = dir.path().join(key);
        assert!(path.exists());
    }

    #[tokio::test]
    async fn test_path_traversal_prevention() {
        let dir = tempdir().unwrap();
        let manager = HybridBlobManager::new_for_test(dir.path().to_path_buf()).await;

        let key = "../../../etc/passwd";
        let data = b"hack";

        let res = manager.write_blob(key, data).await;
        assert!(res.is_err());
        assert!(res.unwrap_err().contains("path traversal not allowed"));
    }

    #[tokio::test]
    async fn test_cleanup_old_blobs() {
        let dir = tempdir().unwrap();
        let path1 = dir.path().join("new.txt");
        let path2 = dir.path().join("old.txt");

        tokio::fs::write(&path1, "new").await.unwrap();
        tokio::fs::write(&path2, "old").await.unwrap();

        tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;
        HybridBlobManager::cleanup_old_blobs(dir.path(), std::time::Duration::from_millis(0)).await;

        assert!(!path1.exists());
        assert!(!path2.exists());
    }

    #[tokio::test]
    async fn test_cloud_mock() {
        let manager = HybridBlobManager::new_cloud_for_test();

        let key = "org-1/test.txt";
        let data = b"cloud data";

        // Mock should just return ok
        manager.write_blob(key, data).await.unwrap();
        let read_data = manager.read_blob(key).await.unwrap();

        // The mock read returns the data written if implemented, otherwise empty depending on mock.
        // Actually, the test panicked because `read_data` was `cloud data` and the assert expected `""`.
        assert_eq!(read_data, b"cloud data");
    }
}
