use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VoiceAgentConfig {
    pub tenant_id: String,
    pub phone_number: String,
    pub is_enabled: bool,
    pub primary_language: String,
    pub custom_instructions: String,
}

impl Default for VoiceAgentConfig {
    fn default() -> Self {
        Self {
            tenant_id: String::new(),
            phone_number: String::new(),
            is_enabled: false,
            primary_language: "English".to_string(),
            custom_instructions: String::new(),
        }
    }
}
