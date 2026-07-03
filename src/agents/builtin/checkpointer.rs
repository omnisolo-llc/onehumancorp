use async_trait::async_trait;
use chrono::{DateTime, Utc};
/// Master Catalog B.7. State Management
use serde::{Deserialize, Serialize};
use sqlx::Row;
use std::path::PathBuf;
use std::process::Command as StdCommand;
use tokio::process::Command;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Checkpoint {
    pub thread_id: String,
    pub checkpoint_id: String,
    pub parent_id: Option<String>,
    pub data: serde_json::Value,
    pub metadata: serde_json::Value,
    pub created_at: DateTime<Utc>,
}

/// Local progress files as structured scratchpads (Claude Code mechanic)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProgressFile {
    pub current_objective: String,
    pub status: String,
    pub notes: Vec<String>,
}

impl Default for ProgressFile {
    fn default() -> Self {
        Self {
            current_objective: "Uninitialized".to_string(),
            status: "pending".to_string(),
            notes: vec![],
        }
    }
}

#[async_trait]
pub trait CheckpointSaver: Send + Sync {
    async fn get_checkpoint(
        &self,
        thread_id: &str,
        checkpoint_id: &str,
    ) -> Result<Option<Checkpoint>, String>;
    async fn put_checkpoint(&self, checkpoint: Checkpoint) -> Result<(), String>;
    async fn list_checkpoints(&self, thread_id: &str) -> Result<Vec<Checkpoint>, String>;
    async fn list_threads(&self) -> Result<Vec<String>, String> {
        Ok(vec![])
    }
    #[allow(unused_variables)]
    async fn restore_checkpoint(&self, checkpoint_id: &str) -> Result<(), String> {
        Ok(())
    }
    fn storage_prefix(&self) -> &'static str {
        "db"
    }
}

pub struct PgCheckpointer {
    pool: sqlx::PgPool,
}

impl PgCheckpointer {
    pub fn new(pool: sqlx::PgPool) -> Self {
        PgCheckpointer { pool }
    }
}

/// GitCheckpointer implements the Master Catalog "State Management: Git Commit Checkpointing" mechanic
/// inspired by Claude Code. It handles true time-travel debugging and progress file management
/// by executing native `git` commands on a local repository scratchpad.
///
/// **The Claude Code Mechanic:**
/// 1. Uses git commits as checkpoints at super-step boundaries.
/// 2. Maintains local `progress files` as structured scratchpads for the Ralph Loop and agent context.
/// 3. Enables the orchestrator to revert the entire workspace state reliably on LLM-recoverable errors
///    or user rollbacks via `git reset --hard` and `git clean -fdx`.
pub struct GitCheckpointer {
    // State Management: Git Commit Checkpointing Mechanic
    repo_path: PathBuf,
}

impl GitCheckpointer {
    fn safe_tag_name(id: &str) -> String {
        format!(
            "checkpoint-{}",
            id.replace(|c: char| !c.is_alphanumeric() && c != '-' && c != '_', "_")
        )
    }

    pub fn scratchpad_file_path(&self, thread_id: &str) -> PathBuf {
        self.repo_path
            .join(format!(".scratchpad_{}.json", thread_id))
    }

    pub fn new(repo_path: PathBuf) -> Self {
        // Run git init, check error
        let init_out = StdCommand::new("git")
            .arg("init")
            .current_dir(&repo_path)
            .output()
            .expect("Failed to execute git init");
        if !init_out.status.success() {
            tracing::warn!(
                "git init failed: {}",
                String::from_utf8_lossy(&init_out.stderr)
            );
        }

        let name_out = StdCommand::new("git")
            .args(["config", "user.name", "Agent"])
            .current_dir(&repo_path)
            .output()
            .expect("Failed to execute git config user.name");
        if !name_out.status.success() {
            tracing::warn!(
                "git config user.name failed: {}",
                String::from_utf8_lossy(&name_out.stderr)
            );
        }

        let err_out = StdCommand::new("git")
            .args(["config", "user.email", "agent@ohc.local"])
            .current_dir(&repo_path)
            .output()
            .expect("Failed to execute git config user.email");
        if !err_out.status.success() {
            tracing::warn!(
                "git cmd failed (err): {}",
                String::from_utf8_lossy(&err_out.stderr)
            );
        }

        GitCheckpointer { repo_path }
    }

    fn progress_file_path(&self, thread_id: &str) -> PathBuf {
        self.repo_path
            .join(format!(".agent_progress_{}.json", thread_id))
    }

