use crate::agent::{Agent, AgentEvent, AgentRunConfig};
use ohc_builtin_agent_core::types::{ChatRequest, Message, ToolCall};
use ohc_builtin_agent_tools::Tool;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use tokio::sync::{mpsc, oneshot};

/// Represents a single task in the DAG.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DAGTask {
    pub id: String,
    pub tool: String,
    pub args: serde_json::Value,
    #[serde(default)]
    pub dependencies: Vec<String>,
}

/// The LLMCompiler engine that separates planning from execution to achieve concurrent speedup.
pub struct LLMCompiler {
    pub agent: Arc<Agent>,
}

impl LLMCompiler {
    pub fn new(agent: Arc<Agent>) -> Self {
        Self { agent }
    }

    /// Phase 1: Planning (DAG Generation)
    pub async fn plan<F>(
        &self,
        cfg: &AgentRunConfig,
        initial_message: &str,
        tools: &[Tool],
        on_event: &mut F,
    ) -> Result<Vec<DAGTask>, Box<dyn std::error::Error + Send + Sync>>
    where
        F: FnMut(AgentEvent) + Send + Sync,
    {
        on_event(AgentEvent::RunStarted { iteration: 0 });

        let planner_system = format!(
            "You are an expert LLMCompiler planner. Create a strict JSON plan representing a Directed Acyclic Graph (DAG) of tasks to solve the user's request.\n\
            Your output MUST be a valid JSON array of objects, where each object has:\n\
            - `id`: a unique string ID for the task (e.g., \"task_1\")\n\
            - `tool`: the exact name of the tool to use\n\
            - `args`: a JSON object containing the arguments for the tool. To use the output of a previous task, use the syntax \"${{task_id}}\" in the arguments.\n\
            - `dependencies`: a JSON array of task IDs that this task depends on (must be completed before this task can start).\n\n\
            Available tools:\n{}\n\n\
            Return ONLY the JSON array. Do not include markdown formatting or any other text.",
            serde_json::to_string_pretty(&tools.iter().map(|t| crate::types::ToolDefinition {
                name: t.name.clone(),
                description: t.description.clone(),
                parameters: t.parameters.clone(),
            }).collect::<Vec<_>>()).unwrap_or_default()
        );

        let plan_req = ChatRequest {
            model: cfg.model.clone(),
            system: planner_system,
            messages: vec![Message::user(initial_message)],
            tools: vec![],
            max_tokens: cfg.max_tokens,
            temperature: 0.0,
        };

        let plan_resp = self.agent.llm.chat(plan_req.clone()).await?;
        let plan_json_text = plan_resp.message.content.trim().trim_start_matches("```json").trim_start_matches("```").trim_end_matches("```").trim();

        let plan: Vec<DAGTask> = match serde_json::from_str(plan_json_text) {
            Ok(p) => p,
            Err(e) => {
                // Fallback mechanic: Legacy RetryWithErrorOutputParser
                let mut attempt = 0;
                let mut current_req = plan_req;
                tracing::debug!("Output Parsing: Fallback logic triggered in DAG planner.");
                let mut last_error = e.to_string();
                let mut final_plan = None;

                current_req.messages.push(Message::assistant(plan_resp.message.content.clone()));
                let error_msg = format!("Failed to parse output as valid JSON matching the DAGTask schema. Error: {}. Please fix the JSON and return only the raw JSON array without markdown formatting.", e);
                current_req.messages.push(Message::user(error_msg));

                while attempt < 3 {
                    attempt += 1;
                    let resp = self.agent.llm.chat(current_req.clone()).await?;
                    let completion = resp.message.content.clone();

                    let json_text = completion.trim().trim_start_matches("```json").trim_start_matches("```").trim_end_matches("```").trim();
                    match serde_json::from_str::<Vec<DAGTask>>(json_text) {
                        Ok(p) => {
                            final_plan = Some(p);
                            break;
                        }
                        Err(err) => {
                            last_error = err.to_string();
                            current_req.messages.push(Message::assistant(completion));
                            let next_error = format!("Failed to parse output as valid JSON matching the DAGTask schema. Error: {}. Please fix the JSON and return only the raw JSON array without markdown formatting.", err);
                            current_req.messages.push(Message::user(next_error));
                        }
                    }
                }

                if let Some(p) = final_plan {
                    p
                } else {
                    return Err(format!("Failed to parse DAG planner output as JSON array after retries. Last error: {}", last_error).into());
                }
            }
        };

        Self::validate_dag(&plan)?;
        Ok(plan)
    }

