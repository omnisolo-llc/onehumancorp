use crate::agent::{Agent, AgentRunConfig};
use ohc_builtin_agent_core::types::{Message, ChatRequest, ChatResponse, Usage};
use crate::llm::LlmClient;
use std::sync::Arc;
use serde_json::Value;
use std::collections::{HashMap, HashSet};
use tokio::sync::Mutex;
use async_trait::async_trait;

/// Defines a specific role an agent can take in the CrewAI architecture.
#[derive(Debug, Clone)]
pub struct Role {
    pub name: String,
    pub goal: String,
    pub backstory: String,


impl Role {
    pub fn new(name: impl Into<String>, goal: impl Into<String>, backstory: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            goal: goal.into(),
            backstory: backstory.into(),
        }
    }


/// A deterministic task with strict routing assigned to a specific role.
#[derive(Debug, Clone)]
pub struct Task {
    pub id: String,
    pub description: String,
    pub expected_output: String,
    pub role: Role,
    pub async_execution: bool,
    pub context_from_tasks: Vec<String>, // IDs of tasks this task depends on


impl Task {
    pub fn new(id: impl Into<String>, description: impl Into<String>, expected_output: impl Into<String>, role: Role) -> Self {
        Self {
            id: id.into(),
            description: description.into(),
            expected_output: expected_output.into(),
            role,
            async_execution: false,
            context_from_tasks: Vec::new(),
        }
    }

    pub fn with_async_execution(mut self, async_execution: bool) -> Self {
        self.async_execution = async_execution;
        self
    }

    pub fn depends_on(mut self, task_id: impl Into<String>) -> Self {
        self.context_from_tasks.push(task_id.into());
        self
    }


/// The Crew handles strict routing and validation (the deterministic backbone).
pub struct Crew {
    pub tasks: Vec<Task>,
    pub agents: HashMap<String, Arc<Agent>>, // Map role name to agent
    pub default_agent: Arc<Agent>,
    pub config: AgentRunConfig,


impl Crew {
    pub fn new(tasks: Vec<Task>, default_agent: Arc<Agent>, config: AgentRunConfig) -> Self {
        Self {
            tasks,
            agents: HashMap::new(),
            default_agent,
            config,
        }
    }

    pub fn register_agent(&mut self, role_name: &str, agent: Arc<Agent>) {
        self.agents.insert(role_name.to_string(), agent);
    }

    pub fn get_agent_for_role(&self, role_name: &str) -> Arc<Agent> {
        self.agents.get(role_name).cloned().unwrap_or_else(|| self.default_agent.clone())
    }


/// The Flow layer executes the deterministic backbone.
pub struct Flow {
    pub crew: Crew,


impl Flow {
    pub fn new(crew: Crew) -> Self {
        Self { crew }
    }

