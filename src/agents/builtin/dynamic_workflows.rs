/// Dynamic workflows orchestrate many subagents from a script Claude writes and you can rerun.
/// Use them for codebase audits, large migrations, and cross-checked research.
///
/// Limits implemented:
/// - Up to 16 concurrent agents.
/// - Up to 1,000 agents total per run.
use std::sync::Arc;
use tokio::sync::{Semaphore, RwLock, mpsc, watch};
use std::collections::HashMap;
use std::path::PathBuf;
use tokio::fs;

/// Claude Code Mechanic: Save the workflow for reuse & Pass input to a saved workflow.
/// Handles loading and saving workflow scripts to `.ohc/workflows/` and `~/.ohc/workflows/`.
pub struct WorkflowManager {
    project_dir: PathBuf,
    global_dir: Option<PathBuf>,
}

impl WorkflowManager {
    pub fn new(project_dir: impl Into<PathBuf>) -> Self {
        let global_dir = dirs::home_dir().map(|h| h.join(".ohc").join("workflows"));
        Self {
            project_dir: project_dir.into(),
            global_dir,
        }
    }

    /// Sanitizes the workflow name to prevent path traversal
    fn sanitize_name(name: &str) -> String {
        name.chars().filter(|c| c.is_alphanumeric() || *c == '_' || *c == '-').collect()
    }

    /// Saves a workflow script.
    /// If `is_global` is true, saves to `~/.ohc/workflows/`.
    /// Otherwise, saves to `<project_dir>/.ohc/workflows/`.
    pub async fn save_workflow(&self, name: &str, script: &str, is_global: bool) -> Result<(), String> {
        let safe_name = Self::sanitize_name(name);
        if safe_name.is_empty() {
            return Err("Invalid workflow name".to_string());
        }

        let target_dir = if is_global {
            self.global_dir.clone().ok_or("Home directory not found")?
        } else {
            self.project_dir.join(".ohc").join("workflows")
        };

        fs::create_dir_all(&target_dir).await.map_err(|e| format!("Failed to create dir: {}", e))?;
        let path = target_dir.join(format!("{}.js", safe_name));
        fs::write(&path, script).await.map_err(|e| format!("Failed to write script: {}", e))?;

        Ok(())
    }

    /// Loads a workflow script by name.
    /// Checks `<project_dir>/.ohc/workflows/` first, then falls back to `~/.ohc/workflows/`.
    pub async fn load_workflow(&self, name: &str) -> Result<String, String> {
        let safe_name = Self::sanitize_name(name);
        if safe_name.is_empty() {
            return Err("Invalid workflow name".to_string());
        }
        let filename = format!("{}.js", safe_name);

        // Check project dir
        let project_path = self.project_dir.join(".ohc").join("workflows").join(&filename);
        if fs::try_exists(&project_path).await.unwrap_or(false) {
            return fs::read_to_string(&project_path).await.map_err(|e| format!("Failed to read project script: {}", e));
        }

        // Check global dir
        if let Some(global_dir) = &self.global_dir {
            let global_path = global_dir.join(&filename);
            if fs::try_exists(&global_path).await.unwrap_or(false) {
                return fs::read_to_string(&global_path).await.map_err(|e| format!("Failed to read global script: {}", e));
            }
        }

        Err(format!("Workflow '{}' not found", name))
    }
}


#[derive(Debug)]
pub struct DynamicWorkflow {
    pub script: String,

    max_concurrent: usize,
    max_total_agents: usize,

    // State management for pause/resume mechanic
    pub cached_results: Arc<RwLock<HashMap<String, String>>>,
    pub pause_tx: watch::Sender<bool>,
    pub pause_rx: watch::Receiver<bool>,

    pub args: Option<serde_json::Value>,
}

#[derive(Debug, Clone)]
pub struct Task {
    pub id: String,
    pub instructions: String,
    pub args: Option<serde_json::Value>,
}

#[derive(Debug, Clone)]
pub struct TaskResult {
    pub task_id: String,
    pub output: String,
}

#[async_trait::async_trait]
pub trait WorkflowAgent: Send + Sync {
    async fn execute(&self, task: Task) -> Result<TaskResult, String>;
}

impl DynamicWorkflow {
    pub fn new(script: &str, args: Option<serde_json::Value>) -> Self {
        let (pause_tx, pause_rx) = watch::channel(false);
        Self {
            script: script.to_string(),
            max_concurrent: 16,
            max_total_agents: 1000,
            cached_results: Arc::new(RwLock::new(HashMap::new())),
            pause_tx,
            pause_rx,
            args,
        }
    }

    /// Pause the running workflow
    pub fn pause(&self) {
        let _ = self.pause_tx.send(true);
    }

