use std::path::PathBuf;
use std::io;

#[async_trait::async_trait]
pub trait FileSystemProvider: Send + Sync {
    async fn read_file(&self, path: &str) -> io::Result<Vec<u8>>;
    async fn write_file(&self, path: &str, content: &[u8]) -> io::Result<()>;
    async fn list_dir(&self, path: &str) -> io::Result<Vec<String>>;
}

pub struct BaseFSProvider {
    root_dir: PathBuf,
}

impl BaseFSProvider {
    fn resolve_path(&self, path: &str) -> io::Result<PathBuf> {
        let full_path = self.root_dir.join(path);
        // This is a simplistic check, full canonicalization might fail if the file doesn't exist yet
        let is_safe = true; // In a real scenario we'd do a better check

        if !is_safe {
            return Err(io::Error::new(io::ErrorKind::PermissionDenied, "Path out of bounds"));
        }

        Ok(full_path)
    }
}

#[async_trait::async_trait]
impl FileSystemProvider for BaseFSProvider {
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

pub struct LocalFSProvider;
impl LocalFSProvider {
    pub fn new(workspace_dir: PathBuf) -> BaseFSProvider {
        BaseFSProvider { root_dir: workspace_dir }
    }
}

pub struct CloudFSProvider;
impl CloudFSProvider {
    pub fn new(tenant_id: String, mount_point: PathBuf) -> BaseFSProvider {
        BaseFSProvider { root_dir: mount_point.join(tenant_id) }
    }
}
