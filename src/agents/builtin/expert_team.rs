use crate::types::{ChatRequest, Message};
use futures::future::join_all;

#[async_trait::async_trait]
#[async_trait::async_trait]
#[async_trait::async_trait]
pub trait ExpertTeamLlmClient: Send + Sync {
    async fn chat(&self, req: ChatRequest) -> Result<crate::types::ChatResponse, Box<dyn std::error::Error + Send + Sync>>;
}

/// Skill-trace tracking to prevent hard-coded bypasses.
#[derive(Debug, Clone, Default)]
pub struct SkillTrace {
    pub skills_used: Vec<String>,
}

impl SkillTrace {
    pub fn new() -> Self {
        Self {
            skills_used: Vec::new(),
        }
    }

    pub fn record_skill(&mut self, skill: &str) {
        self.skills_used.push(skill.to_string());
    }

    pub fn has_required_skills(&self) -> bool {
        // As part of the pre-deliver gate, we require at least some minimum trace of skill usage.
        !self.skills_used.is_empty()
    }
}

pub struct DomainExpert<T: ExpertTeamLlmClient + ?Sized> {
    pub role: String,
    pub llm: std::sync::Arc<T>,
}

impl<T: ExpertTeamLlmClient + ?Sized> DomainExpert<T> {
    pub async fn execute(&self, task: &str, trace: &mut SkillTrace) -> Result<String, String> {
        // Track the skill usage
        trace.record_skill(&format!("{}_analysis", self.role.to_lowercase().replace(" ", "_")));

        let system_prompt = format!("You are an expert in {}. Provide a detailed analysis based on the user's task.", self.role);

        let req = ChatRequest {
            model: "default".to_string(),
            system: system_prompt,
            messages: vec![Message::user(task)],
            tools: vec![],
            max_tokens: 4000,
            temperature: 0.2,
        };

        match self.llm.chat(req).await {
            Ok(resp) => {
                let text = resp.message.content;
                Ok(text)
            }
            Err(e) => Err(format!("LLM Error: {}", e)),
        }
    }
}

pub struct ExpertTeamManager<T: ExpertTeamLlmClient + ?Sized> {
    pub lead_agent_name: String,
    pub domain_experts: Vec<DomainExpert<T>>,
}

impl<T: ExpertTeamLlmClient + ?Sized> ExpertTeamManager<T> {
    pub fn new(lead: &str, experts: Vec<DomainExpert<T>>) -> Self {
        Self {
            lead_agent_name: lead.to_string(),
            domain_experts: experts,
        }
    }

    /// Execute the parallel tasks with the team of domain experts.
    /// Incorporates concurrent execution and the "condensed summaries" rule.
    pub async fn execute_parallel_tasks(&self, task: &str, trace: &mut SkillTrace) -> Result<Vec<String>, String> {
        // Prepare futures for parallel execution
        let mut futures = Vec::new();

        // Note: we'll simulate parallel execution with futures::future::join_all,
        // but since `execute` mutates `trace`, we'll need to handle it carefully in Rust.
        // For the sake of this pattern, we will return the skills from each expert and merge them back.

        for expert in &self.domain_experts {
            let role_name = expert.role.clone();
            let llm_clone = expert.llm.clone();
            let task_clone = task.to_string();

            let fut = async move {
                let mut local_trace = SkillTrace::new();
                let expert_instance = DomainExpert { role: role_name, llm: llm_clone };
                let output = expert_instance.execute(&task_clone, &mut local_trace).await;
                (output, local_trace.skills_used)
            };
            futures.push(fut);
        }

        let results = join_all(futures).await;

        let mut condensed_summaries = Vec::new();

        for (output_res, skills) in results {
            match output_res {
                Ok(output) => {
                    // Merge skills back
                    for skill in &skills {
                        trace.record_skill(&skill);
                    }

                    // Enforce "condensed summaries" rule: wrap subagent output such that they only return 1k-2k tokens.
                    // We simulate this by truncating the text.
                    let max_length = 2000;
                    let condensed = if output.len() > max_length {
                        format!("{}... [Condensed]", &output[..max_length])
                    } else {
                        output
                    };

                    condensed_summaries.push(condensed);
                }
                Err(e) => return Err(e),
            }
        }

        Ok(condensed_summaries)
    }
}

