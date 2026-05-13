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

    #[tokio::test]
    async fn test_crewai_flow_extended_1() {
        let client = Arc::new(MockLlmClientCrew {
            responses: Mutex::new(vec![
                "Analysis complete 1".to_string(),
                "{\"report\": \"JSON data 1\"}".to_string(),
            ]),
        });

        let agent = Arc::new(Agent::new(client, vec![]));
        let cfg = AgentRunConfig::default();

        let role1 = Role::new("Researcher 1", "Find data 1", "You are an expert researcher.");
        let task1 = Task::new("Research topic X 1", "Plain text summary", role1);

        let role2 = Role::new("Writer 1", "Write report 1", "You write JSON reports.");
        let task2 = Task::new("Write JSON report 1", "JSON", role2);

        let crew = Crew::new(vec![task1, task2], agent, cfg);
        let flow = Flow::new(crew);

        let results = flow.execute().await;
        assert!(results.is_ok());
        let outputs = results.unwrap();
        assert_eq!(outputs.len(), 2);
        assert_eq!(outputs[0], "Analysis complete 1");
        assert_eq!(outputs[1], "{\"report\": \"JSON data 1\"}");
    }

    #[tokio::test]
    async fn test_crewai_flow_extended_2() {
        let client = Arc::new(MockLlmClientCrew {
            responses: Mutex::new(vec![
                "Analysis complete 2".to_string(),
                "{\"report\": \"JSON data 2\"}".to_string(),
            ]),
        });

        let agent = Arc::new(Agent::new(client, vec![]));
        let cfg = AgentRunConfig::default();

        let role1 = Role::new("Researcher 2", "Find data 2", "You are an expert researcher.");
        let task1 = Task::new("Research topic X 2", "Plain text summary", role1);

        let role2 = Role::new("Writer 2", "Write report 2", "You write JSON reports.");
        let task2 = Task::new("Write JSON report 2", "JSON", role2);

        let crew = Crew::new(vec![task1, task2], agent, cfg);
        let flow = Flow::new(crew);

        let results = flow.execute().await;
        assert!(results.is_ok());
        let outputs = results.unwrap();
        assert_eq!(outputs.len(), 2);
        assert_eq!(outputs[0], "Analysis complete 2");
        assert_eq!(outputs[1], "{\"report\": \"JSON data 2\"}");
    }

    #[tokio::test]
    async fn test_crewai_flow_extended_3() {
        let client = Arc::new(MockLlmClientCrew {
            responses: Mutex::new(vec![
                "Analysis complete 3".to_string(),
                "{\"report\": \"JSON data 3\"}".to_string(),
            ]),
        });

        let agent = Arc::new(Agent::new(client, vec![]));
        let cfg = AgentRunConfig::default();

        let role1 = Role::new("Researcher 3", "Find data 3", "You are an expert researcher.");
        let task1 = Task::new("Research topic X 3", "Plain text summary", role1);

        let role2 = Role::new("Writer 3", "Write report 3", "You write JSON reports.");
        let task2 = Task::new("Write JSON report 3", "JSON", role2);

        let crew = Crew::new(vec![task1, task2], agent, cfg);
        let flow = Flow::new(crew);

        let results = flow.execute().await;
        assert!(results.is_ok());
        let outputs = results.unwrap();
        assert_eq!(outputs.len(), 2);
        assert_eq!(outputs[0], "Analysis complete 3");
        assert_eq!(outputs[1], "{\"report\": \"JSON data 3\"}");
    }

    #[tokio::test]
    async fn test_crewai_flow_extended_4() {
        let client = Arc::new(MockLlmClientCrew {
            responses: Mutex::new(vec![
                "Analysis complete 4".to_string(),
                "{\"report\": \"JSON data 4\"}".to_string(),
            ]),
        });

        let agent = Arc::new(Agent::new(client, vec![]));
        let cfg = AgentRunConfig::default();

        let role1 = Role::new("Researcher 4", "Find data 4", "You are an expert researcher.");
        let task1 = Task::new("Research topic X 4", "Plain text summary", role1);

        let role2 = Role::new("Writer 4", "Write report 4", "You write JSON reports.");
        let task2 = Task::new("Write JSON report 4", "JSON", role2);

        let crew = Crew::new(vec![task1, task2], agent, cfg);
        let flow = Flow::new(crew);

        let results = flow.execute().await;
        assert!(results.is_ok());
        let outputs = results.unwrap();
        assert_eq!(outputs.len(), 2);
        assert_eq!(outputs[0], "Analysis complete 4");
        assert_eq!(outputs[1], "{\"report\": \"JSON data 4\"}");
    }

    #[tokio::test]
    async fn test_crewai_flow_extended_5() {
        let client = Arc::new(MockLlmClientCrew {
            responses: Mutex::new(vec![
                "Analysis complete 5".to_string(),
                "{\"report\": \"JSON data 5\"}".to_string(),
            ]),
        });

        let agent = Arc::new(Agent::new(client, vec![]));
        let cfg = AgentRunConfig::default();

        let role1 = Role::new("Researcher 5", "Find data 5", "You are an expert researcher.");
        let task1 = Task::new("Research topic X 5", "Plain text summary", role1);

        let role2 = Role::new("Writer 5", "Write report 5", "You write JSON reports.");
        let task2 = Task::new("Write JSON report 5", "JSON", role2);

        let crew = Crew::new(vec![task1, task2], agent, cfg);
        let flow = Flow::new(crew);

        let results = flow.execute().await;
        assert!(results.is_ok());
        let outputs = results.unwrap();
        assert_eq!(outputs.len(), 2);
        assert_eq!(outputs[0], "Analysis complete 5");
        assert_eq!(outputs[1], "{\"report\": \"JSON data 5\"}");
    }

    #[tokio::test]
    async fn test_crewai_flow_extended_6() {
        let client = Arc::new(MockLlmClientCrew {
            responses: Mutex::new(vec![
                "Analysis complete 6".to_string(),
                "{\"report\": \"JSON data 6\"}".to_string(),
            ]),
        });

        let agent = Arc::new(Agent::new(client, vec![]));
        let cfg = AgentRunConfig::default();

        let role1 = Role::new("Researcher 6", "Find data 6", "You are an expert researcher.");
        let task1 = Task::new("Research topic X 6", "Plain text summary", role1);

        let role2 = Role::new("Writer 6", "Write report 6", "You write JSON reports.");
        let task2 = Task::new("Write JSON report 6", "JSON", role2);

        let crew = Crew::new(vec![task1, task2], agent, cfg);
        let flow = Flow::new(crew);

        let results = flow.execute().await;
        assert!(results.is_ok());
        let outputs = results.unwrap();
        assert_eq!(outputs.len(), 2);
        assert_eq!(outputs[0], "Analysis complete 6");
        assert_eq!(outputs[1], "{\"report\": \"JSON data 6\"}");
    }

    #[tokio::test]
    async fn test_crewai_flow_extended_7() {
        let client = Arc::new(MockLlmClientCrew {
            responses: Mutex::new(vec![
                "Analysis complete 7".to_string(),
                "{\"report\": \"JSON data 7\"}".to_string(),
            ]),
        });

        let agent = Arc::new(Agent::new(client, vec![]));
        let cfg = AgentRunConfig::default();

        let role1 = Role::new("Researcher 7", "Find data 7", "You are an expert researcher.");
        let task1 = Task::new("Research topic X 7", "Plain text summary", role1);

        let role2 = Role::new("Writer 7", "Write report 7", "You write JSON reports.");
        let task2 = Task::new("Write JSON report 7", "JSON", role2);

        let crew = Crew::new(vec![task1, task2], agent, cfg);
        let flow = Flow::new(crew);

        let results = flow.execute().await;
        assert!(results.is_ok());
        let outputs = results.unwrap();
        assert_eq!(outputs.len(), 2);
        assert_eq!(outputs[0], "Analysis complete 7");
        assert_eq!(outputs[1], "{\"report\": \"JSON data 7\"}");
    }

    #[tokio::test]
    async fn test_crewai_flow_extended_8() {
        let client = Arc::new(MockLlmClientCrew {
            responses: Mutex::new(vec![
                "Analysis complete 8".to_string(),
                "{\"report\": \"JSON data 8\"}".to_string(),
            ]),
        });

        let agent = Arc::new(Agent::new(client, vec![]));
        let cfg = AgentRunConfig::default();

        let role1 = Role::new("Researcher 8", "Find data 8", "You are an expert researcher.");
        let task1 = Task::new("Research topic X 8", "Plain text summary", role1);

        let role2 = Role::new("Writer 8", "Write report 8", "You write JSON reports.");
        let task2 = Task::new("Write JSON report 8", "JSON", role2);

        let crew = Crew::new(vec![task1, task2], agent, cfg);
        let flow = Flow::new(crew);

        let results = flow.execute().await;
        assert!(results.is_ok());
        let outputs = results.unwrap();
        assert_eq!(outputs.len(), 2);
        assert_eq!(outputs[0], "Analysis complete 8");
        assert_eq!(outputs[1], "{\"report\": \"JSON data 8\"}");
    }

    #[tokio::test]
    async fn test_crewai_flow_extended_9() {
        let client = Arc::new(MockLlmClientCrew {
            responses: Mutex::new(vec![
                "Analysis complete 9".to_string(),
                "{\"report\": \"JSON data 9\"}".to_string(),
            ]),
        });

        let agent = Arc::new(Agent::new(client, vec![]));
        let cfg = AgentRunConfig::default();

        let role1 = Role::new("Researcher 9", "Find data 9", "You are an expert researcher.");
        let task1 = Task::new("Research topic X 9", "Plain text summary", role1);

        let role2 = Role::new("Writer 9", "Write report 9", "You write JSON reports.");
        let task2 = Task::new("Write JSON report 9", "JSON", role2);

        let crew = Crew::new(vec![task1, task2], agent, cfg);
        let flow = Flow::new(crew);

        let results = flow.execute().await;
        assert!(results.is_ok());
        let outputs = results.unwrap();
        assert_eq!(outputs.len(), 2);
        assert_eq!(outputs[0], "Analysis complete 9");
        assert_eq!(outputs[1], "{\"report\": \"JSON data 9\"}");
    }

    #[tokio::test]
    async fn test_crewai_flow_extended_10() {
        let client = Arc::new(MockLlmClientCrew {
            responses: Mutex::new(vec![
                "Analysis complete 10".to_string(),
                "{\"report\": \"JSON data 10\"}".to_string(),
            ]),
        });

        let agent = Arc::new(Agent::new(client, vec![]));
        let cfg = AgentRunConfig::default();

        let role1 = Role::new("Researcher 10", "Find data 10", "You are an expert researcher.");
        let task1 = Task::new("Research topic X 10", "Plain text summary", role1);

        let role2 = Role::new("Writer 10", "Write report 10", "You write JSON reports.");
        let task2 = Task::new("Write JSON report 10", "JSON", role2);

        let crew = Crew::new(vec![task1, task2], agent, cfg);
        let flow = Flow::new(crew);

        let results = flow.execute().await;
        assert!(results.is_ok());
        let outputs = results.unwrap();
        assert_eq!(outputs.len(), 2);
        assert_eq!(outputs[0], "Analysis complete 10");
        assert_eq!(outputs[1], "{\"report\": \"JSON data 10\"}");
    }

    #[tokio::test]
    async fn test_crewai_flow_extended_11() {
        let client = Arc::new(MockLlmClientCrew {
            responses: Mutex::new(vec![
                "Analysis complete 11".to_string(),
                "{\"report\": \"JSON data 11\"}".to_string(),
            ]),
        });

        let agent = Arc::new(Agent::new(client, vec![]));
        let cfg = AgentRunConfig::default();

        let role1 = Role::new("Researcher 11", "Find data 11", "You are an expert researcher.");
        let task1 = Task::new("Research topic X 11", "Plain text summary", role1);

        let role2 = Role::new("Writer 11", "Write report 11", "You write JSON reports.");
        let task2 = Task::new("Write JSON report 11", "JSON", role2);

        let crew = Crew::new(vec![task1, task2], agent, cfg);
        let flow = Flow::new(crew);

        let results = flow.execute().await;
        assert!(results.is_ok());
        let outputs = results.unwrap();
        assert_eq!(outputs.len(), 2);
        assert_eq!(outputs[0], "Analysis complete 11");
        assert_eq!(outputs[1], "{\"report\": \"JSON data 11\"}");
    }

    #[tokio::test]
    async fn test_crewai_flow_extended_12() {
        let client = Arc::new(MockLlmClientCrew {
            responses: Mutex::new(vec![
                "Analysis complete 12".to_string(),
                "{\"report\": \"JSON data 12\"}".to_string(),
            ]),
        });

        let agent = Arc::new(Agent::new(client, vec![]));
        let cfg = AgentRunConfig::default();

        let role1 = Role::new("Researcher 12", "Find data 12", "You are an expert researcher.");
        let task1 = Task::new("Research topic X 12", "Plain text summary", role1);

        let role2 = Role::new("Writer 12", "Write report 12", "You write JSON reports.");
        let task2 = Task::new("Write JSON report 12", "JSON", role2);

        let crew = Crew::new(vec![task1, task2], agent, cfg);
        let flow = Flow::new(crew);

        let results = flow.execute().await;
        assert!(results.is_ok());
        let outputs = results.unwrap();
        assert_eq!(outputs.len(), 2);
        assert_eq!(outputs[0], "Analysis complete 12");
        assert_eq!(outputs[1], "{\"report\": \"JSON data 12\"}");
    }

    #[tokio::test]
    async fn test_crewai_flow_extended_13() {
        let client = Arc::new(MockLlmClientCrew {
            responses: Mutex::new(vec![
                "Analysis complete 13".to_string(),
                "{\"report\": \"JSON data 13\"}".to_string(),
            ]),
        });

        let agent = Arc::new(Agent::new(client, vec![]));
        let cfg = AgentRunConfig::default();

        let role1 = Role::new("Researcher 13", "Find data 13", "You are an expert researcher.");
        let task1 = Task::new("Research topic X 13", "Plain text summary", role1);

        let role2 = Role::new("Writer 13", "Write report 13", "You write JSON reports.");
        let task2 = Task::new("Write JSON report 13", "JSON", role2);

        let crew = Crew::new(vec![task1, task2], agent, cfg);
        let flow = Flow::new(crew);

        let results = flow.execute().await;
        assert!(results.is_ok());
        let outputs = results.unwrap();
        assert_eq!(outputs.len(), 2);
        assert_eq!(outputs[0], "Analysis complete 13");
        assert_eq!(outputs[1], "{\"report\": \"JSON data 13\"}");
    }

    #[tokio::test]
    async fn test_crewai_flow_extended_14() {
        let client = Arc::new(MockLlmClientCrew {
            responses: Mutex::new(vec![
                "Analysis complete 14".to_string(),
                "{\"report\": \"JSON data 14\"}".to_string(),
            ]),
        });

        let agent = Arc::new(Agent::new(client, vec![]));
        let cfg = AgentRunConfig::default();

        let role1 = Role::new("Researcher 14", "Find data 14", "You are an expert researcher.");
        let task1 = Task::new("Research topic X 14", "Plain text summary", role1);

        let role2 = Role::new("Writer 14", "Write report 14", "You write JSON reports.");
        let task2 = Task::new("Write JSON report 14", "JSON", role2);

        let crew = Crew::new(vec![task1, task2], agent, cfg);
        let flow = Flow::new(crew);

        let results = flow.execute().await;
        assert!(results.is_ok());
        let outputs = results.unwrap();
        assert_eq!(outputs.len(), 2);
        assert_eq!(outputs[0], "Analysis complete 14");
        assert_eq!(outputs[1], "{\"report\": \"JSON data 14\"}");
    }

    #[tokio::test]
    async fn test_crewai_flow_extended_15() {
        let client = Arc::new(MockLlmClientCrew {
            responses: Mutex::new(vec![
                "Analysis complete 15".to_string(),
                "{\"report\": \"JSON data 15\"}".to_string(),
            ]),
        });

        let agent = Arc::new(Agent::new(client, vec![]));
        let cfg = AgentRunConfig::default();

        let role1 = Role::new("Researcher 15", "Find data 15", "You are an expert researcher.");
        let task1 = Task::new("Research topic X 15", "Plain text summary", role1);

        let role2 = Role::new("Writer 15", "Write report 15", "You write JSON reports.");
        let task2 = Task::new("Write JSON report 15", "JSON", role2);

        let crew = Crew::new(vec![task1, task2], agent, cfg);
        let flow = Flow::new(crew);

        let results = flow.execute().await;
        assert!(results.is_ok());
        let outputs = results.unwrap();
        assert_eq!(outputs.len(), 2);
        assert_eq!(outputs[0], "Analysis complete 15");
        assert_eq!(outputs[1], "{\"report\": \"JSON data 15\"}");
    }

    #[tokio::test]
    async fn test_crewai_flow_extended_16() {
        let client = Arc::new(MockLlmClientCrew {
            responses: Mutex::new(vec![
                "Analysis complete 16".to_string(),
                "{\"report\": \"JSON data 16\"}".to_string(),
            ]),
        });

        let agent = Arc::new(Agent::new(client, vec![]));
        let cfg = AgentRunConfig::default();

        let role1 = Role::new("Researcher 16", "Find data 16", "You are an expert researcher.");
        let task1 = Task::new("Research topic X 16", "Plain text summary", role1);

        let role2 = Role::new("Writer 16", "Write report 16", "You write JSON reports.");
        let task2 = Task::new("Write JSON report 16", "JSON", role2);

        let crew = Crew::new(vec![task1, task2], agent, cfg);
        let flow = Flow::new(crew);

        let results = flow.execute().await;
        assert!(results.is_ok());
        let outputs = results.unwrap();
        assert_eq!(outputs.len(), 2);
        assert_eq!(outputs[0], "Analysis complete 16");
        assert_eq!(outputs[1], "{\"report\": \"JSON data 16\"}");
    }

    #[tokio::test]
    async fn test_crewai_flow_extended_17() {
        let client = Arc::new(MockLlmClientCrew {
            responses: Mutex::new(vec![
                "Analysis complete 17".to_string(),
                "{\"report\": \"JSON data 17\"}".to_string(),
            ]),
        });

        let agent = Arc::new(Agent::new(client, vec![]));
        let cfg = AgentRunConfig::default();

        let role1 = Role::new("Researcher 17", "Find data 17", "You are an expert researcher.");
        let task1 = Task::new("Research topic X 17", "Plain text summary", role1);

        let role2 = Role::new("Writer 17", "Write report 17", "You write JSON reports.");
        let task2 = Task::new("Write JSON report 17", "JSON", role2);

        let crew = Crew::new(vec![task1, task2], agent, cfg);
        let flow = Flow::new(crew);

        let results = flow.execute().await;
        assert!(results.is_ok());
        let outputs = results.unwrap();
        assert_eq!(outputs.len(), 2);
        assert_eq!(outputs[0], "Analysis complete 17");
        assert_eq!(outputs[1], "{\"report\": \"JSON data 17\"}");
    }

    #[tokio::test]
    async fn test_crewai_flow_extended_18() {
        let client = Arc::new(MockLlmClientCrew {
            responses: Mutex::new(vec![
                "Analysis complete 18".to_string(),
                "{\"report\": \"JSON data 18\"}".to_string(),
            ]),
        });

        let agent = Arc::new(Agent::new(client, vec![]));
        let cfg = AgentRunConfig::default();

        let role1 = Role::new("Researcher 18", "Find data 18", "You are an expert researcher.");
        let task1 = Task::new("Research topic X 18", "Plain text summary", role1);

        let role2 = Role::new("Writer 18", "Write report 18", "You write JSON reports.");
        let task2 = Task::new("Write JSON report 18", "JSON", role2);

        let crew = Crew::new(vec![task1, task2], agent, cfg);
        let flow = Flow::new(crew);

        let results = flow.execute().await;
        assert!(results.is_ok());
        let outputs = results.unwrap();
        assert_eq!(outputs.len(), 2);
        assert_eq!(outputs[0], "Analysis complete 18");
        assert_eq!(outputs[1], "{\"report\": \"JSON data 18\"}");
    }

    #[tokio::test]
    async fn test_crewai_flow_extended_19() {
        let client = Arc::new(MockLlmClientCrew {
            responses: Mutex::new(vec![
                "Analysis complete 19".to_string(),
                "{\"report\": \"JSON data 19\"}".to_string(),
            ]),
        });

        let agent = Arc::new(Agent::new(client, vec![]));
        let cfg = AgentRunConfig::default();

        let role1 = Role::new("Researcher 19", "Find data 19", "You are an expert researcher.");
        let task1 = Task::new("Research topic X 19", "Plain text summary", role1);

        let role2 = Role::new("Writer 19", "Write report 19", "You write JSON reports.");
        let task2 = Task::new("Write JSON report 19", "JSON", role2);

        let crew = Crew::new(vec![task1, task2], agent, cfg);
        let flow = Flow::new(crew);

        let results = flow.execute().await;
        assert!(results.is_ok());
        let outputs = results.unwrap();
        assert_eq!(outputs.len(), 2);
        assert_eq!(outputs[0], "Analysis complete 19");
        assert_eq!(outputs[1], "{\"report\": \"JSON data 19\"}");
    }

    #[tokio::test]
    async fn test_crewai_flow_extended_20() {
        let client = Arc::new(MockLlmClientCrew {
            responses: Mutex::new(vec![
                "Analysis complete 20".to_string(),
                "{\"report\": \"JSON data 20\"}".to_string(),
            ]),
        });

        let agent = Arc::new(Agent::new(client, vec![]));
        let cfg = AgentRunConfig::default();

        let role1 = Role::new("Researcher 20", "Find data 20", "You are an expert researcher.");
        let task1 = Task::new("Research topic X 20", "Plain text summary", role1);

        let role2 = Role::new("Writer 20", "Write report 20", "You write JSON reports.");
        let task2 = Task::new("Write JSON report 20", "JSON", role2);

        let crew = Crew::new(vec![task1, task2], agent, cfg);
        let flow = Flow::new(crew);

        let results = flow.execute().await;
        assert!(results.is_ok());
        let outputs = results.unwrap();
        assert_eq!(outputs.len(), 2);
        assert_eq!(outputs[0], "Analysis complete 20");
        assert_eq!(outputs[1], "{\"report\": \"JSON data 20\"}");
    }

    #[tokio::test]
    async fn test_crewai_flow_extended_21() {
        let client = Arc::new(MockLlmClientCrew {
            responses: Mutex::new(vec![
                "Analysis complete 21".to_string(),
                "{\"report\": \"JSON data 21\"}".to_string(),
            ]),
        });

        let agent = Arc::new(Agent::new(client, vec![]));
        let cfg = AgentRunConfig::default();

        let role1 = Role::new("Researcher 21", "Find data 21", "You are an expert researcher.");
        let task1 = Task::new("Research topic X 21", "Plain text summary", role1);

        let role2 = Role::new("Writer 21", "Write report 21", "You write JSON reports.");
        let task2 = Task::new("Write JSON report 21", "JSON", role2);

        let crew = Crew::new(vec![task1, task2], agent, cfg);
        let flow = Flow::new(crew);

        let results = flow.execute().await;
        assert!(results.is_ok());
        let outputs = results.unwrap();
        assert_eq!(outputs.len(), 2);
        assert_eq!(outputs[0], "Analysis complete 21");
        assert_eq!(outputs[1], "{\"report\": \"JSON data 21\"}");
    }

    #[tokio::test]
    async fn test_crewai_flow_extended_22() {
        let client = Arc::new(MockLlmClientCrew {
            responses: Mutex::new(vec![
                "Analysis complete 22".to_string(),
                "{\"report\": \"JSON data 22\"}".to_string(),
            ]),
        });

        let agent = Arc::new(Agent::new(client, vec![]));
        let cfg = AgentRunConfig::default();

        let role1 = Role::new("Researcher 22", "Find data 22", "You are an expert researcher.");
        let task1 = Task::new("Research topic X 22", "Plain text summary", role1);

        let role2 = Role::new("Writer 22", "Write report 22", "You write JSON reports.");
        let task2 = Task::new("Write JSON report 22", "JSON", role2);

        let crew = Crew::new(vec![task1, task2], agent, cfg);
        let flow = Flow::new(crew);

        let results = flow.execute().await;
        assert!(results.is_ok());
        let outputs = results.unwrap();
        assert_eq!(outputs.len(), 2);
        assert_eq!(outputs[0], "Analysis complete 22");
        assert_eq!(outputs[1], "{\"report\": \"JSON data 22\"}");
    }

    #[tokio::test]
    async fn test_crewai_flow_extended_23() {
        let client = Arc::new(MockLlmClientCrew {
            responses: Mutex::new(vec![
                "Analysis complete 23".to_string(),
                "{\"report\": \"JSON data 23\"}".to_string(),
            ]),
        });

        let agent = Arc::new(Agent::new(client, vec![]));
        let cfg = AgentRunConfig::default();

        let role1 = Role::new("Researcher 23", "Find data 23", "You are an expert researcher.");
        let task1 = Task::new("Research topic X 23", "Plain text summary", role1);

        let role2 = Role::new("Writer 23", "Write report 23", "You write JSON reports.");
        let task2 = Task::new("Write JSON report 23", "JSON", role2);

        let crew = Crew::new(vec![task1, task2], agent, cfg);
        let flow = Flow::new(crew);

        let results = flow.execute().await;
        assert!(results.is_ok());
        let outputs = results.unwrap();
        assert_eq!(outputs.len(), 2);
        assert_eq!(outputs[0], "Analysis complete 23");
        assert_eq!(outputs[1], "{\"report\": \"JSON data 23\"}");
    }

    #[tokio::test]
    async fn test_crewai_flow_extended_24() {
        let client = Arc::new(MockLlmClientCrew {
            responses: Mutex::new(vec![
                "Analysis complete 24".to_string(),
                "{\"report\": \"JSON data 24\"}".to_string(),
            ]),
        });

        let agent = Arc::new(Agent::new(client, vec![]));
        let cfg = AgentRunConfig::default();

        let role1 = Role::new("Researcher 24", "Find data 24", "You are an expert researcher.");
        let task1 = Task::new("Research topic X 24", "Plain text summary", role1);

        let role2 = Role::new("Writer 24", "Write report 24", "You write JSON reports.");
        let task2 = Task::new("Write JSON report 24", "JSON", role2);

        let crew = Crew::new(vec![task1, task2], agent, cfg);
        let flow = Flow::new(crew);

        let results = flow.execute().await;
        assert!(results.is_ok());
        let outputs = results.unwrap();
        assert_eq!(outputs.len(), 2);
        assert_eq!(outputs[0], "Analysis complete 24");
        assert_eq!(outputs[1], "{\"report\": \"JSON data 24\"}");
    }

    #[tokio::test]
    async fn test_crewai_flow_extended_25() {
        let client = Arc::new(MockLlmClientCrew {
            responses: Mutex::new(vec![
                "Analysis complete 25".to_string(),
                "{\"report\": \"JSON data 25\"}".to_string(),
            ]),
        });

        let agent = Arc::new(Agent::new(client, vec![]));
        let cfg = AgentRunConfig::default();

        let role1 = Role::new("Researcher 25", "Find data 25", "You are an expert researcher.");
        let task1 = Task::new("Research topic X 25", "Plain text summary", role1);

        let role2 = Role::new("Writer 25", "Write report 25", "You write JSON reports.");
        let task2 = Task::new("Write JSON report 25", "JSON", role2);

        let crew = Crew::new(vec![task1, task2], agent, cfg);
        let flow = Flow::new(crew);

        let results = flow.execute().await;
        assert!(results.is_ok());
        let outputs = results.unwrap();
        assert_eq!(outputs.len(), 2);
        assert_eq!(outputs[0], "Analysis complete 25");
        assert_eq!(outputs[1], "{\"report\": \"JSON data 25\"}");
    }

    #[tokio::test]
    async fn test_crewai_flow_extended_26() {
        let client = Arc::new(MockLlmClientCrew {
            responses: Mutex::new(vec![
                "Analysis complete 26".to_string(),
                "{\"report\": \"JSON data 26\"}".to_string(),
            ]),
        });

        let agent = Arc::new(Agent::new(client, vec![]));
        let cfg = AgentRunConfig::default();

        let role1 = Role::new("Researcher 26", "Find data 26", "You are an expert researcher.");
        let task1 = Task::new("Research topic X 26", "Plain text summary", role1);

        let role2 = Role::new("Writer 26", "Write report 26", "You write JSON reports.");
        let task2 = Task::new("Write JSON report 26", "JSON", role2);

        let crew = Crew::new(vec![task1, task2], agent, cfg);
        let flow = Flow::new(crew);

        let results = flow.execute().await;
        assert!(results.is_ok());
        let outputs = results.unwrap();
        assert_eq!(outputs.len(), 2);
        assert_eq!(outputs[0], "Analysis complete 26");
        assert_eq!(outputs[1], "{\"report\": \"JSON data 26\"}");
    }

    #[tokio::test]
    async fn test_crewai_flow_extended_27() {
        let client = Arc::new(MockLlmClientCrew {
            responses: Mutex::new(vec![
                "Analysis complete 27".to_string(),
                "{\"report\": \"JSON data 27\"}".to_string(),
            ]),
        });

        let agent = Arc::new(Agent::new(client, vec![]));
        let cfg = AgentRunConfig::default();

        let role1 = Role::new("Researcher 27", "Find data 27", "You are an expert researcher.");
        let task1 = Task::new("Research topic X 27", "Plain text summary", role1);

        let role2 = Role::new("Writer 27", "Write report 27", "You write JSON reports.");
        let task2 = Task::new("Write JSON report 27", "JSON", role2);

        let crew = Crew::new(vec![task1, task2], agent, cfg);
        let flow = Flow::new(crew);

        let results = flow.execute().await;
        assert!(results.is_ok());
        let outputs = results.unwrap();
        assert_eq!(outputs.len(), 2);
        assert_eq!(outputs[0], "Analysis complete 27");
        assert_eq!(outputs[1], "{\"report\": \"JSON data 27\"}");
    }

    #[tokio::test]
    async fn test_crewai_flow_extended_28() {
        let client = Arc::new(MockLlmClientCrew {
            responses: Mutex::new(vec![
                "Analysis complete 28".to_string(),
                "{\"report\": \"JSON data 28\"}".to_string(),
            ]),
        });

        let agent = Arc::new(Agent::new(client, vec![]));
        let cfg = AgentRunConfig::default();

        let role1 = Role::new("Researcher 28", "Find data 28", "You are an expert researcher.");
        let task1 = Task::new("Research topic X 28", "Plain text summary", role1);

        let role2 = Role::new("Writer 28", "Write report 28", "You write JSON reports.");
        let task2 = Task::new("Write JSON report 28", "JSON", role2);

        let crew = Crew::new(vec![task1, task2], agent, cfg);
        let flow = Flow::new(crew);

        let results = flow.execute().await;
        assert!(results.is_ok());
        let outputs = results.unwrap();
        assert_eq!(outputs.len(), 2);
        assert_eq!(outputs[0], "Analysis complete 28");
        assert_eq!(outputs[1], "{\"report\": \"JSON data 28\"}");
    }

    #[tokio::test]
    async fn test_crewai_flow_extended_29() {
        let client = Arc::new(MockLlmClientCrew {
            responses: Mutex::new(vec![
                "Analysis complete 29".to_string(),
                "{\"report\": \"JSON data 29\"}".to_string(),
            ]),
        });

        let agent = Arc::new(Agent::new(client, vec![]));
        let cfg = AgentRunConfig::default();

        let role1 = Role::new("Researcher 29", "Find data 29", "You are an expert researcher.");
        let task1 = Task::new("Research topic X 29", "Plain text summary", role1);

        let role2 = Role::new("Writer 29", "Write report 29", "You write JSON reports.");
        let task2 = Task::new("Write JSON report 29", "JSON", role2);

        let crew = Crew::new(vec![task1, task2], agent, cfg);
        let flow = Flow::new(crew);

        let results = flow.execute().await;
        assert!(results.is_ok());
        let outputs = results.unwrap();
        assert_eq!(outputs.len(), 2);
        assert_eq!(outputs[0], "Analysis complete 29");
        assert_eq!(outputs[1], "{\"report\": \"JSON data 29\"}");
    }

    #[tokio::test]
    async fn test_crewai_flow_extended_30() {
        let client = Arc::new(MockLlmClientCrew {
            responses: Mutex::new(vec![
                "Analysis complete 30".to_string(),
                "{\"report\": \"JSON data 30\"}".to_string(),
            ]),
        });

        let agent = Arc::new(Agent::new(client, vec![]));
        let cfg = AgentRunConfig::default();

        let role1 = Role::new("Researcher 30", "Find data 30", "You are an expert researcher.");
        let task1 = Task::new("Research topic X 30", "Plain text summary", role1);

        let role2 = Role::new("Writer 30", "Write report 30", "You write JSON reports.");
        let task2 = Task::new("Write JSON report 30", "JSON", role2);

        let crew = Crew::new(vec![task1, task2], agent, cfg);
        let flow = Flow::new(crew);

        let results = flow.execute().await;
        assert!(results.is_ok());
        let outputs = results.unwrap();
        assert_eq!(outputs.len(), 2);
        assert_eq!(outputs[0], "Analysis complete 30");
        assert_eq!(outputs[1], "{\"report\": \"JSON data 30\"}");
    }

    #[tokio::test]
    async fn test_crewai_flow_extended_31() {
        let client = Arc::new(MockLlmClientCrew {
            responses: Mutex::new(vec![
                "Analysis complete 31".to_string(),
                "{\"report\": \"JSON data 31\"}".to_string(),
            ]),
        });

        let agent = Arc::new(Agent::new(client, vec![]));
        let cfg = AgentRunConfig::default();

        let role1 = Role::new("Researcher 31", "Find data 31", "You are an expert researcher.");
        let task1 = Task::new("Research topic X 31", "Plain text summary", role1);

        let role2 = Role::new("Writer 31", "Write report 31", "You write JSON reports.");
        let task2 = Task::new("Write JSON report 31", "JSON", role2);

        let crew = Crew::new(vec![task1, task2], agent, cfg);
        let flow = Flow::new(crew);

        let results = flow.execute().await;
        assert!(results.is_ok());
        let outputs = results.unwrap();
        assert_eq!(outputs.len(), 2);
        assert_eq!(outputs[0], "Analysis complete 31");
        assert_eq!(outputs[1], "{\"report\": \"JSON data 31\"}");
    }

    #[tokio::test]
    async fn test_crewai_flow_extended_32() {
        let client = Arc::new(MockLlmClientCrew {
            responses: Mutex::new(vec![
                "Analysis complete 32".to_string(),
                "{\"report\": \"JSON data 32\"}".to_string(),
            ]),
        });

        let agent = Arc::new(Agent::new(client, vec![]));
        let cfg = AgentRunConfig::default();

        let role1 = Role::new("Researcher 32", "Find data 32", "You are an expert researcher.");
        let task1 = Task::new("Research topic X 32", "Plain text summary", role1);

        let role2 = Role::new("Writer 32", "Write report 32", "You write JSON reports.");
        let task2 = Task::new("Write JSON report 32", "JSON", role2);

        let crew = Crew::new(vec![task1, task2], agent, cfg);
        let flow = Flow::new(crew);

        let results = flow.execute().await;
        assert!(results.is_ok());
        let outputs = results.unwrap();
        assert_eq!(outputs.len(), 2);
        assert_eq!(outputs[0], "Analysis complete 32");
        assert_eq!(outputs[1], "{\"report\": \"JSON data 32\"}");
    }

    #[tokio::test]
    async fn test_crewai_flow_extended_33() {
        let client = Arc::new(MockLlmClientCrew {
            responses: Mutex::new(vec![
                "Analysis complete 33".to_string(),
                "{\"report\": \"JSON data 33\"}".to_string(),
            ]),
        });

        let agent = Arc::new(Agent::new(client, vec![]));
        let cfg = AgentRunConfig::default();

        let role1 = Role::new("Researcher 33", "Find data 33", "You are an expert researcher.");
        let task1 = Task::new("Research topic X 33", "Plain text summary", role1);

        let role2 = Role::new("Writer 33", "Write report 33", "You write JSON reports.");
        let task2 = Task::new("Write JSON report 33", "JSON", role2);

        let crew = Crew::new(vec![task1, task2], agent, cfg);
        let flow = Flow::new(crew);

        let results = flow.execute().await;
        assert!(results.is_ok());
        let outputs = results.unwrap();
        assert_eq!(outputs.len(), 2);
        assert_eq!(outputs[0], "Analysis complete 33");
        assert_eq!(outputs[1], "{\"report\": \"JSON data 33\"}");
    }

    #[tokio::test]
    async fn test_crewai_flow_extended_34() {
        let client = Arc::new(MockLlmClientCrew {
            responses: Mutex::new(vec![
                "Analysis complete 34".to_string(),
                "{\"report\": \"JSON data 34\"}".to_string(),
            ]),
        });

        let agent = Arc::new(Agent::new(client, vec![]));
        let cfg = AgentRunConfig::default();

        let role1 = Role::new("Researcher 34", "Find data 34", "You are an expert researcher.");
        let task1 = Task::new("Research topic X 34", "Plain text summary", role1);

        let role2 = Role::new("Writer 34", "Write report 34", "You write JSON reports.");
        let task2 = Task::new("Write JSON report 34", "JSON", role2);

        let crew = Crew::new(vec![task1, task2], agent, cfg);
        let flow = Flow::new(crew);

        let results = flow.execute().await;
        assert!(results.is_ok());
        let outputs = results.unwrap();
        assert_eq!(outputs.len(), 2);
        assert_eq!(outputs[0], "Analysis complete 34");
        assert_eq!(outputs[1], "{\"report\": \"JSON data 34\"}");
    }

    #[tokio::test]
    async fn test_crewai_flow_extended_35() {
        let client = Arc::new(MockLlmClientCrew {
            responses: Mutex::new(vec![
                "Analysis complete 35".to_string(),
                "{\"report\": \"JSON data 35\"}".to_string(),
            ]),
        });

        let agent = Arc::new(Agent::new(client, vec![]));
        let cfg = AgentRunConfig::default();

        let role1 = Role::new("Researcher 35", "Find data 35", "You are an expert researcher.");
        let task1 = Task::new("Research topic X 35", "Plain text summary", role1);

        let role2 = Role::new("Writer 35", "Write report 35", "You write JSON reports.");
        let task2 = Task::new("Write JSON report 35", "JSON", role2);

        let crew = Crew::new(vec![task1, task2], agent, cfg);
        let flow = Flow::new(crew);

        let results = flow.execute().await;
        assert!(results.is_ok());
        let outputs = results.unwrap();
        assert_eq!(outputs.len(), 2);
        assert_eq!(outputs[0], "Analysis complete 35");
        assert_eq!(outputs[1], "{\"report\": \"JSON data 35\"}");
    }

    #[tokio::test]
    async fn test_crewai_flow_extended_36() {
        let client = Arc::new(MockLlmClientCrew {
            responses: Mutex::new(vec![
                "Analysis complete 36".to_string(),
                "{\"report\": \"JSON data 36\"}".to_string(),
            ]),
        });

        let agent = Arc::new(Agent::new(client, vec![]));
        let cfg = AgentRunConfig::default();

        let role1 = Role::new("Researcher 36", "Find data 36", "You are an expert researcher.");
        let task1 = Task::new("Research topic X 36", "Plain text summary", role1);

        let role2 = Role::new("Writer 36", "Write report 36", "You write JSON reports.");
        let task2 = Task::new("Write JSON report 36", "JSON", role2);

        let crew = Crew::new(vec![task1, task2], agent, cfg);
        let flow = Flow::new(crew);

        let results = flow.execute().await;
        assert!(results.is_ok());
        let outputs = results.unwrap();
        assert_eq!(outputs.len(), 2);
        assert_eq!(outputs[0], "Analysis complete 36");
        assert_eq!(outputs[1], "{\"report\": \"JSON data 36\"}");
    }

    #[tokio::test]
    async fn test_crewai_flow_extended_37() {
        let client = Arc::new(MockLlmClientCrew {
            responses: Mutex::new(vec![
                "Analysis complete 37".to_string(),
                "{\"report\": \"JSON data 37\"}".to_string(),
            ]),
        });

        let agent = Arc::new(Agent::new(client, vec![]));
        let cfg = AgentRunConfig::default();

        let role1 = Role::new("Researcher 37", "Find data 37", "You are an expert researcher.");
        let task1 = Task::new("Research topic X 37", "Plain text summary", role1);

        let role2 = Role::new("Writer 37", "Write report 37", "You write JSON reports.");
        let task2 = Task::new("Write JSON report 37", "JSON", role2);

        let crew = Crew::new(vec![task1, task2], agent, cfg);
        let flow = Flow::new(crew);

        let results = flow.execute().await;
        assert!(results.is_ok());
        let outputs = results.unwrap();
        assert_eq!(outputs.len(), 2);
        assert_eq!(outputs[0], "Analysis complete 37");
        assert_eq!(outputs[1], "{\"report\": \"JSON data 37\"}");
    }

    #[tokio::test]
    async fn test_crewai_flow_extended_38() {
        let client = Arc::new(MockLlmClientCrew {
            responses: Mutex::new(vec![
                "Analysis complete 38".to_string(),
                "{\"report\": \"JSON data 38\"}".to_string(),
            ]),
        });

        let agent = Arc::new(Agent::new(client, vec![]));
        let cfg = AgentRunConfig::default();

        let role1 = Role::new("Researcher 38", "Find data 38", "You are an expert researcher.");
        let task1 = Task::new("Research topic X 38", "Plain text summary", role1);

        let role2 = Role::new("Writer 38", "Write report 38", "You write JSON reports.");
        let task2 = Task::new("Write JSON report 38", "JSON", role2);

        let crew = Crew::new(vec![task1, task2], agent, cfg);
        let flow = Flow::new(crew);

        let results = flow.execute().await;
        assert!(results.is_ok());
        let outputs = results.unwrap();
        assert_eq!(outputs.len(), 2);
        assert_eq!(outputs[0], "Analysis complete 38");
        assert_eq!(outputs[1], "{\"report\": \"JSON data 38\"}");
    }

    #[tokio::test]
    async fn test_crewai_flow_extended_39() {
        let client = Arc::new(MockLlmClientCrew {
            responses: Mutex::new(vec![
                "Analysis complete 39".to_string(),
                "{\"report\": \"JSON data 39\"}".to_string(),
            ]),
        });

        let agent = Arc::new(Agent::new(client, vec![]));
        let cfg = AgentRunConfig::default();

        let role1 = Role::new("Researcher 39", "Find data 39", "You are an expert researcher.");
        let task1 = Task::new("Research topic X 39", "Plain text summary", role1);

        let role2 = Role::new("Writer 39", "Write report 39", "You write JSON reports.");
        let task2 = Task::new("Write JSON report 39", "JSON", role2);

        let crew = Crew::new(vec![task1, task2], agent, cfg);
        let flow = Flow::new(crew);

        let results = flow.execute().await;
        assert!(results.is_ok());
        let outputs = results.unwrap();
        assert_eq!(outputs.len(), 2);
        assert_eq!(outputs[0], "Analysis complete 39");
        assert_eq!(outputs[1], "{\"report\": \"JSON data 39\"}");
    }
}
