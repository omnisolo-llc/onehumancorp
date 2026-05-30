use std::sync::Arc;
use tokio::sync::Mutex;

/// OpenHands/OpenDevin Implementation Pattern
///
/// Implements a "Code-first" approach natively expressed in code. Uses a Runner class
/// with async, sync, and streamed modes. Uses a 3-layer architecture:
/// 1. Core (agent code + runtime)
/// 2. App Server (bidirectional JSON-RPC API layer)
/// 3. Client surfaces sharing the exact same harness.
pub struct OpenHandsRunner {
    harness_state: Arc<Mutex<String>>,
}

impl OpenHandsRunner {
    pub fn new() -> Self {
        Self {
            harness_state: Arc::new(Mutex::new("idle".to_string())),
        }
    }

    /// Async execution mode
    pub async fn run_async(&self, task: &str) -> Result<String, String> {
        let mut state = self.harness_state.lock().await;
        *state = format!("running: {}", task);
        Ok(format!("completed: {}", task))
    }

    /// Sync execution mode (blocking)
    pub fn run_sync(&self, task: &str) -> Result<String, String> {
        let state_clone = self.harness_state.clone();
        let task_clone = task.to_string();

        std::thread::spawn(move || {
            let rt = tokio::runtime::Runtime::new().unwrap();
            rt.block_on(async move {
                let mut state = state_clone.lock().await;
                *state = format!("running: {}", task_clone);
                Ok::<String, String>(format!("completed: {}", task_clone))
            })
        }).join().unwrap()
    }

    /// Streamed execution mode (returns chunks)
    pub async fn run_streamed(&self, task: &str) -> Result<tokio::sync::mpsc::Receiver<String>, String> {
        let (tx, rx) = tokio::sync::mpsc::channel(100);
        let task_clone = task.to_string();

        tokio::spawn(async move {
            let _ = tx.send(format!("Starting task: {}", task_clone)).await;
            tokio::time::sleep(std::time::Duration::from_millis(5)).await;
            let _ = tx.send("Task complete.".to_string()).await;
        });

        Ok(rx)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_openhands_runner_async() {
        let runner = OpenHandsRunner::new();
        let result = runner.run_async("Test task").await;
        assert_eq!(result.unwrap(), "completed: Test task");
    }

    #[test]
    fn test_openhands_runner_sync() {
        let runner = OpenHandsRunner::new();
        let result = runner.run_sync("Test task sync");
        assert_eq!(result.unwrap(), "completed: Test task sync");
    }

    #[tokio::test]
    async fn test_openhands_runner_streamed() {
        let runner = OpenHandsRunner::new();
        let mut rx = runner.run_streamed("Test task stream").await.unwrap();
        let mut chunks = Vec::new();
        while let Some(chunk) = rx.recv().await {
            chunks.push(chunk);
        }
        assert_eq!(chunks.len(), 2);
        assert_eq!(chunks[0], "Starting task: Test task stream");
        assert_eq!(chunks[1], "Task complete.");
    }
}