    /// Resume the paused workflow
    pub fn resume(&self) {
        let _ = self.pause_tx.send(false);
    }

    pub async fn run_workflow(
        &self,
        mut tasks: Vec<Task>,
        agent_factory: Arc<dyn WorkflowAgent>,
    ) -> Result<Vec<TaskResult>, String> {
        if tasks.len() > self.max_total_agents {
            return Err(format!(
                "Workflow abort: Requested {} agents, which exceeds the max limit of {}",
                tasks.len(),
                self.max_total_agents
            ));
        }

        // Inject workflow args into each task
        for task in tasks.iter_mut() {
            if task.args.is_none() {
                task.args = self.args.clone();
            }
        }

        let semaphore = Arc::new(Semaphore::new(self.max_concurrent));
        let (tx, mut rx) = mpsc::channel(tasks.len().max(1));

        let mut handles = Vec::new();
        let cached_ref = self.cached_results.clone();

        for task in tasks {
            let sem = semaphore.clone();
            let agent = agent_factory.clone();
            let tx_clone = tx.clone();
            let mut pause_rx = self.pause_rx.clone();
            let cached = cached_ref.clone();

            handles.push(tokio::spawn(async move {
                // Wait while paused via watch channel
                loop {
                    let is_paused = *pause_rx.borrow_and_update();
                    if !is_paused {
                        break;
                    }
                    if pause_rx.changed().await.is_err() {
                        break; // Channel closed
                    }
                }

                // Check cache first (for resume mechanics)
                let cached_result = {
                    let c = cached.read().await;
                    c.get(&task.id).cloned()
                };

                if let Some(res) = cached_result {
                    let _ = tx_clone.send(Ok(TaskResult { task_id: task.id, output: res })).await;
                    return;
                }

                let _permit = sem.acquire().await.unwrap();
                let res = agent.execute(task.clone()).await;

                if let Ok(ref tr) = res {
                    let mut c = cached.write().await;
                    c.insert(task.id.clone(), tr.output.clone());
                }

                let _ = tx_clone.send(res).await;
            }));
        }

        // drop original tx so rx will close after all tasks finish
        drop(tx);

        let mut results = Vec::new();
        while let Some(res) = rx.recv().await {
            match res {
                Ok(r) => results.push(r),
                Err(e) => {
                    for handle in &handles {
                        handle.abort();
                    }
                    return Err(e);
                }
            }
        }

        for handle in handles {
            handle
                .await
                .map_err(|e| format!("Task failed to join: {}", e))?;
        }

        Ok(results)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};

    struct MockAgent {
        active_agents: Arc<AtomicUsize>,
        max_active_observed: Arc<AtomicUsize>,
    }

    #[async_trait::async_trait]
    impl WorkflowAgent for MockAgent {
        async fn execute(&self, task: Task) -> Result<TaskResult, String> {
            let current = self.active_agents.fetch_add(1, Ordering::SeqCst) + 1;

            // update max observed
            let mut max = self.max_active_observed.load(Ordering::SeqCst);
            while current > max {
                match self.max_active_observed.compare_exchange_weak(
                    max,
                    current,
                    Ordering::SeqCst,
                    Ordering::SeqCst,
                ) {
                    Ok(_) => break,
                    Err(x) => max = x,
                }
            }

            tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;

            self.active_agents.fetch_sub(1, Ordering::SeqCst);

            Ok(TaskResult {
                task_id: task.id,
                output: format!("Processed: {}", task.instructions),
            })
        }
    }


    #[tokio::test]
    async fn test_workflow_manager_save_and_load() {
        let temp_dir = tempfile::tempdir().unwrap();
        let project_dir = temp_dir.path().join("project");
        let global_dir = temp_dir.path().join("home").join(".ohc").join("workflows");

        // Mocking the manager to use our temp global dir
        let mut manager = WorkflowManager::new(project_dir.clone());
        manager.global_dir = Some(global_dir.clone());

        // 1. Save and load project workflow
        manager.save_workflow("my_project_wf", "project script", false).await.unwrap();
        let loaded = manager.load_workflow("my_project_wf").await.unwrap();
        assert_eq!(loaded, "project script");

        // 2. Save and load global workflow
        manager.save_workflow("my_global_wf", "global script", true).await.unwrap();
        let loaded = manager.load_workflow("my_global_wf").await.unwrap();
        assert_eq!(loaded, "global script");

        // 3. Precedence test (project overrides global)
        manager.save_workflow("override_wf", "global script", true).await.unwrap();
        manager.save_workflow("override_wf", "project script", false).await.unwrap();
        let loaded = manager.load_workflow("override_wf").await.unwrap();
        assert_eq!(loaded, "project script"); // Should prioritize project

        // 4. Not found
        let res = manager.load_workflow("non_existent").await;
        assert!(res.is_err());
        assert!(res.unwrap_err().contains("not found"));

        // 5. Path traversal protection
        let res = manager.save_workflow("../../../etc/passwd", "evil", true).await;
        assert!(res.is_err());
        assert!(res.unwrap_err().contains("Invalid workflow name"));

        let loaded = manager.load_workflow("../../../etc/passwd").await;
        assert!(loaded.is_err());
        assert!(loaded.unwrap_err().contains("Invalid workflow name"));
    }

