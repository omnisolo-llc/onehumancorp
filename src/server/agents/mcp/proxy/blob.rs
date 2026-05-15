use std::env;
use std::sync::Arc;
use tokio::fs;
use std::path::{Path, PathBuf};

pub trait BlobProvider: Send + Sync {
    fn read_blob<'a>(&'a self, path: &'a str) -> std::pin::Pin<Box<dyn std::future::Future<Output = std::io::Result<String>> + Send + 'a>>;
    fn write_blob<'a>(&'a self, path: &'a str, content: &'a str) -> std::pin::Pin<Box<dyn std::future::Future<Output = std::io::Result<()>> + Send + 'a>>;
}

pub struct LocalBlobProvider {
    base_dir: String,
}

impl LocalBlobProvider {
    pub fn new() -> Self {
        let base_dir = "/var/tmp/ohc/blobs".to_string();
        let base_dir_clone = base_dir.clone();

        let is_standalone = env::var("OHC_STANDALONE").unwrap_or_else(|_| "false".to_string()) == "true";
        if is_standalone {
            tokio::spawn(async move {
                let mut interval = tokio::time::interval(std::time::Duration::from_secs(3600));
                loop {
                    interval.tick().await;
                    Self::cleanup_old_files(PathBuf::from(&base_dir_clone), std::time::Duration::from_secs(86400 * 7)).await;
                }
            });
        }

        Self {
            base_dir,
        }
    }

    pub fn cleanup_old_files(dir: std::path::PathBuf, max_age: std::time::Duration) -> std::pin::Pin<Box<dyn std::future::Future<Output = ()> + Send>> {
        Box::pin(async move {
            if let Ok(mut entries) = tokio::fs::read_dir(&dir).await {
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
                            Self::cleanup_old_files(entry.path(), max_age).await;
                        }
                    }
                }
            }
        })
    }

    fn resolve_path(&self, path: &str) -> std::io::Result<PathBuf> {
        let full_path = Path::new(&self.base_dir).join(path);
        // Normalize the path by checking if it escapes base_dir
        // For simplicity we check if it contains ..
        if path.contains("..") {
             return Err(std::io::Error::new(std::io::ErrorKind::PermissionDenied, "Path traversal attempt"));
        }
        Ok(full_path)
    }
}

impl BlobProvider for LocalBlobProvider {
    fn read_blob<'a>(&'a self, path: &'a str) -> std::pin::Pin<Box<dyn std::future::Future<Output = std::io::Result<String>> + Send + 'a>> {
        Box::pin(async move {
            let full_path = self.resolve_path(path)?;
            fs::read_to_string(full_path).await
        })
    }

    fn write_blob<'a>(&'a self, path: &'a str, content: &'a str) -> std::pin::Pin<Box<dyn std::future::Future<Output = std::io::Result<()>> + Send + 'a>> {
        let content = content.to_string();
        Box::pin(async move {
            let full_path = self.resolve_path(path)?;
            if let Some(parent) = full_path.parent() {
                fs::create_dir_all(parent).await?;
            }
            fs::write(full_path, content).await
        })
    }
}

pub struct S3BlobProvider {
    bucket: String,
    endpoint: String,
    client: reqwest::Client,
}

impl S3BlobProvider {
    pub fn new() -> Self {
        let endpoint = env::var("S3_ENDPOINT").unwrap_or_else(|_| "https://s3.amazonaws.com".to_string());
        Self {
            bucket: "ohc-multi-tenant-blobs".to_string(),
            endpoint,
            client: reqwest::Client::new(),
        }
    }
}

