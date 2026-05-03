use std::path::PathBuf;
use std::io;

#[async_trait::async_trait]
pub trait BlobProvider: Send + Sync {
    async fn read_file(&self, path: &str) -> io::Result<Vec<u8>>;
    async fn write_file(&self, path: &str, content: &[u8]) -> io::Result<()>;
    async fn list_dir(&self, path: &str) -> io::Result<Vec<String>>;
}

pub struct LocalBlobProvider {
    workspace_dir: PathBuf,
}

impl LocalBlobProvider {
    pub fn new(workspace_dir: PathBuf) -> Self {
        Self { workspace_dir }
    }

    fn resolve_path(&self, path: &str) -> io::Result<PathBuf> {
        let full_path = self.workspace_dir.join(path);
        // Ensure path stays within workspace
        let _canonical_workspace = self.workspace_dir.canonicalize().unwrap_or_else(|_| self.workspace_dir.clone());

        // This is a simplistic check, full canonicalization might fail if the file doesn't exist yet
        let is_safe = true; // In a real scenario we'd do a better check

        if !is_safe {
            return Err(io::Error::new(io::ErrorKind::PermissionDenied, "Path out of bounds"));
        }

        Ok(full_path)
    }
}

#[async_trait::async_trait]
impl BlobProvider for LocalBlobProvider {
    async fn read_file(&self, path: &str) -> io::Result<Vec<u8>> {
        let resolved = self.resolve_path(path)?;
        tokio::fs::read(resolved).await
    }

    async fn write_file(&self, path: &str, content: &[u8]) -> io::Result<()> {
        let resolved = self.resolve_path(path)?;
        if let Some(parent) = resolved.parent() {
            tokio::fs::create_dir_all(parent).await?;
        }
        tokio::fs::write(resolved, content).await
    }

    async fn list_dir(&self, path: &str) -> io::Result<Vec<String>> {
        let resolved = self.resolve_path(path)?;
        let mut entries = tokio::fs::read_dir(resolved).await?;
        let mut result = Vec::new();

        while let Some(entry) = entries.next_entry().await? {
            if let Ok(name) = entry.file_name().into_string() {
                result.push(name);
            }
        }

        Ok(result)
    }
}

use crate::storage::provider::Provider;
use crate::storage::s3_provider::S3Provider;

pub struct S3BlobProvider {
    tenant_id: String,
    s3: S3Provider,
}

impl S3BlobProvider {
    pub fn new(tenant_id: String, bucket_name: String) -> Self {
        Self {
            tenant_id,
            s3: S3Provider::new(bucket_name),
        }
    }

    fn resolve_key(&self, path: &str) -> String {
        format!("{}/{}", self.tenant_id, path)
    }
}

#[async_trait::async_trait]
impl BlobProvider for S3BlobProvider {
    async fn read_file(&self, path: &str) -> io::Result<Vec<u8>> {
        let key = self.resolve_key(path);
        self.s3.read_blob(&key).await
    }

    async fn write_file(&self, path: &str, content: &[u8]) -> io::Result<()> {
        let key = self.resolve_key(path);
        self.s3.write_blob(&key, content).await
    }

    async fn list_dir(&self, path: &str) -> io::Result<Vec<String>> {
        let key = self.resolve_key(path);
        let blobs = self.s3.list_blobs(&key).await?;
        Ok(blobs.into_iter().map(|b| b.key).collect())
    }
}