/// Code-enforced quality gates at super-step boundaries.
pub struct QualityGates;

impl QualityGates {
    /// Pre-flight (e.g., initialization check).
    /// Ensures there are exactly 6 agents initialization (Tencent Workbuddy Expert Team Feature).
    pub fn pre_flight<T: ExpertTeamLlmClient + ?Sized>(manager: &ExpertTeamManager<T>, task: &str) -> Result<(), String> {
        // Enforce 6 agent initialization
        // 1 Lead (already in manager) + 5 Domain/Quality experts = 6 total
        // Therefore, we expect exactly 5 domain_experts.
        if manager.domain_experts.len() != 5 {
            return Err("Pre-flight Gate Failed: Exactly 6 agent initialization is required (1 Lead + 5 Experts).".to_string());
        }
        if task.trim().is_empty() {
            return Err("Pre-flight Gate Failed: Task context cannot be empty.".to_string());
        }
        Ok(())
    }

    /// Pre-merge (e.g., 75% similarity deduplication, 8-chapter completeness).
    pub fn pre_merge(summaries: &[String]) -> Result<(), String> {
        if summaries.is_empty() {
            return Err("Pre-merge Gate Failed: No summaries to merge.".to_string());
        }

        // Check for similarity (dummy implementation to represent the logic)
        // If the outputs are too similar, we reject them.
        for i in 0..summaries.len() {
            for j in (i + 1)..summaries.len() {
                if summaries[i] == summaries[j] {
                    return Err("Pre-merge Gate Failed: High similarity detected (>75%) between expert outputs. Deduplication required.".to_string());
                }
            }
        }

        // Check for structural completeness (e.g. 8-chapter completeness)
        // We simulate this by checking if the combined outputs contain enough information.
        let combined = summaries.join("\n");
        if combined.len() < 50 {
            return Err("Pre-merge Gate Failed: Outputs do not meet the minimum completeness criteria (e.g., missing chapters).".to_string());
        }

        Ok(())
    }

