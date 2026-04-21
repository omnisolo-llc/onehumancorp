// Package modelpb contains Go types matching model.proto.
// These are manually maintained stubs so that callers can use the types
// without protoc-generated code – JSON marshalling is used in the application.
package modelpb

// ProviderType mirrors the ohc.model.ProviderType enum from model.proto.
type ProviderType int32

const (
	ProviderType_PROVIDER_TYPE_UNSPECIFIED     ProviderType = 0
	ProviderType_PROVIDER_TYPE_OPENAI          ProviderType = 1
	ProviderType_PROVIDER_TYPE_ANTHROPIC       ProviderType = 2
	ProviderType_PROVIDER_TYPE_GOOGLE          ProviderType = 3
	ProviderType_PROVIDER_TYPE_GROQ            ProviderType = 4
	ProviderType_PROVIDER_TYPE_OLLAMA          ProviderType = 5
	ProviderType_PROVIDER_TYPE_OPENROUTER      ProviderType = 6
	ProviderType_PROVIDER_TYPE_KILO            ProviderType = 7
	ProviderType_PROVIDER_TYPE_AZURE           ProviderType = 8
	ProviderType_PROVIDER_TYPE_AMAZON_BEDROCK  ProviderType = 9
	ProviderType_PROVIDER_TYPE_MINIMAX         ProviderType = 10
	ProviderType_PROVIDER_TYPE_CUSTOM          ProviderType = 99
)

// ModelStatus mirrors the ohc.model.ModelStatus enum from model.proto.
type ModelStatus int32

const (
	ModelStatus_MODEL_STATUS_UNSPECIFIED ModelStatus = 0
	ModelStatus_MODEL_STATUS_ACTIVE      ModelStatus = 1
	ModelStatus_MODEL_STATUS_BETA        ModelStatus = 2
	ModelStatus_MODEL_STATUS_DEPRECATED  ModelStatus = 3
	ModelStatus_MODEL_STATUS_DISABLED    ModelStatus = 4
)

// Modality mirrors the ohc.model.Modality enum from model.proto.
type Modality int32

const (
	Modality_MODALITY_UNSPECIFIED   Modality = 0
	Modality_MODALITY_TEXT          Modality = 1
	Modality_MODALITY_AUDIO_INPUT   Modality = 2
	Modality_MODALITY_AUDIO_OUTPUT  Modality = 3
	Modality_MODALITY_IMAGE_INPUT   Modality = 4
	Modality_MODALITY_VIDEO_INPUT   Modality = 5
	Modality_MODALITY_PDF_INPUT     Modality = 6
)

// ModelCost holds pricing information for a model.
type ModelCost struct {
	InputPerToken      float64
	OutputPerToken     float64
	CacheReadPerToken  float64
	CacheWritePerToken float64
	InputPerMillion    float64
	OutputPerMillion   float64
}

// ModelContextLimit holds context window limits for a model.
type ModelContextLimit struct {
	ContextWindow   int32
	MaxInputTokens  int32
	MaxOutputTokens int32
}

// ModelCapabilities describes what a model supports.
type ModelCapabilities struct {
	SupportsReasoning       bool
	SupportsToolCalling     bool
	SupportsTemperature     bool
	InputModalities         []Modality
	OutputModalities        []Modality
	SupportsStreaming        bool
	SupportsVision          bool
	SupportsFunctionCalling bool
}

// PredefinedModel is a known model from the model catalog.
type PredefinedModel struct {
	ModelId          string
	DisplayName      *string
	Description      *string
	ProviderType     ProviderType
	Family           *string
	ReleaseDate      *string
	Icon             interface{} // ModelIcon – unused in stubs
	Cost             *ModelCost
	ContextLimit     *ModelContextLimit
	Capabilities     *ModelCapabilities
	Status           ModelStatus
	RecommendedIndex *int32
	IsFree           *bool
	Variants         interface{} // []ModelVariant – unused in stubs
}

// PredefinedModelProvider describes a built-in provider configuration.
type PredefinedModelProvider struct {
	Type              ProviderType
	Name              *string
	BaseUrl           *string
	ApiKeyEnvVar      *string
	DefaultTimeoutMs  int32
	DocumentationUrl  *string
	SupportsStreaming  bool
}