    /// Validate that the graph has no cyclic dependencies.
    fn validate_dag(plan: &[DAGTask]) -> Result<(), String> {
        let mut adj = HashMap::new();
        let mut in_degree = HashMap::new();
        let mut ids = HashSet::new();

        for task in plan {
            ids.insert(task.id.clone());
            in_degree.insert(task.id.clone(), 0);
        }

        for task in plan {
            for dep in &task.dependencies {
                if !ids.contains(dep) {
                    return Err(format!("Task '{}' depends on unknown task '{}'", task.id, dep));
                }
                adj.entry(dep.clone()).or_insert_with(Vec::new).push(task.id.clone());
                *in_degree.entry(task.id.clone()).or_insert(0) += 1;
            }
        }

        let mut queue = Vec::new();
        for (id, deg) in &in_degree {
            if *deg == 0 {
                queue.push(id.clone());
            }
        }

        let mut visited_count = 0;
        while let Some(curr) = queue.pop() {
            visited_count += 1;
            if let Some(neighbors) = adj.get(&curr) {
                for next in neighbors {
                    if let Some(deg) = in_degree.get_mut(next) {
                        *deg -= 1;
                        if *deg == 0 {
                            queue.push(next.clone());
                        }
                    }
                }
            }
        }

        if visited_count != ids.len() {
            return Err("Cyclic dependency detected in DAG planner output.".to_string());
        }

        Ok(())
    }

    /// Recursively substitute variables like `${task_id}` in JSON values with their actual execution outputs.
    fn substitute_variables(val: &mut serde_json::Value, results: &HashMap<String, String>) {
        match val {
            serde_json::Value::String(s) => {
                let mut new_s = s.clone();
                for (id, res) in results {
                    let token = format!("${{{}}}", id);
                    new_s = new_s.replace(&token, res);
                }
                *val = serde_json::Value::String(new_s);
            }
            serde_json::Value::Array(arr) => {
                for item in arr {
                    Self::substitute_variables(item, results);
                }
            }
            serde_json::Value::Object(obj) => {
                for (_, v) in obj.iter_mut() {
                    Self::substitute_variables(v, results);
                }
            }
            _ => {}
        }
    }

    /// Phase 2: Concurrent Execution of the DAG
    pub async fn execute_dag<F>(
        &self,
        cfg: &AgentRunConfig,
        plan: Vec<DAGTask>,
        session_tools: &[Tool],
        on_event: &mut F,
    ) -> Result<String, Box<dyn std::error::Error + Send + Sync>>
    where
        F: FnMut(AgentEvent) + Send + Sync,
    {
        on_event(AgentEvent::RunStarted { iteration: 1 });

        let mut in_degree = HashMap::new();
        let mut adj = HashMap::new();
        let mut task_map = HashMap::new();

        for task in &plan {
            task_map.insert(task.id.clone(), task.clone());
            in_degree.insert(task.id.clone(), 0);
        }

        for task in &plan {
            for dep in &task.dependencies {
                adj.entry(dep.clone()).or_insert_with(Vec::new).push(task.id.clone());
                *in_degree.entry(task.id.clone()).or_insert(0) += 1;
            }
        }

        let (tx, mut rx) = mpsc::channel(100);
        let mut active_tasks = 0;
        let mut completed_results = HashMap::new();
        let mut executed_steps = Vec::new();

        // Keep a clone of tx for spawning dependent tasks inside the loop
        let loop_tx = tx.clone();

        // Start tasks with 0 dependencies
        for (id, deg) in &in_degree {
            if *deg == 0 {
                let task = task_map.get(id).unwrap().clone();
                self.spawn_task(task, cfg.clone(), session_tools.to_vec(), tx.clone(), completed_results.clone());
                active_tasks += 1;
            }
        }

        drop(tx); // Drop the parent's original sender. The loop_tx will be dropped at the end.

        while active_tasks > 0 {
            if let Some((id, result)) = rx.recv().await {
                active_tasks -= 1;

                let task = task_map.get(&id).unwrap();
                let output = match result {
                    Ok(res) => {
                        on_event(AgentEvent::ToolCall {
                            name: task.tool.clone(),
                            args_json: task.args.to_string(),
                            result: res.clone(),
                            iteration: executed_steps.len() as i32,
                        });
                        executed_steps.push(format!("Task '{}': Tool '{}' with args '{}' -> Result: '{}'", task.id, task.tool, task.args, res));
                        res
                    }
                    Err(crate::types::ToolError::LlmRecoverable(msg)) => {
                        // Pass LlmRecoverable errors to the execution summary so the replier can handle them
                        let err_msg = format!("Error executing task '{}' (LlmRecoverable): {}", id, msg);
                        on_event(AgentEvent::ToolCall {
                            name: task.tool.clone(),
                            args_json: task.args.to_string(),
                            result: err_msg.clone(),
                            iteration: executed_steps.len() as i32,
                        });
                        executed_steps.push(format!("Task '{}': Tool '{}' with args '{}' -> Result: '{}'", task.id, task.tool, task.args, err_msg));
                        err_msg
                    }
                    Err(crate::types::ToolError::UserFixable(msg)) => {
                        let err = format!("USER_FIXABLE: {}", msg);
                        on_event(AgentEvent::UserInterventionRequired { error: err.clone() });
                        return Err(err.into());
                    }
                    Err(crate::types::ToolError::Fatal(msg)) => {
                        return Err(format!("Fatal tool error in DAG: {}", msg).into());
                    }
                    Err(crate::types::ToolError::Unexpected(msg)) => {
                        return Err(format!("Unexpected tool error in DAG: {}", msg).into());
                    }
                    Err(crate::types::ToolError::Transient(msg)) => {
                        // Transients that survived retries
                        return Err(format!("Transient error failed after retries in DAG: {}", msg).into());
                    }
                    Err(e) => {
                        return Err(format!("Unknown error in DAG: {:?}", e).into());
                    }
                };

                completed_results.insert(id.clone(), output);

                // Trigger dependent tasks
                // If the current task had a recoverable error, its dependent tasks will execute with the error message as their variable input,
                // which allows them to either fix it or pass it up.
                // If it was a fatal error, the DAG would have returned early above.
                if let Some(neighbors) = adj.get(&id) {
                    for next in neighbors {
                        if let Some(deg) = in_degree.get_mut(next) {
                            *deg -= 1;
                            if *deg == 0 {
                                let next_task = task_map.get(next).unwrap().clone();
                                self.spawn_task(next_task, cfg.clone(), session_tools.to_vec(), loop_tx.clone(), completed_results.clone());
                                active_tasks += 1;
                            }
                        }
                    }
                }
            } else {
                // Channel closed prematurely
                break;
            }
        }

        let execution_summary = executed_steps.join("\n\n");
        Ok(execution_summary)
    }

