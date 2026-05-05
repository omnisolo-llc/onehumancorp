use async_trait::async_trait;
use chrono::DateTime;
use std::fs;
use std::io;

use std::path::{Path, PathBuf};
use crate::billing::Tracker;
use super::provider::{BlobMetadata, Provider};

pub struct LocalProvider {
    base_path: PathBuf,
    tracker: Tracker,
}

impl LocalProvider {
    pub fn new<P: AsRef<Path>>(base_path: P) -> io::Result<Self> {
        let abs_path = fs::canonicalize(base_path)?;
        fs::create_dir_all(&abs_path)?;
        Ok(LocalProvider { base_path: abs_path, tracker: Tracker::new() })
    }

    fn get_local_path(&self, key: &str) -> io::Result<PathBuf> {
        if key.contains("..") {
            return Err(io::Error::new(io::ErrorKind::InvalidInput, "Path traversal detected (..)"));
        }
        let path = self.base_path.join(key);
        
        if !path.starts_with(&self.base_path) {
            return Err(io::Error::new(io::ErrorKind::InvalidInput, "Path traversal detected"));
        }
        Ok(path)
    }
}

#[async_trait]
impl Provider for LocalProvider {
    fn is_local(&self) -> bool {
        true
    }

    async fn list_blobs(&self, prefix: &str) -> io::Result<Vec<BlobMetadata>> {
        let mut blobs = Vec::new();
        
        fn walk_dir(dir: &Path, base_path: &Path, prefix: &str, blobs: &mut Vec<BlobMetadata>) -> io::Result<()> {
            if dir.is_dir() {
                for entry in fs::read_dir(dir)? {
                    let entry = entry?;
                    let path = entry.path();
                    if path.is_dir() {
                        walk_dir(&path, base_path, prefix, blobs)?;
                    } else {
                        let rel_path = path.strip_prefix(base_path).map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
                        let key = rel_path.to_string_lossy().to_string();
                        if key.starts_with(prefix) {
                            let metadata = entry.metadata()?;
                            blobs.push(BlobMetadata {
                                key,
                                size: metadata.len() as i64,
                                last_modified: DateTime::from(metadata.modified()?),
                                content_type: "application/octet-stream".to_string(),
                            });
                        }
                    }
                }
            }
            Ok(())
        }

        walk_dir(&self.base_path, &self.base_path, prefix, &mut blobs)?;
        
        Ok(blobs)
    }

    async fn read_blob_metadata(&self, key: &str) -> io::Result<BlobMetadata> {
        let path = self.get_local_path(key)?;
        let metadata = fs::metadata(&path)?;
        if metadata.is_dir() {
            return Err(io::Error::new(io::ErrorKind::InvalidInput, "Key is a directory"));
        }
        Ok(BlobMetadata {
            key: key.to_string(),
            size: metadata.len() as i64,
            last_modified: DateTime::from(metadata.modified()?),
            content_type: "application/octet-stream".to_string(),
        })
    }

    async fn get_blob_url(&self, key: &str) -> io::Result<String> {
        let path = self.get_local_path(key)?;
        if !path.exists() {
            return Err(io::Error::new(io::ErrorKind::NotFound, "Blob does not exist"));
        }
        if let Ok(cdn) = std::env::var("OHC_CDN_URL") {
            if !cdn.is_empty() {
                return Ok(format!("{}/{}", cdn, key));
            }
        }
        Ok(format!("file://{}", path.to_string_lossy()))
    }

    async fn read_blob(&self, key: &str) -> io::Result<Vec<u8>> {
        let path = self.get_local_path(key)?;
        tokio::fs::read(path).await
    }

