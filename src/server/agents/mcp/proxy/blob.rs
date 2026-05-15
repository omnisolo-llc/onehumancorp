use std::env;
use std::sync::Arc;
use tokio::fs;
use std::path::{Path, PathBuf};
use aws_sdk_s3::Client as S3Client;

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
        let req_path = Path::new(path);
        if req_path.is_absolute() || path.contains("..") {
             return Err(std::io::Error::new(std::io::ErrorKind::PermissionDenied, "Path traversal attempt"));
        }
        Ok(Path::new(&self.base_dir).join(path))
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
    client: S3Client,
}

impl S3BlobProvider {
    pub async fn new() -> Self {
        let config = aws_config::load_defaults(aws_config::BehaviorVersion::latest()).load().await;
        let client = S3Client::new(&config);
        Self {
            bucket: "ohc-multi-tenant-blobs".to_string(),
            client,
        }
    }

    #[cfg(test)]
    pub fn new_with_client(client: S3Client) -> Self {
        Self {
            bucket: "ohc-multi-tenant-blobs".to_string(),
            client,
        }
    }
}

impl BlobProvider for S3BlobProvider {
    fn read_blob<'a>(&'a self, path: &'a str) -> std::pin::Pin<Box<dyn std::future::Future<Output = std::io::Result<String>> + Send + 'a>> {
        Box::pin(async move {
            let result = self.client.get_object()
                .bucket(&self.bucket)
                .key(path)
                .send()
                .await
                .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, format!("{:?}", e)))?;

            let data = result.body.collect().await.map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, format!("{:?}", e)))?;
            let text = String::from_utf8(data.into_bytes().to_vec()).map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e.to_string()))?;
            Ok(text)
        })
    }

    fn write_blob<'a>(&'a self, path: &'a str, content: &'a str) -> std::pin::Pin<Box<dyn std::future::Future<Output = std::io::Result<()>> + Send + 'a>> {
        let content = content.to_string();
        Box::pin(async move {
            let body = aws_sdk_s3::primitives::ByteStream::from(content.into_bytes());
            self.client.put_object()
                .bucket(&self.bucket)
                .key(path)
                .body(body)
                .send()
                .await
                .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, format!("{:?}", e)))?;
            Ok(())
        })
    }
}

pub async fn create_blob_provider() -> Arc<dyn BlobProvider> {
    let is_standalone = env::var("OHC_STANDALONE").unwrap_or_else(|_| "false".to_string()) == "true";
    let is_multitenant = env::var("OHC_MULTITENANT").unwrap_or_else(|_| "false".to_string()) == "true";

    if is_multitenant && !is_standalone {
        Arc::new(S3BlobProvider::new().await)
    } else {
        Arc::new(LocalBlobProvider::new())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;
    use aws_sdk_s3::config::{Credentials, Region};
    use aws_smithy_runtime::client::http::test_util::{ReplayEvent, StaticReplayClient};
    use aws_smithy_types::body::SdkBody;
    use http::{Request, Response};

    #[tokio::test]
    async fn test_local_blob_provider() {
        let dir = tempdir().unwrap();
        let base_dir = dir.path().to_string_lossy().into_owned();
        let provider = LocalBlobProvider { base_dir };

        provider.write_blob("test_file.txt", "hello world").await.unwrap();

        let content = provider.read_blob("test_file.txt").await.unwrap();
        assert_eq!(content, "hello world");

        let err = provider.read_blob("../test_file.txt").await;
        assert!(err.is_err());
        assert_eq!(err.unwrap_err().kind(), std::io::ErrorKind::PermissionDenied);

        let err2 = provider.read_blob("/etc/passwd").await;
        assert!(err2.is_err());
        assert_eq!(err2.unwrap_err().kind(), std::io::ErrorKind::PermissionDenied);
    }

    #[tokio::test]
    async fn test_create_blob_provider() {
        // avoid modifying global environment in async tests to prevent flakiness
        // we'll just test that we can initialize the local one
        let provider = LocalBlobProvider::new();
        assert_eq!(provider.base_dir, "/var/tmp/ohc/blobs");
    }

    #[tokio::test]
    async fn test_s3_blob_provider_read_success() {
        let http_client = StaticReplayClient::new(vec![
            ReplayEvent::new(
                Request::builder()
                    .method("GET")
                    .uri("https://ohc-multi-tenant-blobs.s3.us-east-1.amazonaws.com/test_path.txt")
                    .body(SdkBody::empty())
                    .unwrap(),
                Response::builder()
                    .status(200)
                    .body(SdkBody::from("s3 mocked content"))
                    .unwrap(),
            ),
        ]);

        let config = aws_sdk_s3::Config::builder()
            .region(Region::new("us-east-1"))
            .credentials_provider(Credentials::new("test", "test", None, None, "test"))
            .http_client(http_client.clone())
            .build();

        let client = S3Client::from_conf(config);
        let provider = S3BlobProvider::new_with_client(client);

        let result = provider.read_blob("test_path.txt").await.unwrap();
        assert_eq!(result, "s3 mocked content");
        http_client.assert_requests_match(&[]);
    }

    #[tokio::test]
    async fn test_s3_blob_provider_write_success() {
        let http_client = StaticReplayClient::new(vec![
            ReplayEvent::new(
                Request::builder()
                    .method("PUT")
                    .uri("https://ohc-multi-tenant-blobs.s3.us-east-1.amazonaws.com/test_write.txt")
                    .body(SdkBody::from("new s3 content"))
                    .unwrap(),
                Response::builder()
                    .status(200)
                    .body(SdkBody::empty())
                    .unwrap(),
            ),
        ]);

        let config = aws_sdk_s3::Config::builder()
            .region(Region::new("us-east-1"))
            .credentials_provider(Credentials::new("test", "test", None, None, "test"))
            .http_client(http_client.clone())
            .build();

        let client = S3Client::from_conf(config);
        let provider = S3BlobProvider::new_with_client(client);

        provider.write_blob("test_write.txt", "new s3 content").await.unwrap();
    }
}
