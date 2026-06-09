use ohc_builtin_agent_core::expert_team::{
    DomainExpert, ExpertTeamLlmClient, ExpertTeamManager, SkillTrace,
};
use ohc_builtin_agent_core::types::{ChatRequest, ChatResponse, Message, Usage};
use std::sync::Arc;

struct MockSotaLlm {
    role_resp: String,
}

#[async_trait::async_trait]
impl ExpertTeamLlmClient for MockSotaLlm {
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
async fn test_expert_team_sota_quality_gates() {
    let experts = vec![
        DomainExpert {
            role: "Industry Researcher".to_string(),
            llm: Arc::new(MockSotaLlm {
                role_resp: "Chapter 1 Chapter 2 unique research".to_string(),
            }),
        },
        DomainExpert {
            role: "Financial Analyst".to_string(),
            llm: Arc::new(MockSotaLlm {
                role_resp: "Chapter 3 Chapter 4 unique finance".to_string(),
            }),
        },
        DomainExpert {
            role: "Strategic Analyst".to_string(),
            llm: Arc::new(MockSotaLlm {
                role_resp: "Chapter 5 Chapter 6 unique strategy".to_string(),
            }),
        },
        DomainExpert {
            role: "Process Supervisor".to_string(),
            llm: Arc::new(MockSotaLlm {
                role_resp: "Chapter 7 unique process".to_string(),
            }),
        },
        DomainExpert {
            role: "Quality Auditor".to_string(),
            llm: Arc::new(MockSotaLlm {
                role_resp: "Chapter 8 unique quality".to_string(),
            }),
        },
    ];
    let manager = ExpertTeamManager::new("Project Director", experts);

    let task = "Analyze the new AI market trends. Chart: Required.";
    let mut trace = SkillTrace::new();

    // Test Pre-deliver Word Count failure (less than 20,000 words)
    let lead_llm_short = Arc::new(MockSotaLlm {
        role_resp: "Too short. Chart: Included. Analysis: Deep.".to_string(),
    });

    let result = manager
        .run_full_expert_workflow(task, &mut trace, lead_llm_short)
        .await;
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("Final output is too short"));

    // Test Pre-deliver Chart failure
    let word_padding = "word ".repeat(20000);
    let lead_llm_no_chart = Arc::new(MockSotaLlm {
        role_resp: format!("Thorough report but no chart. {}", word_padding),
    });

    let result = manager
        .run_full_expert_workflow(task, &mut trace, lead_llm_no_chart)
        .await;
    assert!(result.is_err());
    assert!(
        result
            .unwrap_err()
            .contains("Missing required chart/analysis/graph verification")
    );

    // Test Pre-merge Similarity failure
    let duplicate_experts = vec![
        DomainExpert { role: "Expert 1".to_string(), llm: Arc::new(MockSotaLlm { role_resp: "Chapter 1 Chapter 2 Chapter 3 Chapter 4 Chapter 5 Chapter 6 Chapter 7 Chapter 8 Same".to_string() }) },
        DomainExpert { role: "Expert 2".to_string(), llm: Arc::new(MockSotaLlm { role_resp: "Chapter 1 Chapter 2 Chapter 3 Chapter 4 Chapter 5 Chapter 6 Chapter 7 Chapter 8 Same".to_string() }) },
        DomainExpert { role: "Expert 3".to_string(), llm: Arc::new(MockSotaLlm { role_resp: "Chapter 1 Chapter 2 Chapter 3 Chapter 4 Chapter 5 Chapter 6 Chapter 7 Chapter 8 Same".to_string() }) },
        DomainExpert { role: "Expert 4".to_string(), llm: Arc::new(MockSotaLlm { role_resp: "Chapter 1 Chapter 2 Chapter 3 Chapter 4 Chapter 5 Chapter 6 Chapter 7 Chapter 8 Same".to_string() }) },
        DomainExpert { role: "Expert 5".to_string(), llm: Arc::new(MockSotaLlm { role_resp: "Chapter 1 Chapter 2 Chapter 3 Chapter 4 Chapter 5 Chapter 6 Chapter 7 Chapter 8 Same".to_string() }) },
    ];
    let manager_dup = ExpertTeamManager::new("Lead", duplicate_experts);
    let mut trace_dup = SkillTrace::new();
    let lead_llm_ok = Arc::new(MockSotaLlm {
        role_resp: format!("Final report. Chart: Provided. word {}", word_padding),
    });

    let result = manager_dup
        .run_full_expert_workflow(task, &mut trace_dup, lead_llm_ok)
        .await;
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("High similarity detected"));

    // Test Success case
    let mut trace_final = SkillTrace::new();
    let lead_llm_final = Arc::new(MockSotaLlm {
        role_resp: format!(
            "Executive Summary: Market Trends. Chart: Included. Analysis: Comprehensive. word {}",
            word_padding
        ),
    });

    let result = manager
        .run_full_expert_workflow(task, &mut trace_final, lead_llm_final)
        .await;
    assert!(result.is_ok());
    assert!(trace_final.has_required_skills());
}

#[tokio::test]
async fn test_expert_team_condensation_rule() {
    let long_output = "long ".repeat(3000); // 15000 chars, > 6000
    let experts = vec![
        DomainExpert {
            role: "Researcher".to_string(),
            llm: Arc::new(MockSotaLlm {
                role_resp: long_output.clone(),
            }),
        },
        DomainExpert {
            role: "Analyst 1".to_string(),
            llm: Arc::new(MockSotaLlm {
                role_resp: "Chapter 3".to_string(),
            }),
        },
        DomainExpert {
            role: "Analyst 2".to_string(),
            llm: Arc::new(MockSotaLlm {
                role_resp: "Chapter 5".to_string(),
            }),
        },
        DomainExpert {
            role: "Analyst 3".to_string(),
            llm: Arc::new(MockSotaLlm {
                role_resp: "Chapter 7".to_string(),
            }),
        },
        DomainExpert {
            role: "Analyst 4".to_string(),
            llm: Arc::new(MockSotaLlm {
                role_resp: "Chapter 8".to_string(),
            }),
        },
    ];
    let manager = ExpertTeamManager::new("Lead", experts);
    let mut trace = SkillTrace::new();

    let summaries = manager
        .execute_parallel_tasks("Task", &mut trace)
        .await
        .unwrap();
    assert!(summaries[0].contains("Condensed for Harness"));
    assert!(
        summaries[0].len()
            <= 6000 + " [Condensed for Harness: 1k-2k tokens limit reached]".len() + 10
    );
}