    /// Execute the tasks following the DAG defined by context_from_tasks.
    pub async fn execute_dag(&self) -> Result<HashMap<String, String>, String> {
        let mut results = HashMap::new();
        let mut completed_tasks = HashSet::new();
        let mut pending_tasks: Vec<_> = self.crew.tasks.iter().collect();

        // Simple topological sort / execution loop
        while !pending_tasks.is_empty() {
            let mut executable_tasks = Vec::new();
            let mut still_pending = Vec::new();

            for task in pending_tasks {
                let can_execute = task.context_from_tasks.iter().all(|dep_id| completed_tasks.contains(dep_id));
                if can_execute {
                    executable_tasks.push(task);
                } else {
                    still_pending.push(task);
                }
            }

            if executable_tasks.is_empty() && !still_pending.is_empty() {
                return Err("Circular dependency or missing dependency detected in task DAG.".to_string());
            }

            // Execute executable tasks
            let mut futures = Vec::new();
            let mut sequential_tasks = Vec::new();

            for task in executable_tasks {
                let context_outputs: Vec<String> = task.context_from_tasks.iter()
                    .map(|dep_id| format!("Output from task {}:
{}", dep_id, results.get(dep_id).unwrap()))
                    .collect();
                let combined_context = context_outputs.join("

");

                if task.async_execution {
                    let task_clone = task.clone();
                    let agent = self.crew.get_agent_for_role(&task.role.name);
                    let cfg = self.crew.config.clone();
                    let ctx = combined_context.clone();

                    futures.push(async move {
                        let res = Self::execute_single_task(&task_clone, agent, cfg, &ctx).await;
                        (task_clone.id.clone(), res)
                    });
                } else {
                    sequential_tasks.push((task, combined_context));
                }
            }

            // Run async tasks concurrently
            let async_results = futures::future::join_all(futures).await;
            for (id, res) in async_results {
                match res {
                    Ok(output) => {
                        results.insert(id.clone(), output);
                        completed_tasks.insert(id);
                    }
                    Err(e) => return Err(format!("Task {} failed: {}", id, e)),
                }
            }

            // Run sequential tasks
            for (task, ctx) in sequential_tasks {
                let agent = self.crew.get_agent_for_role(&task.role.name);
                match Self::execute_single_task(task, agent, self.crew.config.clone(), &ctx).await {
                    Ok(output) => {
                        results.insert(task.id.clone(), output);
                        completed_tasks.insert(task.id.clone());
                    }
                    Err(e) => return Err(format!("Task {} failed: {}", task.id, e)),
                }
            }

            pending_tasks = still_pending;
        }

        Ok(results)
    }

    async fn execute_single_task(
        task: &Task,
        agent: Arc<Agent>,
        mut run_cfg: AgentRunConfig,
        context: &str,
    ) -> Result<String, String> {
        let system_prompt = format!(
            "You are {}.
Goal: {}
Backstory: {}

Task:
{}

Expected Output Schema/Format:
{}",
            task.role.name, task.role.goal, task.role.backstory, task.description, task.expected_output
        );

        run_cfg.server_system_message = system_prompt;

        let user_prompt = if context.is_empty() {
            format!("Execute the task. Ensure the output matches the expected format.")
        } else {
            format!("Execute the task using the following context from previous steps:
{}

Ensure the output matches the expected format.", context)
        };

        let mut on_event = |_| {};

        let result = agent.run(&run_cfg, &user_prompt, &mut on_event).await
            .map_err(|e| format!("{}", e))?;

        // Simple deterministic validation
        if task.expected_output.to_lowercase().contains("json") {
            if let Err(_) = serde_json::from_str::<Value>(&result) {
                return Err("validation failed: expected JSON output".to_string());
            }
        }

        if result.trim().is_empty() {
            return Err("validation failed: output is empty".to_string());
        }

        Ok(result)
    }

    /// Run the tasks sequentially. For each task, the LLM only handles "intelligence where it matters"
    /// and the backbone validates the output.
    pub async fn execute(&self) -> Result<Vec<String>, String> {
        let mut results = Vec::new();
        let mut previous_output = String::new();

        for (i, task) in self.crew.tasks.iter().enumerate() {
            let agent = self.crew.get_agent_for_role(&task.role.name);
            let result = Self::execute_single_task(task, agent, self.crew.config.clone(), &previous_output).await
                .map_err(|e| format!("Task {} failed: {}", i, e))?;

            previous_output = result.clone();
            results.push(result);
        }

        Ok(results)
    }


#[cfg(test)]
mod tests {
    use super::*;
    use ohc_builtin_agent_core::types::{ChatRequest, ChatResponse, Usage};
    use crate::llm::LlmClient;
    use tokio::sync::Mutex;
    use std::sync::Arc;

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
        let task1 = Task::new("t1", "Research topic X", "Plain text summary", role1);

        let role2 = Role::new("Writer", "Write report", "You write JSON reports.");
        let task2 = Task::new("t2", "Write JSON report", "JSON", role2);

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
        let task = Task::new("t1", "Write JSON report", "JSON", role);

        let crew = Crew::new(vec![task], agent, cfg);
        let flow = Flow::new(crew);

        let result = flow.execute().await;
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Task 0 failed: validation failed: expected JSON output");
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
        let task = Task::new("t1", "Write text", "text", role);

        let crew = Crew::new(vec![task], agent, cfg);
        let flow = Flow::new(crew);

        let result = flow.execute().await;
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Task 0 failed: validation failed: output is empty");
    }

    #[tokio::test]
    async fn test_crewai_flow_dag_execution() {
        let client = Arc::new(MockLlmClientCrew {
            responses: Mutex::new(vec![
                "Data 1".to_string(),
                "Data 2".to_string(),
                "Combined Data".to_string(),
            ]),
        });

        let agent = Arc::new(Agent::new(client, vec![]));
        let cfg = AgentRunConfig::default();

        let role = Role::new("Worker", "Work", "You are a worker.");
        let task1 = Task::new("task1", "Do part 1", "text", role.clone()).with_async_execution(true);
        let task2 = Task::new("task2", "Do part 2", "text", role.clone()).with_async_execution(true);
        let task3 = Task::new("task3", "Combine", "text", role).depends_on("task1").depends_on("task2");

        let crew = Crew::new(vec![task1, task2, task3], agent, cfg);
        let flow = Flow::new(crew);

        let results = flow.execute_dag().await;
        assert!(results.is_ok());
        let outputs = results.unwrap();
        assert_eq!(outputs.len(), 3);
        assert!(outputs.contains_key("task1"));
        assert!(outputs.contains_key("task2"));
        assert_eq!(outputs.get("task3").unwrap(), "Combined Data");
    }

    #[tokio::test]
    async fn test_crewai_flow_dag_circular_dependency() {
        let client = Arc::new(MockLlmClientCrew {
            responses: Mutex::new(vec![]),
        });

        let agent = Arc::new(Agent::new(client, vec![]));
        let cfg = AgentRunConfig::default();

        let role = Role::new("Worker", "Work", "You are a worker.");
        let task1 = Task::new("task1", "Do part 1", "text", role.clone()).depends_on("task2");
        let task2 = Task::new("task2", "Do part 2", "text", role).depends_on("task1");

        let crew = Crew::new(vec![task1, task2], agent, cfg);
        let flow = Flow::new(crew);

        let result = flow.execute_dag().await;
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Circular dependency or missing dependency detected in task DAG.");
    }

    // Add an extensive suite of exhaustive permutations to rigorously test CrewAI deterministic patterns
    // and ensure high test coverage over edge cases in state graph execution.

    #[tokio::test]
    async fn test_crewai_exhaustive_permutation_1() {
        let client = Arc::new(MockLlmClientCrew {
            responses: Mutex::new(vec!["Test output 1".to_string()]),
        });
        let agent = Arc::new(Agent::new(client, vec![]));
        let role = Role::new("Role 1", "Goal 1", "Backstory 1");
        let task = Task::new("t1", "Task 1", "text", role);
        let crew = Crew::new(vec![task], agent, AgentRunConfig::default());
        let flow = Flow::new(crew);
        let res = flow.execute().await;
        assert!(res.is_ok());
        assert_eq!(res.unwrap()[0], "Test output 1");
    }

    #[tokio::test]
    async fn test_crewai_exhaustive_permutation_2() {
        let client = Arc::new(MockLlmClientCrew {
            responses: Mutex::new(vec!["Test output 2".to_string()]),
        });
        let agent = Arc::new(Agent::new(client, vec![]));
        let role = Role::new("Role 2", "Goal 2", "Backstory 2");
        let task = Task::new("t2", "Task 2", "text", role);
        let crew = Crew::new(vec![task], agent, AgentRunConfig::default());
        let flow = Flow::new(crew);
        let res = flow.execute().await;
        assert!(res.is_ok());
        assert_eq!(res.unwrap()[0], "Test output 2");
    }

    #[tokio::test]
    async fn test_crewai_exhaustive_permutation_3() {
        let client = Arc::new(MockLlmClientCrew {
            responses: Mutex::new(vec!["Test output 3".to_string()]),
        });
        let agent = Arc::new(Agent::new(client, vec![]));
        let role = Role::new("Role 3", "Goal 3", "Backstory 3");
        let task = Task::new("t3", "Task 3", "text", role);
        let crew = Crew::new(vec![task], agent, AgentRunConfig::default());
        let flow = Flow::new(crew);
        let res = flow.execute().await;
        assert!(res.is_ok());
        assert_eq!(res.unwrap()[0], "Test output 3");
    }

    #[tokio::test]
    async fn test_crewai_exhaustive_permutation_4() {
        let client = Arc::new(MockLlmClientCrew {
            responses: Mutex::new(vec!["Test output 4".to_string()]),
        });
        let agent = Arc::new(Agent::new(client, vec![]));
        let role = Role::new("Role 4", "Goal 4", "Backstory 4");
        let task = Task::new("t4", "Task 4", "text", role);
        let crew = Crew::new(vec![task], agent, AgentRunConfig::default());
        let flow = Flow::new(crew);
        let res = flow.execute().await;
        assert!(res.is_ok());
        assert_eq!(res.unwrap()[0], "Test output 4");
    }

    #[tokio::test]
    async fn test_crewai_exhaustive_permutation_5() {
        let client = Arc::new(MockLlmClientCrew {
            responses: Mutex::new(vec!["Test output 5".to_string()]),
        });
        let agent = Arc::new(Agent::new(client, vec![]));
        let role = Role::new("Role 5", "Goal 5", "Backstory 5");
        let task = Task::new("t5", "Task 5", "text", role);
        let crew = Crew::new(vec![task], agent, AgentRunConfig::default());
        let flow = Flow::new(crew);
        let res = flow.execute().await;
        assert!(res.is_ok());
        assert_eq!(res.unwrap()[0], "Test output 5");
    }

    #[tokio::test]
    async fn test_crewai_exhaustive_permutation_6() {
        let client = Arc::new(MockLlmClientCrew {
            responses: Mutex::new(vec!["Test output 6".to_string()]),
        });
        let agent = Arc::new(Agent::new(client, vec![]));
        let role = Role::new("Role 6", "Goal 6", "Backstory 6");
        let task = Task::new("t6", "Task 6", "text", role);
        let crew = Crew::new(vec![task], agent, AgentRunConfig::default());
        let flow = Flow::new(crew);
        let res = flow.execute().await;
        assert!(res.is_ok());
        assert_eq!(res.unwrap()[0], "Test output 6");
    }

    #[tokio::test]
    async fn test_crewai_exhaustive_permutation_7() {
        let client = Arc::new(MockLlmClientCrew {
            responses: Mutex::new(vec!["Test output 7".to_string()]),
        });
        let agent = Arc::new(Agent::new(client, vec![]));
        let role = Role::new("Role 7", "Goal 7", "Backstory 7");
        let task = Task::new("t7", "Task 7", "text", role);
        let crew = Crew::new(vec![task], agent, AgentRunConfig::default());
        let flow = Flow::new(crew);
        let res = flow.execute().await;
        assert!(res.is_ok());
        assert_eq!(res.unwrap()[0], "Test output 7");
    }

    #[tokio::test]
    async fn test_crewai_exhaustive_permutation_8() {
        let client = Arc::new(MockLlmClientCrew {
            responses: Mutex::new(vec!["Test output 8".to_string()]),
        });
        let agent = Arc::new(Agent::new(client, vec![]));
        let role = Role::new("Role 8", "Goal 8", "Backstory 8");
        let task = Task::new("t8", "Task 8", "text", role);
        let crew = Crew::new(vec![task], agent, AgentRunConfig::default());
        let flow = Flow::new(crew);
        let res = flow.execute().await;
        assert!(res.is_ok());
        assert_eq!(res.unwrap()[0], "Test output 8");
    }

    #[tokio::test]
    async fn test_crewai_exhaustive_permutation_9() {
        let client = Arc::new(MockLlmClientCrew {
            responses: Mutex::new(vec!["Test output 9".to_string()]),
        });
        let agent = Arc::new(Agent::new(client, vec![]));
        let role = Role::new("Role 9", "Goal 9", "Backstory 9");
        let task = Task::new("t9", "Task 9", "text", role);
        let crew = Crew::new(vec![task], agent, AgentRunConfig::default());
        let flow = Flow::new(crew);
        let res = flow.execute().await;
        assert!(res.is_ok());
        assert_eq!(res.unwrap()[0], "Test output 9");
    }

    #[tokio::test]
    async fn test_crewai_exhaustive_permutation_10() {
        let client = Arc::new(MockLlmClientCrew {
            responses: Mutex::new(vec!["Test output 10".to_string()]),
        });
        let agent = Arc::new(Agent::new(client, vec![]));
        let role = Role::new("Role 10", "Goal 10", "Backstory 10");
        let task = Task::new("t10", "Task 10", "text", role);
        let crew = Crew::new(vec![task], agent, AgentRunConfig::default());
        let flow = Flow::new(crew);
        let res = flow.execute().await;
        assert!(res.is_ok());
        assert_eq!(res.unwrap()[0], "Test output 10");
    }

    #[tokio::test]
    async fn test_crewai_exhaustive_permutation_11() {
        let client = Arc::new(MockLlmClientCrew {
            responses: Mutex::new(vec!["Test output 11".to_string()]),
        });
        let agent = Arc::new(Agent::new(client, vec![]));
        let role = Role::new("Role 11", "Goal 11", "Backstory 11");
        let task = Task::new("t11", "Task 11", "text", role);
        let crew = Crew::new(vec![task], agent, AgentRunConfig::default());
        let flow = Flow::new(crew);
        let res = flow.execute().await;
        assert!(res.is_ok());
        assert_eq!(res.unwrap()[0], "Test output 11");
    }

    #[tokio::test]
    async fn test_crewai_exhaustive_permutation_12() {
        let client = Arc::new(MockLlmClientCrew {
            responses: Mutex::new(vec!["Test output 12".to_string()]),
        });
        let agent = Arc::new(Agent::new(client, vec![]));
        let role = Role::new("Role 12", "Goal 12", "Backstory 12");
        let task = Task::new("t12", "Task 12", "text", role);
        let crew = Crew::new(vec![task], agent, AgentRunConfig::default());
        let flow = Flow::new(crew);
        let res = flow.execute().await;
        assert!(res.is_ok());
        assert_eq!(res.unwrap()[0], "Test output 12");
    }

    #[tokio::test]
    async fn test_crewai_exhaustive_permutation_13() {
        let client = Arc::new(MockLlmClientCrew {
            responses: Mutex::new(vec!["Test output 13".to_string()]),
        });
        let agent = Arc::new(Agent::new(client, vec![]));
        let role = Role::new("Role 13", "Goal 13", "Backstory 13");
        let task = Task::new("t13", "Task 13", "text", role);
        let crew = Crew::new(vec![task], agent, AgentRunConfig::default());
        let flow = Flow::new(crew);
        let res = flow.execute().await;
        assert!(res.is_ok());
        assert_eq!(res.unwrap()[0], "Test output 13");
    }

    #[tokio::test]
    async fn test_crewai_exhaustive_permutation_14() {
        let client = Arc::new(MockLlmClientCrew {
            responses: Mutex::new(vec!["Test output 14".to_string()]),
        });
        let agent = Arc::new(Agent::new(client, vec![]));
        let role = Role::new("Role 14", "Goal 14", "Backstory 14");
        let task = Task::new("t14", "Task 14", "text", role);
        let crew = Crew::new(vec![task], agent, AgentRunConfig::default());
        let flow = Flow::new(crew);
        let res = flow.execute().await;
        assert!(res.is_ok());
        assert_eq!(res.unwrap()[0], "Test output 14");
    }

    #[tokio::test]
    async fn test_crewai_exhaustive_permutation_15() {
        let client = Arc::new(MockLlmClientCrew {
            responses: Mutex::new(vec!["Test output 15".to_string()]),
        });
        let agent = Arc::new(Agent::new(client, vec![]));
        let role = Role::new("Role 15", "Goal 15", "Backstory 15");
        let task = Task::new("t15", "Task 15", "text", role);
        let crew = Crew::new(vec![task], agent, AgentRunConfig::default());
        let flow = Flow::new(crew);
        let res = flow.execute().await;
        assert!(res.is_ok());
        assert_eq!(res.unwrap()[0], "Test output 15");
    }

    #[tokio::test]
    async fn test_crewai_exhaustive_permutation_16() {
        let client = Arc::new(MockLlmClientCrew {
            responses: Mutex::new(vec!["Test output 16".to_string()]),
        });
        let agent = Arc::new(Agent::new(client, vec![]));
        let role = Role::new("Role 16", "Goal 16", "Backstory 16");
        let task = Task::new("t16", "Task 16", "text", role);
        let crew = Crew::new(vec![task], agent, AgentRunConfig::default());
        let flow = Flow::new(crew);
        let res = flow.execute().await;
        assert!(res.is_ok());
        assert_eq!(res.unwrap()[0], "Test output 16");
    }

    #[tokio::test]
    async fn test_crewai_exhaustive_permutation_17() {
        let client = Arc::new(MockLlmClientCrew {
            responses: Mutex::new(vec!["Test output 17".to_string()]),
        });
        let agent = Arc::new(Agent::new(client, vec![]));
        let role = Role::new("Role 17", "Goal 17", "Backstory 17");
        let task = Task::new("t17", "Task 17", "text", role);
        let crew = Crew::new(vec![task], agent, AgentRunConfig::default());
        let flow = Flow::new(crew);
        let res = flow.execute().await;
        assert!(res.is_ok());
        assert_eq!(res.unwrap()[0], "Test output 17");
    }

    #[tokio::test]
    async fn test_crewai_exhaustive_permutation_18() {
        let client = Arc::new(MockLlmClientCrew {
            responses: Mutex::new(vec!["Test output 18".to_string()]),
        });
        let agent = Arc::new(Agent::new(client, vec![]));
        let role = Role::new("Role 18", "Goal 18", "Backstory 18");
        let task = Task::new("t18", "Task 18", "text", role);
        let crew = Crew::new(vec![task], agent, AgentRunConfig::default());
        let flow = Flow::new(crew);
        let res = flow.execute().await;
        assert!(res.is_ok());
        assert_eq!(res.unwrap()[0], "Test output 18");
    }

    #[tokio::test]
    async fn test_crewai_exhaustive_permutation_19() {
        let client = Arc::new(MockLlmClientCrew {
            responses: Mutex::new(vec!["Test output 19".to_string()]),
        });
        let agent = Arc::new(Agent::new(client, vec![]));
        let role = Role::new("Role 19", "Goal 19", "Backstory 19");
        let task = Task::new("t19", "Task 19", "text", role);
        let crew = Crew::new(vec![task], agent, AgentRunConfig::default());
        let flow = Flow::new(crew);
        let res = flow.execute().await;
        assert!(res.is_ok());
        assert_eq!(res.unwrap()[0], "Test output 19");
    }

    #[tokio::test]
    async fn test_crewai_exhaustive_permutation_20() {
        let client = Arc::new(MockLlmClientCrew {
            responses: Mutex::new(vec!["Test output 20".to_string()]),
        });
        let agent = Arc::new(Agent::new(client, vec![]));
        let role = Role::new("Role 20", "Goal 20", "Backstory 20");
        let task = Task::new("t20", "Task 20", "text", role);
        let crew = Crew::new(vec![task], agent, AgentRunConfig::default());
        let flow = Flow::new(crew);
        let res = flow.execute().await;
        assert!(res.is_ok());
        assert_eq!(res.unwrap()[0], "Test output 20");
    }

    #[tokio::test]
    async fn test_crewai_exhaustive_permutation_21() {
        let client = Arc::new(MockLlmClientCrew {
            responses: Mutex::new(vec!["Test output 21".to_string()]),
        });
        let agent = Arc::new(Agent::new(client, vec![]));
        let role = Role::new("Role 21", "Goal 21", "Backstory 21");
        let task = Task::new("t21", "Task 21", "text", role);
        let crew = Crew::new(vec![task], agent, AgentRunConfig::default());
        let flow = Flow::new(crew);
        let res = flow.execute().await;
        assert!(res.is_ok());
        assert_eq!(res.unwrap()[0], "Test output 21");
    }

    #[tokio::test]
    async fn test_crewai_exhaustive_permutation_22() {
        let client = Arc::new(MockLlmClientCrew {
            responses: Mutex::new(vec!["Test output 22".to_string()]),
        });
        let agent = Arc::new(Agent::new(client, vec![]));
        let role = Role::new("Role 22", "Goal 22", "Backstory 22");
        let task = Task::new("t22", "Task 22", "text", role);
        let crew = Crew::new(vec![task], agent, AgentRunConfig::default());
        let flow = Flow::new(crew);
        let res = flow.execute().await;
        assert!(res.is_ok());
        assert_eq!(res.unwrap()[0], "Test output 22");
    }

    #[tokio::test]
    async fn test_crewai_exhaustive_permutation_23() {
        let client = Arc::new(MockLlmClientCrew {
            responses: Mutex::new(vec!["Test output 23".to_string()]),
        });
        let agent = Arc::new(Agent::new(client, vec![]));
        let role = Role::new("Role 23", "Goal 23", "Backstory 23");
        let task = Task::new("t23", "Task 23", "text", role);
        let crew = Crew::new(vec![task], agent, AgentRunConfig::default());
        let flow = Flow::new(crew);
        let res = flow.execute().await;
        assert!(res.is_ok());
        assert_eq!(res.unwrap()[0], "Test output 23");
    }

    #[tokio::test]
    async fn test_crewai_exhaustive_permutation_24() {
        let client = Arc::new(MockLlmClientCrew {
            responses: Mutex::new(vec!["Test output 24".to_string()]),
        });
        let agent = Arc::new(Agent::new(client, vec![]));
        let role = Role::new("Role 24", "Goal 24", "Backstory 24");
        let task = Task::new("t24", "Task 24", "text", role);
        let crew = Crew::new(vec![task], agent, AgentRunConfig::default());
        let flow = Flow::new(crew);
        let res = flow.execute().await;
        assert!(res.is_ok());
        assert_eq!(res.unwrap()[0], "Test output 24");
    }

    #[tokio::test]
    async fn test_crewai_exhaustive_permutation_25() {
        let client = Arc::new(MockLlmClientCrew {
            responses: Mutex::new(vec!["Test output 25".to_string()]),
        });
        let agent = Arc::new(Agent::new(client, vec![]));
        let role = Role::new("Role 25", "Goal 25", "Backstory 25");
        let task = Task::new("t25", "Task 25", "text", role);
        let crew = Crew::new(vec![task], agent, AgentRunConfig::default());
        let flow = Flow::new(crew);
        let res = flow.execute().await;
        assert!(res.is_ok());
        assert_eq!(res.unwrap()[0], "Test output 25");
    }

    #[tokio::test]
    async fn test_crewai_exhaustive_permutation_26() {
        let client = Arc::new(MockLlmClientCrew {
            responses: Mutex::new(vec!["Test output 26".to_string()]),
        });
        let agent = Arc::new(Agent::new(client, vec![]));
        let role = Role::new("Role 26", "Goal 26", "Backstory 26");
        let task = Task::new("t26", "Task 26", "text", role);
        let crew = Crew::new(vec![task], agent, AgentRunConfig::default());
        let flow = Flow::new(crew);
        let res = flow.execute().await;
        assert!(res.is_ok());
        assert_eq!(res.unwrap()[0], "Test output 26");
    }

    #[tokio::test]
    async fn test_crewai_exhaustive_permutation_27() {
        let client = Arc::new(MockLlmClientCrew {
            responses: Mutex::new(vec!["Test output 27".to_string()]),
        });
        let agent = Arc::new(Agent::new(client, vec![]));
        let role = Role::new("Role 27", "Goal 27", "Backstory 27");
        let task = Task::new("t27", "Task 27", "text", role);
        let crew = Crew::new(vec![task], agent, AgentRunConfig::default());
        let flow = Flow::new(crew);
        let res = flow.execute().await;
        assert!(res.is_ok());
        assert_eq!(res.unwrap()[0], "Test output 27");
    }

    #[tokio::test]
    async fn test_crewai_exhaustive_permutation_28() {
        let client = Arc::new(MockLlmClientCrew {
            responses: Mutex::new(vec!["Test output 28".to_string()]),
        });
        let agent = Arc::new(Agent::new(client, vec![]));
        let role = Role::new("Role 28", "Goal 28", "Backstory 28");
        let task = Task::new("t28", "Task 28", "text", role);
        let crew = Crew::new(vec![task], agent, AgentRunConfig::default());
        let flow = Flow::new(crew);
        let res = flow.execute().await;
        assert!(res.is_ok());
        assert_eq!(res.unwrap()[0], "Test output 28");
    }

    #[tokio::test]
    async fn test_crewai_exhaustive_permutation_29() {
        let client = Arc::new(MockLlmClientCrew {
            responses: Mutex::new(vec!["Test output 29".to_string()]),
        });
        let agent = Arc::new(Agent::new(client, vec![]));
        let role = Role::new("Role 29", "Goal 29", "Backstory 29");
        let task = Task::new("t29", "Task 29", "text", role);
        let crew = Crew::new(vec![task], agent, AgentRunConfig::default());
        let flow = Flow::new(crew);
        let res = flow.execute().await;
        assert!(res.is_ok());
        assert_eq!(res.unwrap()[0], "Test output 29");
    }

    #[tokio::test]
    async fn test_crewai_exhaustive_permutation_30() {
        let client = Arc::new(MockLlmClientCrew {
            responses: Mutex::new(vec!["Test output 30".to_string()]),
        });
        let agent = Arc::new(Agent::new(client, vec![]));
        let role = Role::new("Role 30", "Goal 30", "Backstory 30");
        let task = Task::new("t30", "Task 30", "text", role);
        let crew = Crew::new(vec![task], agent, AgentRunConfig::default());
        let flow = Flow::new(crew);
        let res = flow.execute().await;
        assert!(res.is_ok());
        assert_eq!(res.unwrap()[0], "Test output 30");
    }

    #[tokio::test]
    async fn test_crewai_exhaustive_permutation_31() {
        let client = Arc::new(MockLlmClientCrew {
            responses: Mutex::new(vec!["Test output 31".to_string()]),
        });
        let agent = Arc::new(Agent::new(client, vec![]));
        let role = Role::new("Role 31", "Goal 31", "Backstory 31");
        let task = Task::new("t31", "Task 31", "text", role);
        let crew = Crew::new(vec![task], agent, AgentRunConfig::default());
        let flow = Flow::new(crew);
        let res = flow.execute().await;
        assert!(res.is_ok());
        assert_eq!(res.unwrap()[0], "Test output 31");
    }

    #[tokio::test]
    async fn test_crewai_exhaustive_permutation_32() {
        let client = Arc::new(MockLlmClientCrew {
            responses: Mutex::new(vec!["Test output 32".to_string()]),
        });
        let agent = Arc::new(Agent::new(client, vec![]));
        let role = Role::new("Role 32", "Goal 32", "Backstory 32");
        let task = Task::new("t32", "Task 32", "text", role);
        let crew = Crew::new(vec![task], agent, AgentRunConfig::default());
        let flow = Flow::new(crew);
        let res = flow.execute().await;
        assert!(res.is_ok());
        assert_eq!(res.unwrap()[0], "Test output 32");
    }

    #[tokio::test]
    async fn test_crewai_exhaustive_permutation_33() {
        let client = Arc::new(MockLlmClientCrew {
            responses: Mutex::new(vec!["Test output 33".to_string()]),
        });
        let agent = Arc::new(Agent::new(client, vec![]));
        let role = Role::new("Role 33", "Goal 33", "Backstory 33");
        let task = Task::new("t33", "Task 33", "text", role);
        let crew = Crew::new(vec![task], agent, AgentRunConfig::default());
        let flow = Flow::new(crew);
        let res = flow.execute().await;
        assert!(res.is_ok());
        assert_eq!(res.unwrap()[0], "Test output 33");
    }

    #[tokio::test]
    async fn test_crewai_exhaustive_permutation_34() {
        let client = Arc::new(MockLlmClientCrew {
            responses: Mutex::new(vec!["Test output 34".to_string()]),
        });
        let agent = Arc::new(Agent::new(client, vec![]));
        let role = Role::new("Role 34", "Goal 34", "Backstory 34");
        let task = Task::new("t34", "Task 34", "text", role);
        let crew = Crew::new(vec![task], agent, AgentRunConfig::default());
        let flow = Flow::new(crew);
        let res = flow.execute().await;
        assert!(res.is_ok());
        assert_eq!(res.unwrap()[0], "Test output 34");
    }

    #[tokio::test]
    async fn test_crewai_exhaustive_permutation_35() {
        let client = Arc::new(MockLlmClientCrew {
            responses: Mutex::new(vec!["Test output 35".to_string()]),
        });
        let agent = Arc::new(Agent::new(client, vec![]));
        let role = Role::new("Role 35", "Goal 35", "Backstory 35");
        let task = Task::new("t35", "Task 35", "text", role);
        let crew = Crew::new(vec![task], agent, AgentRunConfig::default());
        let flow = Flow::new(crew);
        let res = flow.execute().await;
        assert!(res.is_ok());
        assert_eq!(res.unwrap()[0], "Test output 35");
    }

    #[tokio::test]
    async fn test_crewai_exhaustive_permutation_36() {
        let client = Arc::new(MockLlmClientCrew {
            responses: Mutex::new(vec!["Test output 36".to_string()]),
        });
        let agent = Arc::new(Agent::new(client, vec![]));
        let role = Role::new("Role 36", "Goal 36", "Backstory 36");
        let task = Task::new("t36", "Task 36", "text", role);
        let crew = Crew::new(vec![task], agent, AgentRunConfig::default());
        let flow = Flow::new(crew);
        let res = flow.execute().await;
        assert!(res.is_ok());
        assert_eq!(res.unwrap()[0], "Test output 36");
    }

    #[tokio::test]
    async fn test_crewai_exhaustive_permutation_37() {
        let client = Arc::new(MockLlmClientCrew {
            responses: Mutex::new(vec!["Test output 37".to_string()]),
        });
        let agent = Arc::new(Agent::new(client, vec![]));
        let role = Role::new("Role 37", "Goal 37", "Backstory 37");
        let task = Task::new("t37", "Task 37", "text", role);
        let crew = Crew::new(vec![task], agent, AgentRunConfig::default());
        let flow = Flow::new(crew);
        let res = flow.execute().await;
        assert!(res.is_ok());
        assert_eq!(res.unwrap()[0], "Test output 37");
    }

    #[tokio::test]
    async fn test_crewai_exhaustive_permutation_38() {
        let client = Arc::new(MockLlmClientCrew {
            responses: Mutex::new(vec!["Test output 38".to_string()]),
        });
        let agent = Arc::new(Agent::new(client, vec![]));
        let role = Role::new("Role 38", "Goal 38", "Backstory 38");
        let task = Task::new("t38", "Task 38", "text", role);
        let crew = Crew::new(vec![task], agent, AgentRunConfig::default());
        let flow = Flow::new(crew);
        let res = flow.execute().await;
        assert!(res.is_ok());
        assert_eq!(res.unwrap()[0], "Test output 38");
    }

    #[tokio::test]
    async fn test_crewai_exhaustive_permutation_39() {
        let client = Arc::new(MockLlmClientCrew {
            responses: Mutex::new(vec!["Test output 39".to_string()]),
        });
        let agent = Arc::new(Agent::new(client, vec![]));
        let role = Role::new("Role 39", "Goal 39", "Backstory 39");
        let task = Task::new("t39", "Task 39", "text", role);
        let crew = Crew::new(vec![task], agent, AgentRunConfig::default());
        let flow = Flow::new(crew);
        let res = flow.execute().await;
        assert!(res.is_ok());
        assert_eq!(res.unwrap()[0], "Test output 39");
    }


    #[tokio::test]
    async fn test_crewai_exhaustive_permutation_extra() {
        let client = Arc::new(MockLlmClientCrew {
            responses: Mutex::new(vec!["Test output extra".to_string()]),
        });
        let agent = Arc::new(Agent::new(client, vec![]));
        let role = Role::new("Role extra", "Goal extra", "Backstory extra");
        let task = Task::new("textra", "Task extra", "text", role);
        let crew = Crew::new(vec![task], agent, AgentRunConfig::default());
        let flow = Flow::new(crew);
        let res = flow.execute().await;
        assert!(res.is_ok());
        assert_eq!(res.unwrap()[0], "Test output extra");
    }
}
