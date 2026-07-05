#![allow(clippy::needless_borrow)]
use crate::types::{ChatRequest, Message};


/// Tencent Workbuddy (Expert Team) Feature
/// Product: Tencent Cloud Intelligent Agent Platform
/// Reference impl (BP-Factory): 6-person specialized agent team for business plans
/// Lead agent (Project Director) + Domain experts (Industry Researcher, Financial Analyst, Strategic Analyst) + Quality control (Process Supervisor, Quality Auditor)

#[async_trait::async_trait]
pub trait ExpertTeamLlmClient: Send + Sync {
    async fn chat(
        &self,
        req: ChatRequest,
    ) -> Result<crate::types::ChatResponse, Box<dyn std::error::Error + Send + Sync>>;
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

    pub fn has_required_skills(&self, expected_roles: &[String]) -> bool {
        // As part of the pre-deliver gate, we require at least some minimum trace of skill usage from all required roles.
        if self.skills_used.is_empty() {
            return false;
        }
        for role in expected_roles {
            let skill_name = format!("{}_analysis", role.to_lowercase().replace(" ", "_"));
            if !self.skills_used.contains(&skill_name) {
                return false;
            }
        }
        true
    }
}


pub struct ProjectDirector<T: ExpertTeamLlmClient + ?Sized> {
    pub name: String,
    pub llm: std::sync::Arc<T>,
}

impl<T: ExpertTeamLlmClient + 'static> ProjectDirector<T> {
    pub async fn coordinate(&self, task: &str, experts: Vec<DomainExpert<T>>, trace: &mut SkillTrace) -> Result<String, String> {
        let manager = ExpertTeamManager::new(&self.name, experts);
        manager.run_full_expert_workflow(task, trace, self.llm.clone()).await
    }
}

pub struct IndustryResearcher<T: ExpertTeamLlmClient + ?Sized> {
    pub role: String,
    pub llm: std::sync::Arc<T>,
}

impl<T: ExpertTeamLlmClient + ?Sized> IndustryResearcher<T> {
    pub fn into_expert(self) -> DomainExpert<T> {
        DomainExpert { role: self.role, llm: self.llm }
    }
}

pub struct FinancialAnalyst<T: ExpertTeamLlmClient + ?Sized> {
    pub role: String,
    pub llm: std::sync::Arc<T>,
}

impl<T: ExpertTeamLlmClient + ?Sized> FinancialAnalyst<T> {
    pub fn into_expert(self) -> DomainExpert<T> {
        DomainExpert { role: self.role, llm: self.llm }
    }
}

pub struct StrategicAnalyst<T: ExpertTeamLlmClient + ?Sized> {
    pub role: String,
    pub llm: std::sync::Arc<T>,
}

impl<T: ExpertTeamLlmClient + ?Sized> StrategicAnalyst<T> {
    pub fn into_expert(self) -> DomainExpert<T> {
        DomainExpert { role: self.role, llm: self.llm }
    }
}

pub struct ProcessSupervisor<T: ExpertTeamLlmClient + ?Sized> {
    pub role: String,
    pub llm: std::sync::Arc<T>,
}

impl<T: ExpertTeamLlmClient + ?Sized> ProcessSupervisor<T> {
    pub fn into_expert(self) -> DomainExpert<T> {
        DomainExpert { role: self.role, llm: self.llm }
    }
}

pub struct QualityAuditor<T: ExpertTeamLlmClient + ?Sized> {
    pub role: String,
    pub llm: std::sync::Arc<T>,
}

impl<T: ExpertTeamLlmClient + ?Sized> QualityAuditor<T> {
    pub fn into_expert(self) -> DomainExpert<T> {
        DomainExpert { role: self.role, llm: self.llm }
    }
}

pub struct DomainExpert<T: ExpertTeamLlmClient + ?Sized> {
    pub role: String,
    pub llm: std::sync::Arc<T>,
}