    pub async fn merge_scratchpad_state(
        scratchpad_path: &std::path::PathBuf,
        checkpoint_id: &str,
    ) -> Result<serde_json::Value, String> {
        let mut scratchpad_json_val = serde_json::to_value(ProgressFile::default()).unwrap();

        if scratchpad_path.exists() {
            if let Ok(content) = tokio::fs::read_to_string(scratchpad_path).await {
                if let Ok(mut ralph_prog) =
                    serde_json::from_str::<crate::ralph_loop::RalphProgress>(&content)
                {
                    ralph_prog
                        .notes
                        .push(format!("Checkpoint {}", checkpoint_id));
                    scratchpad_json_val = serde_json::to_value(&ralph_prog).unwrap();
                } else if let Ok(mut generic_json) =
                    serde_json::from_str::<serde_json::Value>(&content)
                {
                    if let Some(obj) = generic_json.as_object_mut() {
                        obj.insert(
                            "current_objective".to_string(),
                            serde_json::Value::String(format!("Checkpoint {}", checkpoint_id)),
                        );
                    }
                    scratchpad_json_val = generic_json;
                }
            }
        } else {
            let pf = ProgressFile {
                current_objective: format!("Checkpoint {}", checkpoint_id),
                ..Default::default()
            };
            scratchpad_json_val = serde_json::to_value(&pf).unwrap();
        }
        Ok(scratchpad_json_val)
    }
}

#[async_trait]
impl CheckpointSaver for GitCheckpointer {
    async fn get_checkpoint(
        &self,
        thread_id: &str,
        checkpoint_id: &str,
    ) -> Result<Option<Checkpoint>, String> {
        let file_name = format!(".agent_progress_{}.json", thread_id);

        // Try safe tag name first, then fallback to legacy checkpoint-, then raw
        let refs_to_try = vec![
            Self::safe_tag_name(checkpoint_id),
            format!("checkpoint-{}", checkpoint_id),
            checkpoint_id.to_string(),
        ];

        let mut output = None;

        for target_ref in refs_to_try {
            let res = Command::new("git")
                .arg("show")
                .arg(format!("{}:{}", target_ref, file_name))
                .current_dir(&self.repo_path)
                .output()
                .await
                .map_err(|e| e.to_string())?;

            if res.status.success() {
                output = Some(res);
                break;
            }
        }

        let output = match output {
            Some(o) => o,
            None => return Ok(None),
        };

        let content = String::from_utf8_lossy(&output.stdout);
        let cp: Checkpoint = serde_json::from_str(&content).map_err(|e| e.to_string())?;

        Ok(Some(cp))
    }

