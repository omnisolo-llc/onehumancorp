/// CrewAI: Role-based + Flow deterministic backbone
use crate::agent::{Agent, AgentRunConfig};
use ohc_builtin_agent_core::types::Message;
use std::sync::Arc;
use serde_json::Value;

/// Defines a specific role an agent can take in the CrewAI architecture (Role-based architecture).
#[derive(Debug, Clone)]
pub struct Role {
    pub name: String,
    pub goal: String,
    pub backstory: String,
}

impl Role {
    pub fn new(name: impl Into<String>, goal: impl Into<String>, backstory: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            goal: goal.into(),
            backstory: backstory.into(),
        }
    }
}

/// A deterministic task with strict routing assigned to a specific role.
#[derive(Debug, Clone)]
pub struct Task {
    pub id: String,
    pub description: String,
    pub expected_output: String,
    pub role: Role,
    pub dependencies: Vec<String>,
}

impl Task {
    pub fn new(id: impl Into<String>, description: impl Into<String>, expected_output: impl Into<String>, role: Role, dependencies: Vec<String>) -> Self {
        Self {
            id: id.into(),
            description: description.into(),
            expected_output: expected_output.into(),
            role,
            dependencies,
        }
    }
}

/// The Crew handles strict routing and validation (the deterministic backbone).
pub struct Crew {
    pub tasks: Vec<Task>,
    pub agent: Arc<Agent>,
    pub config: AgentRunConfig,
}

impl Crew {
    pub fn new(tasks: Vec<Task>, agent: Arc<Agent>, config: AgentRunConfig) -> Self {
        Self { tasks, agent, config }
    }
}

/// The Flow layer executes the deterministic backbone.
pub struct Flow {
    pub crew: Crew,
}

impl Flow {
    pub fn new(crew: Crew) -> Self {
        Self { crew }
    }

