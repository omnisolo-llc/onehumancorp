use std::path::{Path, PathBuf, Component};
use std::io;

#[async_trait::async_trait]
pub trait FileSystemProvider: Send + Sync {
    async fn read_file(&self, path: &str) -> io::Result<Vec<u8>>;
    async fn write_file(&self, path: &str, content: &[u8]) -> io::Result<()>;
    async fn list_dir(&self, path: &str) -> io::Result<Vec<String>>;
    async fn search_files(&self, path: &str, query: &str) -> io::Result<Vec<String>>;
}

fn normalize_path(path: &Path) -> PathBuf {
    let mut normalized = PathBuf::new();
    for component in path.components() {
        match component {
            Component::ParentDir => {
                normalized.pop();
            }
            Component::Normal(c) => {
                normalized.push(c);
            }
            Component::RootDir => {
                normalized.push(Component::RootDir);
            }
            Component::Prefix(prefix) => {
                normalized.push(Component::Prefix(prefix));
            }
            Component::CurDir => {}
        }
    }
    normalized
}

pub struct BaseFSProvider {
    root_dir: PathBuf,
}

impl BaseFSProvider {
    fn resolve_path(&self, path_str: &str) -> io::Result<PathBuf> {
        let path = Path::new(path_str);
        // Prevent absolute path bypassing
        let path = if path.is_absolute() {
            path.strip_prefix("/").unwrap_or(path)
        } else {
            path
        };

        let full_path = self.root_dir.join(path);
        let normalized = normalize_path(&full_path);
        let root_normalized = normalize_path(&self.root_dir);

        if !normalized.starts_with(&root_normalized) {
            return Err(io::Error::new(io::ErrorKind::PermissionDenied, "Path out of bounds"));
        }

        Ok(normalized)
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

    async fn search_files(&self, path: &str, query: &str) -> io::Result<Vec<String>> {
        let resolved = self.resolve_path(path)?;
        let mut result = Vec::new();
        let query = query.to_string();

        let mut dirs_to_visit = vec![resolved.clone()];
        while let Some(dir) = dirs_to_visit.pop() {
            if let Ok(mut entries) = tokio::fs::read_dir(&dir).await {
                while let Ok(Some(entry)) = entries.next_entry().await {
                    let path = entry.path();
                    if path.is_dir() {
                        dirs_to_visit.push(path);
                    } else if let Ok(name) = entry.file_name().into_string() {
                        if name.contains(&query) {
                            if let Ok(rel_path) = path.strip_prefix(&resolved) {
                                result.push(rel_path.to_string_lossy().to_string());
                            } else {
                                result.push(name);
                            }
                        }
                    }
                }
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
