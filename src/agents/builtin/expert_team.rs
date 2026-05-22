use crate::agent::{Agent, AgentRunConfig};
use std::sync::Arc;
use tokio::task::JoinSet;
use serde::{Deserialize, Serialize};

/// Tencent Workbuddy (Expert Team) Feature:
/// - Lead agent orchestrates and delegates to specialized domain expert agents
/// - Parallel execution of expert tasks (not sequential)
/// - Code-enforced quality gates at super-step boundaries
/// - Skill-trace tracking to prevent hard-coded bypasses
/// - Each expert agent returns condensed summaries (1k-2k tokens), never full context

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillTrace {
    pub expert_name: String,
    pub task_description: String,
    pub condensed_summary: String,
}

pub struct ExpertTeamConfig {
    pub lead_agent: Arc<Agent>,
    pub domain_experts: Vec<(String, Arc<Agent>)>,
    pub quality_auditor: Arc<Agent>,
    pub min_words_pre_deliver: usize,
}

pub struct ExpertTeam {
    pub config: ExpertTeamConfig,
}

impl ExpertTeam {
    pub fn new(config: ExpertTeamConfig) -> Self {
        Self { config }
    }

    pub async fn execute_project(
        &self,
        project_prompt: &str,
        run_config: &AgentRunConfig,
    ) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        // Pre-flight Quality Gate
        if self.config.domain_experts.is_empty() {
            return Err("Pre-flight gate failed: No domain experts initialized. Must have at least 1 expert.".into());
        }

        // 1. Lead Agent Decomposes the Task (Here we simplify by assigning the same core project to all experts with domain context)
        // In a full implementation, Lead Agent would generate specific JSON sub-tasks.
        let mut join_set = JoinSet::new();

        // 2. Parallel execution of expert tasks (not sequential)
        for (expert_name, expert_agent) in &self.config.domain_experts {
            let expert_name = expert_name.clone();
            let expert_agent = expert_agent.clone();
            let project_prompt = project_prompt.to_string();
            let mut expert_run_config = run_config.clone();

            // Constraint: Each expert agent returns condensed summaries (max 2000 tokens)
            expert_run_config.max_tokens = 2000;

            join_set.spawn(async move {
                let task_prompt = format!(
                    "Project: {}\nYour Expert Role: {}. Provide a condensed summary of your findings (under 2000 tokens).",
                    project_prompt, expert_name
                );
                let mut on_event = |_e: crate::agent::AgentEvent| {};
                let result = expert_agent.run(&expert_run_config, &task_prompt, &mut on_event).await;
                (expert_name, project_prompt, result)
            });
        }

        let mut traces = Vec::new();
        let mut success_count = 0;

        while let Some(res) = join_set.join_next().await {
            match res {
                Ok((name, task, Ok(summary))) => {
                    success_count += 1;
                    traces.push(SkillTrace {
                        expert_name: name,
                        task_description: task,
                        condensed_summary: summary,
                    });
                }
                Ok((name, _, Err(e))) => {
                    return Err(format!("Domain expert {} failed: {}", name, e).into());
                }
                Err(e) => {
                    return Err(format!("Task spawn failed: {}", e).into());
                }
            }
        }

        // Pre-merge Quality Gate
        // Code-enforced: 100% completeness required (all experts must succeed)
        if success_count != self.config.domain_experts.len() {
            return Err("Pre-merge gate failed: Not all domain experts completed their tasks.".into());
        }

        // Skill-trace tracking to prevent hard-coded bypasses
        let has_empty_traces = traces.iter().any(|t| t.condensed_summary.trim().is_empty());
        if has_empty_traces {
            return Err("Pre-merge gate failed: Skill-trace completeness check failed (empty summary).".into());
        }

        // Combine traces
        let mut combined_summaries = String::new();
        for trace in &traces {
            combined_summaries.push_str(&format!("--- Expert: {} ---\n{}\n\n", trace.expert_name, trace.condensed_summary));
        }

        // 3. Quality Control (Quality Auditor)
        let qc_prompt = format!(
            "You are the Quality Auditor. Verify the following expert summaries. Output 'APPROVE' if they are satisfactory, or 'REJECT: <reason>' if they are lacking.\n\n{}",
            combined_summaries
        );
        let mut on_event = |_e: crate::agent::AgentEvent| {};
        let qc_result = self.config.quality_auditor.run(run_config, &qc_prompt, &mut on_event).await?;

        if qc_result.contains("REJECT") {
            return Err(format!("Quality Control rejected the expert outputs: {}", qc_result).into());
        }

        // 4. Lead Agent Synthesizes the Final Deliverable
        let lead_prompt = format!(
            "Synthesize the following expert reports into a final cohesive business plan:\n{}",
            combined_summaries
        );
        let final_result = self.config.lead_agent.run(run_config, &lead_prompt, &mut on_event).await?;

