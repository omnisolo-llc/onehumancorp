use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum ProviderType {
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
pub enum ModelStatus {
    Active,
    Beta,
    Deprecated,
    Disabled,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum Modality {
    Text,
    AudioInput,
    AudioOutput,
    ImageInput,
    VideoInput,
    PdfInput,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelCost {
    pub input_per_token: f64,
    pub output_per_token: f64,
    pub cache_read_per_token: f64,
    pub cache_write_per_token: f64,
    pub input_per_million: f64,
    pub output_per_million: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelContextLimit {
    pub context_window: i32,
    pub max_input_tokens: i32,
    pub max_output_tokens: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelCapabilities {
    pub supports_reasoning: bool,
    pub supports_tool_calling: bool,
    pub supports_temperature: bool,
    pub input_modalities: Vec<Modality>,
    pub output_modalities: Vec<Modality>,
    pub supports_streaming: bool,
    pub supports_vision: bool,
    pub supports_function_calling: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelIcon {
    pub url: String,
    pub color: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelVariant {
    pub id: String,
    pub name: String,
    pub disabled: bool,
    pub options: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelProvider {
    pub id: String,
    pub r#type: ProviderType,
    pub name: String,
    pub tenant_id: String,
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
pub struct ModelInstance {
    pub id: String,
    pub name: String,
    pub tenant_id: String,
    pub provider_type: ProviderType,
    pub provider_id: String,
    pub model_id: String,
    pub display_name: String,
    pub description: String,
    pub icon: ModelIcon,
    pub cost: ModelCost,
    pub context_limit: ModelContextLimit,
    pub capabilities: ModelCapabilities,
    pub status: ModelStatus,
    pub recommended_index: i32,
    pub is_free: bool,
    pub release_date: String,
    pub family: String,
    pub metadata: HashMap<String, String>,
    pub variants: Vec<ModelVariant>,
    pub created_at_unix: i64,
    pub updated_at_unix: i64,
}

impl ModelInstance {
    pub fn created_at(&self) -> DateTime<Utc> {
        DateTime::from_timestamp(self.created_at_unix, 0).unwrap_or_default()
    }

    pub fn updated_at(&self) -> DateTime<Utc> {
        DateTime::from_timestamp(self.updated_at_unix, 0).unwrap_or_default()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelBinding {
    pub id: String,
    pub tenant_id: String,
    pub agent_id: String,
    pub model_instance_id: String,
    pub is_default: bool,
    pub priority: i32,
    pub created_at_unix: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrganizationModelConfig {
    pub tenant_id: String,
    pub providers: Vec<ModelProvider>,
    pub model_instances: Vec<ModelInstance>,
    pub bindings: Vec<ModelBinding>,
    pub enabled_provider_types: Vec<String>,
    pub disabled_model_ids: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlobalModelConfig {
    pub default_providers: Vec<ModelProvider>,
    pub default_models: Vec<ModelInstance>,
    pub provider_api_env_vars: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResolvedModel {
    pub model: ModelInstance,
    pub provider: ModelProvider,
    pub endpoint: String,
    pub headers: HashMap<String, String>,
}