    fn storage_prefix(&self) -> &'static str {
        "git"
    }

    async fn put_checkpoint(&self, checkpoint: Checkpoint) -> Result<(), String> {
        let file_path = self.progress_file_path(&checkpoint.thread_id);
        let scratchpad_path = self.scratchpad_file_path(&checkpoint.thread_id);

        let json_data = serde_json::to_string_pretty(&checkpoint).map_err(|e| e.to_string())?;
        tokio::fs::write(&file_path, json_data)
            .await
            .map_err(|e| e.to_string())?;

        let scratchpad_json_val =
            Self::merge_scratchpad_state(&scratchpad_path, &checkpoint.checkpoint_id).await?;

        let scratchpad_json =
            serde_json::to_string_pretty(&scratchpad_json_val).map_err(|e| e.to_string())?;
        tokio::fs::write(&scratchpad_path, scratchpad_json)
            .await
            .map_err(|e| e.to_string())?;

        // 0. Conflict Resolution / Stale Lock Files: Ensure no stale git index lock prevents us from adding files
        let lock_file = self.repo_path.join(".git/index.lock");
        if lock_file.exists() {
            let _ = tokio::fs::remove_file(&lock_file).await;
            tracing::warn!("Removed stale git index.lock file before checkpointing.");
        }

        // 0.5. Missing .gitignore defaults: Ensure we don't snapshot massive build directories if user forgot to ignore them
        let gitignore_path = self.repo_path.join(".gitignore");
        if !gitignore_path.exists() {
            let default_ignore =
                "target/\nnode_modules/\n.idea/\n.vscode/\ndist/\nbuild/\n.scratchpad_*.json\n";
            let _ = tokio::fs::write(&gitignore_path, default_ignore).await;
            tracing::info!("Created default .gitignore to prevent massive snapshotting.");
        } else {
            let content = tokio::fs::read_to_string(&gitignore_path)
                .await
                .unwrap_or_default();
            if !content.contains(".scratchpad_*.json") {
                let _ = tokio::fs::write(
                    &gitignore_path,
                    format!("{}\n.scratchpad_*.json\n", content),
                )
                .await;
            }
        }

        // 1. Stage ALL modified files in the workspace to allow true time-travel debugging
        let add_out = Command::new("git")
            .arg("add")
            .arg("-A")
            .current_dir(&self.repo_path)
            .output()
            .await
            .map_err(|e| format!("Failed to execute git add: {}", e))?;

        if !add_out.status.success() {
            return Err(format!(
                "git add failed: {}",
                String::from_utf8_lossy(&add_out.stderr)
            ));
        }

        // 2. Commit the changes
        let commit_msg = format!("Checkpoint: {}", checkpoint.checkpoint_id);
        let output = Command::new("git")
            .arg("commit")
            .arg("--allow-empty")
            .arg("-m")
            .arg(&commit_msg)
            .current_dir(&self.repo_path)
            .output()
            .await
            .map_err(|e| format!("Failed to execute git commit: {}", e))?;

        if !output.status.success() {
            return Err(format!(
                "Failed to commit: {}",
                String::from_utf8_lossy(&output.stderr)
            ));
        }

        let tag_name = Self::safe_tag_name(&checkpoint.checkpoint_id);
        let tag_output = Command::new("git")
            .arg("tag")
            .arg("-f")
            .arg(&tag_name)
            .current_dir(&self.repo_path)
            .output()
            .await
            .map_err(|e| format!("Failed to execute git tag: {}", e))?;

        if !tag_output.status.success() {
            return Err(format!(
                "Failed to tag: {}",
                String::from_utf8_lossy(&tag_output.stderr)
            ));
        }

        Ok(())
    }

    async fn restore_checkpoint(&self, checkpoint_id: &str) -> Result<(), String> {
        let refs_to_try = vec![
            Self::safe_tag_name(checkpoint_id),
            format!("checkpoint-{}", checkpoint_id),
            checkpoint_id.to_string(),
        ];

        // 1. Stash uncommitted and untracked changes to support safe time-travel debugging
        let stash_out = Command::new("git")
            .arg("stash")
            .arg("push")
            .arg("--include-untracked")
            .arg("-m")
            .arg(format!(
                "Auto-stash before restoring checkpoint {}",
                checkpoint_id
            ))
            .current_dir(&self.repo_path)
            .output()
            .await
            .map_err(|e| e.to_string())?;

        if !stash_out.status.success() {
            tracing::warn!(
                "Auto-stash failed: {}",
                String::from_utf8_lossy(&stash_out.stderr)
            );
        }

        // 2. Pre-clean to remove any remaining untracked files (that couldn't be stashed) that might block the checkout
        let pre_clean = Command::new("git")
            .arg("clean")
            .arg("-fdx")
            .current_dir(&self.repo_path)
            .output()
            .await
            .map_err(|e| e.to_string())?;

        if !pre_clean.status.success() {
            tracing::warn!(
                "Pre-clean failed: {}",
                String::from_utf8_lossy(&pre_clean.stderr)
            );
        }

        // 3. Reset HEAD to ensure we are in a clean state before checkout
        let reset_head = Command::new("git")
            .arg("reset")
            .arg("--hard")
            .arg("HEAD")
            .current_dir(&self.repo_path)
            .output()
            .await
            .map_err(|e| e.to_string())?;

        if !reset_head.status.success() {
            tracing::warn!(
                "Reset HEAD failed: {}",
                String::from_utf8_lossy(&reset_head.stderr)
            );
        }

        let branch_name = format!(
            "agent-restore-{}",
            Self::safe_tag_name(checkpoint_id).replace("checkpoint-", "")
        );
        let mut success = false;
        let mut last_err = String::new();

        // 3. Checkout the target tag into a new branch
        for target_ref in refs_to_try {
            let output = Command::new("git")
                .arg("checkout")
                .arg("-B")
                .arg(&branch_name)
                .arg(&target_ref)
                .current_dir(&self.repo_path)
                .output()
                .await
                .map_err(|e| e.to_string())?;

            if output.status.success() {
                success = true;
                break;
            } else {
                last_err = String::from_utf8_lossy(&output.stderr).into_owned();
            }
        }

        if !success {
            return Err(format!(
                "Failed to restore workspace (checkout): {}",
                last_err
            ));
        }

        // 4. Robust Restore Edge Cases: Reset to HEAD of the new branch and clean remaining untracked and ignored files to ensure spotless working tree.
        let reset_branch = Command::new("git")
            .arg("reset")
            .arg("--hard")
            .arg("HEAD")
            .current_dir(&self.repo_path)
            .output()
            .await
            .map_err(|e| e.to_string())?;

        if !reset_branch.status.success() {
            return Err(format!(
                "Failed to restore workspace (reset branch): {}",
                String::from_utf8_lossy(&reset_branch.stderr)
            ));
        }

        let clean_output = Command::new("git")
            .arg("clean")
            .arg("-fdx")
            .current_dir(&self.repo_path)
            .output()
            .await
            .map_err(|e| e.to_string())?;

        if !clean_output.status.success() {
            return Err(format!(
                "Failed to restore workspace (clean): {}",
                String::from_utf8_lossy(&clean_output.stderr)
            ));
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
            .await
            .map_err(|e| e.to_string())?;

        if !output.status.success() {
            return Ok(vec![]);
        }

        let hashes = String::from_utf8_lossy(&output.stdout);
        let mut checkpoints = Vec::new();

        for hash in hashes.lines() {
            let hash = hash.trim();
            if hash.is_empty() {
                continue;
            }

            if let Ok(Some(cp)) = self.get_checkpoint(thread_id, hash).await {
                checkpoints.push(cp);
            }
        }

        Ok(checkpoints)
    }

    async fn list_threads(&self) -> Result<Vec<String>, String> {
        let output = Command::new("git")
            .arg("ls-tree")
            .arg("-r")
            .arg("HEAD")
            .arg("--name-only")
            .current_dir(&self.repo_path)
            .output()
            .await
            .map_err(|e| e.to_string())?;

        if !output.status.success() {
            return Ok(vec![]);
        }

        let mut threads = std::collections::HashSet::new();
        let files = String::from_utf8_lossy(&output.stdout);
        for file in files.lines() {
            let file = file.trim();
            if file.starts_with(".agent_progress_") && file.ends_with(".json") {
                let thread_id = file
                    .trim_start_matches(".agent_progress_")
                    .trim_end_matches(".json");
                threads.insert(thread_id.to_string());
            }
        }

        Ok(threads.into_iter().collect())
    }
}

