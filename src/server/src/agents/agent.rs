use async_trait::async_trait;
use std::sync::Arc;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum Status {
    IDLE,
    ACTIVE,
    #[serde(rename = "IN_MEETING")]
    InMeeting,
    BLOCKED,
    #[serde(rename = "WAITING_FOR_TOOLS")]
    WaitingForTools,
}

pub const EVENT_TASK: &str = "task";
pub const EVENT_STATUS: &str = "status";
pub const EVENT_HANDOFF: &str = "handoff";
pub const EVENT_CODE_REVIEWED: &str = "CodeReviewed";
pub const EVENT_TESTS_FAILED: &str = "TestsFailed";
pub const EVENT_TESTS_PASSED: &str = "TestsPassed";
pub const EVENT_SPEC_APPROVED: &str = "SpecApproved";
pub const EVENT_BLOCKER_RAISED: &str = "BlockerRaised";
pub const EVENT_BLOCKER_CLEARED: &str = "BlockerCleared";
pub const EVENT_PR_CREATED: &str = "PRCreated";
pub const EVENT_PR_MERGED: &str = "PRMerged";
pub const EVENT_DESIGN_REVIEWED: &str = "DesignReviewed";
pub const EVENT_APPROVAL_NEEDED: &str = "ApprovalNeeded";

pub const AVAILABLE_MCP_BUNDLES: &[&str] = &[
    "github",
];

#[async_trait]
pub trait Transport: Send + Sync {
    async fn send(&self, message: &[u8]) -> Result<(), String>;
    async fn receive(&self) -> Result<Vec<u8>, String>;
    async fn close(&self) -> Result<(), String>;
}

pub trait AgentExt {
    fn base_system_prompt(&self) -> String;
}

impl AgentExt for crate::ohc::orchestration::Agent {
    fn base_system_prompt(&self) -> String {
        let mut prompt = format!(
            "You are an autonomous AI agent representing One Human Corp (OHC). You operate within the bounds of your Role: {}.\n",
            self.role
        );
        if std::env::var("OHC_STANDALONE").unwrap_or_default() == "true" {
            prompt += "\n# Memory Fallback (Standalone Mode)\n";
            prompt += "The directories .ohc/memory/auto/ and .ohc/memory/team/ already exist. Write state to them directly.\n";
        }
        prompt
    }
}
