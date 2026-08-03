use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum _ProviderType {
    Openai,
    Anthropic,
    Google,
    Groq,
    Ollama,
    Openrouter,
    Kilo,
    Azure,
    AmazonBedrock,
    Custom,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum _ModelStatus {
    Active,
    Beta,
    Deprecated,
    Disabled,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum _Modality {
    Text,
    AudioInput,
    AudioOutput,
    ImageInput,
    VideoInput,
    PdfInput,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct _ModelCost {
    pub input_per_token: f64,
    pub output_per_token: f64,
    pub cache_read_per_token: f64,
    pub cache_write_per_token: f64,
    pub input_per_million: f64,
    pub output_per_million: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct _ModelContextLimit {
    pub context_window: i32,
    pub max_input_tokens: i32,
    pub max_output_tokens: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct _ModelCapabilities {
    pub supports_reasoning: bool,
    pub supports_tool_calling: bool,
    pub supports_temperature: bool,
    pub input_modalities: Vec<_Modality>,
    pub output_modalities: Vec<_Modality>,
    pub supports_streaming: bool,
    pub supports_vision: bool,
    pub supports_function_calling: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct _ModelIcon {
    pub url: String,
    pub color: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct _ModelVariant {
    pub id: String,
    pub name: String,
    pub disabled: bool,
    pub options: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct _ModelProvider {
    pub id: String,
    pub r#type: _ProviderType,
    pub name: String,
    pub organization_id: String,
    pub api_key_env_var: String,
    pub base_url: String,
    pub timeout_ms: i32,
    pub chunk_timeout_ms: i32,
    pub headers: HashMap<String, String>,
    pub options: HashMap<String, String>,
    pub enabled: bool,
    pub env_vars: Vec<String>,
    pub npm_package: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct _ModelInstance {
    pub id: String,
    pub name: String,
    pub organization_id: String,
    pub provider_type: _ProviderType,
    pub provider_id: String,
    pub model_id: String,
    pub display_name: String,
    pub description: String,
    pub icon: _ModelIcon,
    pub cost: _ModelCost,
    pub context_limit: _ModelContextLimit,
    pub capabilities: _ModelCapabilities,
    pub status: _ModelStatus,
    pub recommended_index: i32,
    pub is_free: bool,
    pub release_date: String,
    pub family: String,
    pub metadata: HashMap<String, String>,
    pub variants: Vec<_ModelVariant>,
    pub created_at_unix: i64,
    pub updated_at_unix: i64,
}

impl _ModelInstance {
    pub fn _created_at(&self) -> DateTime<Utc> {
        DateTime::from_timestamp(self.created_at_unix, 0).unwrap_or_default()
    }

    pub fn _updated_at(&self) -> DateTime<Utc> {
        DateTime::from_timestamp(self.updated_at_unix, 0).unwrap_or_default()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct _ModelBinding {
    pub id: String,
    pub organization_id: String,
    pub agent_id: String,
    pub model_instance_id: String,
    pub is_default: bool,
    pub priority: i32,
    pub created_at_unix: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct _OrganizationModelConfig {
    pub organization_id: String,
    pub providers: Vec<_ModelProvider>,
    pub model_instances: Vec<_ModelInstance>,
    pub bindings: Vec<_ModelBinding>,
    pub enabled_provider_types: Vec<String>,
    pub disabled_model_ids: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct _GlobalModelConfig {
    pub default_providers: Vec<_ModelProvider>,
    pub default_models: Vec<_ModelInstance>,
    pub provider_api_env_vars: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct _ResolvedModel {
    pub model: _ModelInstance,
    pub provider: _ModelProvider,
    pub endpoint: String,
    pub headers: HashMap<String, String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_model_instance_timestamps() {
        let instance = _ModelInstance {
            id: "model-1".to_string(),
            name: "gpt-4".to_string(),
            organization_id: "org-1".to_string(),
            provider_type: _ProviderType::Openai,
            provider_id: "prov-1".to_string(),
            model_id: "gpt-4-0613".to_string(),
            display_name: "GPT-4".to_string(),
            description: "OpenAI GPT-4".to_string(),
            icon: _ModelIcon { url: "".to_string(), color: "".to_string() },
            cost: _ModelCost {
                input_per_token: 0.0, output_per_token: 0.0,
                cache_read_per_token: 0.0, cache_write_per_token: 0.0,
                input_per_million: 0.0, output_per_million: 0.0,
            },
            context_limit: _ModelContextLimit {
                context_window: 8192, max_input_tokens: 8192, max_output_tokens: 8192,
            },
            capabilities: _ModelCapabilities {
                supports_reasoning: false, supports_tool_calling: true,
                supports_temperature: true, input_modalities: vec![],
                output_modalities: vec![], supports_streaming: true,
                supports_vision: false, supports_function_calling: true,
            },
            status: _ModelStatus::Active,
            recommended_index: 1,
            is_free: false,
            release_date: "".to_string(),
            family: "".to_string(),
            metadata: HashMap::new(),
            variants: vec![],
            created_at_unix: 1680000000,
            updated_at_unix: 1680000000,
        };

        let created_at = instance._created_at();
        assert_eq!(created_at.timestamp(), 1680000000);

        let updated_at = instance._updated_at();
        assert_eq!(updated_at.timestamp(), 1680000000);
    }
}
