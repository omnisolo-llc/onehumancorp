use crate::ohc::orchestration::{AgentConfig, PromptTuningConfig, WizardResponse};

pub fn handle_config_wizard(_config: AgentConfig) -> WizardResponse {
    println!("Received ConfigWizard request in wizard service");
    WizardResponse {
        success: true,
        message: "success".to_string(),
    }
}

pub fn handle_prompt_tuning(_config: PromptTuningConfig) -> WizardResponse {
    println!("Received PromptTuning request in wizard service");
    WizardResponse {
        success: true,
        message: "success".to_string(),
    }
}
