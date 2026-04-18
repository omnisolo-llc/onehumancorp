// Package wizardpb contains Go types matching wizard.proto.
// These are manually maintained to match the wizard.proto definition.
// They mirror the proto messages so that callers do not need protoc-generated
// code – plain JSON marshalling is used instead.
package wizardpb

// ProviderType mirrors ohc.model.ProviderType from model.proto.
// Values must stay in sync with model.proto.
type ProviderType int32

const (
	ProviderTypeUnspecified   ProviderType = 0
	ProviderTypeOpenAI        ProviderType = 1
	ProviderTypeAnthropic     ProviderType = 2
	ProviderTypeGoogle        ProviderType = 3
	ProviderTypeGroq          ProviderType = 4
	ProviderTypeOllama        ProviderType = 5
	ProviderTypeOpenRouter    ProviderType = 6
	ProviderTypeKilo          ProviderType = 7
	ProviderTypeAzure         ProviderType = 8
	ProviderTypeAmazonBedrock ProviderType = 9
	ProviderTypeMinimax       ProviderType = 10
	ProviderTypeCustom        ProviderType = 99
)

// WizardMode mirrors the WizardMode proto enum.
type WizardMode int32

const (
	WizardModeUnspecified     WizardMode = 0
	WizardModeForm            WizardMode = 1
	WizardModeNaturalLanguage WizardMode = 2
)

// ModelProviderConfig mirrors the ModelProviderConfig proto message.
type ModelProviderConfig struct {
	ID           string       `json:"id,omitempty"`
	ProviderType ProviderType `json:"provider_type,omitempty"`
	Name         string       `json:"name,omitempty"`
	BaseURL      string       `json:"base_url,omitempty"`
	APIKey       string       `json:"api_key,omitempty"`
	Model        string       `json:"model,omitempty"`
	Models       []string     `json:"models,omitempty"`
	Enabled      bool         `json:"enabled,omitempty"`
	IsOfficial   bool         `json:"is_official,omitempty"`
}

// WizardConfigureRequest mirrors the WizardConfigureRequest proto message.
type WizardConfigureRequest struct {
	ListenAddr    string              `json:"listen_addr,omitempty"`
	DBPath        string              `json:"db_path,omitempty"`
	PostgresURL   string              `json:"postgres_url,omitempty"`
	RedisURL      string              `json:"redis_url,omitempty"`
	CentrifugeURL string              `json:"centrifuge_url,omitempty"`
	MinimaxAPIKey string              `json:"minimax_api_key,omitempty"`
	AiProviders   []ModelProviderConfig `json:"ai_providers,omitempty"`
	Extras        map[string]string   `json:"extras,omitempty"`
}

// WizardStatusResponse mirrors the WizardStatusResponse proto message.
type WizardStatusResponse struct {
	Configured     bool `json:"configured"`
	StepServer     bool `json:"step_server"`
	StepAiProvider bool `json:"step_ai_provider"`
	StepCentrifuge bool `json:"step_centrifuge"`
}

// WizardBootstrapBusinessRequest mirrors the WizardBootstrapBusinessRequest proto message.
type WizardBootstrapBusinessRequest struct {
	Prompt      string     `json:"prompt,omitempty"`
	CompanyName string     `json:"company_name,omitempty"`
	Industry    string     `json:"industry,omitempty"`
	CompanySize string     `json:"company_size,omitempty"`
	Goals       []string   `json:"goals,omitempty"`
	Deployment  string     `json:"deployment_preference,omitempty"`
	AdminName   string     `json:"admin_name,omitempty"`
	AdminEmail  string     `json:"admin_email,omitempty"`
	Mode        WizardMode `json:"mode,omitempty"`
}

// WizardNlChatRequest mirrors the WizardNlChatRequest proto message.
type WizardNlChatRequest struct {
	SessionID    string                          `json:"session_id,omitempty"`
	Message      string                          `json:"message,omitempty"`
	PartialState *WizardBootstrapBusinessRequest `json:"partial_state,omitempty"`
}

// WizardNlChatResponse mirrors the WizardNlChatResponse proto message.
type WizardNlChatResponse struct {
	SessionID     string            `json:"session_id,omitempty"`
	Reply         string            `json:"reply,omitempty"`
	FieldUpdates  map[string]string `json:"field_updates,omitempty"`
	ReadyToSubmit bool              `json:"ready_to_submit,omitempty"`
}

// ModelProviderWizardRequest mirrors the ModelProviderWizardRequest proto message.
type ModelProviderWizardRequest struct {
	Step      int32               `json:"step,omitempty"`
	Provider  ModelProviderConfig `json:"provider,omitempty"`
	AgentType string              `json:"agent_type,omitempty"`
}

// ModelProviderWizardResponse mirrors the ModelProviderWizardResponse proto message.
type ModelProviderWizardResponse struct {
	Step               int32               `json:"step,omitempty"`
	TotalSteps         int32               `json:"total_steps,omitempty"`
	Title              string              `json:"title,omitempty"`
	Instruction        string              `json:"instruction,omitempty"`
	ValidationErrors   []string            `json:"validation_errors,omitempty"`
	Complete           bool                `json:"complete,omitempty"`
	ConfiguredProvider ModelProviderConfig `json:"configured_provider,omitempty"`
}
