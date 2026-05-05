use crate::agent::{Agent, AgentRunConfig};
use ohc_builtin_agent_core::types::Message;
use std::sync::Arc;
use serde_json::Value;

/// Defines a specific role an agent can take in the CrewAI architecture.
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
    pub description: String,
    pub expected_output: String,
    pub role: Role,
}

impl Task {
    pub fn new(description: impl Into<String>, expected_output: impl Into<String>, role: Role) -> Self {
        Self {
            description: description.into(),
            expected_output: expected_output.into(),
            role,
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

    /// Run the tasks sequentially. For each task, the LLM only handles "intelligence where it matters"
    /// and the backbone validates the output.
    pub async fn execute(&self) -> Result<Vec<String>, String> {
        let mut results = Vec::new();
        let mut previous_output = String::new();

        for (i, task) in self.crew.tasks.iter().enumerate() {
            let system_prompt = format!(
                "You are {}.\nGoal: {}\nBackstory: {}\n\nTask:\n{}\n\nExpected Output Schema/Format:\n{}",
                task.role.name, task.role.goal, task.role.backstory, task.description, task.expected_output
            );

            let mut run_cfg = self.crew.config.clone();
            run_cfg.server_system_message = system_prompt;

            // Build the prompt containing context from the previous task, if any.
            let user_prompt = if previous_output.is_empty() {
                format!("Execute the task. Ensure the output matches the expected format.")
            } else {
                format!("Execute the task using the following context from the previous step:\n{}\n\nEnsure the output matches the expected format.", previous_output)
            };

            let mut on_event = |_| {};

            let result = self.crew.agent.run(&run_cfg, &user_prompt, &mut on_event).await
                .map_err(|e| format!("Task {} failed: {}", i, e))?;

            // Simple deterministic validation: ensure it's not empty, and if expected_output hints at JSON, check JSON parseability.
            if task.expected_output.to_lowercase().contains("json") {
                if let Err(_) = serde_json::from_str::<Value>(&result) {
                    return Err(format!("Task {} failed validation: expected JSON output", i));
                }
            }

            if result.trim().is_empty() {
                return Err(format!("Task {} failed validation: output is empty", i));
            }

            previous_output = result.clone();
            results.push(result);
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
        let task1 = Task::new("Research topic X", "Plain text summary", role1);

        let role2 = Role::new("Writer", "Write report", "You write JSON reports.");
        let task2 = Task::new("Write JSON report", "JSON", role2);

        let crew = Crew::new(vec![task1, task2], agent, cfg);
        let flow = Flow::new(crew);

        let results = flow.execute().await;
        assert!(results.is_ok());
        let outputs = results.unwrap();
        assert_eq!(outputs.len(), 2);
        assert_eq!(outputs[0], "Analysis complete");
        assert_eq!(outputs[1], "{\"report\": \"JSON data\"}");
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
        let task = Task::new("Write JSON report", "JSON", role);

        let crew = Crew::new(vec![task], agent, cfg);
        let flow = Flow::new(crew);

        let result = flow.execute().await;
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Task 0 failed validation: expected JSON output");
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
        let task = Task::new("Write text", "text", role);

        let crew = Crew::new(vec![task], agent, cfg);
        let flow = Flow::new(crew);

        let result = flow.execute().await;
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Task 0 failed validation: output is empty");
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
        let task = Task::new("Write text", "text", role);

        let crew = Crew::new(vec![task], agent, cfg);
        let flow = Flow::new(crew);

        let result = flow.execute().await;
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Task 0 failed"));
        assert!(result.unwrap_err().contains("LLM completely failed"));
    }
}