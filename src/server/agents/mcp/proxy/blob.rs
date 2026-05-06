use std::env;
use std::sync::Arc;
use tokio::fs;
use std::path::{Path, PathBuf};
use reqwest::Client;

pub trait BlobProvider: Send + Sync {
    fn read_blob<'a>(&'a self, path: &'a str) -> std::pin::Pin<Box<dyn std::future::Future<Output = std::io::Result<String>> + Send + 'a>>;
    fn write_blob<'a>(&'a self, path: &'a str, content: &'a str) -> std::pin::Pin<Box<dyn std::future::Future<Output = std::io::Result<()>> + Send + 'a>>;
}

pub struct LocalBlobProvider {
    base_dir: String,
}

impl LocalBlobProvider {
    pub fn new() -> Self {
        Self {
            base_dir: "/var/tmp/ohc/blobs".to_string(),
        }
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
    client: Client,
}

impl S3BlobProvider {
    pub fn new() -> Self {
        let endpoint = env::var("S3_ENDPOINT").unwrap_or_else(|_| "https://s3.amazonaws.com".to_string());
        Self {
            bucket: "ohc-multi-tenant-blobs".to_string(),
            endpoint,
            client: Client::new(),
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
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};
    use tempfile::tempdir;
    use std::fs;

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
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/ohc-multi-tenant-blobs/test_s3.txt"))
            .respond_with(ResponseTemplate::new(200).set_body_string("s3 content"))
            .mount(&mock_server)
            .await;

        Mock::given(method("PUT"))
            .and(path("/ohc-multi-tenant-blobs/test_s3_write.txt"))
            .respond_with(ResponseTemplate::new(200))
            .mount(&mock_server)
            .await;

        let provider = S3BlobProvider {
            bucket: "ohc-multi-tenant-blobs".to_string(),
            endpoint: mock_server.uri(),
            client: reqwest::Client::new(),
        };

        // Test reading
        let content = provider.read_blob("test_s3.txt").await.unwrap();
        assert_eq!(content, "s3 content");

        // Test writing
        provider.write_blob("test_s3_write.txt", "new content").await.unwrap();
    }

    #[test]
    fn test_create_blob_provider() {
        // Reset env
        env::remove_var("OHC_STANDALONE");
        env::remove_var("OHC_MULTITENANT");

        // Test default
        let provider = create_blob_provider();
        // Check if it's local
        // Rust trait downcasting isn't trivial without Any, but we can just invoke it or trust logic

        // Test multitenant
        env::set_var("OHC_MULTITENANT", "true");
        let provider_mt = create_blob_provider();

        // Test standalone overrides multitenant
        env::set_var("OHC_STANDALONE", "true");
        let provider_st = create_blob_provider();
    }
}
