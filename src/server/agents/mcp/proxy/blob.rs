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
    use axum::{
        Router,
        extract::State,
        http::StatusCode,
        routing::{get, put},
    };
    use std::sync::atomic::{AtomicUsize, Ordering};
    use tempfile::tempdir;

    #[derive(Clone)]
    struct MockS3State {
        gets: Arc<AtomicUsize>,
        puts: Arc<AtomicUsize>,
    }

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
        let state = MockS3State {
            gets: Arc::new(AtomicUsize::new(0)),
            puts: Arc::new(AtomicUsize::new(0)),
        };
        let app = Router::new()
            .route(
                "/ohc-multi-tenant-blobs/test_s3.txt",
                get(|State(state): State<MockS3State>| async move {
                    state.gets.fetch_add(1, Ordering::SeqCst);
                    "s3 content"
                }),
            )
            .route(
                "/ohc-multi-tenant-blobs/test_s3_write.txt",
                put(|State(state): State<MockS3State>| async move {
                    state.puts.fetch_add(1, Ordering::SeqCst);
                    StatusCode::OK
                }),
            )
            .with_state(state.clone());

        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let endpoint = format!("http://{}", listener.local_addr().unwrap());
        tokio::spawn(async move {
            axum::serve(listener, app).await.unwrap();
        });

        let provider = S3BlobProvider {
            bucket: "ohc-multi-tenant-blobs".to_string(),
            endpoint,
            client: reqwest::Client::new(),
        };

        // Test reading
        let content = provider.read_blob("test_s3.txt").await.unwrap();
        assert_eq!(content, "s3 content");
        assert_eq!(state.gets.load(Ordering::SeqCst), 1);

        // Test writing
        provider.write_blob("test_s3_write.txt", "new content").await.unwrap();
        assert_eq!(state.puts.load(Ordering::SeqCst), 1);
    }

    #[test]
    fn test_create_blob_provider() {
        temp_env::with_vars(
            vec![
                ("OHC_STANDALONE", None::<&str>),
                ("OHC_MULTITENANT", None::<&str>),
            ],
            || {
                let _provider = create_blob_provider();
            },
        );

        temp_env::with_vars(
            vec![
                ("OHC_STANDALONE", None::<&str>),
                ("OHC_MULTITENANT", Some("true")),
            ],
            || {
                let _provider_mt = create_blob_provider();
            },
        );

        temp_env::with_vars(
            vec![
                ("OHC_STANDALONE", Some("true")),
                ("OHC_MULTITENANT", Some("true")),
            ],
            || {
                let _provider_st = create_blob_provider();
            },
        );
    }
}
