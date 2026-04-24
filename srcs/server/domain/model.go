package domain

import "time"

type ProviderType string

const (
	ProviderTypeOpenAI        ProviderType = "openai"
	ProviderTypeAnthropic     ProviderType = "anthropic"
	ProviderTypeGoogle        ProviderType = "google"
	ProviderTypeGroq          ProviderType = "groq"
	ProviderTypeOllama        ProviderType = "ollama"
	ProviderTypeOpenRouter    ProviderType = "openrouter"
	ProviderTypeKilo          ProviderType = "kilo"
	ProviderTypeAzure         ProviderType = "azure"
	ProviderTypeAmazonBedrock ProviderType = "amazon_bedrock"
	ProviderTypeCustom        ProviderType = "custom"
)

type ModelStatus string

const (
	ModelStatusActive     ModelStatus = "active"
	ModelStatusBeta       ModelStatus = "beta"
	ModelStatusDeprecated ModelStatus = "deprecated"
	ModelStatusDisabled   ModelStatus = "disabled"
)

type Modality string

const (
	ModalityText        Modality = "text"
	ModalityAudioInput  Modality = "audio_input"
	ModalityAudioOutput Modality = "audio_output"
	ModalityImageInput  Modality = "image_input"
	ModalityVideoInput  Modality = "video_input"
	ModalityPDFInput    Modality = "pdf_input"
)

type ModelCost struct {
	InputPerToken      float64 `json:"input_per_token"`
	OutputPerToken     float64 `json:"output_per_token"`
	CacheReadPerToken  float64 `json:"cache_read_per_token"`
	CacheWritePerToken float64 `json:"cache_write_per_token"`
	InputPerMillion    float64 `json:"input_per_million"`
	OutputPerMillion   float64 `json:"output_per_million"`
}

type ModelContextLimit struct {
	ContextWindow   int32 `json:"context_window"`
	MaxInputTokens  int32 `json:"max_input_tokens"`
	MaxOutputTokens int32 `json:"max_output_tokens"`
}

type ModelCapabilities struct {
	SupportsReasoning       bool       `json:"supports_reasoning"`
	SupportsToolCalling     bool       `json:"supports_tool_calling"`
	SupportsTemperature     bool       `json:"supports_temperature"`
	InputModalities         []Modality `json:"input_modalities"`
	OutputModalities        []Modality `json:"output_modalities"`
	SupportsStreaming       bool       `json:"supports_streaming"`
	SupportsVision          bool       `json:"supports_vision"`
	SupportsFunctionCalling bool       `json:"supports_function_calling"`
}

type ModelIcon struct {
	URL   string `json:"url"`
	Color string `json:"color"`
}

type ModelVariant struct {
	ID       string            `json:"id"`
	Name     string            `json:"name"`
	Disabled bool              `json:"disabled"`
	Options  map[string]string `json:"options"`
}

type ModelProvider struct {
	ID             string            `json:"id"`
	Type           ProviderType      `json:"type"`
	Name           string            `json:"name"`
	OrganizationID string            `json:"organization_id"`
	APIKeyEnvVar   string            `json:"api_key_env_var"`
	BaseURL        string            `json:"base_url"`
	TimeoutMs      int32             `json:"timeout_ms"`
	ChunkTimeoutMs int32             `json:"chunk_timeout_ms"`
	Headers        map[string]string `json:"headers"`
	Options        map[string]string `json:"options"`
	Enabled        bool              `json:"enabled"`
	EnvVars        []string          `json:"env_vars"`
	NPMPackage     string            `json:"npm_package"`
}

type ModelInstance struct {
	ID               string            `json:"id"`
	Name             string            `json:"name"`
	OrganizationID   string            `json:"organization_id"`
	ProviderType     ProviderType      `json:"provider_type"`
	ProviderID       string            `json:"provider_id"`
	ModelID          string            `json:"model_id"`
	DisplayName      string            `json:"display_name"`
	Description      string            `json:"description"`
	Icon             ModelIcon         `json:"icon"`
	Cost             ModelCost         `json:"cost"`
	ContextLimit     ModelContextLimit `json:"context_limit"`
	Capabilities     ModelCapabilities `json:"capabilities"`
	Status           ModelStatus       `json:"status"`
	RecommendedIndex int32             `json:"recommended_index"`
	IsFree           bool              `json:"is_free"`
	ReleaseDate      string            `json:"release_date"`
	Family           string            `json:"family"`
	Metadata         map[string]string `json:"metadata"`
	Variants         []ModelVariant    `json:"variants"`
	CreatedAtUnix    int64             `json:"created_at_unix"`
	UpdatedAtUnix    int64             `json:"updated_at_unix"`
}

func (m *ModelInstance) CreatedAt() time.Time {
	return time.Unix(m.CreatedAtUnix, 0)
}

func (m *ModelInstance) UpdatedAt() time.Time {
	return time.Unix(m.UpdatedAtUnix, 0)
}

type ModelBinding struct {
	ID              string `json:"id"`
	OrganizationID  string `json:"organization_id"`
	AgentID         string `json:"agent_id"`
	ModelInstanceID string `json:"model_instance_id"`
	IsDefault       bool   `json:"is_default"`
	Priority        int32  `json:"priority"`
	CreatedAtUnix   int64  `json:"created_at_unix"`
}

type OrganizationModelConfig struct {
	OrganizationID       string          `json:"organization_id"`
	Providers            []ModelProvider `json:"providers"`
	ModelInstances       []ModelInstance `json:"model_instances"`
	Bindings             []ModelBinding  `json:"bindings"`
	EnabledProviderTypes []string        `json:"enabled_provider_types"`
	DisabledModelIDs     []string        `json:"disabled_model_ids"`
}

type GlobalModelConfig struct {
	DefaultProviders   []ModelProvider   `json:"default_providers"`
	DefaultModels      []ModelInstance   `json:"default_models"`
	ProviderAPIEnvVars map[string]string `json:"provider_api_env_vars"`
}

type ResolvedModel struct {
	Model    *ModelInstance    `json:"model"`
	Provider *ModelProvider    `json:"provider"`
	Endpoint string            `json:"endpoint"`
	Headers  map[string]string `json:"headers"`
}
