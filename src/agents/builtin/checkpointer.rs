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
    /// Legacy unscoped restore cannot establish checkpoint ownership.
    async fn restore_checkpoint(&self, _checkpoint_id: &str) -> Result<(), String> {
        Err("Task-scoped checkpoint restore required; use restore_checkpoint_for_thread".into())
    }
    /// Stores must explicitly implement task-scoped restoration. No legacy fallback.
    async fn restore_checkpoint_for_thread(
        &self,
        _thread_id: &str,
        _checkpoint_id: &str,
    ) -> Result<(), String> {
        Err("Task-scoped checkpoint restore is not supported by this store".into())
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
/// 3. Restores a validated task checkpoint on a fresh branch, preserving dirty/untracked
///    work in a recovery stash and leaving ignored files untouched. The configured
///    repository must be an isolated task workspace; task IDs are not tenant authorization.
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

    fn task_ref_name(thread_id: &str, checkpoint_id: &str) -> String {
        use sha2::{Digest, Sha256};
        // NUL is prohibited in task IDs, so the separator cannot be ambiguous.
        let digest = Sha256::new()
            .chain_update(thread_id.as_bytes())
            .chain_update([0])
            .chain_update(checkpoint_id.as_bytes())
            .finalize();
        format!("refs/ohc/checkpoints/{digest:x}")
    }

    fn validate_thread_id(thread_id: &str) -> Result<(), String> {
        if thread_id.is_empty()
            || thread_id
                .chars()
                .any(|c| c == '/' || c == '\\' || c.is_control())
        {
            return Err("Invalid checkpoint task ID".into());
        }
        Ok(())
    }

    async fn git_output(&self, args: &[&str]) -> Result<std::process::Output, String> {
        Command::new("git")
            .args(args)
            .env("GIT_OPTIONAL_LOCKS", "0")
            .current_dir(&self.repo_path)
            .output()
            .await
            .map_err(|e| format!("Failed to execute git: {e}"))
    }

    async fn ensure_index_unlocked(&self) -> Result<(), String> {
        // rev-parse also locates the correct index in a linked worktree.
        let output = self
            .git_output(&["rev-parse", "--git-path", "index.lock"])
            .await?;
        if !output.status.success() {
            return Err(format!(
                "Cannot locate Git index: {}",
                String::from_utf8_lossy(&output.stderr)
            ));
        }
        let path = String::from_utf8(output.stdout).map_err(|e| e.to_string())?;
        let path = self.repo_path.join(path.trim());
        match tokio::fs::symlink_metadata(path).await {
            Ok(_) => {
                Err("Git index is locked by another operation; retry after it finishes".into())
            }
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(()),
            Err(e) => Err(format!("Cannot inspect Git index lock: {e}")),
        }
    }

    async fn resolve_checkpoint(
        &self,
        thread_id: &str,
        checkpoint_id: &str,
        allow_commit_id: bool,
    ) -> Result<Option<(Checkpoint, String)>, String> {
        Self::validate_thread_id(thread_id)?;
        if checkpoint_id.is_empty() || checkpoint_id.contains('\0') {
            return Err("Invalid checkpoint ID".into());
        }
        let mut refs = vec![
            (Self::task_ref_name(thread_id, checkpoint_id), false),
            (
                format!("refs/tags/{}", Self::safe_tag_name(checkpoint_id)),
                false,
            ),
            (format!("refs/tags/checkpoint-{checkpoint_id}"), false),
        ];
        // History enumeration reads commits by immutable object ID. Arbitrary Git
        // revision expressions and branch names are not checkpoint identities.
        if allow_commit_id
            && matches!(checkpoint_id.len(), 40 | 64)
            && checkpoint_id.bytes().all(|c| c.is_ascii_hexdigit())
        {
            refs.push((checkpoint_id.to_owned(), true));
        }
        for (reference, is_commit_id) in refs {
            let revision = format!("{reference}^{{commit}}");
            let resolved = self
                .git_output(&["rev-parse", "--verify", "--end-of-options", &revision])
                .await?;
            if !resolved.status.success() {
                continue;
            }
            let oid = String::from_utf8(resolved.stdout)
                .map_err(|e| e.to_string())?
                .trim()
                .to_owned();
            let object = format!("{oid}:.agent_progress_{thread_id}.json");
            let output = self.git_output(&["show", &object]).await?;
            if !output.status.success() {
                continue;
            }
            let checkpoint: Checkpoint =
                serde_json::from_slice(&output.stdout).map_err(|e| e.to_string())?;
            if checkpoint.thread_id != thread_id
                || (!is_commit_id && checkpoint.checkpoint_id != checkpoint_id)
            {
                continue;
            }
            let scoped_ref = Self::task_ref_name(thread_id, checkpoint_id);
            if !is_commit_id && reference != scoped_ref {
                // Legacy global tags can carry an older task's unchanged progress
                // file. Accept them only when this commit actually records this
                // task's checkpoint, never merely because a stale file is present.
                let file_name = format!(".agent_progress_{thread_id}.json");
                let changed_at = self
                    .git_output(&[
                        "--literal-pathspecs",
                        "log",
                        "-1",
                        "--format=%H",
                        &oid,
                        "--",
                        &file_name,
                    ])
                    .await?;
                if !changed_at.status.success()
                    || String::from_utf8_lossy(&changed_at.stdout).trim() != oid
                {
                    continue;
                }
            }
            return Ok(Some((checkpoint, oid)));
        }
        Ok(None)
    }

    async fn ensure_no_ignored_collision(&self, oid: &str) -> Result<(), String> {
        let target = self
            .git_output(&["ls-tree", "-rz", "--name-only", oid])
            .await?;
        let ignored = self
            .git_output(&[
                "ls-files",
                "--others",
                "--ignored",
                "--exclude-standard",
                "--directory",
                "-z",
            ])
            .await?;
        if !target.status.success() || !ignored.status.success() {
            return Err("Cannot verify ignored files before checkpoint restore".into());
        }
        for ignored_path in ignored.stdout.split(|b| *b == 0).filter(|p| !p.is_empty()) {
            let ignored_path = ignored_path.strip_suffix(b"/").unwrap_or(ignored_path);
            for target_path in target.stdout.split(|b| *b == 0).filter(|p| !p.is_empty()) {
                if target_path == ignored_path
                    || target_path
                        .strip_prefix(ignored_path)
                        .is_some_and(|p| p.starts_with(b"/"))
                    || ignored_path
                        .strip_prefix(target_path)
                        .is_some_and(|p| p.starts_with(b"/"))
                {
                    return Err(format!(
                        "Checkpoint restore would overwrite ignored path {}; move it before restoring",
                        String::from_utf8_lossy(ignored_path)
                    ));
                }
            }
        }
        Ok(())
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
            .args(["config", "user.email", "agent@omnisolo.local"])
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
        Ok(self
            .resolve_checkpoint(thread_id, checkpoint_id, true)
            .await?
            .map(|(checkpoint, _)| checkpoint))
    }

    fn storage_prefix(&self) -> &'static str {
        "git"
    }

    async fn put_checkpoint(&self, checkpoint: Checkpoint) -> Result<(), String> {
        Self::validate_thread_id(&checkpoint.thread_id)?;
        self.ensure_index_unlocked().await?;
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

        let checkpoint_ref = Self::task_ref_name(&checkpoint.thread_id, &checkpoint.checkpoint_id);
        let ref_output = Command::new("git")
            .arg("update-ref")
            .arg(&checkpoint_ref)
            .arg("HEAD")
            .current_dir(&self.repo_path)
            .output()
            .await
            .map_err(|e| format!("Failed to execute git update-ref: {}", e))?;

        if !ref_output.status.success() {
            return Err(format!(
                "Failed to record checkpoint ref: {}",
                String::from_utf8_lossy(&ref_output.stderr)
            ));
        }

        // Keep an unoccupied legacy alias for older read-only consumers, but
        // never move another task's or sanitized-ID collision's alias.
        let legacy_tag = Self::safe_tag_name(&checkpoint.checkpoint_id);
        let existing = self
            .git_output(&[
                "show-ref",
                "--verify",
                "--quiet",
                &format!("refs/tags/{legacy_tag}"),
            ])
            .await?;
        if !existing.status.success() {
            let alias = self
                .git_output(&["tag", &legacy_tag, &checkpoint_ref])
                .await?;
            if !alias.status.success() {
                tracing::warn!(
                    "Legacy checkpoint alias was not created; task-scoped checkpoint remains available: {}",
                    String::from_utf8_lossy(&alias.stderr)
                );
            }
        }

        Ok(())
    }

    async fn restore_checkpoint_for_thread(
        &self,
        thread_id: &str,
        checkpoint_id: &str,
    ) -> Result<(), String> {
        // Membership and immutable target resolution must precede ALL mutations.
        let (_, oid) = self
            .resolve_checkpoint(thread_id, checkpoint_id, false)
            .await?
            .ok_or_else(|| "Checkpoint not found for requested task".to_owned())?;
        self.ensure_index_unlocked().await?;
        self.ensure_no_ignored_collision(&oid).await?;

        let previous_stash = self
            .git_output(&["rev-parse", "--verify", "refs/stash"])
            .await?;
        let message = format!("Auto-stash before restoring checkpoint {checkpoint_id}");
        let stash = self
            .git_output(&["stash", "push", "--include-untracked", "-m", &message])
            .await?;
        if !stash.status.success() {
            return Err(format!(
                "Checkpoint restore aborted: could not preserve workspace in stash: {}",
                String::from_utf8_lossy(&stash.stderr)
            ));
        }
        let recovery = self
            .git_output(&["rev-parse", "--verify", "refs/stash"])
            .await?;
        let recovery = if recovery.status.success() && recovery.stdout != previous_stash.stdout {
            Some(String::from_utf8_lossy(&recovery.stdout).trim().to_owned())
        } else {
            None
        };
        let recovery_description = recovery.as_deref().unwrap_or("none (no new stash created)");
        let branch = format!("agent-restore-{}", uuid::Uuid::new_v4());
        // -b does not reset an existing branch; --no-overwrite-ignore prevents
        // ignored work created after preflight from being silently overwritten.
        let checkout = self
            .git_output(&["checkout", "--no-overwrite-ignore", "-b", &branch, &oid])
            .await
            .map_err(|error| format!("{error}; recovery stash: {recovery_description}"))?;
        if !checkout.status.success() {
            return Err(format!(
                "Checkpoint checkout failed; recovery stash: {recovery_description}: {}",
                String::from_utf8_lossy(&checkout.stderr)
            ));
        }
        tracing::info!(
            thread_id,
            checkpoint_id,
            recovery_stash = ?recovery,
            "Checkpoint restored; any saved work remains in the reported recovery stash"
        );
        Ok(())
    }
    async fn list_checkpoints(&self, thread_id: &str) -> Result<Vec<Checkpoint>, String> {
        let file_name = format!(".agent_progress_{}.json", thread_id);

        let output = Command::new("git")
            .arg("--literal-pathspecs")
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

    async fn restore_checkpoint_for_thread(
        &self,
        thread_id: &str,
        checkpoint_id: &str,
    ) -> Result<(), String> {
        if thread_id.is_empty() || checkpoint_id.is_empty() {
            return Err("Task ID and checkpoint ID are required".into());
        }
        let mut transaction = self.pool.begin().await.map_err(|e| e.to_string())?;
        let row = sqlx::query(
            "SELECT created_at FROM swarm_checkpoints WHERE thread_id = $1 AND checkpoint_id = $2 FOR UPDATE",
        )
        .bind(thread_id)
        .bind(checkpoint_id)
        .fetch_optional(&mut *transaction)
        .await
        .map_err(|e| e.to_string())?
        .ok_or_else(|| "Checkpoint not found for requested task".to_owned())?;
        let target_time: DateTime<Utc> = row.get("created_at");
        sqlx::query("DELETE FROM swarm_checkpoints WHERE thread_id = $1 AND created_at > $2")
            .bind(thread_id)
            .bind(target_time)
            .execute(&mut *transaction)
            .await
            .map_err(|e| e.to_string())?;
        transaction.commit().await.map_err(|e| e.to_string())
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
fn is_private_checkpoint_test_url(raw: &str) -> bool {
    let Ok(url) = url::Url::parse(raw) else {
        return false;
    };
    matches!(url.scheme(), "postgres" | "postgresql")
        && url
            .host_str()
            .and_then(|host| host.parse::<std::net::IpAddr>().ok())
            .is_some_and(|host| host.is_loopback())
        && url.username() == "postgres"
        && url.path() == "/ohc_checkpoint_test"
        && url.query().is_none()
        && url.fragment().is_none()
        && !raw.chars().any(char::is_control)
}

#[cfg(test)]
pub(crate) async fn private_checkpoint_test_pool() -> sqlx::PgPool {
    let raw = std::env::var("OHC_CHECKPOINT_TEST_DATABASE_URL")
        .expect("explicit disposable OHC_CHECKPOINT_TEST_DATABASE_URL required");
    assert!(
        is_private_checkpoint_test_url(&raw),
        "checkpoint tests require an explicit loopback ohc_checkpoint_test database without URL options"
    );
    sqlx::postgres::PgPoolOptions::new()
        .max_connections(1)
        .max_lifetime(None)
        .idle_timeout(None)
        .after_connect(|connection, _| Box::pin(async move {
            sqlx::query("SET search_path TO pg_temp").execute(&mut *connection).await?;
            sqlx::query("CREATE TEMPORARY TABLE swarm_checkpoints (thread_id TEXT, checkpoint_id TEXT, parent_id TEXT, checkpoint BYTEA, metadata BYTEA, created_at TIMESTAMPTZ, PRIMARY KEY (thread_id, checkpoint_id))").execute(&mut *connection).await?;
            Ok(())
        }))
        .connect(&raw).await.expect("disposable checkpoint database must be reachable")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn checkpoint_test_database_url_requires_an_owned_loopback_database() {
        for valid in [
            "postgresql://postgres@127.0.0.1:55434/ohc_checkpoint_test",
            "postgres://postgres:ignored_fixture@127.0.0.1:5432/ohc_checkpoint_test",
        ] {
            assert!(is_private_checkpoint_test_url(valid));
        }
        for invalid in [
            "postgres://postgres@db.example.com/ohc_checkpoint_test",
            "postgres://postgres@127.0.0.1/postgres",
            "postgres://postgres@127.0.0.1/ohc_checkpoint_test?options=-csearch_path=public",
            "postgres://postgres@127.0.0.1/ohc_checkpoint_test#fragment",
            "postgres://postgres@127.0.0.1/ohc_checkpoint_test\n",
        ] {
            assert!(!is_private_checkpoint_test_url(invalid));
        }
    }

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
        let pool = private_checkpoint_test_pool().await;

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
        saver
            .restore_checkpoint_for_thread("thread-restore-1", "cp-2")
            .await
            .unwrap();

        // Verify that cp-3 is gone, but cp-2 and cp-1 remain
        let after = saver.list_checkpoints("thread-restore-1").await.unwrap();
        assert_eq!(after.len(), 2);
        assert!(after.iter().any(|c| c.checkpoint_id == "cp-1"));
        assert!(after.iter().any(|c| c.checkpoint_id == "cp-2"));
        assert!(!after.iter().any(|c| c.checkpoint_id == "cp-3"));
    }

    #[tokio::test]
    async fn test_pg_checkpointer_save_and_load() {
        let pool = private_checkpoint_test_pool().await;

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
        let restore_res = saver
            .restore_checkpoint_for_thread("thread-1", "cp-1")
            .await;
        assert!(restore_res.is_ok());
    }

    #[tokio::test]
    async fn test_pg_checkpointer_list_checkpoints() {
        let pool = private_checkpoint_test_pool().await;
        let saver = PgCheckpointer::new(pool);
        assert!(
            saver
                .list_checkpoints("thread-list")
                .await
                .unwrap()
                .is_empty()
        );
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

        // An existing lock belongs to another operation until its owner removes it.
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
        assert!(res2.is_err());
        assert_eq!(tokio::fs::read(&lock_file).await.unwrap(), b"test");

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
        saver
            .restore_checkpoint_for_thread("thread-git-restore", "cp-restore-1")
            .await
            .unwrap();

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
        let result = saver
            .restore_checkpoint_for_thread("thread-missing", "non-existent-checkpoint")
            .await;
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
        saver
            .restore_checkpoint_for_thread("thread-git-safe", "cp bad tag :?* ")
            .await
            .unwrap();

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
        saver
            .restore_checkpoint_for_thread("thread-git-stash", "cp-stash-1")
            .await
            .unwrap();

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

#[cfg(test)]
mod restore_safety_tests {
    use super::*;
    use std::collections::BTreeMap;
    use std::path::Path;

    fn git(repo: &Path, args: &[&str]) -> Vec<u8> {
        let output = StdCommand::new("git")
            .args(args)
            .env("GIT_OPTIONAL_LOCKS", "0")
            .current_dir(repo)
            .output()
            .unwrap();
        assert!(
            output.status.success(),
            "git {args:?}: {}",
            String::from_utf8_lossy(&output.stderr)
        );
        output.stdout
    }

    fn snapshot(root: &Path) -> BTreeMap<PathBuf, Vec<u8>> {
        fn visit(root: &Path, dir: &Path, result: &mut BTreeMap<PathBuf, Vec<u8>>) {
            for entry in std::fs::read_dir(dir).unwrap() {
                let path = entry.unwrap().path();
                if path.is_dir() {
                    visit(root, &path, result);
                } else {
                    result.insert(
                        path.strip_prefix(root).unwrap().to_owned(),
                        std::fs::read(path).unwrap(),
                    );
                }
            }
        }
        let mut result = BTreeMap::new();
        visit(root, root, &mut result);
        result
    }

    fn checkpoint(thread: &str, id: &str) -> Checkpoint {
        Checkpoint {
            thread_id: thread.into(),
            checkpoint_id: id.into(),
            parent_id: None,
            data: serde_json::json!({"step": id}),
            metadata: serde_json::json!({}),
            created_at: Utc::now(),
        }
    }

    fn remove_task_checkpoint_refs(repo: &Path) {
        let refs = String::from_utf8(git(repo, &["for-each-ref", "--format=%(refname)"])).unwrap();
        for reference in refs.lines().filter(|reference| {
            reference.starts_with("refs/ohc/checkpoints/")
                || reference.starts_with("refs/tags/checkpoint-task-")
        }) {
            git(repo, &["update-ref", "-d", reference]);
        }
    }

    async fn dirty_repo() -> (tempfile::TempDir, GitCheckpointer) {
        let dir = tempfile::tempdir().unwrap();
        let saver = GitCheckpointer::new(dir.path().into());
        std::fs::write(dir.path().join("tracked.txt"), "committed").unwrap();
        saver
            .put_checkpoint(checkpoint("task-a", "cp-a"))
            .await
            .unwrap();
        std::fs::write(dir.path().join("tracked.txt"), "staged work").unwrap();
        git(dir.path(), &["add", "tracked.txt"]);
        std::fs::write(dir.path().join("tracked.txt"), "unstaged work").unwrap();
        std::fs::write(dir.path().join("untracked.txt"), "untracked work").unwrap();
        std::fs::create_dir(dir.path().join("target")).unwrap();
        std::fs::write(dir.path().join("target/important.txt"), "ignored work").unwrap();
        (dir, saver)
    }

    #[tokio::test]
    async fn missing_restore_does_not_mutate_files_index_or_refs() {
        let (dir, saver) = dirty_repo().await;
        let before = snapshot(dir.path());
        assert!(saver.restore_checkpoint("missing").await.is_err());
        assert!(
            snapshot(dir.path()) == before,
            "invalid restore mutated repository content"
        );
    }

    #[tokio::test]
    async fn checkpoint_creation_never_removes_existing_index_lock() {
        let (dir, saver) = dirty_repo().await;
        std::fs::write(
            dir.path().join(".git/index.lock"),
            "owned by another process",
        )
        .unwrap();
        let before = snapshot(dir.path());
        assert!(
            saver
                .put_checkpoint(checkpoint("task-a", "cp-b"))
                .await
                .is_err()
        );
        assert!(snapshot(dir.path()) == before, "repository content changed");
    }

    #[tokio::test]
    async fn scoped_invalid_restore_does_not_mutate_files_index_or_refs() {
        let (dir, saver) = dirty_repo().await;
        let before = snapshot(dir.path());
        for (thread, id) in [
            ("task-a", "missing"),
            ("task-b", "cp-a"),
            ("../other", "cp-a"),
            ("", "cp-a"),
            ("task-a", ""),
        ] {
            assert!(
                saver
                    .restore_checkpoint_for_thread(thread, id)
                    .await
                    .is_err()
            );
            assert!(
                snapshot(dir.path()) == before,
                "invalid scoped restore changed repository"
            );
        }
        assert!(saver.restore_checkpoint("cp-a").await.is_err());
        assert!(
            snapshot(dir.path()) == before,
            "unscoped restore changed repository"
        );
    }

    #[tokio::test]
    async fn named_checkpoint_reads_reject_stale_other_task_metadata_and_tag_collisions() {
        let dir = tempfile::tempdir().unwrap();
        let saver = GitCheckpointer::new(dir.path().into());
        saver
            .put_checkpoint(checkpoint("task-a", "cp:a"))
            .await
            .unwrap();
        saver
            .put_checkpoint(checkpoint("task-b", "cp-b"))
            .await
            .unwrap();
        assert!(
            saver
                .get_checkpoint("task-a", "cp-b")
                .await
                .unwrap()
                .is_none()
        );
        assert!(
            saver
                .get_checkpoint("task-a", "cp?a")
                .await
                .unwrap()
                .is_none()
        );
        let before = snapshot(dir.path());
        assert!(
            saver
                .restore_checkpoint_for_thread("task-a", "cp-b")
                .await
                .is_err()
        );
        assert!(
            saver
                .restore_checkpoint_for_thread("task-a", "cp?a")
                .await
                .is_err()
        );
        assert!(snapshot(dir.path()) == before);
        let hash = String::from_utf8(git(dir.path(), &["rev-parse", "refs/tags/checkpoint-cp_a"]))
            .unwrap();
        assert_eq!(
            saver
                .get_checkpoint("task-a", hash.trim())
                .await
                .unwrap()
                .unwrap()
                .checkpoint_id,
            "cp:a"
        );
    }

    #[tokio::test]
    async fn restore_preserves_existing_index_lock_without_mutation() {
        let (dir, saver) = dirty_repo().await;
        std::fs::write(dir.path().join(".git/index.lock"), "another process").unwrap();
        let before = snapshot(dir.path());
        assert!(
            saver
                .restore_checkpoint_for_thread("task-a", "cp-a")
                .await
                .is_err()
        );
        assert!(snapshot(dir.path()) == before);
    }

    #[tokio::test]
    async fn restore_rejects_ignored_file_and_directory_collisions_before_stashing() {
        for ignored_is_directory in [false, true] {
            let dir = tempfile::tempdir().unwrap();
            let saver = GitCheckpointer::new(dir.path().into());
            std::fs::write(dir.path().join("collision"), "checkpoint content").unwrap();
            saver
                .put_checkpoint(checkpoint("task-a", "cp-old"))
                .await
                .unwrap();
            git(dir.path(), &["rm", "collision"]);
            std::fs::write(
                dir.path().join(".gitignore"),
                "collision\n.scratchpad_*.json\n",
            )
            .unwrap();
            saver
                .put_checkpoint(checkpoint("task-a", "cp-current"))
                .await
                .unwrap();
            if ignored_is_directory {
                std::fs::create_dir(dir.path().join("collision")).unwrap();
                std::fs::write(
                    dir.path().join("collision/private"),
                    "ignored directory contents",
                )
                .unwrap();
            } else {
                std::fs::write(dir.path().join("collision"), "ignored file contents").unwrap();
            }
            std::fs::write(dir.path().join("untracked"), "work waiting to be stashed").unwrap();
            let before = snapshot(dir.path());
            assert!(
                saver
                    .restore_checkpoint_for_thread("task-a", "cp-old")
                    .await
                    .is_err()
            );
            assert!(
                snapshot(dir.path()) == before,
                "ignored collision changed repository before rejection"
            );
        }
    }

    #[tokio::test]
    async fn failed_stash_preserves_dirty_untracked_and_ignored_work() {
        let (dir, saver) = dirty_repo().await;
        git(dir.path(), &["config", "user.name", ""]);
        git(dir.path(), &["config", "user.email", ""]);
        let head = git(dir.path(), &["rev-parse", "HEAD"]);
        let refs = git(dir.path(), &["show-ref"]);
        assert!(
            saver
                .restore_checkpoint_for_thread("task-a", "cp-a")
                .await
                .is_err()
        );
        assert_eq!(git(dir.path(), &["rev-parse", "HEAD"]), head);
        assert_eq!(git(dir.path(), &["show-ref"]), refs);
        for (name, content) in [
            ("tracked.txt", "unstaged work"),
            ("untracked.txt", "untracked work"),
            ("target/important.txt", "ignored work"),
        ] {
            assert_eq!(
                std::fs::read_to_string(dir.path().join(name)).unwrap(),
                content
            );
        }
        assert_eq!(git(dir.path(), &["show", ":tracked.txt"]), b"staged work");
    }

    #[tokio::test]
    async fn successful_restore_keeps_ignored_work_and_recoverable_staged_untracked_work() {
        let (dir, saver) = dirty_repo().await;
        saver
            .restore_checkpoint_for_thread("task-a", "cp-a")
            .await
            .unwrap();
        assert_eq!(
            std::fs::read_to_string(dir.path().join("tracked.txt")).unwrap(),
            "committed"
        );
        assert_eq!(
            std::fs::read_to_string(dir.path().join("target/important.txt")).unwrap(),
            "ignored work"
        );
        assert_eq!(
            git(dir.path(), &["show", "refs/stash:tracked.txt"]),
            b"unstaged work"
        );
        assert_eq!(
            git(dir.path(), &["show", "refs/stash^2:tracked.txt"]),
            b"staged work"
        );
        assert_eq!(
            git(dir.path(), &["show", "refs/stash^3:untracked.txt"]),
            b"untracked work"
        );
        git(dir.path(), &["stash", "apply", "--index"]);
        assert_eq!(
            std::fs::read_to_string(dir.path().join("untracked.txt")).unwrap(),
            "untracked work"
        );
    }

    #[tokio::test]
    async fn linked_worktree_index_lock_is_respected_before_checkpoint_writes() {
        let dir = tempfile::tempdir().unwrap();
        let saver = GitCheckpointer::new(dir.path().into());
        saver
            .put_checkpoint(checkpoint("task-a", "cp-a"))
            .await
            .unwrap();
        let linked_parent = tempfile::tempdir().unwrap();
        let linked = linked_parent.path().join("worktree");
        git(
            dir.path(),
            &["worktree", "add", "-b", "linked", linked.to_str().unwrap()],
        );
        let linked_saver = GitCheckpointer::new(linked.clone());
        let lock =
            String::from_utf8(git(&linked, &["rev-parse", "--git-path", "index.lock"])).unwrap();
        std::fs::write(lock.trim(), "linked worktree owner").unwrap();
        let before = snapshot(&linked);
        assert!(
            linked_saver
                .put_checkpoint(checkpoint("task-a", "cp-b"))
                .await
                .is_err()
        );
        assert!(
            linked_saver
                .restore_checkpoint_for_thread("task-a", "cp-a")
                .await
                .is_err()
        );
        assert!(snapshot(&linked) == before);
        assert_eq!(
            std::fs::read_to_string(lock.trim()).unwrap(),
            "linked worktree owner"
        );
    }

    #[tokio::test]
    async fn duplicate_ids_in_different_tasks_restore_the_correct_git_commit() {
        let dir = tempfile::tempdir().unwrap();
        let saver = GitCheckpointer::new(dir.path().into());
        for (thread, content) in [("task-a", "task a state"), ("task-b", "task b state")] {
            std::fs::write(dir.path().join("tracked"), content).unwrap();
            saver
                .put_checkpoint(checkpoint(thread, "shared"))
                .await
                .unwrap();
        }
        saver
            .restore_checkpoint_for_thread("task-a", "shared")
            .await
            .unwrap();
        assert_eq!(
            std::fs::read_to_string(dir.path().join("tracked")).unwrap(),
            "task a state"
        );
        saver
            .restore_checkpoint_for_thread("task-b", "shared")
            .await
            .unwrap();
        assert_eq!(
            std::fs::read_to_string(dir.path().join("tracked")).unwrap(),
            "task b state"
        );
    }

    #[tokio::test]
    async fn colliding_sanitized_ids_retain_independent_git_checkpoints() {
        let dir = tempfile::tempdir().unwrap();
        let saver = GitCheckpointer::new(dir.path().into());
        for (id, content) in [("cp:a", "colon state"), ("cp?a", "question state")] {
            std::fs::write(dir.path().join("tracked"), content).unwrap();
            saver
                .put_checkpoint(checkpoint("task-a", id))
                .await
                .unwrap();
        }
        saver
            .restore_checkpoint_for_thread("task-a", "cp:a")
            .await
            .unwrap();
        assert_eq!(
            std::fs::read_to_string(dir.path().join("tracked")).unwrap(),
            "colon state"
        );
        saver
            .restore_checkpoint_for_thread("task-a", "cp?a")
            .await
            .unwrap();
        assert_eq!(
            std::fs::read_to_string(dir.path().join("tracked")).unwrap(),
            "question state"
        );
    }

    #[tokio::test]
    async fn legacy_global_tag_cannot_restore_a_stale_other_task_snapshot() {
        let dir = tempfile::tempdir().unwrap();
        let saver = GitCheckpointer::new(dir.path().into());
        saver
            .put_checkpoint(checkpoint("task-a", "shared"))
            .await
            .unwrap();
        saver
            .put_checkpoint(checkpoint("task-b", "shared"))
            .await
            .unwrap();
        // Simulate a repository created by the earlier globally-tagged store.
        remove_task_checkpoint_refs(dir.path());
        git(dir.path(), &["tag", "-f", "checkpoint-shared", "HEAD"]);
        std::fs::write(dir.path().join("untracked"), "preserve work").unwrap();
        let before = snapshot(dir.path());
        assert!(
            saver
                .restore_checkpoint_for_thread("task-a", "shared")
                .await
                .is_err()
        );
        assert!(
            snapshot(dir.path()) == before,
            "ambiguous legacy target mutated repository"
        );
        saver
            .restore_checkpoint_for_thread("task-b", "shared")
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn unambiguous_legacy_tag_remains_restorable() {
        let dir = tempfile::tempdir().unwrap();
        let saver = GitCheckpointer::new(dir.path().into());
        std::fs::write(dir.path().join("tracked"), "legacy state").unwrap();
        saver
            .put_checkpoint(checkpoint("task-a", "legacy"))
            .await
            .unwrap();
        remove_task_checkpoint_refs(dir.path());
        std::fs::write(dir.path().join("tracked"), "new state").unwrap();
        saver
            .restore_checkpoint_for_thread("task-a", "legacy")
            .await
            .unwrap();
        assert_eq!(
            std::fs::read_to_string(dir.path().join("tracked")).unwrap(),
            "legacy state"
        );
    }

    #[tokio::test]
    async fn crafted_legacy_alias_cannot_impersonate_another_tasks_canonical_ref() {
        use sha2::{Digest, Sha256};
        let dir = tempfile::tempdir().unwrap();
        let saver = GitCheckpointer::new(dir.path().into());
        std::fs::write(dir.path().join("tracked"), "task a legacy state").unwrap();
        saver
            .put_checkpoint(checkpoint("task-a", "original"))
            .await
            .unwrap();
        remove_task_checkpoint_refs(dir.path());
        let digest = Sha256::new()
            .chain_update(b"task-a")
            .chain_update([0])
            .chain_update(b"original")
            .finalize();
        let crafted_id = format!("task-{digest:x}");
        std::fs::write(dir.path().join("tracked"), "task b state").unwrap();
        saver
            .put_checkpoint(checkpoint("task-b", &crafted_id))
            .await
            .unwrap();
        saver
            .restore_checkpoint_for_thread("task-a", "original")
            .await
            .unwrap();
        assert_eq!(
            std::fs::read_to_string(dir.path().join("tracked")).unwrap(),
            "task a legacy state"
        );
    }

    #[tokio::test]
    async fn legacy_provenance_treats_task_id_as_a_literal_path() {
        let dir = tempfile::tempdir().unwrap();
        let saver = GitCheckpointer::new(dir.path().into());
        saver
            .put_checkpoint(checkpoint("*", "shared"))
            .await
            .unwrap();
        saver
            .put_checkpoint(checkpoint("task-b", "shared"))
            .await
            .unwrap();
        remove_task_checkpoint_refs(dir.path());
        git(dir.path(), &["tag", "-f", "checkpoint-shared", "HEAD"]);
        std::fs::write(dir.path().join("untracked"), "preserve work").unwrap();
        let before = snapshot(dir.path());
        assert!(
            saver
                .restore_checkpoint_for_thread("*", "shared")
                .await
                .is_err()
        );
        assert!(
            snapshot(dir.path()) == before,
            "glob task ID bypassed legacy membership validation"
        );
    }
}
