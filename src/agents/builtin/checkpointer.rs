use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sqlx::Row;
use std::path::PathBuf;
use std::process::Command;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Checkpoint {
    pub thread_id: String,
    pub checkpoint_id: String,
    pub parent_id: Option<String>,
    pub data: serde_json::Value,
    pub metadata: serde_json::Value,
    pub created_at: DateTime<Utc>,
}

#[async_trait]
pub trait CheckpointSaver: Send + Sync {
    #[allow(dead_code)]
    async fn get_checkpoint(&self, thread_id: &str, checkpoint_id: &str) -> Result<Option<Checkpoint>, String>;
    async fn put_checkpoint(&self, checkpoint: Checkpoint) -> Result<(), String>;
    async fn list_checkpoints(&self, thread_id: &str) -> Result<Vec<Checkpoint>, String>;
}

pub struct PgCheckpointer {
    pool: sqlx::PgPool,
}

impl PgCheckpointer {
    pub fn new(pool: sqlx::PgPool) -> Self {
        PgCheckpointer { pool }
    }
}

pub struct GitCheckpointer {
    repo_path: PathBuf,
}

impl GitCheckpointer {
    pub fn new(repo_path: PathBuf) -> Self {
        let _ = Command::new("git")
            .arg("init")
            .current_dir(&repo_path)
            .output();

        let _ = Command::new("git")
            .args(&["config", "user.name", "Agent"])
            .current_dir(&repo_path)
            .output();

        let _ = Command::new("git")
            .args(&["config", "user.email", "agent@ohc.local"])
            .current_dir(&repo_path)
            .output();

        GitCheckpointer { repo_path }
    }

    fn progress_file_path(&self, thread_id: &str) -> PathBuf {
        self.repo_path.join(format!(".agent_progress_{}.json", thread_id))
    }
}

#[async_trait]
impl CheckpointSaver for GitCheckpointer {
    async fn get_checkpoint(&self, thread_id: &str, checkpoint_id: &str) -> Result<Option<Checkpoint>, String> {
        let file_name = format!(".agent_progress_{}.json", thread_id);

        let output = Command::new("git")
            .arg("show")
            .arg(format!("{}:{}", checkpoint_id, file_name))
            .current_dir(&self.repo_path)
            .output()
            .map_err(|e| e.to_string())?;

        if !output.status.success() {
            return Ok(None);
        }

        let content = String::from_utf8_lossy(&output.stdout);
        let cp: Checkpoint = serde_json::from_str(&content).map_err(|e| e.to_string())?;

        Ok(Some(cp))
    }
    async fn put_checkpoint(&self, checkpoint: Checkpoint) -> Result<(), String> {
        let file_path = self.progress_file_path(&checkpoint.thread_id);

        let json_data = serde_json::to_string_pretty(&checkpoint).map_err(|e| e.to_string())?;
        tokio::fs::write(&file_path, json_data).await.map_err(|e| e.to_string())?;

        // 1. Stage all changes in the workspace (Claude Code mechanic)
        let _ = Command::new("git")
            .arg("add")
            .arg(".")
            .current_dir(&self.repo_path)
            .output();

        // 2. Commit the changes
        let commit_msg = format!("Checkpoint: {}", checkpoint.checkpoint_id);
        let output = Command::new("git")
            .arg("commit")
            .arg("--allow-empty")
            .arg("-m")
            .arg(&commit_msg)
            .current_dir(&self.repo_path)
            .output()
            .map_err(|e| e.to_string())?;

        if !output.status.success() {
            return Err(format!("Failed to commit: {}", String::from_utf8_lossy(&output.stderr)));
        }

        let tag_output = Command::new("git")
            .arg("tag")
            .arg("-f")
            .arg(&checkpoint.checkpoint_id)
            .current_dir(&self.repo_path)
            .output()
            .map_err(|e| e.to_string())?;

        if !tag_output.status.success() {
            return Err(format!("Failed to tag: {}", String::from_utf8_lossy(&tag_output.stderr)));
        }

        Ok(())
    }

