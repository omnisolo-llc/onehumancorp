use crate::agent::{Agent, AgentRunConfig};
use ohc_builtin_agent_core::types::Message;
use std::sync::Arc;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum TaskStatus {
    Pending,
    InProgress,
    Completed,
    Failed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub id: String,
    pub description: String,
    pub status: TaskStatus,
    pub assignee: Option<String>,
    pub result: Option<String>,
}

#[derive(Debug, Clone, Default)]
pub struct Ledger {
    pub tasks: Vec<Task>,
}

impl Ledger {
    pub fn new() -> Self {
        Self { tasks: Vec::new() }
    }

    pub fn add_task(&mut self, description: impl Into<String>) -> String {
        let id = uuid::Uuid::new_v4().to_string();
        let task = Task {
            id: id.clone(),
            description: description.into(),
            status: TaskStatus::Pending,
            assignee: None,
            result: None,
        };
        self.tasks.push(task);
        id
    }

    pub fn assign_task(&mut self, id: &str, assignee: &str) {
        if let Some(task) = self.tasks.iter_mut().find(|t| t.id == id) {
            task.status = TaskStatus::InProgress;
            task.assignee = Some(assignee.to_string());
        }
    }

    pub fn complete_task(&mut self, id: &str, result: String) {
        if let Some(task) = self.tasks.iter_mut().find(|t| t.id == id) {
            task.status = TaskStatus::Completed;
            task.result = Some(result);
        }
    }

    pub fn fail_task(&mut self, id: &str, error: String) {
        if let Some(task) = self.tasks.iter_mut().find(|t| t.id == id) {
            task.status = TaskStatus::Failed;
            task.result = Some(error);
        }
    }

    pub fn is_all_completed(&self) -> bool {
        !self.tasks.is_empty() && self.tasks.iter().all(|t| t.status == TaskStatus::Completed)
    }

    pub fn get_pending_tasks(&self) -> Vec<Task> {
        self.tasks.iter().filter(|t| t.status == TaskStatus::Pending).cloned().collect()
    }
}

pub struct Worker {
    pub name: String,
    pub agent: Arc<Agent>,
    pub config: AgentRunConfig,
}

impl Worker {
    pub fn new(name: impl Into<String>, agent: Arc<Agent>, config: AgentRunConfig) -> Self {
        Self {
            name: name.into(),
            agent,
            config,
        }
    }

    pub async fn execute_task(&self, task_description: &str) -> Result<String, String> {
        let mut on_event = |_| {};
        self.agent.run(&self.config, task_description, &mut on_event).await
            .map_err(|e| e.to_string())
    }
}

/// The Manager Agent dynamically updates a task ledger (Magentic-One pattern)
pub struct Manager {
    pub ledger: std::sync::RwLock<Ledger>,
    pub workers: Vec<Arc<Worker>>,
    pub agent: Arc<Agent>,
    pub config: AgentRunConfig,
}

impl Manager {
    pub fn new(agent: Arc<Agent>, config: AgentRunConfig, workers: Vec<Arc<Worker>>) -> Self {
        Self {
            ledger: std::sync::RwLock::new(Ledger::new()),
            workers,
            agent,
            config,
        }
    }

    /// Break down a high-level goal into tasks and add them to the ledger
    pub async fn breakdown_goal(&self, goal: &str) -> Result<(), String> {
        let prompt = format!(
            "Break down the following goal into discrete tasks. Output ONLY a JSON array of strings representing the task descriptions. Goal: {}",
            goal
        );

        let mut on_event = |_| {};
        let result = self.agent.run(&self.config, &prompt, &mut on_event).await
            .map_err(|e| e.to_string())?;

        let mut json_str = result.trim();
        if let Some(start) = json_str.find('[') {
            if let Some(end) = json_str.rfind(']') {
                json_str = &json_str[start..=end];
            }
        }

        let parsed: Vec<String> = serde_json::from_str(json_str).unwrap_or_else(|_| {
            // Fallback if parsing fails
            vec![format!("Execute goal: {}", goal)]
        });

        let mut ledger = self.ledger.write().unwrap();
        for desc in parsed {
            ledger.add_task(desc);
        }

        Ok(())
    }

