use ohc_builtin_agent::expert_team::{
    DomainExpert, ExpertTeamLlmClient, ExpertTeamManager, QualityGates, SkillTrace,
};
use ohc_builtin_agent::types::{ChatRequest, ChatResponse, Message, Usage};
use std::sync::Arc;

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
async fn test_expert_workflow_missing_chapters_failure() {
    let experts = vec![
        DomainExpert {
            role: "Industry Researcher".to_string(),
            llm: Arc::new(MockExpertLlm {
                role_resp: "Research summary... Chapter 1 and Chapter 2".to_string(),
            }) as Arc<dyn ExpertTeamLlmClient>,
        },
        DomainExpert {
            role: "Financial Analyst".to_string(),
            llm: Arc::new(MockExpertLlm {
                role_resp: "Financial summary... Chapter 3 and Chapter 4".to_string(),
            }) as Arc<dyn ExpertTeamLlmClient>,
        },
        DomainExpert {
            role: "Strategic Analyst".to_string(),
            llm: Arc::new(MockExpertLlm {
                role_resp: "Strategic summary... Chapter 5 and Chapter 6".to_string(),
            }) as Arc<dyn ExpertTeamLlmClient>,
        },
        DomainExpert {
            role: "Process Supervisor".to_string(),
            llm: Arc::new(MockExpertLlm {
                role_resp: "Process summary... Chapter 7".to_string(),
            }) as Arc<dyn ExpertTeamLlmClient>,
        },
        // Misses chapter 8 on purpose
        DomainExpert {
            role: "Quality Auditor".to_string(),
            llm: Arc::new(MockExpertLlm {
                role_resp: "Quality summary... no chapters here".to_string(),
            }) as Arc<dyn ExpertTeamLlmClient>,
        },
    ];
    let manager = ExpertTeamManager::new("Project Director", experts);

    let task = "Analyze the new AI market trends for the upcoming quarter. Chart: Required. Analysis: Deep.";
    let mut trace = SkillTrace::new();

    let lead_llm = Arc::new(MockExpertLlm {
        role_resp: "Combined Executive Summary...".to_string(),
    });

    let result = manager
        .run_full_expert_workflow(task, &mut trace, lead_llm)
        .await;
    assert!(matches!(result, Err(ref e) if e.contains("Missing: Chapter 8")));
}

#[test]
fn test_pre_flight_failure_empty_task() {
    let experts = vec![
        DomainExpert {
            role: "Lone Wolf".to_string(),
            llm: Arc::new(MockExpertLlm {
                role_resp: "".to_string(),
            }) as Arc<dyn ExpertTeamLlmClient>,
        },
        DomainExpert {
            role: "Lone Wolf".to_string(),
            llm: Arc::new(MockExpertLlm {
                role_resp: "".to_string(),
            }) as Arc<dyn ExpertTeamLlmClient>,
        },
        DomainExpert {
            role: "Lone Wolf".to_string(),
            llm: Arc::new(MockExpertLlm {
                role_resp: "".to_string(),
            }) as Arc<dyn ExpertTeamLlmClient>,
        },
        DomainExpert {
            role: "Lone Wolf".to_string(),
            llm: Arc::new(MockExpertLlm {
                role_resp: "".to_string(),
            }) as Arc<dyn ExpertTeamLlmClient>,
        },
        DomainExpert {
            role: "Lone Wolf".to_string(),
            llm: Arc::new(MockExpertLlm {
                role_resp: "".to_string(),
            }) as Arc<dyn ExpertTeamLlmClient>,
        },
    ];
    let manager = ExpertTeamManager::new("Lead", experts);
    let res = QualityGates::pre_flight(&manager, "   ");
    assert!(matches!(res, Err(ref e) if e.contains("Task context cannot be empty")));
}

#[tokio::test]
async fn test_expert_workflow_timeout_isolation() {
    struct TimeoutLlm;
    #[async_trait::async_trait]
    impl ExpertTeamLlmClient for TimeoutLlm {
        async fn chat(
            &self,
            _req: ChatRequest,
        ) -> Result<ChatResponse, Box<dyn std::error::Error + Send + Sync>> {
            tokio::time::sleep(std::time::Duration::from_millis(50)).await; // Faster for tests
            Err("Timeout simulated".into())
        }
    }

    let experts = vec![
        DomainExpert {
            role: "Fast Expert".to_string(),
            llm: Arc::new(MockExpertLlm {
                role_resp: "Done!".to_string(),
            }) as Arc<dyn ExpertTeamLlmClient>,
        },
        DomainExpert {
            role: "Slow Expert".to_string(),
            llm: Arc::new(TimeoutLlm) as Arc<dyn ExpertTeamLlmClient>,
        },
    ];
    let manager = ExpertTeamManager::new("Lead", experts);
    let mut trace = SkillTrace::new();

    let result = manager.execute_parallel_tasks("Task", &mut trace).await;
    assert!(matches!(result, Err(ref e) if e.contains("Timeout simulated")));
}

#[tokio::test]
async fn test_expert_workflow_consensus_conflict() {
    let mut trace = SkillTrace::new();
    let lead_llm = Arc::new(MockExpertLlm {
        role_resp: "Combined Executive Summary: Conflicting opinions detected. The market might go up or down. Chart: Provided. Analysis: Provided. word ".repeat(20000)
    });

    let experts = vec![
        DomainExpert {
            role: "Bullish Expert".to_string(),
            llm: Arc::new(MockExpertLlm {
                role_resp: "Chapter 1 Chapter 2 The market is going UP. Buy immediately!"
                    .to_string(),
            }) as Arc<dyn ExpertTeamLlmClient>,
        },
        DomainExpert {
            role: "Bearish Expert".to_string(),
            llm: Arc::new(MockExpertLlm {
                role_resp: "Chapter 3 Chapter 4 The market is going DOWN. Sell immediately!"
                    .to_string(),
            }) as Arc<dyn ExpertTeamLlmClient>,
        },
        DomainExpert {
            role: "Neutral Expert 1".to_string(),
            llm: Arc::new(MockExpertLlm {
                role_resp: "Chapter 5 Chapter 6 The market is stable.".to_string(),
            }) as Arc<dyn ExpertTeamLlmClient>,
        },
        DomainExpert {
            role: "Neutral Expert 2".to_string(),
            llm: Arc::new(MockExpertLlm {
                role_resp: "Chapter 7 It will remain stable.".to_string(),
            }) as Arc<dyn ExpertTeamLlmClient>,
        },
        DomainExpert {
            role: "Quality Auditor".to_string(),
            llm: Arc::new(MockExpertLlm {
                role_resp: "Chapter 8 Final review complete.".to_string(),
            }) as Arc<dyn ExpertTeamLlmClient>,
        },
    ];
    let manager = ExpertTeamManager::new("Lead", experts);

    let result = manager
        .run_full_expert_workflow(
            "Analyze the market. Chart: Required. Analysis: Deep.",
            &mut trace,
            lead_llm,
        )
        .await;
    assert!(result.is_ok(), "Expected consensus resolution to succeed");
    assert!(
        result
            .expect("should succeed in test")
            .contains("Conflicting opinions detected")
    );
}