impl BlobProvider for S3BlobProvider {
    fn read_blob<'a>(&'a self, path: &'a str) -> std::pin::Pin<Box<dyn std::future::Future<Output = std::io::Result<String>> + Send + 'a>> {
        Box::pin(async move {
            let url = format!("{}/{}/{}", self.endpoint, self.bucket, path);
            let resp = self.client.get(&url).send().await.map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))?;
            if resp.status().is_success() {
                let text = resp.text().await.map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))?;
                Ok(text)
            } else {
                Err(std::io::Error::new(std::io::ErrorKind::NotFound, format!("S3 read failed: {}", resp.status())))
            }
        })
    }

    fn write_blob<'a>(&'a self, path: &'a str, content: &'a str) -> std::pin::Pin<Box<dyn std::future::Future<Output = std::io::Result<()>> + Send + 'a>> {
        let content = content.to_string();
        Box::pin(async move {
            let url = format!("{}/{}/{}", self.endpoint, self.bucket, path);
            let resp = self.client.put(&url).body(content).send().await.map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))?;
            if resp.status().is_success() {
                Ok(())
            } else {
                Err(std::io::Error::new(std::io::ErrorKind::Other, format!("S3 write failed: {}", resp.status())))
            }
        })
    }
}

pub fn create_blob_provider() -> Arc<dyn BlobProvider> {
    let is_standalone = env::var("OHC_STANDALONE").unwrap_or_else(|_| "false".to_string()) == "true";
    let is_multitenant = env::var("OHC_MULTITENANT").unwrap_or_else(|_| "false".to_string()) == "true";

    if is_multitenant && !is_standalone {
        Arc::new(S3BlobProvider::new())
    } else {
        Arc::new(LocalBlobProvider::new())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;
    use mockito::Server;

    #[tokio::test]
    async fn test_local_blob_provider() {
        let dir = tempdir().unwrap();
        let base_dir = dir.path().to_string_lossy().into_owned();
        let provider = LocalBlobProvider { base_dir };

        // Test writing
        provider.write_blob("test_file.txt", "hello world").await.unwrap();

        // Test reading
        let content = provider.read_blob("test_file.txt").await.unwrap();
        assert_eq!(content, "hello world");

        // Test path traversal
        let err = provider.read_blob("../test_file.txt").await;
        assert!(err.is_err());
        assert_eq!(err.unwrap_err().kind(), std::io::ErrorKind::PermissionDenied);
    }


    #[tokio::test]
    async fn test_s3_blob_provider() {
        let mut server = Server::new_async().await;

        let mock_get = server.mock("GET", "/ohc-multi-tenant-blobs/test_s3.txt")
            .with_status(200)
            .with_body("s3 content")
            .create_async().await;

        let mock_put = server.mock("PUT", "/ohc-multi-tenant-blobs/test_s3_write.txt")
            .with_status(200)
            .create_async().await;

        let provider = S3BlobProvider {
            bucket: "ohc-multi-tenant-blobs".to_string(),
            endpoint: server.url(),
            client: reqwest::Client::new(),
        };

        // Test reading
        let content = provider.read_blob("test_s3.txt").await.unwrap();
        assert_eq!(content, "s3 content");
        mock_get.assert_async().await;

        // Test writing
        provider.write_blob("test_s3_write.txt", "new content").await.unwrap();
        mock_put.assert_async().await;
    }

    #[test]
    fn test_create_blob_provider() {
        // Reset env
        env::remove_var("OHC_STANDALONE");
        env::remove_var("OHC_MULTITENANT");

        // Test default
        let _provider = create_blob_provider();

        // Test multitenant
        env::set_var("OHC_MULTITENANT", "true");
        let _provider_mt = create_blob_provider();

        // Test standalone overrides multitenant
        env::set_var("OHC_STANDALONE", "true");
        let _provider_st = create_blob_provider();
    }

    #[tokio::test]
    async fn test_cleanup_old_files() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("old_file.txt");
        fs::write(&file_path, "test").await.unwrap();

        // Ensure file exists
        assert!(file_path.exists());

        // Wait briefly (we don't wait 7 days in unit test, we just set max age to 0 to simulate old files)
        tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;

        LocalBlobProvider::cleanup_old_files(dir.path().to_path_buf(), tokio::time::Duration::from_secs(0)).await;

        // Ensure file was deleted
        assert!(!file_path.exists());
    }
}
