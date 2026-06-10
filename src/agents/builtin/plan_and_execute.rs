use crate::output_parser::{parse_structured_output, LlmClientForParser};
use crate::tools::Tool;
use ohc_builtin_agent_core::types::{ChatRequest, Message, ToolError};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use tokio::sync::{watch, Mutex, RwLock};

/// SOTA Harness Patterns: 2. ReAct vs Plan-and-Execute (LLMCompiler pattern)
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
            system: system_prompt,
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

    fn verify_dag(plan: &ExecutionPlan) -> Result<(), String> {
        let mut task_map = HashMap::new();
        for task in &plan.tasks {
            task_map.insert(task.task_id.clone(), task.clone());
        }

        // Check for missing dependencies
        for task in &plan.tasks {
            for dep in &task.dependencies {
                if !task_map.contains_key(dep) {
                    return Err(format!(
                        "Task {} depends on missing task {}",
                        task.task_id, dep
                    ));
                }
            }
        }

        // Cycle detection using DFS
        let mut visited = HashSet::new();
        let mut rec_stack = HashSet::new();

        for task in &plan.tasks {
            if Self::is_cyclic(&task.task_id, &task_map, &mut visited, &mut rec_stack) {
                return Err("Cycle detected in execution plan".to_string());
            }
        }

        Ok(())
    }

    fn is_cyclic(
        task_id: &str,
        task_map: &HashMap<String, TaskNode>,
        visited: &mut HashSet<String>,
        rec_stack: &mut HashSet<String>,
    ) -> bool {
        if !visited.contains(task_id) {
            visited.insert(task_id.to_string());
            rec_stack.insert(task_id.to_string());

            if let Some(task) = task_map.get(task_id) {
                for dep in &task.dependencies {
                    if !visited.contains(dep) && Self::is_cyclic(dep, task_map, visited, rec_stack)
                    {
                        return true;
                    } else if rec_stack.contains(dep) {
                        return true;
                    }
                }
            }
        }
        rec_stack.remove(task_id);
        false
    }

    pub async fn execute_plan(
        &self,
        plan: ExecutionPlan,
    ) -> Result<HashMap<String, String>, String> {
        Self::verify_dag(&plan)?;

        let results: Arc<RwLock<HashMap<String, String>>> = Arc::new(RwLock::new(HashMap::new()));
        let mut completion_txs = HashMap::new();
        let mut completion_rxs = HashMap::new();

        for task in &plan.tasks {
            let (tx, rx) = watch::channel(false);
            completion_txs.insert(task.task_id.clone(), tx);
            completion_rxs.insert(task.task_id.clone(), rx);
        }

        let mutating_tool_lock = Arc::new(Mutex::new(()));
        let mut join_handles = Vec::new();

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

        for task in plan.tasks {
            let tools_clone = self.tools.clone();
            let results_clone = results.clone();
            let mut dep_rxs = Vec::new();
            for dep in &task.dependencies {
                if let Some(rx) = completion_rxs.get(dep) {
                    dep_rxs.push(rx.clone());
                }
            }
            let tx = completion_txs.remove(&task.task_id).unwrap();
            let mut_lock = mutating_tool_lock.clone();

            let handle = tokio::spawn(async move {
                // Wait for all dependencies
                for mut rx in dep_rxs {
                    let _ = rx.wait_for(|&completed| completed).await;
                }

                let tool = tools_clone
                    .get(&task.tool_name)
                    .ok_or_else(|| format!("Tool not found: {}", task.tool_name))?;

                let mut resolved_args = task.arguments.clone();
                let r = results_clone.read().await;
                replace_in_json(&mut resolved_args, &r);
                drop(r);

                // Serialize mutating tools
                let _guard = if !tool.is_read_only {
                    Some(mut_lock.lock().await)
                } else {
                    None
                };

                let res = crate::tool_executor_engine::ToolExecutionEngine::execute_tool_with_langgraph_mechanics(
                    &tool,
                    &ohc_builtin_agent_core::types::ToolCall{id: task.task_id.clone(), name: task.tool_name.clone(), arguments: resolved_args},
                    2
                ).await;

                let output = match res {
                    Ok(r) => r,
                    Err(ohc_builtin_agent_core::types::ToolError::LlmRecoverable(msg)) => {
                        format!(
                            "LLM-Recoverable Error: {}. Please analyze this error, correct your tool arguments, and try again.",
                            msg
                        )
                    }
                    Err(e) => return Err(format!("Tool execution failed: {}", e)),
                };

                results_clone
                    .write()
                    .await
                    .insert(task.task_id.clone(), output);
                let _ = tx.send(true);
                Ok(())
            });
            join_handles.push(handle);
        }

        for handle in join_handles {
            handle.await.map_err(|e| e.to_string())??;
        }

        let final_res = results.read().await.clone();
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

    #[tokio::test]
    async fn test_dag_cycle_detection() {
        let orchestrator = PlanAndExecuteOrchestrator::new(vec![]);
        let plan = ExecutionPlan {
            tasks: vec![
                TaskNode {
                    task_id: "task_1".to_string(),
                    tool_name: "tool_a".to_string(),
                    arguments: serde_json::json!({}),
                    dependencies: vec!["task_2".to_string()],
                },
                TaskNode {
                    task_id: "task_2".to_string(),
                    tool_name: "tool_b".to_string(),
                    arguments: serde_json::json!({}),
                    dependencies: vec!["task_1".to_string()],
                },
            ],
        };

        let result = orchestrator.execute_plan(plan).await;
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Cycle detected in execution plan");
    }

    #[tokio::test]
    async fn test_dag_missing_dependency() {
        let orchestrator = PlanAndExecuteOrchestrator::new(vec![]);
        let plan = ExecutionPlan {
            tasks: vec![TaskNode {
                task_id: "task_1".to_string(),
                tool_name: "tool_a".to_string(),
                arguments: serde_json::json!({}),
                dependencies: vec!["non_existent_task".to_string()],
            }],
        };

        let result = orchestrator.execute_plan(plan).await;
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            "Task task_1 depends on missing task non_existent_task"
        );
    }
}