    /// Run the tasks following topological sorting (strict routing).
    /// For each task, the LLM only handles "intelligence where it matters"
    /// and the backbone validates the output.
    pub async fn execute(&self) -> Result<std::collections::HashMap<String, String>, String> {
        // Step 1: Topological sort
        let mut in_degree = std::collections::HashMap::new();
        let mut adj = std::collections::HashMap::new();
        let mut task_map = std::collections::HashMap::new();

        for task in &self.crew.tasks {
            if task_map.contains_key(&task.id) {
                return Err(format!("Duplicate task ID detected: {}", task.id));
            }
            in_degree.insert(task.id.clone(), 0);
            task_map.insert(task.id.clone(), task);
        }

        for task in &self.crew.tasks {
            for dep in &task.dependencies {
                if !task_map.contains_key(dep) {
                    return Err(format!("Task {} depends on missing task {}", task.id, dep));
                }
                *in_degree.entry(task.id.clone()).or_insert(0) += 1;
                adj.entry(dep.clone()).or_insert_with(Vec::new).push(task.id.clone());
            }
        }

        let mut queue = std::collections::VecDeque::new();
        // Iterate over self.crew.tasks to maintain a deterministic initialization order
        for task in &self.crew.tasks {
            if let Some(&deg) = in_degree.get(&task.id) {
                if deg == 0 {
                    queue.push_back(task.id.clone());
                }
            }
        }

        let mut sorted_tasks = Vec::new();
        while let Some(id) = queue.pop_front() {
            sorted_tasks.push(id.clone());
            if let Some(neighbors) = adj.get(&id) {
                for next_id in neighbors {
                    if let Some(deg) = in_degree.get_mut(next_id) {
                        *deg -= 1;
                        if *deg == 0 {
                            queue.push_back(next_id.clone());
                        }
                    }
                }
            }
        }

        if sorted_tasks.len() != self.crew.tasks.len() {
            return Err("Cycle detected in task dependencies".to_string());
        }

        // Step 2: Execute strictly in topological order
        let mut results = std::collections::HashMap::new();

        for task_id in sorted_tasks {
            let task = task_map.get(&task_id).unwrap();

            let system_prompt = format!(
                "You are {}.\nGoal: {}\nBackstory: {}\n\nTask:\n{}\n\nExpected Output Schema/Format:\n{}",
                task.role.name, task.role.goal, task.role.backstory, task.description, task.expected_output
            );

            let mut run_cfg = self.crew.config.clone();
            run_cfg.server_system_message = system_prompt;

            // Gather context from dependencies
            let mut context_str = String::new();
            for dep in &task.dependencies {
                if let Some(dep_out) = results.get(dep) {
                    context_str.push_str(&format!("[Dependency {} Output]:\n{}\n\n", dep, dep_out));
                }
            }

            let user_prompt = if context_str.is_empty() {
                format!("Execute the task. Ensure the output matches the expected format.")
            } else {
                format!("Execute the task using the following context from dependencies:\n{}Ensure the output matches the expected format.", context_str)
            };

            let mut on_event = |_| {};

            let result = self.crew.agent.run(&run_cfg, &user_prompt, &mut on_event).await
                .map_err(|e| format!("Task {} failed: {}", task.id, e))?;

            // Simple deterministic validation
            if task.expected_output.to_lowercase().contains("json") {
                if let Err(_) = serde_json::from_str::<Value>(&result) {
                    return Err(format!("Task {} failed validation: expected JSON output", task.id));
                }
            }

            if result.trim().is_empty() {
                return Err(format!("Task {} failed validation: output is empty", task.id));
            }

            results.insert(task.id.clone(), result);
        }

        Ok(results)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ohc_builtin_agent_core::types::{ChatRequest, ChatResponse, Usage, ToolCall};
    use crate::llm::client::LlmClient;
    use tokio::sync::Mutex;

    struct MockLlmClientCrew {
        responses: Mutex<Vec<String>>,
    }

    #[async_trait::async_trait]
    impl LlmClient for MockLlmClientCrew {
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

    #[tokio::test]
    async fn test_crewai_flow_success() {
        let client = Arc::new(MockLlmClientCrew {
            responses: Mutex::new(vec![
                "Analysis complete".to_string(),
                "{\"report\": \"JSON data\"}".to_string(),
            ]),
        });

        let agent = Arc::new(Agent::new(client, vec![]));
        let cfg = AgentRunConfig::default();

        let role1 = Role::new("Researcher", "Find data", "You are an expert researcher.");
        let task1 = Task::new("T1", "Research topic X", "Plain text summary", role1, vec![]);

        let role2 = Role::new("Writer", "Write report", "You write JSON reports.");
        let task2 = Task::new("T2", "Write JSON report", "JSON", role2, vec!["T1".to_string()]);

        let crew = Crew::new(vec![task1, task2], agent, cfg);
        let flow = Flow::new(crew);

        let results = flow.execute().await;
        assert!(results.is_ok());
        let outputs = results.unwrap();
        assert_eq!(outputs.len(), 2);
        assert_eq!(outputs.get("T1").unwrap(), "Analysis complete");
        assert_eq!(outputs.get("T2").unwrap(), "{\"report\": \"JSON data\"}");
    }

    #[tokio::test]
    async fn test_crewai_flow_topological_sort_success() {
        let client = Arc::new(MockLlmClientCrew {
            responses: Mutex::new(vec![
                "A output".to_string(),
                "B output".to_string(),
                "C output".to_string(),
                "D output".to_string(),
            ]),
        });

        let agent = Arc::new(Agent::new(client, vec![]));
        let cfg = AgentRunConfig::default();

        let role = Role::new("Test", "Test", "Test");
        let task_d = Task::new("D", "Task D", "text", role.clone(), vec!["B".to_string(), "C".to_string()]);
        let task_b = Task::new("B", "Task B", "text", role.clone(), vec!["A".to_string()]);
        let task_c = Task::new("C", "Task C", "text", role.clone(), vec!["A".to_string()]);
        let task_a = Task::new("A", "Task A", "text", role.clone(), vec![]);

        // Out of order insertion
        let crew = Crew::new(vec![task_d, task_b, task_c, task_a], agent, cfg);
        let flow = Flow::new(crew);

        let results = flow.execute().await;
        assert!(results.is_ok());
        let outputs = results.unwrap();
        assert_eq!(outputs.len(), 4);
        assert_eq!(outputs.get("A").unwrap(), "A output");
        assert_eq!(outputs.get("B").unwrap(), "B output");
        assert_eq!(outputs.get("C").unwrap(), "C output");
        assert_eq!(outputs.get("D").unwrap(), "D output");
    }

    #[tokio::test]
    async fn test_crewai_flow_cycle_detection() {
        let client = Arc::new(MockLlmClientCrew { responses: Mutex::new(vec![]) });
        let agent = Arc::new(Agent::new(client, vec![]));
        let cfg = AgentRunConfig::default();

        let role = Role::new("Test", "Test", "Test");
        let task_a = Task::new("A", "Task A", "text", role.clone(), vec!["B".to_string()]);
        let task_b = Task::new("B", "Task B", "text", role.clone(), vec!["A".to_string()]);

        let crew = Crew::new(vec![task_a, task_b], agent, cfg);
        let flow = Flow::new(crew);

        let result = flow.execute().await;
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Cycle detected in task dependencies");
    }

    #[tokio::test]
    async fn test_crewai_flow_missing_dependency() {
        let client = Arc::new(MockLlmClientCrew { responses: Mutex::new(vec![]) });
        let agent = Arc::new(Agent::new(client, vec![]));
        let cfg = AgentRunConfig::default();

        let role = Role::new("Test", "Test", "Test");
        let task_a = Task::new("A", "Task A", "text", role.clone(), vec!["NON_EXISTENT".to_string()]);

        let crew = Crew::new(vec![task_a], agent, cfg);
        let flow = Flow::new(crew);

        let result = flow.execute().await;
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Task A depends on missing task NON_EXISTENT");
    }

    #[tokio::test]
    async fn test_crewai_flow_json_validation_failure() {
        let client = Arc::new(MockLlmClientCrew {
            responses: Mutex::new(vec![
                "This is not JSON text".to_string(),
            ]),
        });

        let agent = Arc::new(Agent::new(client, vec![]));
        let cfg = AgentRunConfig::default();

        let role = Role::new("Writer", "Write report", "You write JSON reports.");
        let task = Task::new("T1", "Write JSON report", "JSON", role, vec![]);

        let crew = Crew::new(vec![task], agent, cfg);
        let flow = Flow::new(crew);

        let result = flow.execute().await;
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Task T1 failed validation: expected JSON output");
    }

    #[tokio::test]
    async fn test_crewai_flow_empty_output_failure() {
        let client = Arc::new(MockLlmClientCrew {
            responses: Mutex::new(vec![
                "   ".to_string(),
            ]),
        });

        let agent = Arc::new(Agent::new(client, vec![]));
        let cfg = AgentRunConfig::default();

        let role = Role::new("Writer", "Write report", "You write text.");
        let task = Task::new("T1", "Write text", "text", role, vec![]);

        let crew = Crew::new(vec![task], agent, cfg);
        let flow = Flow::new(crew);

        let result = flow.execute().await;
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Task T1 failed validation: output is empty");
    }

    #[tokio::test]
    async fn test_crewai_flow_agent_error() {
        struct FailingLlmClient;
        #[async_trait::async_trait]
        impl LlmClient for FailingLlmClient {
            async fn chat(&self, _req: ChatRequest) -> Result<ChatResponse, Box<dyn std::error::Error + Send + Sync>> {
                Err("LLM completely failed".into())
            }
        }

        let agent = Arc::new(Agent::new(Arc::new(FailingLlmClient), vec![]));
        let cfg = AgentRunConfig::default();

        let role = Role::new("Writer", "Write report", "You write text.");
        let task = Task::new("T1", "Write text", "text", role, vec![]);

        let crew = Crew::new(vec![task], agent, cfg);
        let flow = Flow::new(crew);

        let result = flow.execute().await;
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Task T1 failed"));
        assert!(result.unwrap_err().contains("LLM completely failed"));
    }
}