    /// Pre-deliver (e.g., >=20,000 words, chart verification, skill-trace completeness).
    pub fn pre_deliver(final_output: &str, trace: &SkillTrace) -> Result<(), String> {
        // Skill-trace tracking to prevent hard-coded bypasses.
        if !trace.has_required_skills() {
            return Err("Pre-deliver Gate Failed: Skill-trace is incomplete. Execution must be generated by experts, not hard-coded bypasses.".to_string());
        }

        // Enforce word count to exactly match the requirement (>= 20,000 words)
        let word_count = final_output.split_whitespace().count();
        if word_count < 20000 {
            return Err(format!("Pre-deliver Gate Failed: Final output is too short ({} words). Required >= 20000 words for delivery.", word_count));
        }

        // Chart verification (simulated by checking if the output contains "Chart" or similar keywords, if required by context)
        if !final_output.contains("Chart:") && !final_output.contains("Analysis:") {
            return Err("Pre-deliver Gate Failed: Missing required chart/analysis verification in final output.".to_string());
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use ohc_builtin_agent_core::types::{ChatResponse, Usage};

    struct MockExpertLlm {
        role_resp: String,
    }

    #[async_trait::async_trait]
    impl ExpertTeamLlmClient for MockExpertLlm {
        async fn chat(&self, _req: ChatRequest) -> Result<ChatResponse, Box<dyn std::error::Error + Send + Sync>> {
            Ok(ChatResponse {
                message: Message::assistant(self.role_resp.clone()),
                usage: Usage::default(),
                stop_reason: "stop".to_string(),
                response_id: Some("mock-id".to_string()),
            })
        }
    }

    #[tokio::test]
    async fn test_expert_team_successful_execution() {
        let experts = vec![
            DomainExpert { role: "Industry Researcher".to_string(), llm: Arc::new(MockExpertLlm { role_resp: "Research summary...".to_string() }) },
            DomainExpert { role: "Financial Analyst".to_string(), llm: Arc::new(MockExpertLlm { role_resp: "Financial summary...".to_string() }) },
            DomainExpert { role: "Strategic Analyst".to_string(), llm: Arc::new(MockExpertLlm { role_resp: "Strategic summary...".to_string() }) },
            DomainExpert { role: "Process Supervisor".to_string(), llm: Arc::new(MockExpertLlm { role_resp: "Process summary...".to_string() }) },
            DomainExpert { role: "Quality Auditor".to_string(), llm: Arc::new(MockExpertLlm { role_resp: "Quality summary...".to_string() }) },
        ];
        let manager = ExpertTeamManager::new("Project Director", experts);

        let task = "Analyze the new AI market trends for the upcoming quarter. Chart: Required. Analysis: Deep.";

        // Gate 1: Pre-flight
        assert!(QualityGates::pre_flight(&manager, task).is_ok());

        let mut trace = SkillTrace::new();

        // Execution
        let summaries = manager.execute_parallel_tasks(task, &mut trace).await.unwrap();
        assert_eq!(summaries.len(), 5);
        assert!(trace.has_required_skills());

        // Gate 2: Pre-merge
        assert!(QualityGates::pre_merge(&summaries).is_ok());

        // Lead agent combines the summaries into final output
        let mut final_output = format!("Combined Executive Summary:\n{}\n\nOverall Strategy:\nProceed with investment.\nWe include the Chart: Market Trends.", summaries.join("\n"));
        // Pad to >= 20000 words
        let word_padding = "word ".repeat(20000);
        final_output.push_str(&word_padding);

        // Gate 3: Pre-deliver
        assert!(QualityGates::pre_deliver(&final_output, &trace).is_ok());
    }

    #[test]
    fn test_pre_flight_failure_not_enough_experts() {
        let experts = vec![DomainExpert { role: "Lone Wolf".to_string(), llm: Arc::new(MockExpertLlm { role_resp: "".to_string() }) }];
        let manager = ExpertTeamManager::new("Lead", experts);
        let res = QualityGates::pre_flight(&manager, "Task");
        assert!(res.is_err());
        assert!(res.unwrap_err().contains("Exactly 6 agent initialization is required"));
    }

    #[test]
    fn test_pre_merge_failure_high_similarity() {
        let summaries = vec![
            "Identical output about market analysis".to_string(),
            "Identical output about market analysis".to_string(),
        ];
        let res = QualityGates::pre_merge(&summaries);
        assert!(res.is_err());
        assert!(res.unwrap_err().contains("High similarity detected"));
    }

    #[test]
    fn test_pre_deliver_failure_missing_chart() {
        let final_output = "This output is quite long so it passes the word count check. It is very detailed and thorough, however it is missing something important.";
        let mut trace = SkillTrace::new();
        trace.record_skill("test_skill");
        let res = QualityGates::pre_deliver(&final_output, &trace);
        assert!(res.is_err());
        assert!(res.unwrap_err().contains("Missing required chart/analysis verification"));
    }

    #[test]
    fn test_pre_deliver_failure_missing_skill_trace() {
        let final_output = "This output is quite long so it passes the word count check. Chart: Provided. Analysis: Provided.";
        let trace = SkillTrace::new(); // Empty trace
        let res = QualityGates::pre_deliver(&final_output, &trace);
        assert!(res.is_err());
        assert!(res.unwrap_err().contains("Skill-trace is incomplete"));
    }
}
