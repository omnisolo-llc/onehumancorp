use std::sync::Arc;
use tokio::sync::Mutex;

/// OpenHands/OpenDevin Implementation Pattern
///
/// Implements a "Code-first" approach natively expressed in code. Uses a Runner class
/// with async, sync, and streamed modes. Uses a 3-layer architecture:
/// 1. Core (agent code + runtime)
/// 2. App Server (bidirectional JSON-RPC API layer)
/// 3. Client surfaces sharing the exact same harness.
// Layer 1: Core (agent code + runtime)
pub struct OpenHandsCore {
    harness_state: Arc<Mutex<String>>,
}

impl Default for OpenHandsCore {
    fn default() -> Self {
        Self::new()
    }
}

impl OpenHandsCore {
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
            let _ = tx.send(format!("Working on: {}", task_clone)).await;
            tokio::time::sleep(std::time::Duration::from_millis(5)).await;
            let _ = tx.send(format!("Task complete: {}", task_clone)).await;
        });

        Ok(rx)
    }
}

// Layer 2: App Server (bidirectional JSON-RPC API layer)
pub struct OpenHandsAppServer {
    core: Arc<OpenHandsCore>,
}

impl OpenHandsAppServer {
    pub fn new(core: Arc<OpenHandsCore>) -> Self {
        Self { core }
    }

    pub async fn handle_rpc_request(&self, request_payload: &str) -> Result<String, String> {
        let result = self.core.run_async(request_payload).await?;
        Ok(format!("AppServer[Core[{}]]", result))
    }
}

// Layer 3: Client surfaces sharing the exact same harness
pub struct OpenHandsClient {
    app_server: OpenHandsAppServer,
}

impl OpenHandsClient {
    pub fn new(app_server: OpenHandsAppServer) -> Self {
        Self { app_server }
    }

    pub async fn execute_task(&self, task: &str) -> Result<String, String> {
        self.app_server.handle_rpc_request(task).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_openhands_runner_3_layer_architecture() {
        // App server wrapping core
        let core = Arc::new(OpenHandsCore::new());
        let app_server = OpenHandsAppServer::new(core);

        // Client wrapping app server
        let client = OpenHandsClient::new(app_server);

        let result = client.execute_task("test client task").await.unwrap();
        assert_eq!(result, "AppServer[Core[completed: test client task]]");
    }

    #[tokio::test]
    async fn test_openhands_runner_streamed_chunks() {
        let core = Arc::new(OpenHandsCore::new());
        let mut rx = core.run_streamed("stream task").await.unwrap();

        let mut chunks = Vec::new();
        while let Some(chunk) = rx.recv().await {
            chunks.push(chunk);
        }

        assert_eq!(chunks.len(), 3);
        assert_eq!(chunks[0], "Starting task: stream task");
        assert_eq!(chunks[1], "Working on: stream task");
        assert_eq!(chunks[2], "Task complete: stream task");
    }

    #[tokio::test]
    async fn test_openhands_runner_async() {
        let runner = OpenHandsCore::new();
        let result = runner.run_async("Test task").await;
        assert_eq!(result.unwrap(), "completed: Test task");
    }

    #[test]
    fn test_openhands_runner_sync() {
        let runner = OpenHandsCore::new();
        let result = runner.run_sync("Test task sync");
        assert_eq!(result.unwrap(), "completed: Test task sync");
    }
}
