use async_trait::async_trait;
use chrono::Utc;
use std::io;
use super::provider::{BlobMetadata, Provider};

pub struct S3Provider {
    bucket_name: String,
}

impl S3Provider {
    pub fn new(bucket_name: String) -> Self {
        S3Provider { bucket_name }
    }
}

#[async_trait]
impl Provider for S3Provider {
    fn is_local(&self) -> bool {
        false
    }

    async fn list_blobs(&self, _prefix: &str) -> io::Result<Vec<BlobMetadata>> {
        // STUB
        Ok(vec![])
    }

    async fn read_blob_metadata(&self, key: &str) -> io::Result<BlobMetadata> {
        // STUB
        Ok(BlobMetadata {
            key: key.to_string(),
            size: 1024,
            last_modified: Utc::now(),
            content_type: "application/octet-stream".to_string(),
        })
    }

    async fn get_blob_url(&self, key: &str) -> io::Result<String> {
        // STUB: Return a fake presigned URL
        Ok(format!("https://s3.amazonaws.com/{}/{}?X-Amz-Signature=stub", self.bucket_name, key))
    }

    async fn read_blob(&self, _key: &str) -> io::Result<Vec<u8>> {
        // STUB
        Ok(vec![])
    }

    async fn write_blob(&self, _key: &str, _data: &[u8]) -> io::Result<()> {
        // STUB
        Ok(())
    }
}
