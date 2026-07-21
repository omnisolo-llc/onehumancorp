use std::sync::Arc;
use crate::services::onboarding::onboarding_agent::{OnboardingAgent, IntakeData};

pub struct ZeroClickAgent {
    onboarding_agent: Arc<OnboardingAgent>,
}

impl ZeroClickAgent {
    pub fn new(onboarding_agent: Arc<OnboardingAgent>) -> Self {
        Self { onboarding_agent }
    }

    pub async fn execute_zero_click_setup(&self, input: &str) -> Result<IntakeData, String> {
        // Delegate to existing onboarding agent for LLM extraction
        self.onboarding_agent.process_intake(input).await
    }
}