impl<T: ExpertTeamLlmClient + ?Sized> DomainExpert<T> {
    pub async fn execute(&self, task: &str, trace: &mut SkillTrace) -> Result<String, String> {
        // Track the skill usage
        trace.record_skill(&format!(
            "{}_analysis",
            self.role.to_lowercase().replace(" ", "_")
        ));

        let system_prompt = format!(
            "You are an expert in {}. Provide a detailed analysis based on the user's task.",
            self.role
        );

        let req = ChatRequest {
            model: "default".to_string(),
            system: ::server_pricing::compression::reduce_tokens(&system_prompt),
            messages: vec![Message::user(task)],
            tools: vec![],
            max_tokens: 4000,
            temperature: 0.2,
        };

        match self.llm.chat(req).await {
            Ok(resp) => {
                let text = resp.message.content;
                tracing::info!("Domain expert {} completed successfully", self.role);
                Ok(text)
            }
            Err(e) => {
                tracing::error!("Domain expert {} LLM Error: {}", self.role, e);
                Err(format!("LLM Error: {}", e))
            }
        }
    }
}

pub struct ExpertTeamManager<T: ExpertTeamLlmClient + ?Sized> {
    pub lead_agent_name: String,
    pub domain_experts: Vec<DomainExpert<T>>,
}

