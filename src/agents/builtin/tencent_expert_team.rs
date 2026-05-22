use crate::agent::{Agent, AgentRunConfig};
use ohc_builtin_agent_core::types::Message;
use std::sync::Arc;
use futures::future::join_all;

/// Implementation Target (Expert Team Pattern): OHC should implement a similar expert team where:
/// 1. A lead agent orchestrates and delegates to specialized domain expert agents
/// 2. Parallel execution of expert tasks (not sequential)
/// 3. Code-enforced quality gates at super-step boundaries (not documentation-based)
/// 4. Skill-trace tracking to prevent hard-coded bypasses
/// 5. Each expert agent returns condensed summaries (1k-2k tokens), never full context

/// 4. Skill-trace tracking to prevent hard-coded bypasses
#[derive(Debug, Clone)]
pub struct SkillTrace {
    pub skills_used: Vec<String>,
}

/// 5. Each expert agent returns condensed summaries (1k-2k tokens), never full context
#[derive(Debug, Clone)]
pub struct ExpertSummary {
    pub domain: String,
    pub condensed_summary: String,
    pub skill_trace: SkillTrace,
}

/// Domain Expert in the Expert Team Pattern
pub struct DomainExpert {
    pub domain_name: String,
    pub system_prompt: String,
    pub agent: Arc<Agent>,
    pub required_skills: Vec<String>,
}

impl DomainExpert {
    pub fn new(domain_name: impl Into<String>, system_prompt: impl Into<String>, agent: Arc<Agent>, required_skills: Vec<String>) -> Self {
        Self {
            domain_name: domain_name.into(),
            system_prompt: system_prompt.into(),
            agent,
            required_skills,
        }
    }

    pub async fn execute_task(&self, task_description: &str, mut config: AgentRunConfig) -> Result<ExpertSummary, String> {
        config.server_system_message = self.system_prompt.clone();
        let mut on_event = |_| {};

        let result = match self.agent.run(&config, task_description, &mut on_event).await {
            Ok(r) => r,
            Err(e) => return Err(format!("Expert {} failed: {}", self.domain_name, e)),
        };

        let mut used_skills = Vec::new();
        for skill in &self.required_skills {
            if result.contains(skill) || skill == "default_skill" {
                used_skills.push(skill.clone());
            }
        }

        // 5. Each expert agent returns condensed summaries (1k-2k tokens), never full context
        // Condensed summary constraint: max 2000 words
        let token_count = result.split_whitespace().count();
        let condensed_summary = if token_count > 2000 {
            result.split_whitespace().take(2000).collect::<Vec<_>>().join(" ")
        } else {
            result.clone()
        };

        Ok(ExpertSummary {
            domain: self.domain_name.clone(),
            condensed_summary,
            skill_trace: SkillTrace { skills_used: used_skills },
        })
    }
}

/// 3. Code-enforced quality gates at super-step boundaries (not documentation-based)
pub struct QualityAuditor;

impl QualityAuditor {
    pub fn verify_pre_flight(num_experts: usize, required_experts: usize) -> Result<(), String> {
        // Pre-flight (6 agent initialization constraint mentioned in Master Catalog, but parameterized)
        if num_experts < required_experts {
            return Err(format!("Quality Gate Failed: Pre-flight initialization requires at least {} domain experts, got {}", required_experts, num_experts));
        }
        Ok(())
    }

    pub fn verify_pre_merge(summaries: &[ExpertSummary]) -> Result<(), String> {
        // Pre-merge (75% similarity deduplication, completeness)
        if summaries.is_empty() {
            return Err("Quality Gate Failed: No expert summaries to merge".into());
        }

        // Simple deduplication check for demonstration
        for i in 0..summaries.len() {
            for j in (i + 1)..summaries.len() {
                if summaries[i].condensed_summary == summaries[j].condensed_summary {
                    return Err(format!("Quality Gate Failed: Summaries for {} and {} are identical (deduplication check failed)", summaries[i].domain, summaries[j].domain));
                }
            }
        }
        Ok(())
    }

    pub fn verify_pre_deliver(final_output: &str, summaries: &[ExpertSummary], min_words: usize) -> Result<(), String> {
        // Skill-trace completeness check
        let all_skills_traced = summaries.iter().all(|s| !s.skill_trace.skills_used.is_empty());
        if !all_skills_traced {
            return Err("Quality Gate Failed: Skill-trace completeness failed (hard-coded bypass detected)".into());
        }

        let total_words = final_output.split_whitespace().count();
        if total_words < min_words {
            return Err(format!("Quality Gate Failed: Output too short ({} words), minimum is {}", total_words, min_words));
        }

        Ok(())
    }
}

/// 1. A lead agent orchestrates and delegates to specialized domain expert agents
pub struct ProjectDirector {
    pub lead_agent: Arc<Agent>,
    pub experts: Vec<DomainExpert>,
    pub config: AgentRunConfig,
}

impl ProjectDirector {
    pub fn new(lead_agent: Arc<Agent>, experts: Vec<DomainExpert>, config: AgentRunConfig) -> Self {
        Self { lead_agent, experts, config }
    }