#[async_trait]
impl CheckpointSaver for PgCheckpointer {
    async fn get_checkpoint(
        &self,
        thread_id: &str,
        checkpoint_id: &str,
    ) -> Result<Option<Checkpoint>, String> {
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
            let data: serde_json::Value =
                serde_json::from_slice(&decompressed_data).map_err(|e| e.to_string())?;
            let metadata: serde_json::Value =
                serde_json::from_slice(&metadata_raw).map_err(|e| e.to_string())?;

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
            let data: serde_json::Value =
                serde_json::from_slice(&decompressed_data).map_err(|e| e.to_string())?;
            let metadata: serde_json::Value =
                serde_json::from_slice(&metadata_raw).map_err(|e| e.to_string())?;

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

    async fn list_threads(&self) -> Result<Vec<String>, String> {
        let rows = sqlx::query("SELECT DISTINCT thread_id FROM swarm_checkpoints")
            .fetch_all(&self.pool)
            .await
            .map_err(|e| e.to_string())?;

        let mut threads = Vec::new();
        for row in rows {
            let thread_id: String = sqlx::Row::get(&row, "thread_id");
            threads.push(thread_id);
        }

        Ok(threads)
    }

    async fn restore_checkpoint(&self, checkpoint_id: &str) -> Result<(), String> {
        // Fetch the target checkpoint's thread_id and created_at timestamp
        let row_opt = sqlx::query(
            "SELECT thread_id, created_at FROM swarm_checkpoints WHERE checkpoint_id = $1",
        )
        .bind(checkpoint_id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

        if let Some(row) = row_opt {
            let thread_id: String = row.get("thread_id");
            let target_time: DateTime<Utc> = row.get("created_at");

            // Delete all checkpoints for this thread that were created AFTER the target checkpoint
            sqlx::query("DELETE FROM swarm_checkpoints WHERE thread_id = $1 AND created_at > $2")
                .bind(thread_id)
                .bind(target_time)
                .execute(&self.pool)
                .await
                .map_err(|e| e.to_string())?;

            Ok(())
        } else {
            Err(format!(
                "Checkpoint {} not found in database",
                checkpoint_id
            ))
        }
    }
}

fn compress_data(data: &[u8]) -> Result<Vec<u8>, String> {
    use flate2::Compression;
    use flate2::write::GzEncoder;
    use std::io::Write;

    let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(data).map_err(|e| e.to_string())?;
    let compressed = encoder.finish().map_err(|e| e.to_string())?;

    use base64::Engine;
    use base64::engine::general_purpose::STANDARD;

    let b64 = STANDARD.encode(&compressed);
    let mut result = Vec::new();
    result.push(b'"');
    result.extend_from_slice(b64.as_bytes());
    result.push(b'"');

    Ok(result)
}

fn decompress_data(data: &[u8]) -> Result<Vec<u8>, String> {
    use base64::Engine;
    use base64::engine::general_purpose::STANDARD;
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
    if decoder.read_to_end(&mut decompressed).is_err() {
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
        use flate2::Compression;
        use flate2::write::GzEncoder;
        use std::io::Write;

        let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
        encoder.write_all(data).unwrap();
        let compressed = encoder.finish().unwrap();

        use base64::Engine;
        use base64::engine::general_purpose::STANDARD;

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
    async fn test_pg_checkpointer_restore() {
        // Fallback testing if Postgres is unavailable
        let pool = match sqlx::postgres::PgPoolOptions::new()
            .connect("postgres://postgres:postgres@localhost/postgres")
            .await
        {
            Ok(p) => p,
            Err(_) => {
                return;
            }
        };

        let _ = sqlx::query("CREATE TABLE IF NOT EXISTS swarm_checkpoints (thread_id TEXT, checkpoint_id TEXT, parent_id TEXT, checkpoint BYTEA, metadata BYTEA, created_at TIMESTAMPTZ, PRIMARY KEY (thread_id, checkpoint_id))").execute(&pool).await;

        // Clean up previous runs
        let _ = sqlx::query("DELETE FROM swarm_checkpoints WHERE thread_id = 'thread-restore-1'")
            .execute(&pool)
            .await;

        let saver = PgCheckpointer::new(pool.clone());

        let t1 = chrono::Utc::now() - chrono::Duration::hours(3);
        let t2 = chrono::Utc::now() - chrono::Duration::hours(2);
        let t3 = chrono::Utc::now() - chrono::Duration::hours(1);

        let cp1 = Checkpoint {
            thread_id: "thread-restore-1".to_string(),
            checkpoint_id: "cp-1".to_string(),
            parent_id: None,
            data: serde_json::json!({"step": 1}),
            metadata: serde_json::json!({}),
            created_at: t1,
        };

        let cp2 = Checkpoint {
            thread_id: "thread-restore-1".to_string(),
            checkpoint_id: "cp-2".to_string(),
            parent_id: Some("cp-1".to_string()),
            data: serde_json::json!({"step": 2}),
            metadata: serde_json::json!({}),
            created_at: t2,
        };

        let cp3 = Checkpoint {
            thread_id: "thread-restore-1".to_string(),
            checkpoint_id: "cp-3".to_string(),
            parent_id: Some("cp-2".to_string()),
            data: serde_json::json!({"step": 3}),
            metadata: serde_json::json!({}),
            created_at: t3,
        };

        saver.put_checkpoint(cp1.clone()).await.unwrap();
        saver.put_checkpoint(cp2.clone()).await.unwrap();
        saver.put_checkpoint(cp3.clone()).await.unwrap();

        // Ensure all 3 exist
        let all = saver.list_checkpoints("thread-restore-1").await.unwrap();
        assert_eq!(all.len(), 3);

        // Restore to middle one
        saver.restore_checkpoint("cp-2").await.unwrap();

        // Verify that cp-3 is gone, but cp-2 and cp-1 remain
        let after = saver.list_checkpoints("thread-restore-1").await.unwrap();
        assert_eq!(after.len(), 2);
        assert!(after.iter().any(|c| c.checkpoint_id == "cp-1"));
        assert!(after.iter().any(|c| c.checkpoint_id == "cp-2"));
        assert!(!after.iter().any(|c| c.checkpoint_id == "cp-3"));
    }

    #[tokio::test]
    async fn test_pg_checkpointer_save_and_load() {
        // Fallback testing if Postgres is unavailable
        let pool = match sqlx::postgres::PgPoolOptions::new()
            .connect("postgres://postgres:postgres@localhost/postgres")
            .await
        {
            Ok(p) => p,
            Err(_) => {
                // To achieve coverage without a real database, we must rely on mocking or just accept
                // that integration tests need a real DB. We'll skip gracefully if no DB.
                return;
            }
        };

        let _ = sqlx::query("CREATE TABLE IF NOT EXISTS swarm_checkpoints (thread_id TEXT, checkpoint_id TEXT, parent_id TEXT, checkpoint BYTEA, metadata BYTEA, created_at TIMESTAMPTZ, PRIMARY KEY (thread_id, checkpoint_id))").execute(&pool).await;

        let saver = PgCheckpointer::new(pool.clone());

        let cp = Checkpoint {
            thread_id: "thread-1".to_string(),
            checkpoint_id: "cp-1".to_string(),
            parent_id: Some("parent-1".to_string()),
            data: serde_json::json!({"step": 1, "data": "some value"}),
            metadata: serde_json::json!({"agent": "SWE-1"}),
            created_at: Utc::now(),
        };

        let res = saver.put_checkpoint(cp.clone()).await;
        assert!(res.is_ok());

        // Test get
        let get_res = saver.get_checkpoint("thread-1", "cp-1").await.unwrap();
        assert!(get_res.is_some());

        // Test list
        let list_res = saver.list_checkpoints("thread-1").await.unwrap();
        assert_eq!(list_res.len(), 1);

        // Test restore (success path)
        let restore_res = saver.restore_checkpoint("cp-1").await;
        assert!(restore_res.is_ok());

        let _ = sqlx::query("DROP TABLE IF EXISTS swarm_checkpoints")
            .execute(&pool)
            .await;
    }

    #[tokio::test]
    async fn test_pg_checkpointer_list_checkpoints() {
        let pool = sqlx::postgres::PgPoolOptions::new()
            .after_release(|conn, _meta| {
                Box::pin(async move {
                    use sqlx::Executor;
                    conn.execute("DISCARD ALL").await?;
                    Ok(true)
                })
            })
            .connect_lazy("postgres://localhost/dummy")
            .unwrap();

        let timeout_duration = std::time::Duration::from_millis(500);
        let query_future = sqlx::query("SELECT 1").execute(&pool);

        if tokio::time::timeout(timeout_duration, query_future)
            .await
            .is_err()
        {
            return; // Skip if database is unavailable or hangs
        }

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

        // Also verify the lockfile clearance logic
        let lock_file = temp_dir.path().join(".git/index.lock");
        tokio::fs::write(&lock_file, b"test").await.unwrap();
        assert!(lock_file.exists());

        let cp2 = Checkpoint {
            thread_id: "thread-git-1".to_string(),
            checkpoint_id: "cp-git-1-b".to_string(),
            parent_id: None,
            data: serde_json::json!({"state": "init2"}),
            metadata: serde_json::json!({"agent": "git-bot"}),
            created_at: Utc::now(),
        };
        let res2 = saver.put_checkpoint(cp2).await;
        assert!(res2.is_ok());
        assert!(
            !lock_file.exists(),
            "The checkpoint operation should remove the stale lockfile"
        );

        // Verify .gitignore creation
        let gitignore = temp_dir.path().join(".gitignore");
        assert!(gitignore.exists());
        let content = tokio::fs::read_to_string(&gitignore).await.unwrap();
        assert!(content.contains("node_modules/"));
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

        let retrieved = saver
            .get_checkpoint("thread-git-2", "cp-git-2")
            .await
            .unwrap();
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

    #[tokio::test]
    async fn test_git_checkpointer_restore() {
        let temp_dir = tempfile::tempdir().unwrap();
        let saver = GitCheckpointer::new(temp_dir.path().to_path_buf());

        let cp1 = Checkpoint {
            thread_id: "thread-git-restore".to_string(),
            checkpoint_id: "cp-restore-1".to_string(),
            parent_id: None,
            data: serde_json::json!({"state": "1"}),
            metadata: serde_json::json!({}),
            created_at: Utc::now(),
        };

        // Write a test file
        let file_path = temp_dir.path().join("test_file.txt");
        std::fs::write(&file_path, "state 1").unwrap();

        saver.put_checkpoint(cp1.clone()).await.unwrap();

        // Write new content to file
        std::fs::write(&file_path, "state 2").unwrap();

        let cp2 = Checkpoint {
            thread_id: "thread-git-restore".to_string(),
            checkpoint_id: "cp-restore-2".to_string(),
            parent_id: Some("cp-restore-1".to_string()),
            data: serde_json::json!({"state": "2"}),
            metadata: serde_json::json!({}),
            created_at: Utc::now(),
        };

        saver.put_checkpoint(cp2.clone()).await.unwrap();

        // Restore to first checkpoint
        saver.restore_checkpoint("cp-restore-1").await.unwrap();

        // Verify the checkpoint file was restored
        let progress_path = temp_dir
            .path()
            .join(format!(".agent_progress_{}.json", "thread-git-restore"));
        let content = std::fs::read_to_string(&progress_path).unwrap();
        assert!(content.contains(r#""state": "1""#));

        // Verify the tracked file was restored
        let file_content = std::fs::read_to_string(&file_path).unwrap();
        assert_eq!(file_content, "state 1");
    }

    #[tokio::test]
    async fn test_git_checkpointer_restore_missing() {
        let temp_dir = tempfile::tempdir().unwrap();
        let saver = GitCheckpointer::new(temp_dir.path().to_path_buf());

        // Attempting to restore a missing checkpoint should fail gracefully
        let result = saver.restore_checkpoint("non-existent-checkpoint").await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_git_checkpointer_ralph_progress_preservation() {
        let temp_dir = tempfile::tempdir().unwrap();
        let saver = GitCheckpointer::new(temp_dir.path().to_path_buf());
        let thread_id = "thread-ralph-preservation";

        // Pre-create a scratchpad that simulates a RalphLoop progress file
        let scratchpad_path = saver.scratchpad_file_path(thread_id);
        let initial_progress = crate::ralph_loop::RalphProgress {
            task_description: "Build a web server".to_string(),
            features: vec![crate::ralph_loop::Feature {
                name: "Step 1".to_string(),
                status: "completed".to_string(),
            }],
            current_feature_index: 1,
            notes: vec!["Initialized".to_string()],
            is_complete: false,
        };

        std::fs::write(
            &scratchpad_path,
            serde_json::to_string(&initial_progress).unwrap(),
        )
        .unwrap();

        let cp1 = Checkpoint {
            thread_id: thread_id.to_string(),
            checkpoint_id: "cp-ralph-1".to_string(),
            parent_id: None,
            data: serde_json::json!({"state": "1"}),
            metadata: serde_json::json!({}),
            created_at: Utc::now(),
        };

        // When put_checkpoint is called, it should intelligently merge rather than overwrite
        saver.put_checkpoint(cp1.clone()).await.unwrap();

        // Verify the scratchpad file still parses as RalphProgress and contains the new note
        let content = std::fs::read_to_string(&scratchpad_path).unwrap();
        let updated_progress: crate::ralph_loop::RalphProgress =
            serde_json::from_str(&content).unwrap();

        assert_eq!(updated_progress.task_description, "Build a web server");
        assert_eq!(updated_progress.features.len(), 1);
        assert_eq!(updated_progress.notes.len(), 2);
        assert!(updated_progress.notes[1].contains("Checkpoint cp-ralph-1"));
    }

    #[tokio::test]
    async fn test_git_checkpointer_generic_json_preservation() {
        let temp_dir = tempfile::tempdir().unwrap();
        let saver = GitCheckpointer::new(temp_dir.path().to_path_buf());
        let thread_id = "thread-generic-preservation";

        // Pre-create a scratchpad with generic JSON
        let scratchpad_path = saver.scratchpad_file_path(thread_id);
        let initial_json = serde_json::json!({
            "unknown_field": "val1",
            "current_objective": "old_obj"
        });

        std::fs::write(
            &scratchpad_path,
            serde_json::to_string(&initial_json).unwrap(),
        )
        .unwrap();

        let cp1 = Checkpoint {
            thread_id: thread_id.to_string(),
            checkpoint_id: "cp-generic-1".to_string(),
            parent_id: None,
            data: serde_json::json!({"state": "1"}),
            metadata: serde_json::json!({}),
            created_at: Utc::now(),
        };

        saver.put_checkpoint(cp1.clone()).await.unwrap();

        let content = std::fs::read_to_string(&scratchpad_path).unwrap();
        let updated_json: serde_json::Value = serde_json::from_str(&content).unwrap();

        assert_eq!(updated_json["unknown_field"], "val1");
        assert_eq!(updated_json["current_objective"], "Checkpoint cp-generic-1");
    }

    #[tokio::test]
    async fn test_git_checkpointer_tag_prefix_match() {
        let temp_dir = tempfile::tempdir().unwrap();
        let saver = GitCheckpointer::new(temp_dir.path().to_path_buf());

        let cp1 = Checkpoint {
            thread_id: "thread-git-tag".to_string(),
            checkpoint_id: "cp-tag-1".to_string(),
            parent_id: None,
            data: serde_json::json!({"state": "1"}),
            metadata: serde_json::json!({}),
            created_at: Utc::now(),
        };

        saver.put_checkpoint(cp1.clone()).await.unwrap();

        // Check if the tag exists with the proper prefix
        let output = std::process::Command::new("git")
            .arg("tag")
            .arg("-l")
            .arg("checkpoint-cp-tag-1")
            .current_dir(temp_dir.path())
            .output()
            .unwrap();

        assert!(output.status.success());
        let tags = String::from_utf8_lossy(&output.stdout);
        assert!(tags.contains("checkpoint-cp-tag-1"));

        // Verify getting the checkpoint by raw ID works via prefix resolution
        let retrieved = saver
            .get_checkpoint("thread-git-tag", "cp-tag-1")
            .await
            .unwrap();
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().checkpoint_id, "cp-tag-1");
    }
}

#[cfg(test)]
mod additional_git_tests {
    use super::*;
    use chrono::Utc;

    #[tokio::test]
    async fn test_git_checkpointer_safe_tags() {
        let temp_dir = tempfile::tempdir().unwrap();
        let saver = GitCheckpointer::new(temp_dir.path().to_path_buf());

        let cp1 = Checkpoint {
            thread_id: "thread-git-safe".to_string(),
            checkpoint_id: "cp bad tag :?* ".to_string(), // Invalid git tag chars
            parent_id: None,
            data: serde_json::json!({"state": "1"}),
            metadata: serde_json::json!({}),
            created_at: Utc::now(),
        };

        let cp2 = Checkpoint {
            thread_id: "thread-git-safe".to_string(),
            checkpoint_id: "cp-git-safe-2".to_string(),
            parent_id: Some("cp bad tag :?* ".to_string()),
            data: serde_json::json!({"state": "2"}),
            metadata: serde_json::json!({}),
            created_at: Utc::now(),
        };

        // put_checkpoint should sanitize the tag internally
        saver.put_checkpoint(cp1.clone()).await.unwrap();
        saver.put_checkpoint(cp2.clone()).await.unwrap();

        // get_checkpoint should use safe_tag_name
        let retrieved = saver
            .get_checkpoint("thread-git-safe", "cp bad tag :?* ")
            .await
            .unwrap()
            .unwrap();
        assert_eq!(retrieved.checkpoint_id, "cp bad tag :?* ");

        // list_checkpoints should still find both
        let list = saver.list_checkpoints("thread-git-safe").await.unwrap();
        assert!(list.len() >= 2);

        // restore_checkpoint should branch and checkout safely
        saver.restore_checkpoint("cp bad tag :?* ").await.unwrap();

        let output = std::process::Command::new("git")
            .arg("branch")
            .arg("--show-current")
            .current_dir(&temp_dir)
            .output()
            .unwrap();

        let branch = String::from_utf8_lossy(&output.stdout).trim().to_string();
        assert!(branch.starts_with("agent-restore-"));
    }
}

#[cfg(test)]
mod restore_stash_tests {
    use super::*;
    use chrono::Utc;

    #[tokio::test]
    async fn test_git_checkpointer_restore_stashes_untracked() {
        let temp_dir = tempfile::tempdir().unwrap();
        let saver = GitCheckpointer::new(temp_dir.path().to_path_buf());

        let cp1 = Checkpoint {
            thread_id: "thread-git-stash".to_string(),
            checkpoint_id: "cp-stash-1".to_string(),
            parent_id: None,
            data: serde_json::json!({"state": "1"}),
            metadata: serde_json::json!({}),
            created_at: Utc::now(),
        };

        // Write a tracked file
        let file_path = temp_dir.path().join("test_tracked.txt");
        std::fs::write(&file_path, "tracked 1").unwrap();

        saver.put_checkpoint(cp1.clone()).await.unwrap();

        // Write an untracked file
        let untracked_file_path = temp_dir.path().join("test_untracked.txt");
        std::fs::write(&untracked_file_path, "untracked work").unwrap();

        // Modifying the tracked file
        std::fs::write(&file_path, "tracked modified").unwrap();

        // Restore to first checkpoint
        saver.restore_checkpoint("cp-stash-1").await.unwrap();

        // Verify the tracked file was restored
        let file_content = std::fs::read_to_string(&file_path).unwrap();
        assert_eq!(file_content, "tracked 1");

        // Verify the untracked file is NO LONGER in the working tree (it was stashed, then clean removed it if not tracked)
        assert!(!untracked_file_path.exists());

        // Verify the stash was created
        let stash_list = std::process::Command::new("git")
            .arg("stash")
            .arg("list")
            .current_dir(temp_dir.path())
            .output()
            .unwrap();

        let stash_content = String::from_utf8_lossy(&stash_list.stdout);
        assert!(stash_content.contains("Auto-stash before restoring checkpoint cp-stash-1"));

        // Pop the stash to verify untracked file comes back
        let stash_pop = std::process::Command::new("git")
            .arg("stash")
            .arg("pop")
            .current_dir(temp_dir.path())
            .output()
            .unwrap();

        assert!(stash_pop.status.success());

        // Now untracked file should exist again
        assert!(untracked_file_path.exists());
        let untracked_content = std::fs::read_to_string(&untracked_file_path).unwrap();
        assert_eq!(untracked_content, "untracked work");
    }
}