impl<T: ExpertTeamLlmClient + ?Sized + 'static> ExpertTeamManager<T> {
    pub fn new(lead: &str, experts: Vec<DomainExpert<T>>) -> Self {
        Self {
            lead_agent_name: lead.to_string(),
            domain_experts: experts,
        }
    }

    /// Execute the full expert workflow applying all code-enforced quality gates.
    pub async fn run_full_expert_workflow(
        &self,
        task: &str,
        trace: &mut SkillTrace,
        lead_llm: std::sync::Arc<dyn ExpertTeamLlmClient>,
    ) -> Result<String, String> {
        tracing::info!("Starting full expert workflow for task: {}", task);
        // 1. Pre-flight Gate
        QualityGates::pre_flight(self, task)?;
        tracing::info!("Pre-flight gate passed");

        // 2. Parallel Execution
        let summaries = self.execute_parallel_tasks(task, trace).await?;
        tracing::info!("Parallel tasks completed with {} summaries", summaries.len());

        // 3. Pre-merge Gate
        QualityGates::pre_merge(&summaries)?;
        tracing::info!("Pre-merge gate passed");

        // 4. Synthesis by Lead LLM
        let combined = summaries.join("\n");
        let synthesize_prompt = format!(
            "Synthesize the following expert summaries into a final report of at least 20000 words. Include any required charts or analysis. Task: {}\nSummaries:\n{}",
            task, combined
        );

        use crate::types::{ChatRequest, Message};
        let req = ChatRequest {
            model: "lead-model".to_string(),
            system: "You are the Project Director. Synthesize the expert reports.".to_string(),
            messages: vec![Message::user(synthesize_prompt)],
            tools: vec![],
            max_tokens: 4000,
            temperature: 0.2,
        };

        let synth_res = lead_llm
            .chat(req)
            .await
            .map_err(|e| format!("Lead LLM failed: {}", e))?;
        let final_output = synth_res.message.content;

        // 5. Pre-deliver Gate
        let expected_roles: Vec<String> = self.domain_experts.iter().map(|e| e.role.clone()).collect();
        QualityGates::pre_deliver(&final_output, trace, &expected_roles)?;
        tracing::info!("Pre-deliver gate passed. Workflow complete.");

        Ok(final_output)
    }

    /// Execute the parallel tasks with the team of domain experts.
    /// Incorporates concurrent execution and the "condensed summaries" rule.
    pub async fn execute_parallel_tasks<'a>(
        &self,
        task: &'a str,
        trace: &'a mut SkillTrace,
    ) -> Result<Vec<String>, String> where T: 'a {
        // Prepare futures for parallel execution
        let mut join_handles = Vec::new();

        // We use true parallel execution via tokio::spawn.
        // Since `execute` mutates `trace`, we will return the skills from each expert
        // and merge them back in the main thread.

        for expert in &self.domain_experts {
            let role_name = expert.role.clone();
            let llm_clone = expert.llm.clone();
            let task_clone = task.to_string();

            let handle = tokio::spawn(async move {
                let mut local_trace = SkillTrace::new();
                let expert_instance = DomainExpert {
                    role: role_name,
                    llm: llm_clone,
                };

                let mut retries = 3;
                let mut last_err = String::new();

                while retries > 0 {
                    match expert_instance.execute(&task_clone, &mut local_trace).await {
                        Ok(res) => return (Ok(res), local_trace.skills_used),
                        Err(e) => {
                            last_err = e;
                            retries -= 1;
                            if retries > 0 {
                                tokio::time::sleep(std::time::Duration::from_millis(100)).await;
                            }
                        }
                    }
                }
                (Err(last_err), local_trace.skills_used)
            });
            join_handles.push(handle);
        }

        let mut results = Vec::new();
        for handle in join_handles {
            match handle.await {
                Ok(res) => results.push(res),
                Err(e) => results.push((Err(format!("Task panicked or cancelled: {:?}", e)), Vec::new())),
            }
        }

        let mut condensed_summaries = Vec::new();

        for (output_res, skills) in results {
            match output_res {
                Ok(output) => {
                    // Merge skills back
                    for skill in &skills {
                        trace.record_skill(skill);
                    }

                    // Enforce "condensed summaries" rule (Industry Standard):
                    // Subagents return condensed summaries (1k-2k tokens), never full context.
                    // tokens ~= characters / 4. 1500 tokens ~= 6000 chars.
                    let max_chars = 6000;
                    let output_str: &str = output.as_ref();
                    let char_count = output_str.chars().count();
                    let condensed = if char_count > max_chars {
                        let truncated: String = output_str.chars().take(max_chars).collect();
                        format!("{}... [Condensed for Harness: 1k-2k tokens limit reached]", truncated)
                    } else {
                        output_str.to_string()
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
    /// Ensures there are exactly 6 agents initialization.
    pub fn pre_flight<T: ExpertTeamLlmClient + ?Sized>(
        manager: &ExpertTeamManager<T>,
        task: &str,
    ) -> Result<(), String> {
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

        // 75% similarity deduplication using Jaccard index on word tokens
        // Normalize strings to be case-insensitive and ignore punctuation.
        let mut token_sets = Vec::with_capacity(summaries.len());
        let mut normalized_summaries = Vec::with_capacity(summaries.len());

        for summary in summaries {
            let normalized: String = summary
                .chars()
                .filter(|c| c.is_alphanumeric() || c.is_whitespace())
                .flat_map(|c| c.to_lowercase())
                .collect();
            normalized_summaries.push(normalized);
        }

        for normalized in &normalized_summaries {
            let set: std::collections::HashSet<&str> = normalized.split_whitespace().collect();
            token_sets.push(set);
        }

        for i in 0..token_sets.len() {
            for j in (i + 1)..token_sets.len() {
                let set_i = &token_sets[i];
                let set_j = &token_sets[j];

                let intersection = set_i.intersection(set_j).count() as f64;
                let union = set_i.union(set_j).count() as f64;

                if union > 0.0 {
                    let jaccard_similarity = intersection / union;
                    if jaccard_similarity > 0.75 {
                        return Err("Pre-merge Gate Failed: High similarity detected (>75%) between expert outputs. Deduplication required.".to_string());
                    }
                }
            }
        }

        // 8-chapter completeness check
        let combined = summaries.join("\n");
        let required_chapters = [
            "Chapter 1",
            "Chapter 2",
            "Chapter 3",
            "Chapter 4",
            "Chapter 5",
            "Chapter 6",
            "Chapter 7",
            "Chapter 8",
        ];

        let mut missing_chapters = Vec::new();
        for &chapter in &required_chapters {
            if !combined.contains(chapter) {
                missing_chapters.push(chapter);
            }
        }

        if !missing_chapters.is_empty() {
            return Err(format!(
                "Pre-merge Gate Failed: Outputs do not meet the minimum completeness criteria. Missing: {}.",
                missing_chapters.join(", ")
            ));
        }

        Ok(())
    }

    /// Pre-deliver (e.g., >=20,000 words, chart verification, skill-trace completeness).
    pub fn pre_deliver(final_output: &str, trace: &SkillTrace, expected_roles: &[String]) -> Result<(), String> {
        // Skill-trace tracking to prevent hard-coded bypasses.
        if !trace.has_required_skills(expected_roles) {
            return Err("Pre-deliver Gate Failed (Skill-trace tracking): Skill-trace is incomplete. Execution must be generated by experts, not hard-coded bypasses.".to_string());
        }

        // Enforce word count to exactly match the requirement (>= 20,000 words)
        // Pattern Requirement: Pre-deliver (>=20,000 words, chart verification)
        let word_count = final_output.split_whitespace().count();
        if word_count < 20000 {
            return Err(format!(
                "Pre-deliver Gate Failed (Expert Team Pattern): Final output is too short ({} words). Required >= 20000 words for delivery.",
                word_count
            ));
        }

        // Chart verification (simulated by checking if the output contains "Chart" or similar keywords, if required by context)
        if !final_output.contains("Chart:") && !final_output.contains("Analysis:") && !final_output.contains("Graph:") {
            return Err("Pre-deliver Gate Failed (Expert Team Pattern): Missing required chart/analysis/graph verification in final output.".to_string());
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;

    use crate::types::{ChatResponse, Usage};

    struct MockExpertLlm {
        role_resp: String,
    }

    #[async_trait::async_trait]
    impl ExpertTeamLlmClient for MockExpertLlm {
        async fn chat(
            &self,
            _req: ChatRequest,
        ) -> Result<ChatResponse, Box<dyn std::error::Error + Send + Sync>> {
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
            DomainExpert { role: "Industry Researcher".to_string(), llm: Arc::new(MockExpertLlm { role_resp: "Research summary... Chapter 1 and Chapter 2 unique words alpha beta".to_string() }) },
            DomainExpert { role: "Financial Analyst".to_string(), llm: Arc::new(MockExpertLlm { role_resp: "Financial summary... Chapter 3 and Chapter 4 distinct terms gamma delta".to_string() }) },
            DomainExpert { role: "Strategic Analyst".to_string(), llm: Arc::new(MockExpertLlm { role_resp: "Strategic summary... Chapter 5 and Chapter 6 different phrasing epsilon zeta".to_string() }) },
            DomainExpert { role: "Process Supervisor".to_string(), llm: Arc::new(MockExpertLlm { role_resp: "Process summary... Chapter 7 some original ideas eta theta".to_string() }) },
            DomainExpert { role: "Quality Auditor".to_string(), llm: Arc::new(MockExpertLlm { role_resp: "Quality summary... Chapter 8 finalizing the text iota kappa".to_string() }) },
        ];
        let manager = ExpertTeamManager::new("Project Director", experts);

        let task = "Analyze the new AI market trends for the upcoming quarter. Chart: Required. Analysis: Deep.";
        let mut trace = SkillTrace::new();
        let word_padding = "word ".repeat(20000);

        let lead_llm = Arc::new(MockExpertLlm {
            role_resp: format!(
                "Combined Executive Summary:\n\nOverall Strategy:\nProceed with investment.\nWe include the Chart: Market Trends. {}",
                word_padding
            ),
        });

        let result = manager
            .run_full_expert_workflow(task, &mut trace, lead_llm)
            .await;
        assert!(result.is_ok(), "Expert workflow failed: {:?}", result.err());
    }


    #[tokio::test]
    async fn test_expert_team_parallel_execution() {
        let director = ProjectDirector { name: "Project Director".to_string(), llm: Arc::new(MockExpertLlm { role_resp: format!("{} Chart: Required. Analysis: Deep.", "word ".repeat(20000)) }) };
        let experts = vec![
            IndustryResearcher { role: "Industry Researcher".to_string(), llm: Arc::new(MockExpertLlm { role_resp: "Research summary Chapter 1 Chapter 2".to_string() }) }.into_expert(),
            FinancialAnalyst { role: "Financial Analyst".to_string(), llm: Arc::new(MockExpertLlm { role_resp: "Financial summary Chapter 3 Chapter 4".to_string() }) }.into_expert(),
            StrategicAnalyst { role: "Strategic Analyst".to_string(), llm: Arc::new(MockExpertLlm { role_resp: "Strategic summary Chapter 5 Chapter 6".to_string() }) }.into_expert(),
            ProcessSupervisor { role: "Process Supervisor".to_string(), llm: Arc::new(MockExpertLlm { role_resp: "Process summary Chapter 7".to_string() }) }.into_expert(),
            QualityAuditor { role: "Quality Auditor".to_string(), llm: Arc::new(MockExpertLlm { role_resp: "Quality summary Chapter 8 Chart: Required. Analysis: Deep.".to_string() }) }.into_expert(),
        ];

        let task = "Test parallel execution Chart: Required. Analysis: Deep.";
        let mut trace = SkillTrace::new();

        let summary = director.coordinate(task, experts, &mut trace).await.unwrap();
        assert!(trace.skills_used.contains(&"industry_researcher_analysis".to_string()));
        assert!(trace.skills_used.contains(&"financial_analyst_analysis".to_string()));
    }

    #[test]

    fn test_pre_flight_failure_not_enough_experts() {
        let experts = vec![DomainExpert {
            role: "Lone Wolf".to_string(),
            llm: Arc::new(MockExpertLlm {
                role_resp: "".to_string(),
            }),
        }];
        let manager = ExpertTeamManager::new("Lead", experts);
        let res = QualityGates::pre_flight(&manager, "Task");
        assert!(res.is_err());
        assert!(matches!(res, Err(e) if e.contains("Exactly 6 agent initialization is required")));
    }

    #[test]
    fn test_pre_merge_failure_high_similarity() {
        let summaries = vec![
            "Identical output about market analysis Chapter 1 Chapter 2 Chapter 3 Chapter 4 Chapter 5 Chapter 6 Chapter 7 Chapter 8".to_string(),
            "Identical output about market analysis Chapter 1 Chapter 2 Chapter 3 Chapter 4 Chapter 5 Chapter 6 Chapter 7 Chapter 8".to_string(),
        ];
        let res = QualityGates::pre_merge(&summaries);
        assert!(res.is_err());
        assert!(matches!(res, Err(e) if e.contains("High similarity detected")));
    }

    #[test]
    fn test_pre_merge_success_low_similarity() {
        let summaries = vec![
            "This is a completely unique summary about AI trends for Chapter 1 and Chapter 2.".to_string(),
            "And here we have financial data talking about profits for Chapter 3 and Chapter 4.".to_string(),
            "Strategic overview with different insights for Chapter 5 and Chapter 6.".to_string(),
            "Process analysis detailing operational changes for Chapter 7 and Chapter 8.".to_string(),
        ];
        let res = QualityGates::pre_merge(&summaries);
        assert!(res.is_ok(), "Expected success but got error: {:?}", res.err());
    }

    #[test]
    fn test_pre_merge_failure_high_similarity_casing() {
        let summaries = vec![
            "IDENTICAL output about market analysis! Chapter 1, Chapter 2, Chapter 3, Chapter 4, Chapter 5, Chapter 6, Chapter 7, Chapter 8.".to_string(),
            "identical output about market analysis chapter 1 chapter 2 chapter 3 chapter 4 chapter 5 chapter 6 chapter 7 chapter 8".to_string(),
        ];
        let res = QualityGates::pre_merge(&summaries);
        assert!(res.is_err());
        assert!(matches!(res, Err(e) if e.contains("High similarity detected")));
    }

    #[test]
    fn test_pre_merge_failure_missing_chapters() {
        let summaries = vec![
            "Research summary... Chapter 1 and Chapter 2 unique words alpha beta".to_string(),
            "Financial summary... Chapter 3 and Chapter 4 distinct terms gamma delta".to_string(),
        ];
        let res = QualityGates::pre_merge(&summaries);
        assert!(res.is_err());
        assert!(
            matches!(res, Err(e) if e.contains("Missing: Chapter 5, Chapter 6, Chapter 7, Chapter 8"))
        );
    }

    #[test]
    fn test_pre_deliver_failure_missing_chart() {
        let final_output = "word ".repeat(20000)
            + "This output is quite long so it passes the word count check. It is very detailed and thorough, however it is missing something important.";
        let mut trace = SkillTrace::new();
        trace.record_skill("test_skill_analysis");
        let expected_roles = vec!["test skill".to_string()];
        let res = QualityGates::pre_deliver(&final_output, &trace, &expected_roles);
        assert!(res.is_err());
        assert!(
            matches!(res, Err(e) if e.contains("Missing required chart/analysis/graph verification"))
        );
    }

    #[test]
    fn test_pre_deliver_failure_missing_skill_trace() {
        let final_output = "This output is quite long so it passes the word count check. Chart: Provided. Analysis: Provided.";
        let trace = SkillTrace::new(); // Empty trace
        let expected_roles = vec!["test skill".to_string()];
        let res = QualityGates::pre_deliver(&final_output, &trace, &expected_roles);
        assert!(res.is_err());
        assert!(matches!(res, Err(e) if e.contains("Skill-trace is incomplete")));
    }
}