    #[tokio::test]
    async fn test_dynamic_workflow_success() {
        let wf = DynamicWorkflow::new("let x = 1;", None);
        let agent = Arc::new(MockAgent {
            active_agents: Arc::new(AtomicUsize::new(0)),
            max_active_observed: Arc::new(AtomicUsize::new(0)),
        });

        let tasks = vec![
            Task {
                id: "1".to_string(),
                instructions: "task 1".to_string(),
                args: None,
            },
            Task {
                id: "2".to_string(),
                instructions: "task 2".to_string(),
                args: None,
            },
        ];

        let results = wf.run_workflow(tasks, agent.clone()).await.unwrap();
        assert_eq!(results.len(), 2);
    }

    #[tokio::test]
    async fn test_dynamic_workflow_concurrency_limit() {
        let wf = DynamicWorkflow::new("orchestrate();", None);
        let agent = Arc::new(MockAgent {
            active_agents: Arc::new(AtomicUsize::new(0)),
            max_active_observed: Arc::new(AtomicUsize::new(0)),
        });

        let mut tasks = Vec::new();
        for i in 0..50 {
            tasks.push(Task {
                id: i.to_string(),
                instructions: format!("task {}", i),
                args: None,
            });
        }

        let results = wf.run_workflow(tasks, agent.clone()).await.unwrap();
        assert_eq!(results.len(), 50);

        let max_observed = agent.max_active_observed.load(Ordering::SeqCst);
        assert!(
            max_observed <= 16,
            "Should not exceed max_concurrent of 16, but got {}",
            max_observed
        );
    }

    #[tokio::test]
    async fn test_dynamic_workflow_total_limit() {
        let wf = DynamicWorkflow::new("orchestrate();", None);
        let agent = Arc::new(MockAgent {
            active_agents: Arc::new(AtomicUsize::new(0)),
            max_active_observed: Arc::new(AtomicUsize::new(0)),
        });

        let mut tasks = Vec::new();
        for i in 0..1005 {
            tasks.push(Task {
                id: i.to_string(),
                instructions: format!("task {}", i),
                args: None,
            });
        }

        let res = wf.run_workflow(tasks, agent).await;
        assert!(res.is_err());
        assert!(res.unwrap_err().contains("exceeds the max limit of 1000"));
    }

    #[tokio::test]
    async fn test_dynamic_workflow_pause_resume_caching() {
        let wf = DynamicWorkflow::new("orchestrate();", None);
        let agent = Arc::new(MockAgent {
            active_agents: Arc::new(AtomicUsize::new(0)),
            max_active_observed: Arc::new(AtomicUsize::new(0)),
        });

        let tasks = vec![
            Task {
                id: "1".to_string(),
                instructions: "task 1".to_string(),
                args: None,
            },
        ];

        // Pre-populate the cache to simulate "already completed"
        {
            let mut c = wf.cached_results.write().await;
            c.insert("1".to_string(), "Cached Output".to_string());
        }

        let results = wf.run_workflow(tasks, agent.clone()).await.unwrap();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].output, "Cached Output");
        // Ensure the agent was NOT actively run because it was cached
        assert_eq!(agent.max_active_observed.load(Ordering::SeqCst), 0);
    }

    #[tokio::test]
    async fn test_dynamic_workflow_with_args() {
        let args = serde_json::json!({
            "target_issues": [1024, 1025, 1030],
            "config": {
                "verbose": true
            }
        });
        let wf = DynamicWorkflow::new("orchestrate();", Some(args.clone()));
        assert_eq!(wf.args, Some(args));

        let agent = Arc::new(MockAgent {
            active_agents: Arc::new(AtomicUsize::new(0)),
            max_active_observed: Arc::new(AtomicUsize::new(0)),
        });

        let tasks = vec![
            Task {
                id: "1".to_string(),
                instructions: "task 1".to_string(),
                args: None,
            },
        ];

        let results = wf.run_workflow(tasks, agent.clone()).await.unwrap();
        assert_eq!(results.len(), 1);
    }
}