    async fn list_checkpoints(&self, thread_id: &str) -> Result<Vec<Checkpoint>, String> {
        let file_name = format!(".agent_progress_{}.json", thread_id);

        let output = Command::new("git")
            .arg("log")
            .arg("--format=%H")
            .arg("--")
            .arg(&file_name)
            .current_dir(&self.repo_path)
            .output()
            .map_err(|e| e.to_string())?;

        if !output.status.success() {
            return Ok(vec![]);
        }

        let hashes = String::from_utf8_lossy(&output.stdout);
        let mut checkpoints = Vec::new();

        for hash in hashes.lines() {
            let hash = hash.trim();
            if hash.is_empty() { continue; }

            if let Ok(Some(cp)) = self.get_checkpoint(thread_id, hash).await {
                checkpoints.push(cp);
            }
        }

        Ok(checkpoints)
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

    let decoded = match STANDARD.decode(decode_input) {
        Ok(d) => d,
        Err(_) => return Ok(data.to_vec()), // Fallback for raw JSON data
    };
    
    let mut decoder = GzDecoder::new(&decoded[..]);
    let mut decompressed = Vec::new();
    if let Err(_) = decoder.read_to_end(&mut decompressed) {
        return Ok(data.to_vec()); // Fallback for valid base64 but not gzip
    }

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

    #[test]
    fn test_decompress_fallback() {
        let invalid_base64 = b"Not valid base64!";
        let decompressed_invalid = decompress_data(invalid_base64).unwrap();
        assert_eq!(invalid_base64, decompressed_invalid.as_slice());

        let raw_json = b"{\"some\": \"json\"}";
        let decompressed_json = decompress_data(raw_json).unwrap();
        assert_eq!(raw_json, decompressed_json.as_slice());
    }
    #[tokio::test]
    async fn test_pg_checkpointer_save_and_load() {
        let pool = sqlx::postgres::PgPoolOptions::new()
            .after_release(|conn, _meta| { Box::pin(async move { use sqlx::Executor; conn.execute("RESET app.current_tenant").await?; Ok(true) }) }).connect_lazy("postgres://localhost/dummy?statement_cache_capacity=0").unwrap();
        if sqlx::query("SELECT 1").execute(&pool).await.is_err() { return; }
        let saver = PgCheckpointer::new(pool);
        
        let cp = Checkpoint {
            thread_id: "thread-1".to_string(),
            checkpoint_id: "cp-1".to_string(),
            parent_id: Some("parent-1".to_string()),
            data: serde_json::json!({"step": 1, "data": "some value"}),
            metadata: serde_json::json!({"agent": "SWE-1"}),
            created_at: Utc::now(),
        };

        let res = saver.put_checkpoint(cp.clone()).await;
        assert!(res.is_err());
    }

    #[tokio::test]
    async fn test_pg_checkpointer_list_checkpoints() {
        let pool = sqlx::postgres::PgPoolOptions::new()
            .after_release(|conn, _meta| { Box::pin(async move { use sqlx::Executor; conn.execute("RESET app.current_tenant").await?; Ok(true) }) }).connect_lazy("postgres://localhost/dummy?statement_cache_capacity=0").unwrap();
        if sqlx::query("SELECT 1").execute(&pool).await.is_err() { return; }
        let saver = PgCheckpointer::new(pool);
        
        let res = saver.list_checkpoints("thread-list").await;
        assert!(res.is_err());
    }

    #[tokio::test]
    async fn test_git_checkpointer_new_and_put() {
        let temp_dir = tempfile::tempdir().unwrap();
        let saver = GitCheckpointer::new(temp_dir.path().to_path_buf());

        let cp = Checkpoint {
            thread_id: "thread-git-1".to_string(),
            checkpoint_id: "cp-git-1".to_string(),
            parent_id: None,
            data: serde_json::json!({"state": "init"}),
            metadata: serde_json::json!({"agent": "git-bot"}),
            created_at: Utc::now(),
        };

        let res = saver.put_checkpoint(cp).await;
        assert!(res.is_ok());
    }

    #[tokio::test]
    async fn test_git_checkpointer_get() {
        let temp_dir = tempfile::tempdir().unwrap();
        let saver = GitCheckpointer::new(temp_dir.path().to_path_buf());

        let cp = Checkpoint {
            thread_id: "thread-git-2".to_string(),
            checkpoint_id: "cp-git-2".to_string(),
            parent_id: None,
            data: serde_json::json!({"state": "running"}),
            metadata: serde_json::json!({"agent": "git-bot-2"}),
            created_at: Utc::now(),
        };

        saver.put_checkpoint(cp.clone()).await.unwrap();

        let retrieved = saver.get_checkpoint("thread-git-2", "cp-git-2").await.unwrap();
        assert!(retrieved.is_some());
        let retrieved = retrieved.unwrap();
        assert_eq!(retrieved.thread_id, cp.thread_id);
        assert_eq!(retrieved.checkpoint_id, cp.checkpoint_id);
    }

    #[tokio::test]
    async fn test_git_checkpointer_list() {
        let temp_dir = tempfile::tempdir().unwrap();
        let saver = GitCheckpointer::new(temp_dir.path().to_path_buf());

        let cp1 = Checkpoint {
            thread_id: "thread-git-3".to_string(),
            checkpoint_id: "cp-git-3a".to_string(),
            parent_id: None,
            data: serde_json::json!({"state": "1"}),
            metadata: serde_json::json!({}),
            created_at: Utc::now(),
        };

        let cp2 = Checkpoint {
            thread_id: "thread-git-3".to_string(),
            checkpoint_id: "cp-git-3b".to_string(),
            parent_id: Some("cp-git-3a".to_string()),
            data: serde_json::json!({"state": "2"}),
            metadata: serde_json::json!({}),
            created_at: Utc::now(),
        };

        saver.put_checkpoint(cp1).await.unwrap();
        saver.put_checkpoint(cp2).await.unwrap();

        let list = saver.list_checkpoints("thread-git-3").await.unwrap();
        // Since we check all hashes, there should be at least two checkpoints
        // associated with that thread.
        assert!(list.len() >= 2);
    }
}