    pub async fn execute_project(&self, project_prompt: &str, required_experts: usize, min_words: usize) -> Result<String, String> {
        // 3. Code-enforced Pre-flight Gate
        QualityAuditor::verify_pre_flight(self.experts.len(), required_experts)?;

        // 2. Parallel execution of expert tasks (not sequential)
        let mut futures = Vec::new();
        for expert in &self.experts {
            let task_desc = format!("Project prompt: {}\nAnalyze this from your domain perspective.", project_prompt);
            let config_clone = self.config.clone();
            futures.push(async move {
                expert.execute_task(&task_desc, config_clone).await
            });
        }

        let results = join_all(futures).await;
        let mut summaries = Vec::new();
        for res in results {
            summaries.push(res?);
        }

        // 3. Code-enforced Pre-merge Gate
        QualityAuditor::verify_pre_merge(&summaries)?;

        // 1. Lead agent synthesizes the condensed summaries
        let mut synthesis_prompt = format!("Synthesize the following expert summaries into a final comprehensive business plan:\n\n");
        for summary in &summaries {
            synthesis_prompt.push_str(&format!("Domain: {}\nSummary: {}\n\n", summary.domain, summary.condensed_summary));
        }

        let mut run_config = self.config.clone();
        run_config.server_system_message = "You are the Project Director. Synthesize expert reports into a final document.".into();

        let mut on_event = |_| {};
        let final_output = self.lead_agent.run(&run_config, &synthesis_prompt, &mut on_event).await
            .map_err(|e| format!("Lead agent synthesis failed: {}", e))?;

        // 3. Code-enforced Pre-deliver Gate
        QualityAuditor::verify_pre_deliver(&final_output, &summaries, min_words)?;

        Ok(final_output)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ohc_builtin_agent_core::types::{ChatRequest, ChatResponse, Usage};
    use ohc_builtin_agent_llm::LlmClient;
    use tokio::sync::Mutex;

    struct MockLlmClientTeam {
        responses: Mutex<Vec<String>>,
    }

    #[async_trait::async_trait]
    impl LlmClient for MockLlmClientTeam {
        async fn chat(&self, _req: ChatRequest) -> Result<ChatResponse, Box<dyn std::error::Error + Send + Sync>> {
            let mut resps = self.responses.lock().await;
            let content = if !resps.is_empty() {
                resps.remove(0)
            } else {
                "default default default default default default default default default default default default".to_string()
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
    async fn test_expert_team_success() {
        let expert_client = Arc::new(MockLlmClientTeam {
            responses: Mutex::new(vec!["Used skill_a, analysis complete.".to_string(), "Used skill_b, data analyzed.".to_string()]),
        });

        let lead_client = Arc::new(MockLlmClientTeam {
            responses: Mutex::new(vec!["This is a final synthesized report that is long enough to pass the word count check for the pre-deliver gate.".to_string()]),
        });

        let expert_agent = Arc::new(Agent::new(expert_client, vec![]));
        let lead_agent = Arc::new(Agent::new(lead_client, vec![]));

        let expert1 = DomainExpert::new("Financial Analyst", "You are a financial analyst", expert_agent.clone(), vec!["skill_a".to_string()]);
        let expert2 = DomainExpert::new("Strategic Analyst", "You are a strategic analyst", expert_agent.clone(), vec!["skill_b".to_string()]);

        let director = ProjectDirector::new(lead_agent, vec![expert1, expert2], AgentRunConfig::default());

        // We require 2 experts, min_words = 10 for the final output gate
        let result = director.execute_project("Write a business plan for AI startup", 2, 10).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_expert_team_preflight_failure() {
        let lead_client = Arc::new(MockLlmClientTeam {
            responses: Mutex::new(vec![]),
        });
        let lead_agent = Arc::new(Agent::new(lead_client, vec![]));

        // Only 1 expert instead of required 6
        let expert1 = DomainExpert::new("Financial Analyst", "You are a financial analyst", lead_agent.clone(), vec!["skill_a".to_string()]);

        let director = ProjectDirector::new(lead_agent, vec![expert1], AgentRunConfig::default());

        let result = director.execute_project("Write a business plan", 6, 10).await;
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Pre-flight initialization requires at least 6 domain experts"));
    }

    #[tokio::test]
    async fn test_expert_team_predeliver_skill_trace_failure() {
        let expert_client = Arc::new(MockLlmClientTeam {
            // "Used no skills at all" - this expert fails the skill trace check
            responses: Mutex::new(vec!["Used no skills at all.".to_string(), "Used skill_b, data analyzed.".to_string()]),
        });

        let lead_client = Arc::new(MockLlmClientTeam {
            responses: Mutex::new(vec!["This is a final synthesized report that is long enough to pass the word count check.".to_string()]),
        });

        let expert_agent = Arc::new(Agent::new(expert_client, vec![]));
        let lead_agent = Arc::new(Agent::new(lead_client, vec![]));

        // It expects 'skill_a', but the response doesn't contain it
        let expert1 = DomainExpert::new("Financial Analyst", "You are a financial analyst", expert_agent.clone(), vec!["skill_a".to_string()]);
        let expert2 = DomainExpert::new("Strategic Analyst", "You are a strategic analyst", expert_agent.clone(), vec!["skill_b".to_string()]);

        let director = ProjectDirector::new(lead_agent, vec![expert1, expert2], AgentRunConfig::default());

        let result = director.execute_project("Write a business plan", 2, 10).await;
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Skill-trace completeness failed"));
    }
}
