use async_trait::async_trait;
use std::collections::HashMap;
use std::path::PathBuf;
use tokio::fs;
use tokio::io::AsyncWriteExt;
use serde::{Deserialize, Serialize};
use crate::memory_store::LongTermMemory;

#[derive(Debug)]
pub struct FileBasedMemory {
    base_dir: std::path::PathBuf,
}

impl FileBasedMemory {
    pub fn new<P: AsRef<std::path::Path>>(base_dir: P) -> Self {
        FileBasedMemory {
            base_dir: base_dir.as_ref().to_path_buf(),
        }
    }

    fn secure_join(&self, elem: &[&str]) -> Result<std::path::PathBuf, String> {
        let mut path = self.base_dir.clone();
        for e in elem {
            if e.contains("..") {
                return Err("path traversal detected (..)".to_string());
            }
            path.push(e);
        }
        if !path.starts_with(&self.base_dir) {
            return Err("invalid path: attempts to traverse outside base directory".to_string());
        }
        Ok(path)
    }
}

#[async_trait]
impl LongTermMemory for FileBasedMemory {
    async fn retrieve(&self, namespace: &str, limit: usize) -> Result<Vec<String>, String> {
        let parts: Vec<&str> = namespace.split(' ').collect();
        if parts.len() < 2 { return Ok(Vec::new()) }
        let path = self.secure_join(&[parts[0], parts[1]])?;
        let data = tokio::fs::read(path).await.map_err(|e| e.to_string())?;
        Ok(vec![String::from_utf8_lossy(&data).to_string()])
    }
    async fn store(&self, content: &str, metadata: Vec<String>) -> Result<(), String> {
        if metadata.len() < 2 { return Ok(()) }
        let namespace = &metadata[0];
        let key = &metadata[1];
        let dir = self.secure_join(&[namespace])?;
        tokio::fs::create_dir_all(&dir).await.map_err(|e| e.to_string())?;
        let path = dir.join(key);
        let mut file = tokio::fs::File::create(path).await.map_err(|e| e.to_string())?;
        file.write_all(content.as_bytes()).await.map_err(|e| e.to_string())?;
        Ok(())
    }

    async fn retrieve_topic(&self, _topic_name: &str) -> Result<String, String> { Ok(String::new()) }
    async fn search_transcripts(&self, _query: &str, _limit: usize) -> Result<Vec<String>, String> { Ok(Vec::new()) }




}