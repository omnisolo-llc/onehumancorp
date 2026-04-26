use async_trait::async_trait;
use chrono::{DateTime, Utc};
use std::sync::Arc;
use sqlx::Row;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[allow(dead_code)]
pub struct Checkpoint {
    pub thread_id: String,
    pub checkpoint_id: String,
    pub parent_id: Option<String>,
    pub data: serde_json::Value,
    pub metadata: serde_json::Value,
    pub created_at: DateTime<Utc>,
}

#[async_trait]
#[allow(dead_code)]
pub trait CheckpointSaver: Send + Sync {
    async fn get_checkpoint(&self, thread_id: &str, checkpoint_id: &str) -> Result<Option<Checkpoint>, String>;
    async fn put_checkpoint(&self, checkpoint: Checkpoint) -> Result<(), String>;
    async fn list_checkpoints(&self, thread_id: &str) -> Result<Vec<Checkpoint>, String>;
}

#[allow(dead_code)]
pub struct PgCheckpointer {
    pool: sqlx::PgPool,
}

#[allow(dead_code)]
impl PgCheckpointer {
    pub fn new(pool: sqlx::PgPool) -> Self {
        PgCheckpointer { pool }
    }
}

#[async_trait]
impl CheckpointSaver for PgCheckpointer {
    async fn get_checkpoint(&self, thread_id: &str, checkpoint_id: &str) -> Result<Option<Checkpoint>, String> {
        let row = sqlx::query(
            "SELECT thread_id, checkpoint_id, parent_id, checkpoint, metadata, created_at FROM swarm_checkpoints WHERE thread_id = $1 AND checkpoint_id = $2"
        )
        .bind(thread_id)
        .bind(checkpoint_id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

        if let Some(row) = row {
            let thread_id: String = row.get("thread_id");
            let checkpoint_id: String = row.get("checkpoint_id");
            let parent_id: Option<String> = row.get("parent_id");
            let checkpoint_raw: Vec<u8> = row.get("checkpoint");
            let metadata_raw: Vec<u8> = row.get("metadata");
            let created_at: DateTime<Utc> = row.get("created_at");

            let decompressed_data = decompress_data(&checkpoint_raw)?;
            let data: serde_json::Value = serde_json::from_slice(&decompressed_data).map_err(|e| e.to_string())?;
            let metadata: serde_json::Value = serde_json::from_slice(&metadata_raw).map_err(|e| e.to_string())?;

            Ok(Some(Checkpoint {
                thread_id,
                checkpoint_id,
                parent_id,
                data,
                metadata,
                created_at,
            }))
        } else {
            Ok(None)
        }
    }

    async fn put_checkpoint(&self, checkpoint: Checkpoint) -> Result<(), String> {
        let data_bytes = serde_json::to_vec(&checkpoint.data).map_err(|e| e.to_string())?;
        let compressed_data = compress_data(&data_bytes)?;
        let metadata_bytes = serde_json::to_vec(&checkpoint.metadata).map_err(|e| e.to_string())?;

        sqlx::query(
            "INSERT INTO swarm_checkpoints (thread_id, checkpoint_id, parent_id, checkpoint, metadata, created_at) VALUES ($1, $2, $3, $4, $5, $6) ON CONFLICT (thread_id, checkpoint_id) DO UPDATE SET parent_id = EXCLUDED.parent_id, checkpoint = EXCLUDED.checkpoint, metadata = EXCLUDED.metadata, created_at = EXCLUDED.created_at"
        )
        .bind(checkpoint.thread_id)
        .bind(checkpoint.checkpoint_id)
        .bind(checkpoint.parent_id)
        .bind(compressed_data)
        .bind(metadata_bytes)
        .bind(checkpoint.created_at)
        .execute(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

        Ok(())
    }

    async fn list_checkpoints(&self, thread_id: &str) -> Result<Vec<Checkpoint>, String> {
        let rows = sqlx::query(
            "SELECT thread_id, checkpoint_id, parent_id, checkpoint, metadata, created_at FROM swarm_checkpoints WHERE thread_id = $1 ORDER BY created_at DESC"
        )
        .bind(thread_id)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

        let mut checkpoints = Vec::new();
        for row in rows {
            let thread_id: String = row.get("thread_id");
            let checkpoint_id: String = row.get("checkpoint_id");
            let parent_id: Option<String> = row.get("parent_id");
            let checkpoint_raw: Vec<u8> = row.get("checkpoint");
            let metadata_raw: Vec<u8> = row.get("metadata");
            let created_at: DateTime<Utc> = row.get("created_at");

            let decompressed_data = decompress_data(&checkpoint_raw)?;
            let data: serde_json::Value = serde_json::from_slice(&decompressed_data).map_err(|e| e.to_string())?;
            let metadata: serde_json::Value = serde_json::from_slice(&metadata_raw).map_err(|e| e.to_string())?;

            checkpoints.push(Checkpoint {
                thread_id,
                checkpoint_id,
                parent_id,
                data,
                metadata,
                created_at,
            });
        }

        Ok(checkpoints)
    }
}

fn compress_data(data: &[u8]) -> Result<Vec<u8>, String> {
    use flate2::write::GzEncoder;
    use flate2::Compression;
    use std::io::Write;

    let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(data).map_err(|e| e.to_string())?;
    let compressed = encoder.finish().map_err(|e| e.to_string())?;
    
    use base64::engine::general_purpose::STANDARD;
    use base64::Engine;
    
    let b64 = STANDARD.encode(&compressed);
    let mut result = Vec::new();
    result.push(b'"');
    result.extend_from_slice(b64.as_bytes());
    result.push(b'"');
    
    Ok(result)
}

fn decompress_data(data: &[u8]) -> Result<Vec<u8>, String> {
    use base64::engine::general_purpose::STANDARD;
    use base64::Engine;
    use flate2::read::GzDecoder;
    use std::io::Read;

    let is_quoted = data.len() >= 2 && data[0] == b'"' && data[data.len() - 1] == b'"';
    let decode_input = if is_quoted {
        &data[1..data.len() - 1]
    } else {
        data
    };

    let decoded = STANDARD.decode(decode_input).map_err(|e| e.to_string())?;
    
    let mut decoder = GzDecoder::new(&decoded[..]);
    let mut decompressed = Vec::new();
    decoder.read_to_end(&mut decompressed).map_err(|e| e.to_string())?;
    
    Ok(decompressed)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compress_decompress() {
        let data = b"Hello, world! This is a test of compression and decompression.";
        let compressed = compress_data(data).unwrap();
        let decompressed = decompress_data(&compressed).unwrap();
        assert_eq!(data, decompressed.as_slice());
    }
    
    #[test]
    fn test_decompress_unquoted() {
        let data = b"Hello, world!";
        use flate2::write::GzEncoder;
        use flate2::Compression;
        use std::io::Write;

        let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
        encoder.write_all(data).unwrap();
        let compressed = encoder.finish().unwrap();
        
        use base64::engine::general_purpose::STANDARD;
        use base64::Engine;
        
        let b64 = STANDARD.encode(&compressed);
        
        let decompressed = decompress_data(b64.as_bytes()).unwrap();
        assert_eq!(data, decompressed.as_slice());
    }
}
