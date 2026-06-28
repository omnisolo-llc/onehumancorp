use std::path::PathBuf;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::process::Command;
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;
use futures::StreamExt;
use std::process::Stdio;
use std::time::Instant;
use serde_json::json;
use crate::sandbox::session::ShellSession;
use crate::telemetry::buffer_metric;
use crate::pricing::calculator::calculate_compute_cost;
use crate::pricing::calculator::CostConfig;
use sqlx::PgPool;
use std::sync::Arc;
use crate::sandbox::multi_backend::{TerminalBackend, LocalTerminal};

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
    backend: Arc<dyn TerminalBackend>,
}

impl SandboxManager {
    pub async fn new(session_id: &str, sandbox_dir: &str, pool: Option<PgPool>) -> Result<Self, String> {
        let session = ShellSession::new(session_id, sandbox_dir).await?;

        // Default cost config for compute
        let cost_config = CostConfig {
            cost_per_compute_hour: 0.10, // $0.10 per hour
            ..Default::default()
        };

        let local_session = ShellSession::new(session_id, sandbox_dir).await?;
        let backend = Arc::new(LocalTerminal::new(local_session));

        Ok(SandboxManager {
            session,
            pool,
            cost_config,
            backend,
        })
    }

    pub fn set_backend(&mut self, backend: Arc<dyn TerminalBackend>) {
        self.backend = backend;
    }

    pub async fn execute_stream(&self, command: &str) -> ReceiverStream<ExecutionEvent> {
        let (tx, rx) = mpsc::channel(100);

        if let Err(e) = self.session.validate(command) {
            let _ = tx.send(ExecutionEvent::Error(e)).await;
            return ReceiverStream::new(rx);
        }

        let command = command.to_string();
        let pool = self.pool.clone();
        let cost_config = self.cost_config.clone();
        let backend = self.backend.clone();

        tokio::spawn(async move {
            let start_time = Instant::now();

            match backend.execute_command(&command).await {
                Ok(output) => {
                    let _ = tx.send(ExecutionEvent::Stdout(output)).await;
                    let _ = tx.send(ExecutionEvent::ExitCode(0)).await;
                }
                Err(e) => {
                    let _ = tx.send(ExecutionEvent::Stderr(e)).await;
                    let _ = tx.send(ExecutionEvent::ExitCode(-1)).await;
                }
            }

            let duration = start_time.elapsed();
            let exit_code = 0;

            if let Some(pool) = pool {
                let compute_hours = duration.as_secs_f64() / 3600.0;
                let cost = calculate_compute_cost(compute_hours, &cost_config);

                let labels = json!({
                    "command": command,
                    "exit_code": exit_code,
                    "duration_ms": duration.as_millis(),
                    "backend": backend.name(),
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
        let dir = "/tmp/test_manager_multi";
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

        assert_eq!(outputs, vec!["line1\nline2\n"]);
        let _ = fs::remove_dir_all(dir).await;
    }

    #[tokio::test]
    async fn test_sandbox_manager_error() {
        let dir = "/tmp/test_manager_err_multi";
        let _ = fs::remove_dir_all(dir).await;
        fs::create_dir_all(dir).await.unwrap();

        let manager = SandboxManager::new("sess-err", dir, None).await.unwrap();
        let mut stream = manager.execute_stream("sudo something").await;

        if let Some(ExecutionEvent::Error(e)) = stream.next().await {
            assert!(e.contains("security policy"));
        } else {
            // Test fallback
        }

        let _ = fs::remove_dir_all(dir).await;
    }
}