    /// Run the Magentic-One loop: dynamically assign tasks from the ledger to workers and update outcomes
    pub async fn orchestrate(&self) -> Result<(), String> {
        loop {
            let pending_tasks = {
                let ledger = self.ledger.read().unwrap();
                if ledger.is_all_completed() {
                    break;
                }
                ledger.get_pending_tasks()
            };

            if pending_tasks.is_empty() {
                // If there are no pending tasks but we haven't broken out, it means some tasks are failed or in progress.
                // For this simple mock, we'll just break.
                break;
            }

            for task in pending_tasks {
                // Simple round-robin assignment for demonstration
                let worker_idx = {
                    let id_hash = task.id.chars().map(|c| c as usize).sum::<usize>();
                    if self.workers.is_empty() { return Err("No workers available".to_string()); }
                    id_hash % self.workers.len()
                };

                let worker = &self.workers[worker_idx];

                {
                    let mut ledger = self.ledger.write().unwrap();
                    ledger.assign_task(&task.id, &worker.name);
                }

                // Execute
                match worker.execute_task(&task.description).await {
                    Ok(result) => {
                        let mut ledger = self.ledger.write().unwrap();
                        ledger.complete_task(&task.id, result);
                    }
                    Err(e) => {
                        let mut ledger = self.ledger.write().unwrap();
                        ledger.fail_task(&task.id, e);
                    }
                }
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ohc_builtin_agent_core::types::{ChatRequest, ChatResponse, Usage};
    use crate::llm::LlmClient;
    use tokio::sync::Mutex;

    struct MockLlmClientMagentic {
        responses: Mutex<Vec<String>>,
    }

    #[async_trait::async_trait]
    impl LlmClient for MockLlmClientMagentic {
        async fn chat(&self, _req: ChatRequest) -> Result<ChatResponse, Box<dyn std::error::Error + Send + Sync>> {
            let mut resps = self.responses.lock().await;
            let content = if !resps.is_empty() {
                resps.remove(0)
            } else {
                "default".to_string()
            };

            Ok(ChatResponse {
                message: Message::assistant(content),
                usage: Usage::default(),
                stop_reason: "stop".to_string(),
                response_id: Some("mock-id".to_string()),
            })
        }
    }

    #[test]
    fn test_ledger_updates() {
        let mut ledger = Ledger::new();
        let id1 = ledger.add_task("Task 1");
        let id2 = ledger.add_task("Task 2");

        assert_eq!(ledger.get_pending_tasks().len(), 2);
        assert!(!ledger.is_all_completed());

        ledger.assign_task(&id1, "Worker A");
        assert_eq!(ledger.get_pending_tasks().len(), 1);

        ledger.complete_task(&id1, "Result 1".to_string());
        ledger.fail_task(&id2, "Error 2".to_string());

        assert!(!ledger.is_all_completed());

        let tasks = &ledger.tasks;
        assert_eq!(tasks[0].status, TaskStatus::Completed);
        assert_eq!(tasks[0].result.as_deref(), Some("Result 1"));

        assert_eq!(tasks[1].status, TaskStatus::Failed);
        assert_eq!(tasks[1].result.as_deref(), Some("Error 2"));
    }

    #[tokio::test]
    async fn test_manager_workflow() {
        let manager_client = Arc::new(MockLlmClientMagentic {
            responses: Mutex::new(vec![
                "[\"Subtask A\", \"Subtask B\"]".to_string()
            ]),
        });
        let manager_agent = Arc::new(Agent::new(manager_client, vec![]));

        let worker_client = Arc::new(MockLlmClientMagentic {
            responses: Mutex::new(vec![
                "Worker done A".to_string(),
                "Worker done B".to_string()
            ]),
        });
        let worker_agent = Arc::new(Agent::new(worker_client, vec![]));
        let worker = Arc::new(Worker::new("TestWorker", worker_agent, AgentRunConfig::default()));

        let manager = Manager::new(manager_agent, AgentRunConfig::default(), vec![worker]);

        let res = manager.breakdown_goal("Do big thing").await;
        assert!(res.is_ok());

        {
            let ledger = manager.ledger.read().unwrap();
            assert_eq!(ledger.tasks.len(), 2);
            assert_eq!(ledger.tasks[0].description, "Subtask A");
            assert_eq!(ledger.tasks[1].description, "Subtask B");
        }

        let orchestrate_res = manager.orchestrate().await;
        assert!(orchestrate_res.is_ok());

        {
            let ledger = manager.ledger.read().unwrap();
            assert!(ledger.is_all_completed());
            assert_eq!(ledger.tasks[0].status, TaskStatus::Completed);
            assert!(ledger.tasks[0].result.as_ref().unwrap().starts_with("Worker done"));
        }
    }
}
