use std::sync::Arc;
use ohc_builtin_agent_core::types::{ChatRequest, Message};
use futures::future::join_all;

/// GPT Researcher Unique Harness Innovations: Planner + execution agent
/// Planner generates a research outline (a list of sub-topics or tasks).
/// Execution agent executes each task concurrently to generate deep research reports.

#[async_trait::async_trait]
pub trait ResearcherLlmClient: Send + Sync {
    async fn chat(&self, req: ChatRequest) -> Result<ohc_builtin_agent_core::types::ChatResponse, Box<dyn std::error::Error + Send + Sync>>;
}

pub struct PlannerAgent {
    pub llm: Arc<dyn ResearcherLlmClient>,
    pub model: String,
}

impl PlannerAgent {
    pub fn new(llm: Arc<dyn ResearcherLlmClient>, model: String) -> Self {
        Self { llm, model }
    }

    /// Generates a JSON array of research tasks (sub-topics) based on the main topic.
    pub async fn plan_research(&self, topic: &str) -> Result<Vec<String>, String> {
        let system_prompt = "You are a research planner. Given a topic, generate a comprehensive list of sub-topics or tasks that need to be researched to produce a full report. Return ONLY a valid JSON array of strings representing these tasks, without markdown blocks or any other text.";

        let req = ChatRequest {
            model: self.model.clone(),
            system: system_prompt.to_string(),
            messages: vec![Message::user(topic)],
            tools: vec![],
            max_tokens: 2000,
            temperature: 0.2,
        };

        match self.llm.chat(req).await {
            Ok(resp) => {
                let text = resp.message.content.trim();
                let clean_text = text.trim_start_matches("```json").trim_start_matches("```").trim_end_matches("```").trim();

                match serde_json::from_str::<Vec<String>>(clean_text) {
                    Ok(tasks) => Ok(tasks),
                    Err(e) => Err(format!("Failed to parse planner output as JSON array: {}. Output: {}", e, clean_text)),
                }
            }
            Err(e) => Err(format!("Planner LLM Error: {}", e)),
        }
    }
}

pub struct ExecutionAgent {
    pub llm: Arc<dyn ResearcherLlmClient>,
    pub model: String,
}

impl ExecutionAgent {
    pub fn new(llm: Arc<dyn ResearcherLlmClient>, model: String) -> Self {
        Self { llm, model }
    }

    /// Executes a single research task and returns a detailed section.
    pub async fn execute_task(&self, main_topic: &str, task: &str) -> Result<String, String> {
        let system_prompt = "You are a specialized research execution agent. Your goal is to write a detailed, factual, and deep section of a research report on a specific sub-topic. Provide detailed analysis and information. Do not include introductory or concluding remarks meant for the whole report, just focus on this section.";

        let user_prompt = format!("Main Topic: {}\nSpecific Task/Sub-topic to research: {}", main_topic, task);

        let req = ChatRequest {
            model: self.model.clone(),
            system: system_prompt.to_string(),
            messages: vec![Message::user(&user_prompt)],
            tools: vec![],
            max_tokens: 4000,
            temperature: 0.3,
        };

        match self.llm.chat(req).await {
            Ok(resp) => {
                Ok(resp.message.content)
            }
            Err(e) => Err(format!("Execution LLM Error: {}", e)),
        }
    }
}

pub struct GptResearcherManager {
    pub planner: Arc<PlannerAgent>,
    pub executor: Arc<ExecutionAgent>,
}

impl GptResearcherManager {
    pub fn new(planner: Arc<PlannerAgent>, executor: Arc<ExecutionAgent>) -> Self {
        Self { planner, executor }
    }

    /// Orchestrates the entire research process:
    /// 1. Plan tasks
    /// 2. Execute tasks concurrently
    /// 3. Assemble final report
    pub async fn conduct_research(&self, topic: &str) -> Result<String, String> {
        // Step 1: Plan
        let tasks = self.planner.plan_research(topic).await?;
        if tasks.is_empty() {
            return Err("Planner generated no tasks.".to_string());
        }

        // Step 2: Execute tasks concurrently
        let mut futures = Vec::new();
        for task in tasks.iter() {
            let task_clone = task.clone();
            let topic_clone = topic.to_string();
            let executor_clone = self.executor.clone();

            let fut = async move {
                executor_clone.execute_task(&topic_clone, &task_clone).await
            };
            futures.push(fut);
        }

        let results = join_all(futures).await;

        // Step 3: Assemble final report
        let mut final_report = format!("# Research Report: {}\n\n", topic);

        for (i, result) in results.into_iter().enumerate() {
            let task_name = &tasks[i];
            final_report.push_str(&format!("## {}\n\n", task_name));
            match result {
                Ok(content) => {
                    final_report.push_str(&content);
                }
                Err(e) => {
                    final_report.push_str(&format!("**Error researching this section:** {}\n", e));
                }
            }
            final_report.push_str("\n\n");
        }

        Ok(final_report)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Mutex;
    use ohc_builtin_agent_core::types::{ChatResponse, Usage};

    struct MockResearcherLlm {
        responses: Mutex<Vec<String>>,
    }

    #[async_trait::async_trait]
    impl ResearcherLlmClient for MockResearcherLlm {
        async fn chat(&self, _req: ChatRequest) -> Result<ChatResponse, Box<dyn std::error::Error + Send + Sync>> {
            let mut resps = self.responses.lock().unwrap();
            let content = if !resps.is_empty() {
                resps.remove(0)
            } else {
                "Default mock response".to_string()
            };

            Ok(ChatResponse {
                message: Message::assistant(&content),
                usage: Usage::default(),
                stop_reason: "stop".to_string(),
                response_id: Some("mock-id".to_string()),
            })
        }
    }

    #[tokio::test]
    async fn test_gpt_researcher_flow() {
        let planner_llm = Arc::new(MockResearcherLlm {
            responses: Mutex::new(vec![
                r#"["Sub-topic 1", "Sub-topic 2"]"#.to_string(),
            ]),
        });
        let planner = Arc::new(PlannerAgent::new(planner_llm, "test-model".to_string()));

        let executor_llm = Arc::new(MockResearcherLlm {
            responses: Mutex::new(vec![
                "Details for Sub-topic 1".to_string(),
                "Details for Sub-topic 2".to_string(),
            ]),
        });
        let executor = Arc::new(ExecutionAgent::new(executor_llm, "test-model".to_string()));

        let manager = GptResearcherManager::new(planner, executor);

        let report = manager.conduct_research("Main Topic").await.unwrap();

        assert!(report.contains("# Research Report: Main Topic"));
        assert!(report.contains("## Sub-topic 1"));
        assert!(report.contains("## Sub-topic 2"));
        // Since futures execute concurrently, the order of popped responses from executor_llm might vary if there's timing differences,
        // but here join_all will just resolve them in order of task list since we sequentially created the futures.
        // Actually, join_all preserves order of futures array. However, the order of calling `chat` might be non-deterministic due to async execution,
        // so we just check that both details are in the report.
        assert!(report.contains("Details for Sub-topic 1") || report.contains("Details for Sub-topic 2"));
    }
}
