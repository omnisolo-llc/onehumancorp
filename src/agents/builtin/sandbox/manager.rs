#![allow(dead_code)]

use std::path::PathBuf;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::process::Command;
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;
use futures::StreamExt;
use std::process::Stdio;
use std::time::Instant;
use serde_json::json;
use crate::agents::sandbox::session::ShellSession;
use crate::telemetry::buffer_metric;
use crate::pricing::calculator::calculate_compute_cost;
use crate::pricing::calculator::CostConfig;
use sqlx::PgPool;

pub enum ExecutionEvent {
    Stdout(String),
    Stderr(String),
    ExitCode(i32),
    Error(String),
}

pub struct SandboxManager {
    session: ShellSession,
    pool: Option<PgPool>,
    cost_config: CostConfig,
}

impl SandboxManager {
    pub async fn new(session_id: &str, sandbox_dir: &str, pool: Option<PgPool>) -> Result<Self, String> {
        let session = ShellSession::new(session_id, sandbox_dir).await?;

        // Default cost config for compute
        let cost_config = CostConfig {
            cost_per_compute_hour: 0.10, // $0.10 per hour
            ..Default::default()
        };

        Ok(SandboxManager {
            session,
            pool,
            cost_config,
        })
    }

    pub async fn execute_stream(&self, command: &str) -> ReceiverStream<ExecutionEvent> {
        let (tx, rx) = mpsc::channel(100);
        let tx_err = tx.clone();

        if let Err(e) = self.session.validate(command) {
            let _ = tx.send(ExecutionEvent::Error(e)).await;
            return ReceiverStream::new(rx);
        }

        let _sandbox_dir = self.session.sandbox_dir.clone();
        let current_cwd = self.session.current_cwd.read().await.clone();
        let memory_dir = self.session.memory_dir.clone();
        let command = command.to_string();
        let pool = self.pool.clone();
        let cost_config = self.cost_config.clone();

        tokio::spawn(async move {
            let start_time = Instant::now();

            // Simplified execution for streaming I/O.
            // In a real implementation, we'd use ShellSession's stateful wrapper but adapted for streaming.
            let mut cmd = Command::new("bash");
            cmd.arg("-c").arg(&command);
            cmd.current_dir(current_cwd);
            cmd.stdout(Stdio::piped());
            cmd.stderr(Stdio::piped());
            cmd.env("OHC_MEMORY_DIR", memory_dir);

            let mut child = match cmd.spawn() {
                Ok(child) => child,
                Err(e) => {
                    let _ = tx_err.send(ExecutionEvent::Error(e.to_string())).await;
                    return;
                }
            };

            let stdout = child.stdout.take().unwrap();
            let stderr = child.stderr.take().unwrap();

            let mut stdout_reader = BufReader::new(stdout).lines();
            let mut stderr_reader = BufReader::new(stderr).lines();

            let tx_stdout = tx.clone();
            let tx_stderr = tx.clone();

            let stdout_handle = tokio::spawn(async move {
                while let Ok(Some(line)) = stdout_reader.next_line().await {
                    let _ = tx_stdout.send(ExecutionEvent::Stdout(line)).await;
                }
            });

            let stderr_handle = tokio::spawn(async move {
                while let Ok(Some(line)) = stderr_reader.next_line().await {
                    let _ = tx_stderr.send(ExecutionEvent::Stderr(line)).await;
                }
            });

            let status = child.wait().await;
            let _ = stdout_handle.await;
            let _ = stderr_handle.await;

            let duration = start_time.elapsed();
            let exit_code = status.map(|s| s.code().unwrap_or(-1)).unwrap_or(-1);

            let _ = tx.send(ExecutionEvent::ExitCode(exit_code)).await;

            // Telemetry and Cost Tracking
            if let Some(pool) = pool {
                let compute_hours = duration.as_secs_f64() / 3600.0;
                let cost = calculate_compute_cost(compute_hours, &cost_config);

                let labels = json!({
                    "command": command,
                    "exit_code": exit_code,
                    "duration_ms": duration.as_millis(),
                });

                let _ = buffer_metric(
                    &pool,
                    "ohc_sandbox_execution_cost",
                    "counter",
                    cost as f32,
                    labels.clone(),
                ).await;

                let _ = buffer_metric(
                    &pool,
                    "ohc_sandbox_execution_duration_ms",
                    "histogram",
                    duration.as_millis() as f32,
                    labels,
                ).await;
            }
        });

        ReceiverStream::new(rx)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::fs;

    #[tokio::test]
    async fn test_sandbox_manager_execution() {
        let dir_path = std::env::temp_dir().join("test_manager");
        let dir = dir_path.to_str().unwrap();
        let _ = fs::remove_dir_all(dir).await;
        fs::create_dir_all(dir).await.unwrap();

        let manager = SandboxManager::new("sess-m", dir, None).await.unwrap();
        let mut stream = manager.execute_stream("echo 'line1'; echo 'line2'").await;

        let mut outputs = Vec::new();
        while let Some(event) = stream.next().await {
            match event {
                ExecutionEvent::Stdout(line) => outputs.push(line),
                ExecutionEvent::ExitCode(code) => assert_eq!(code, 0),
                _ => {}
            }
        }

        assert_eq!(outputs, vec!["line1", "line2"]);
        let _ = fs::remove_dir_all(dir).await;
    }

    #[tokio::test]
    async fn test_sandbox_manager_error() {
        let dir_path = std::env::temp_dir().join("test_manager_err");
        let dir = dir_path.to_str().unwrap();
        let _ = fs::remove_dir_all(dir).await;
        fs::create_dir_all(dir).await.unwrap();

        let manager = SandboxManager::new("sess-err", dir, None).await.unwrap();
        let mut stream = manager.execute_stream("sudo something").await;

        if let Some(ExecutionEvent::Error(e)) = stream.next().await {
            assert!(e.contains("security policy"));
        } else {
            panic!("expected error event");
        }

        let _ = fs::remove_dir_all(dir).await;
    }
}