        // Pre-deliver Quality Gate
        let word_count = final_result.split_whitespace().count();
        if word_count < self.config.min_words_pre_deliver {
            return Err(format!(
                "Pre-deliver gate failed: Deliverable has {} words, but requires >= {}.",
                word_count, self.config.min_words_pre_deliver
            ).into());
        }

        Ok(final_result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::llm::LlmClient;
    use ohc_builtin_agent_core::types::{ChatRequest, ChatResponse, Message, Role, Usage};
    use crate::agent::Agent;
    use std::sync::Mutex;
    use async_trait::async_trait;

    struct MockLlmClient {
        responses: Mutex<Vec<String>>,
    }

    impl MockLlmClient {
        fn new(responses: Vec<&str>) -> Self {
            Self {
                responses: Mutex::new(responses.into_iter().map(String::from).collect()),
            }
        }
    }

    #[async_trait]
    impl LlmClient for MockLlmClient {
        async fn chat(&self, _req: ChatRequest) -> Result<ChatResponse, Box<dyn std::error::Error + Send + Sync>> {
            let mut resps = self.responses.lock().unwrap();
            let content = if !resps.is_empty() {
                resps.remove(0)
            } else {
                "Default mock response".to_string()
            };

            Ok(ChatResponse {
                message: Message {
                    role: Role::Assistant,
                    content,
                    tool_calls: vec![],
                    tool_results: vec![],
                    response_id: Some("mock".to_string()),
                    previous_response_id: None,
                },
                usage: Usage::default(),
                stop_reason: "stop".to_string(),
                response_id: Some("mock".to_string()),
            })
        }
    }

    fn create_mock_agent(responses: Vec<&str>) -> Arc<Agent> {
        let llm = Arc::new(MockLlmClient::new(responses));
        let agent = Agent::new(llm, vec![]);
        // Ensure the agent can run by having some tools or valid state
        Arc::new(agent)
    }

    #[tokio::test]
    async fn test_expert_team_success() {
        let lead_agent = create_mock_agent(vec![
            "Synthesized final plan with many words. ".repeat(100).as_str() // ensure it passes the word count
        ]);
        let expert1 = create_mock_agent(vec!["Expert 1 findings"]);
        let expert2 = create_mock_agent(vec!["Expert 2 findings"]);
        let qc_agent = create_mock_agent(vec!["APPROVE"]);

        let team = ExpertTeam::new(ExpertTeamConfig {
            lead_agent,
            domain_experts: vec![
                ("Industry Researcher".to_string(), expert1),
                ("Financial Analyst".to_string(), expert2),
            ],
            quality_auditor: qc_agent,
            min_words_pre_deliver: 50,
        });

        let config = AgentRunConfig::default();
        let result = team.execute_project("Build a business plan for AI startup", &config).await;

        assert!(result.is_ok(), "Expert team should succeed");
    }

    #[tokio::test]
    async fn test_pre_flight_gate_failure() {
        let lead_agent = create_mock_agent(vec![]);
        let qc_agent = create_mock_agent(vec![]);

        let team = ExpertTeam::new(ExpertTeamConfig {
            lead_agent,
            domain_experts: vec![], // No experts
            quality_auditor: qc_agent,
            min_words_pre_deliver: 10,
        });

        let config = AgentRunConfig::default();
        let result = team.execute_project("Task", &config).await;

        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Pre-flight gate failed"));
    }

    #[tokio::test]
    async fn test_quality_control_rejection() {
        let lead_agent = create_mock_agent(vec![]);
        let expert1 = create_mock_agent(vec!["Poor findings"]);
        let qc_agent = create_mock_agent(vec!["REJECT: insufficient data"]);

        let team = ExpertTeam::new(ExpertTeamConfig {
            lead_agent,
            domain_experts: vec![("Expert".to_string(), expert1)],
            quality_auditor: qc_agent,
            min_words_pre_deliver: 10,
        });

        let config = AgentRunConfig::default();
        let result = team.execute_project("Task", &config).await;

        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Quality Control rejected"));
    }

    #[tokio::test]
    async fn test_pre_deliver_gate_failure() {
        // Output too short to pass word count
        let lead_agent = create_mock_agent(vec!["Too short"]);
        let expert1 = create_mock_agent(vec!["Findings"]);
        let qc_agent = create_mock_agent(vec!["APPROVE"]);

        let team = ExpertTeam::new(ExpertTeamConfig {
            lead_agent,
            domain_experts: vec![("Expert".to_string(), expert1)],
            quality_auditor: qc_agent,
            min_words_pre_deliver: 100, // Requires 100 words
        });

        let config = AgentRunConfig::default();
        let result = team.execute_project("Task", &config).await;

        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Pre-deliver gate failed"));
    }
}