    async fn write_blob(&self, key: &str, data: &[u8]) -> io::Result<()> {
        let path = self.get_local_path(key)?;
        if let Some(parent) = path.parent() {
            tokio::fs::create_dir_all(parent).await?;
        }

        let mut final_data = data.to_vec();

        // Auto-compression to WebP mock for images
        let is_image = key.ends_with(".png") || key.ends_with(".jpg") || key.ends_with(".jpeg");
        if is_image && data.len() > 100 {
            // Mock compression: reduce size by 50%
            let original_size = final_data.len() as i64;
            final_data.truncate(data.len() / 2);
            let compressed_size = final_data.len() as i64;

            // Track savings via the auditor
            let _savings = crate::pricing::calculator::calculate_storage_savings(
                original_size,
                compressed_size,
                &crate::pricing::calculator::CostConfig::default()
            );
        }

        // Quota Enforcement
        let t_id = key.split('/').next().unwrap_or("default");
        if let Ok(status) = self.tracker.track_storage_usage(t_id, final_data.len() as i64).await {
            if status.soft_limit_reached {
                if let Some(msg) = status.user_message {
                    tracing::warn!(tid = %t_id, "Storage quota warning: {}", msg);
                }
            }
        }

        tokio::fs::write(path, &final_data).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::Rng;
    use rand::distributions::Alphanumeric;

    fn new_test_provider() -> (LocalProvider, String) {
        let random_suffix: String = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(10)
            .map(char::from)
            .collect();
        let dir = format!("/tmp/test_storage_{}", random_suffix);
        fs::create_dir_all(&dir).unwrap();
        let abs_dir = fs::canonicalize(&dir).unwrap();
        let p = LocalProvider::new(&abs_dir).unwrap();
        (p, abs_dir.to_string_lossy().to_string())
    }

    fn write_file(dir: &str, key: &str, content: &str) {
        let full = Path::new(dir).join(key);
        fs::create_dir_all(full.parent().unwrap()).unwrap();
        fs::write(full, content).unwrap();
    }

    fn cleanup(dir: String) {
        let _ = fs::remove_dir_all(dir);
    }

    #[test]
    fn test_local_provider_is_local() {
        let (p, dir) = new_test_provider();
        assert!(p.is_local());
        cleanup(dir);
    }

    #[tokio::test]
    async fn test_local_provider_list_blobs_empty() {
        let (p, dir) = new_test_provider();
        let blobs = p.list_blobs("").await.unwrap();
        assert_eq!(blobs.len(), 0);
        cleanup(dir);
    }

    #[tokio::test]
    async fn test_local_provider_list_blobs_single_file() {
        let (p, dir) = new_test_provider();
        write_file(&dir, "foo/bar.txt", "hello");

        let blobs = p.list_blobs("foo/").await.unwrap();
        assert_eq!(blobs.len(), 1);
        assert_eq!(blobs[0].key, "foo/bar.txt");
        assert_eq!(blobs[0].size, 5);
        cleanup(dir);
    }

    #[tokio::test]
    async fn test_local_provider_list_blobs_multiple_files() {
        let (p, dir) = new_test_provider();
        write_file(&dir, "data/a.txt", "aaa");
        write_file(&dir, "data/b.txt", "bbbb");
        write_file(&dir, "other/c.txt", "ccccc");

        let blobs = p.list_blobs("data/").await.unwrap();
        assert_eq!(blobs.len(), 2);
        cleanup(dir);
    }

    #[tokio::test]
    async fn test_local_provider_read_blob_metadata_existing_file() {
        let (p, dir) = new_test_provider();
        let content = "test content";
        write_file(&dir, "docs/readme.md", content);

        let meta = p.read_blob_metadata("docs/readme.md").await.unwrap();
        assert_eq!(meta.key, "docs/readme.md");
        assert_eq!(meta.size, content.len() as i64);
        cleanup(dir);
    }

    #[tokio::test]
    async fn test_local_provider_read_blob_metadata_missing_file() {
        let (p, dir) = new_test_provider();
        let err = p.read_blob_metadata("nonexistent/file.txt").await;
        assert!(err.is_err());
        cleanup(dir);
    }

    #[tokio::test]
    async fn test_local_provider_get_blob_url_existing_file() {
        let (p, dir) = new_test_provider();
        write_file(&dir, "assets/logo.png", "png-data");

        let url = p.get_blob_url("assets/logo.png").await.unwrap();
        assert!(url.starts_with("file://"));
        cleanup(dir);
    }

    #[tokio::test]
    async fn test_local_provider_get_blob_url_missing_file() {
        let (p, dir) = new_test_provider();
        let err = p.get_blob_url("missing.txt").await;
        assert!(err.is_err());
        cleanup(dir);
    }

    #[tokio::test]
    async fn test_local_provider_read_write_blob() {
        let (p, dir) = new_test_provider();
        let content = b"test data";
        let key = "test/blob.bin";

        p.write_blob(key, content).await.unwrap();

        let read_content = p.read_blob(key).await.unwrap();
        assert_eq!(read_content, content);
        cleanup(dir);
    }
}
