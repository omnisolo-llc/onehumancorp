use async_trait::async_trait;
use chrono::{DateTime, Utc};
use std::io;

#[derive(Debug, Clone)]
pub struct BlobMetadata {
    pub key: String,
    pub size: i64,
    pub last_modified: DateTime<Utc>,
    pub content_type: String,
}

#[async_trait]
pub trait Provider: Send + Sync {
    /// IsLocal returns true if the provider is a local filesystem.
    fn is_local(&self) -> bool;
    
    /// ListBlobs returns a list of blob metadata under a given prefix.
    async fn list_blobs(&self, prefix: &str) -> io::Result<Vec<BlobMetadata>>;
    
    /// ReadBlobMetadata returns the metadata for a single blob.
    async fn read_blob_metadata(&self, key: &str) -> io::Result<BlobMetadata>;
    
    /// GetBlobURL returns a presigned or accessible URL for the blob.
    async fn get_blob_url(&self, key: &str) -> io::Result<String>;

    /// ReadBlob reads the content of a blob.
    async fn read_blob(&self, key: &str) -> io::Result<Vec<u8>>;
    
    /// WriteBlob writes the content of a blob.
    async fn write_blob(&self, key: &str, data: &[u8]) -> io::Result<()>;
}
