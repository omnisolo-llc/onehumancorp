use crate::output_parser::{LlmClientForParser, parse_structured_output};
use crate::tools::Tool;
use ohc_builtin_agent_core::types::{ChatRequest, Message, ToolError};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

/// C. 2. ReAct vs Plan-and-Execute: Interleaved (ReAct) vs separate planning. Metric: LLMCompiler achieved 3.6x speedup by separating planning from execution.
/// This module provides a mechanism to explicitly separate planning from execution.
/// A Planner agent first generates a complete DAG of tasks, then an Executor
/// runs them concurrently resolving dependencies.

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskNode {
    pub task_id: String,
    pub tool_name: String,
    pub arguments: serde_json::Value,
    /// List of task_ids that this task depends on
    #[serde(default)]
    pub dependencies: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionPlan {
    pub tasks: Vec<TaskNode>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LLMCompilerState {
    pub results: HashMap<String, String>,
    pub completed_tasks: Vec<String>,
}

pub struct LLMCompilerStateReducer;

impl LLMCompilerStateReducer {
    pub fn reduce(state: &mut LLMCompilerState, task_id: String, output: String) {
        state.results.insert(task_id.clone(), output);
        if !state.completed_tasks.contains(&task_id) {
            state.completed_tasks.push(task_id);
        }
    }
}

pub struct Planner {
    pub llm: Arc<dyn LlmClientForParser>,
}

impl Planner {
    pub async fn create_plan(
        &self,
        user_query: &str,
        tools: &[Tool],
    ) -> Result<ExecutionPlan, ToolError> {
        let mut tool_descriptions = String::new();
        for t in tools {
            tool_descriptions.push_str(&format!("- {}: {}\n", t.name, t.description));
        }

        let system_prompt = format!(
            "You are a Planner agent. Your job is to create a parallelizable execution plan to solve the user's request. \
            Available tools:\n{}\n\
            Generate an ExecutionPlan with a list of tasks. Each task must have a unique `task_id`, a `tool_name`, \
            `arguments`, and an optional list of `dependencies` (other task_ids that must complete before this one). \
            If an argument depends on the output of a previous task, use the syntax `${{task_id}}` in the arguments.",
            tool_descriptions
        );

        let req = ChatRequest {
            model: "gpt-4o".to_string(), // Or get from config if available
            system: ::server_pricing::compression::reduce_tokens(&system_prompt),
            messages: vec![Message::user(user_query.to_string())],
            tools: vec![],
            max_tokens: 4000,
            temperature: 0.0,
        };

        parse_structured_output::<ExecutionPlan>(&self.llm, req, 3).await
    }
}

pub struct PlanAndExecuteOrchestrator {
    pub tools: Arc<HashMap<String, Arc<Tool>>>,
}

impl PlanAndExecuteOrchestrator {
    pub fn new(tools: Vec<Tool>) -> Self {
        let mut tool_map = HashMap::new();
        for t in tools {
            tool_map.insert(t.name.clone(), Arc::new(t));
        }
        Self {
            tools: Arc::new(tool_map),
        }
    }

    pub async fn plan_and_execute(
        &self,
        planner: &Planner,
        user_query: &str,
    ) -> Result<HashMap<String, String>, String> {
        let tool_list: Vec<Tool> = self.tools.values().map(|t| (**t).clone()).collect();
        let plan = planner
            .create_plan(user_query, &tool_list)
            .await
            .map_err(|e| e.to_string())?;
        self.execute_plan(plan).await
    }

    pub async fn execute_plan(
        &self,
        plan: ExecutionPlan,
    ) -> Result<HashMap<String, String>, String> {
        let state = Arc::new(RwLock::new(LLMCompilerState::default()));

        let mut tasks_to_run = plan.tasks.clone();

        // Very basic DAG resolution loop
        // In a production system we'd use Future graphs or something more sophisticated
        loop {
            if tasks_to_run.is_empty() {
                break;
            }

            let state_read = state.read().await;
            let results_read = state_read.results.clone();

            // Find all tasks whose dependencies are met
            let mut ready_tasks = Vec::new();
            let mut remaining_tasks = Vec::new();

            for task in tasks_to_run {
                let mut all_deps_met = true;
                for dep in &task.dependencies {
                    if !results_read.contains_key(dep) {
                        all_deps_met = false;
                        break;
                    }
                }

                if all_deps_met {
                    ready_tasks.push(task);
                } else {
                    remaining_tasks.push(task);
                }
            }

            drop(state_read); // Release the read lock

            if ready_tasks.is_empty() && !remaining_tasks.is_empty() {
                return Err("Deadlock detected: unresolved dependencies in plan".to_string());
            }

            fn replace_in_json(
                value: &mut serde_json::Value,
                results: &std::collections::HashMap<String, String>,
            ) {
                match value {
                    serde_json::Value::String(s) => {
                        let mut new_s = s.clone();
                        for (k, v) in results.iter() {
                            new_s = new_s.replace(&format!("${{{}}}", k), v);
                        }
                        *s = new_s;
                    }
                    serde_json::Value::Array(arr) => {
                        for item in arr {
                            replace_in_json(item, results);
                        }
                    }
                    serde_json::Value::Object(obj) => {
                        for (_, val) in obj.iter_mut() {
                            replace_in_json(val, results);
                        }
                    }
                    _ => {}
                }
            }

            // Split ready tasks into read_only and mutating
            let mut read_only_tasks = Vec::new();
            let mut mutating_tasks = Vec::new();

            for task in ready_tasks {
                let tool = self
                    .tools
                    .get(&task.tool_name)
                    .ok_or_else(|| format!("Tool not found: {}", task.tool_name))?;
                if tool.is_read_only {
                    read_only_tasks.push(task);
                } else {
                    mutating_tasks.push(task);
                }
            }

            // Run read-only tasks concurrently
            let mut ro_handles = Vec::new();
            for task in read_only_tasks {
                let tools_clone = self.tools.clone();
                let results_clone = state.clone();

                let handle = tokio::spawn(async move {
                    let tool = tools_clone
                        .get(&task.tool_name)
                        .ok_or_else(|| format!("Tool not found: {}", task.tool_name))?;

                    let mut resolved_args = task.arguments.clone();

                    let st = results_clone.read().await;
                    replace_in_json(&mut resolved_args, &st.results);
                    drop(st);

                    let res = crate::tool_executor_engine::ToolExecutionEngine::execute_tool_with_langgraph_mechanics(tool, &ohc_builtin_agent_core::types::ToolCall{id: task.task_id.clone(), name: task.tool_name.clone(), arguments: resolved_args}, 2, &crate::agent::AgentRunConfig::default()).await;

                    match res {
                        Ok(r) => Ok::<_, String>((task.task_id, r)),
                        Err(ohc_builtin_agent_core::types::ToolError::LlmRecoverable(msg)) => {
                            Ok::<_, String>((
                                task.task_id,
                                ohc_builtin_agent_core::types::format_llm_recoverable_error(&task.tool_name, &msg),
                            ))
                        }
                        Err(ohc_builtin_agent_core::types::ToolError::UserFixable(msg)) => {
                            Err(format!("USER_FIXABLE: {}", msg))
                        }
                        Err(ohc_builtin_agent_core::types::ToolError::Fatal(msg)) => {
                            Err(format!("Fatal tool error: {}", msg))
                        }
                        Err(ohc_builtin_agent_core::types::ToolError::Unexpected(msg)) => {
                            Err(format!("Unexpected tool error: {}", msg))
                        }
                        Err(e) => Err(format!("Tool execution failed: {}", e)),
                    }
                });
                ro_handles.push(handle);
            }

            for handle in ro_handles {
                let (task_id, output) = handle.await.map_err(|e| e.to_string())??;
                let mut st = state.write().await;
                LLMCompilerStateReducer::reduce(&mut st, task_id, output);
            }

            // Run mutating tasks serially
            for task in mutating_tasks {
                let tool = self
                    .tools
                    .get(&task.tool_name)
                    .ok_or_else(|| format!("Tool not found: {}", task.tool_name))?;
                let mut resolved_args = task.arguments.clone();

                let st = state.read().await;
                replace_in_json(&mut resolved_args, &st.results);
                drop(st);

                let res = crate::tool_executor_engine::ToolExecutionEngine::execute_tool_with_langgraph_mechanics(tool, &ohc_builtin_agent_core::types::ToolCall{id: task.task_id.clone(), name: task.tool_name.clone(), arguments: resolved_args}, 2, &crate::agent::AgentRunConfig::default())
                    .await;

                let final_res = match res {
                    Ok(r) => r,
                    Err(ohc_builtin_agent_core::types::ToolError::LlmRecoverable(msg)) => {
                        ohc_builtin_agent_core::types::format_llm_recoverable_error(&task.tool_name, &msg)
                    }
                    Err(ohc_builtin_agent_core::types::ToolError::UserFixable(msg)) => {
                        return Err(format!("USER_FIXABLE: {}", msg));
                    }
                    Err(ohc_builtin_agent_core::types::ToolError::Fatal(msg)) => {
                        return Err(format!("Fatal tool error: {}", msg));
                    }
                    Err(ohc_builtin_agent_core::types::ToolError::Unexpected(msg)) => {
                        return Err(format!("Unexpected tool error: {}", msg));
                    }
                    Err(e) => return Err(format!("Tool execution failed: {}", e)),
                };
                let mut st = state.write().await;
                LLMCompilerStateReducer::reduce(&mut st, task.task_id.clone(), final_res);
            }

            tasks_to_run = remaining_tasks;
        }

        let final_res = state.read().await.results.clone();
        Ok(final_res)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tools::ToolExecutor;
    use ohc_builtin_agent_core::types::{ChatResponse, Role, ToolCall, Usage};

    struct MockPlannerLlm {
        plan_json: String,
    }

    #[async_trait::async_trait]
    impl crate::output_parser::LlmClientForParser for MockPlannerLlm {
        async fn chat(
            &self,
            _req: ChatRequest,
        ) -> Result<ChatResponse, Box<dyn std::error::Error + Send + Sync>> {
            let tc = ToolCall {
                id: "call_1".to_string(),
                name: "structured_output".to_string(),
                arguments: serde_json::json!({
                    "data": serde_json::from_str::<serde_json::Value>(&self.plan_json).unwrap()
                }),
            };

            let msg = Message {
                role: Role::Assistant,
                content: "".to_string(),
                tool_calls: vec![tc],
                tool_results: vec![],
                response_id: None,
                previous_response_id: None,
            };

            Ok(ChatResponse {
                message: msg,
                usage: Usage::default(),
                stop_reason: "tool_calls".to_string(),
                response_id: Some("mock-id".to_string()),
            })
        }
    }

    struct MockToolExecutor {
        response: String,
    }

    #[async_trait::async_trait]
    impl ToolExecutor for MockToolExecutor {
        async fn execute(&self, args: serde_json::Value) -> Result<String, ToolError> {
            // For testing dependency resolution
            if let Some(dep_val) = args.get("dep_input").and_then(|v| v.as_str()) {
                Ok(format!("{} + {}", self.response, dep_val))
            } else {
                Ok(self.response.clone())
            }
        }
    }

    #[tokio::test]
    async fn test_llm_compiler_state_reducer() {
        let mut state = LLMCompilerState::default();
        assert!(state.results.is_empty());
        assert!(state.completed_tasks.is_empty());

        LLMCompilerStateReducer::reduce(&mut state, "task_1".to_string(), "output_1".to_string());

        assert_eq!(state.results.get("task_1").unwrap(), "output_1");
        assert!(state.completed_tasks.contains(&"task_1".to_string()));

        LLMCompilerStateReducer::reduce(&mut state, "task_2".to_string(), "output_2".to_string());

        assert_eq!(state.results.len(), 2);
        assert_eq!(state.completed_tasks.len(), 2);
    }

    #[tokio::test]
    async fn test_planner_creates_plan() {
        let plan_json = r#"{
            "tasks": [
                {
                    "task_id": "task_1",
                    "tool_name": "tool_a",
                    "arguments": {},
                    "dependencies": []
                }
            ]
        }"#;

        let llm = Arc::new(MockPlannerLlm {
            plan_json: plan_json.to_string(),
        });
        let planner = Planner { llm };

        let plan = planner.create_plan("do something", &[]).await.unwrap();
        assert_eq!(plan.tasks.len(), 1);
        assert_eq!(plan.tasks[0].task_id, "task_1");
    }

    #[tokio::test]
    async fn test_planner_generation() {
        let plan_json = r#"{
            "tasks": [
                {
                    "task_id": "gen_1",
                    "tool_name": "tool_gen",
                    "arguments": {},
                    "dependencies": []
                }
            ]
        }"#;

        let llm = Arc::new(MockPlannerLlm {
            plan_json: plan_json.to_string(),
        });
        let planner = Planner { llm };

        let plan = planner.create_plan("generate plan", &[]).await.unwrap();
        assert_eq!(plan.tasks.len(), 1);
        assert_eq!(plan.tasks[0].task_id, "gen_1");
    }

    #[tokio::test]
    async fn test_executor_parallel_processing() {
        struct MockExecutor {
            resp: String,
        }
        #[async_trait::async_trait]
        impl ToolExecutor for MockExecutor {
            async fn execute(&self, _args: serde_json::Value) -> Result<String, ToolError> {
                tokio::time::sleep(std::time::Duration::from_millis(50)).await;
                Ok(self.resp.clone())
            }
        }
        let t1 = Tool {
            name: "t1".to_string(),
            description: "".to_string(),
            is_read_only: true,
            parameters: serde_json::json!({}),
            execute: Arc::new(MockExecutor {
                resp: "r1".to_string(),
            }),
        };
        let t2 = Tool {
            name: "t2".to_string(),
            description: "".to_string(),
            is_read_only: true,
            parameters: serde_json::json!({}),
            execute: Arc::new(MockExecutor {
                resp: "r2".to_string(),
            }),
        };
        let orchestrator = PlanAndExecuteOrchestrator::new(vec![t1, t2]);
        let plan = ExecutionPlan {
            tasks: vec![
                TaskNode {
                    task_id: "id1".to_string(),
                    tool_name: "t1".to_string(),
                    arguments: serde_json::json!({}),
                    dependencies: vec![],
                },
                TaskNode {
                    task_id: "id2".to_string(),
                    tool_name: "t2".to_string(),
                    arguments: serde_json::json!({}),
                    dependencies: vec![],
                },
            ],
        };

        let start = std::time::Instant::now();
        let res = orchestrator.execute_plan(plan).await.unwrap();
        let elapsed = start.elapsed().as_millis();

        assert_eq!(res.get("id1").unwrap(), "r1");
        assert_eq!(res.get("id2").unwrap(), "r2");
        assert!(
            elapsed < 90,
            "Expected parallel execution to take < 90ms, took {}ms",
            elapsed
        );
    }

    #[tokio::test]
    async fn test_orchestrator_executes_dag() {
        let tool_a = Tool {
            name: "tool_a".to_string(),
            description: "".to_string(),
            is_read_only: true,
            parameters: serde_json::json!({}),
            execute: Arc::new(MockToolExecutor {
                response: "result_a".to_string(),
            }),
        };

        let tool_b = Tool {
            name: "tool_b".to_string(),
            description: "".to_string(),
            is_read_only: true,
            parameters: serde_json::json!({}),
            execute: Arc::new(MockToolExecutor {
                response: "result_b".to_string(),
            }),
        };

        let orchestrator = PlanAndExecuteOrchestrator::new(vec![tool_a, tool_b]);

        let plan = ExecutionPlan {
            tasks: vec![
                TaskNode {
                    task_id: "task_1".to_string(),
                    tool_name: "tool_a".to_string(),
                    arguments: serde_json::json!({}),
                    dependencies: vec![],
                },
                TaskNode {
                    task_id: "task_2".to_string(),
                    tool_name: "tool_b".to_string(),
                    arguments: serde_json::json!({"dep_input": "${task_1}"}),
                    dependencies: vec!["task_1".to_string()],
                },
            ],
        };

        let results = orchestrator.execute_plan(plan).await.unwrap();

        assert_eq!(results.get("task_1").unwrap(), "result_a");
        // task_2 should have received result_a as input due to dependency resolution
        assert_eq!(results.get("task_2").unwrap(), "result_b + result_a");
    }

    #[tokio::test]
    async fn test_orchestrator_executes_read_only_concurrently_mutating_serially() {
        struct TimingToolExecutor {
            sleep_ms: u64,
            response: String,
        }

        #[async_trait::async_trait]
        impl ToolExecutor for TimingToolExecutor {
            async fn execute(&self, _args: serde_json::Value) -> Result<String, ToolError> {
                tokio::time::sleep(std::time::Duration::from_millis(self.sleep_ms)).await;
                Ok(self.response.clone())
            }
        }

        let tool_ro1 = Tool {
            name: "tool_ro1".to_string(),
            description: "".to_string(),
            is_read_only: true,
            parameters: serde_json::json!({}),
            execute: Arc::new(TimingToolExecutor {
                sleep_ms: 100,
                response: "ro1".to_string(),
            }),
        };

        let tool_ro2 = Tool {
            name: "tool_ro2".to_string(),
            description: "".to_string(),
            is_read_only: true,
            parameters: serde_json::json!({}),
            execute: Arc::new(TimingToolExecutor {
                sleep_ms: 100,
                response: "ro2".to_string(),
            }),
        };

        let tool_mut = Tool {
            name: "tool_mut".to_string(),
            description: "".to_string(),
            is_read_only: false,
            parameters: serde_json::json!({}),
            execute: Arc::new(TimingToolExecutor {
                sleep_ms: 100,
                response: "mut".to_string(),
            }),
        };

        let orchestrator = PlanAndExecuteOrchestrator::new(vec![tool_ro1, tool_ro2, tool_mut]);

        let plan = ExecutionPlan {
            tasks: vec![
                TaskNode {
                    task_id: "ro1".to_string(),
                    tool_name: "tool_ro1".to_string(),
                    arguments: serde_json::json!({}),
                    dependencies: vec![],
                },
                TaskNode {
                    task_id: "ro2".to_string(),
                    tool_name: "tool_ro2".to_string(),
                    arguments: serde_json::json!({}),
                    dependencies: vec![],
                },
                TaskNode {
                    task_id: "mut".to_string(),
                    tool_name: "tool_mut".to_string(),
                    arguments: serde_json::json!({}),
                    dependencies: vec![],
                },
            ],
        };

        let start = std::time::Instant::now();
        let results = orchestrator.execute_plan(plan).await.unwrap();
        let elapsed = start.elapsed().as_millis();

        assert_eq!(results.get("ro1").unwrap(), "ro1");
        assert_eq!(results.get("ro2").unwrap(), "ro2");
        assert_eq!(results.get("mut").unwrap(), "mut");

        // The 2 read-only tools should run concurrently (taking ~100ms total).
        // The 1 mutating tool should run sequentially (taking ~100ms total).
        // The total time should be roughly 200ms.
        // If all ran sequentially, it would take ~300ms.
        // We will assert that it took less than 280ms, but more than 150ms to ensure it didn't all run concurrently.
        assert!(
            elapsed >= 150,
            "Execution was too fast, expected >= 150ms, got {}ms",
            elapsed
        );
        assert!(
            elapsed < 280,
            "Execution was too slow, expected < 280ms (concurrent RO), got {}ms",
            elapsed
        );
    }
}