    fn spawn_task(
        &self,
        mut task: DAGTask,
        cfg: AgentRunConfig,
        session_tools: Vec<Tool>,
        tx: mpsc::Sender<(String, Result<String, crate::types::ToolError>)>,
        results_so_far: HashMap<String, String>,
    ) {
        let agent = self.agent.clone();
        tokio::spawn(async move {
            Self::substitute_variables(&mut task.args, &results_so_far);

            let dummy_tc = ToolCall {
                id: format!("dag_{}", task.id),
                name: task.tool.clone(),
                arguments: task.args.clone(),
            };

            // Stage 3 Anthropic Gating Check
            if let Err(e) = Agent::check_tool_gating(&dummy_tc, false, &cfg) {
                let _ = tx.send((task.id.clone(), Err(e))).await;
                return;
            }

            let mut retry_count = 0;
            let max_retries = cfg.max_retries;

            let final_res = loop {
                match agent.execute_tool(&dummy_tc, &session_tools, &[]).await {
                    Ok(res) => break Ok(res),
                    Err(crate::types::ToolError::Transient(msg)) => {
                        if retry_count < max_retries {
                            retry_count += 1;
                            let backoff = std::time::Duration::from_millis(500 * (1 << retry_count));
                            tokio::time::sleep(backoff).await;
                            continue;
                        } else {
                            break Err(crate::types::ToolError::Transient(format!("Transient error after retries: {}", msg)));
                        }
                    }
                    Err(e) => {
                        break Err(e);
                    }
                }
            };

            let _ = tx.send((task.id.clone(), final_res)).await;
        });
    }

    /// Full Plan-and-Execute run function
    pub async fn execute<F>(
        &self,
        cfg: &AgentRunConfig,
        initial_message: &str,
        session_tools: &[Tool],
        on_event: &mut F,
    ) -> Result<String, Box<dyn std::error::Error + Send + Sync>>
    where
        F: FnMut(AgentEvent) + Send + Sync,
    {
        // Phase 1
        let plan = self.plan(cfg, initial_message, session_tools, on_event).await?;

        // Phase 2
        let execution_summary = self.execute_dag(cfg, plan, session_tools, on_event).await?;

        // Phase 3: Replier
        let replier_system = "You are a helpful assistant. Formulate a final response to the user's initial task based on the execution of the planned tasks. Do not attempt to use any further tools.".to_string();
        let final_prompt = format!("Initial task: {}\n\nExecution summary:\n{}\n\nPlease provide the final answer.", initial_message, execution_summary);

        let replier_req = ChatRequest {
            model: cfg.model.clone(),
            system: replier_system,
            messages: vec![Message::user(final_prompt)],
            tools: vec![],
            max_tokens: cfg.max_tokens,
            temperature: cfg.temperature,
        };

        on_event(AgentEvent::RunStarted { iteration: 2 });
        let final_resp = self.agent.llm.chat(replier_req).await?;

        on_event(AgentEvent::TaskComplete { content: final_resp.message.content.clone() });
        Ok(final_resp.message.content)
    }
}
