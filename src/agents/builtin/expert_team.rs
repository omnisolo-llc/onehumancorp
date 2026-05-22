use ohc_builtin_agent_core::types::{ChatRequest, ChatResponse, Message, Role, ToolError};
use ohc_builtin_agent_llm::LlmClient;
use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use tokio::task::JoinSet;
use serde::{Deserialize, Serialize};
use strsim::normalized_levenshtein;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExpertTask {
    pub id: String,
    pub domain: String,
    pub description: String,
    pub context: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExpertResult {
    pub task_id: String,
    pub summary: String,
    pub skill_trace: Vec<String>,
    pub word_count: usize,
}

pub struct ExpertTeam {
    pub lead_agent_client: Arc<dyn LlmClient>,
    pub required_experts: usize,
    pub min_word_count: usize,
}

#[derive(Debug)]
pub enum GateError {
    PreFlight(String),
    PreDeliver(String),
    Execution(String),
}

impl ExpertTeam {
    pub fn new(lead_agent_client: Arc<dyn LlmClient>, required_experts: usize, min_word_count: usize) -> Self {
        Self {
            lead_agent_client,
            required_experts,
            min_word_count,
        }
    }

    /// Run a project using the Expert Team pattern
    pub async fn run_project(&self, project_goal: &str, tasks: Vec<ExpertTask>) -> Result<Vec<ExpertResult>, GateError> {
        // --- PRE-FLIGHT GATE ---
        // 1. Check if the required number of expert domains are initialized
        let unique_domains: HashSet<String> = tasks.iter().map(|t| t.domain.clone()).collect();
        if unique_domains.len() < self.required_experts {
            return Err(GateError::PreFlight(format!(
                "Pre-flight failed: Expected at least {} unique expert domains, but only found {}.",
                self.required_experts,
                unique_domains.len()
            )));
        }

        // --- PARALLEL EXECUTION ---
        let mut join_set = JoinSet::new();
        for task in tasks {
            let client = self.lead_agent_client.clone();
            join_set.spawn(async move {
                Self::run_expert_task(client, task).await
            });
        }

        let mut results = Vec::new();
        while let Some(res) = join_set.join_next().await {
            match res {
                Ok(Ok(expert_result)) => results.push(expert_result),
                Ok(Err(e)) => return Err(GateError::Execution(format!("Expert task failed: {:?}", e))),
                Err(e) => return Err(GateError::Execution(format!("Task panic: {:?}", e))),
            }
        }

        // --- PRE-DELIVER GATE ---
        let mut total_words = 0;
        let mut all_summaries = Vec::new();

        for result in &results {
            total_words += result.word_count;
            all_summaries.push(result.summary.clone());

            // Check skill-trace completeness
            if result.skill_trace.is_empty() {
                return Err(GateError::PreDeliver(format!(
                    "Pre-deliver failed: Expert for task '{}' bypassed execution (empty skill trace).",
                    result.task_id
                )));
            }
        }

        // Check word count
        if total_words < self.min_word_count {
            return Err(GateError::PreDeliver(format!(
                "Pre-deliver failed: Total word count {} is less than required {}.",
                total_words, self.min_word_count
            )));
        }

        // Check deduplication (similarity check)
        for i in 0..all_summaries.len() {
            for j in (i + 1)..all_summaries.len() {
                let sim = normalized_levenshtein(&all_summaries[i], &all_summaries[j]);
                if sim > 0.75 {
                    return Err(GateError::PreDeliver("Pre-deliver failed: High similarity (>= 75%) detected between expert summaries. Deduplication failed.".to_string()));
                }
            }
        }

        Ok(results)
    }

    async fn run_expert_task(client: Arc<dyn LlmClient>, task: ExpertTask) -> Result<ExpertResult, ToolError> {
        let system_prompt = format!(
            "You are an expert in {}. Your task: {}\nContext: {}\n\nCRITICAL: Return a condensed summary (100-2000 words) of your findings and actions. You must output valid JSON in this exact format: {{\"summary\": \"your summary\", \"skill_trace\": [\"action1\", \"action2\"]}}",
            task.domain, task.description, task.context
        );

        let req = ChatRequest {
            model: "default".to_string(),
            system: system_prompt,
            messages: vec![Message::user(task.description.clone())],
            tools: vec![],
            max_tokens: 2048,
            temperature: 0.1,
        };

        let resp = client.chat(req).await.map_err(|e| ToolError::LlmRecoverable(e.to_string()))?;

        // Parse the LLM output as JSON
        #[derive(Deserialize)]
        struct ExpertOutput {
            summary: String,
            skill_trace: Vec<String>,
        }

        let content = resp.message.content.trim();
        let parsed: ExpertOutput = serde_json::from_str(content).unwrap_or_else(|_| {
            // fallback if LLM wraps in markdown
            let clean = content.trim_start_matches("```json").trim_end_matches("```").trim();
            serde_json::from_str(clean).unwrap_or_else(|_| ExpertOutput {
                summary: "Parsing failed, falling back to raw content".to_string(),
                skill_trace: vec!["error_recovery".to_string()],
            })
        });

        let word_count = parsed.summary.split_whitespace().count();

        Ok(ExpertResult {
            task_id: task.id,
            summary: parsed.summary,
            skill_trace: parsed.skill_trace,
            word_count,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ohc_builtin_agent_core::types::Usage;
    use tokio::sync::Mutex;
    use async_trait::async_trait;

    struct MockLlmClient {
        responses: Mutex<HashMap<String, String>>,
    }

    #[async_trait]
    impl LlmClient for MockLlmClient {
        async fn chat(&self, req: ChatRequest) -> Result<ChatResponse, Box<dyn std::error::Error + Send + Sync>> {
            let resps = self.responses.lock().await;
            // Hacky match based on domain
            let mut content = r#"{"summary": "default summary that is decently long to pass word count constraints hopefully.", "skill_trace": ["searched", "analyzed"]}"#.to_string();

            if req.system.contains("Finance Analyst") {
                content = resps.get("Finance Analyst").cloned().unwrap_or(content);
            } else if req.system.contains("Market Researcher") {
                content = resps.get("Market Researcher").cloned().unwrap_or(content);
            } else if req.system.contains("Lazy Expert") {
                content = resps.get("Lazy Expert").cloned().unwrap_or(content);
            } else if req.system.contains("Clone Expert") {
                content = resps.get("Clone Expert").cloned().unwrap_or(content);
            }

            Ok(ChatResponse {
                message: Message::assistant(&content),
                usage: Usage::default(),
                stop_reason: "stop".to_string(),
                response_id: Some("mock-id".to_string()),
            })
        }
    }

    fn create_tasks() -> Vec<ExpertTask> {
        vec![
            ExpertTask {
                id: "1".to_string(),
                domain: "Finance Analyst".to_string(),
                description: "Analyze financials".to_string(),
                context: "Data".to_string(),
            },
            ExpertTask {
                id: "2".to_string(),
                domain: "Market Researcher".to_string(),
                description: "Research market".to_string(),
                context: "Market Data".to_string(),
            },
        ]
    }

    #[tokio::test]
    async fn test_pre_flight_gate() {
        let client = Arc::new(MockLlmClient { responses: Mutex::new(HashMap::new()) });
        let team = ExpertTeam::new(client, 3, 10); // requires 3 experts

        let tasks = create_tasks(); // only 2 domains

        let result = team.run_project("Goal", tasks).await;
        assert!(matches!(result, Err(GateError::PreFlight(_))));
    }

    #[tokio::test]
    async fn test_successful_run() {
        let mut map = HashMap::new();
        map.insert("Finance Analyst".to_string(), r#"{"summary": "Financials look solid with a 20% margin. We recommend proceeding.", "skill_trace": ["read_balance_sheet", "calculate_margin"]}"#.to_string());
        map.insert("Market Researcher".to_string(), r#"{"summary": "The market is growing at 5% annually. Competitors are weak.", "skill_trace": ["search_web", "analyze_competitors"]}"#.to_string());

        let client = Arc::new(MockLlmClient { responses: Mutex::new(map) });
        let team = ExpertTeam::new(client, 2, 10);

        let tasks = create_tasks();
        let result = team.run_project("Goal", tasks).await.unwrap();

        assert_eq!(result.len(), 2);
        assert!(result[0].word_count > 0);
        assert!(!result[0].skill_trace.is_empty());
    }

    #[tokio::test]
    async fn test_pre_deliver_empty_skill_trace() {
        let mut map = HashMap::new();
        map.insert("Finance Analyst".to_string(), r#"{"summary": "Financials look solid.", "skill_trace": []}"#.to_string()); // empty skill trace
        map.insert("Market Researcher".to_string(), r#"{"summary": "The market is growing.", "skill_trace": ["search_web"]}"#.to_string());

        let client = Arc::new(MockLlmClient { responses: Mutex::new(map) });
        let team = ExpertTeam::new(client, 2, 5);

        let tasks = create_tasks();
        let result = team.run_project("Goal", tasks).await;

        assert!(matches!(result, Err(GateError::PreDeliver(ref e)) if e.contains("empty skill trace")));
    }

    #[tokio::test]
    async fn test_pre_deliver_word_count() {
        let mut map = HashMap::new();
        map.insert("Finance Analyst".to_string(), r#"{"summary": "Good.", "skill_trace": ["read"]}"#.to_string());
        map.insert("Market Researcher".to_string(), r#"{"summary": "Bad.", "skill_trace": ["search"]}"#.to_string());

        let client = Arc::new(MockLlmClient { responses: Mutex::new(map) });
        let team = ExpertTeam::new(client, 2, 50); // Requires 50 words

        let tasks = create_tasks();
        let result = team.run_project("Goal", tasks).await;

        assert!(matches!(result, Err(GateError::PreDeliver(ref e)) if e.contains("less than required")));
    }

    #[tokio::test]
    async fn test_pre_deliver_deduplication() {
        let mut map = HashMap::new();
        // Exact same summary for two different experts
        let dup_summary = r#"{"summary": "The quick brown fox jumps over the lazy dog. This is a very specific sentence to trigger similarity.", "skill_trace": ["action"]}"#.to_string();
        map.insert("Finance Analyst".to_string(), dup_summary.clone());
        map.insert("Market Researcher".to_string(), dup_summary);

        let client = Arc::new(MockLlmClient { responses: Mutex::new(map) });
        let team = ExpertTeam::new(client, 2, 10);

        let tasks = create_tasks();
        let result = team.run_project("Goal", tasks).await;

        assert!(matches!(result, Err(GateError::PreDeliver(ref e)) if e.contains("High similarity")));
    }
}
