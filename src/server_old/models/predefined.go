package models

import (
	modelpb "github.com/onehumancorp/mono/src/proto/ohc/model"
)

type ProviderConfig struct {
	Type              modelpb.ProviderType
	Name              string
	BaseURL           string
	APikeyEnvVar      string
	DefaultTimeoutMs  int32
	DocumentationURL  string
	SupportsStreaming bool
}

type ModelConfig struct {
	ModelID                string
	DisplayName            string
	Description            string
	ProviderType           modelpb.ProviderType
	Family                 string
	ReleaseDate            string
	Status                 modelpb.ModelStatus
	IsFree                 bool
	InputCostPerMillion    float64
	OutputCostPerMillion   float64
	CacheReadCostPerToken  float64
	CacheWriteCostPerToken float64
	ContextWindow          int32
	MaxInputTokens         int32
	MaxOutputTokens        int32
	SupportsReasoning      bool
	SupportsToolCalling    bool
	SupportsTemperature    bool
	SupportsVision         bool
	SupportsStreaming      bool
	InputModalities        []modelpb.Modality
	OutputModalities       []modelpb.Modality
	RecommendedIndex       int32
}

var Providers = []ProviderConfig{
	{Type: modelpb.ProviderType_PROVIDER_TYPE_OPENAI, Name: "OpenAI", BaseURL: "https://api.openai.com/v1", APikeyEnvVar: "OPENAI_API_KEY", DefaultTimeoutMs: 120000, DocumentationURL: "https://platform.openai.com/docs/api-reference", SupportsStreaming: true},
	{Type: modelpb.ProviderType_PROVIDER_TYPE_ANTHROPIC, Name: "Anthropic", BaseURL: "https://api.anthropic.com", APikeyEnvVar: "ANTHROPIC_API_KEY", DefaultTimeoutMs: 120000, DocumentationURL: "https://docs.anthropic.com/en/api/reference", SupportsStreaming: false},
	{Type: modelpb.ProviderType_PROVIDER_TYPE_GOOGLE, Name: "Google AI", BaseURL: "https://generativelanguage.googleapis.com/v1beta/openai/", APikeyEnvVar: "GOOGLE_API_KEY", DefaultTimeoutMs: 120000, DocumentationURL: "https://ai.google.dev/api/rest", SupportsStreaming: true},
	{Type: modelpb.ProviderType_PROVIDER_TYPE_GROQ, Name: "Groq", BaseURL: "https://api.groq.com/openai/v1", APikeyEnvVar: "GROQ_API_KEY", DefaultTimeoutMs: 60000, DocumentationURL: "https://console.groq.com/docs/api-reference", SupportsStreaming: true},
	{Type: modelpb.ProviderType_PROVIDER_TYPE_OLLAMA, Name: "Ollama", BaseURL: "http://localhost:11434/v1", APikeyEnvVar: "", DefaultTimeoutMs: 300000, DocumentationURL: "https://github.com/ollama/ollama/blob/main/docs/api.md", SupportsStreaming: true},
	{Type: modelpb.ProviderType_PROVIDER_TYPE_OPENROUTER, Name: "OpenRouter", BaseURL: "https://openrouter.ai/api/v1", APikeyEnvVar: "OPENROUTER_API_KEY", DefaultTimeoutMs: 120000, DocumentationURL: "https://openrouter.ai/docs", SupportsStreaming: true},
	{Type: modelpb.ProviderType_PROVIDER_TYPE_KILO, Name: "Kilo", BaseURL: "https://api.kilo.ai/v1", APikeyEnvVar: "KILO_API_KEY", DefaultTimeoutMs: 120000, DocumentationURL: "https://kilo.ai/docs", SupportsStreaming: true},
	{Type: modelpb.ProviderType_PROVIDER_TYPE_AZURE, Name: "Azure OpenAI", BaseURL: "https://{resource}.openai.azure.com/openai/deployments/{model}", APikeyEnvVar: "AZURE_OPENAI_API_KEY", DefaultTimeoutMs: 120000, DocumentationURL: "https://learn.microsoft.com/en-us/azure/ai-services/openai/reference", SupportsStreaming: true},
	{Type: modelpb.ProviderType_PROVIDER_TYPE_AMAZON_BEDROCK, Name: "Amazon Bedrock", BaseURL: "https://bedrock.{region}.amazonaws.com", APikeyEnvVar: "AWS_ACCESS_KEY_ID", DefaultTimeoutMs: 120000, DocumentationURL: "https://docs.aws.amazon.com/bedrock/", SupportsStreaming: false},
}

var Models = []ModelConfig{
	{ModelID: "gpt-4o", DisplayName: "GPT-4o", Description: "Most capable GPT-4 model with vision and audio support", ProviderType: modelpb.ProviderType_PROVIDER_TYPE_OPENAI, Family: "GPT-4", ReleaseDate: "2024-05-13", Status: modelpb.ModelStatus_MODEL_STATUS_ACTIVE, InputCostPerMillion: 2.50, OutputCostPerMillion: 10.00, ContextWindow: 128000, MaxInputTokens: 128000, MaxOutputTokens: 16384, SupportsToolCalling: true, SupportsTemperature: true, SupportsVision: true, InputModalities: []modelpb.Modality{modelpb.Modality_MODALITY_TEXT, modelpb.Modality_MODALITY_IMAGE_INPUT}, OutputModalities: []modelpb.Modality{modelpb.Modality_MODALITY_TEXT}, SupportsStreaming: true, RecommendedIndex: 1},
	{ModelID: "gpt-4o-mini", DisplayName: "GPT-4o mini", Description: "Smaller, faster, cheaper GPT-4o variant", ProviderType: modelpb.ProviderType_PROVIDER_TYPE_OPENAI, Family: "GPT-4", ReleaseDate: "2024-07-18", Status: modelpb.ModelStatus_MODEL_STATUS_ACTIVE, InputCostPerMillion: 0.15, OutputCostPerMillion: 0.60, ContextWindow: 128000, MaxInputTokens: 128000, MaxOutputTokens: 16384, SupportsToolCalling: true, SupportsTemperature: true, SupportsVision: true, InputModalities: []modelpb.Modality{modelpb.Modality_MODALITY_TEXT, modelpb.Modality_MODALITY_IMAGE_INPUT}, OutputModalities: []modelpb.Modality{modelpb.Modality_MODALITY_TEXT}, SupportsStreaming: true, RecommendedIndex: 2},
	{ModelID: "o1-preview", DisplayName: "o1 Preview", Description: "Reasoning model for complex problems", ProviderType: modelpb.ProviderType_PROVIDER_TYPE_OPENAI, Family: "o-series", ReleaseDate: "2024-09-12", Status: modelpb.ModelStatus_MODEL_STATUS_BETA, InputCostPerMillion: 15.00, OutputCostPerMillion: 60.00, ContextWindow: 128000, MaxInputTokens: 128000, MaxOutputTokens: 32768, SupportsReasoning: true, SupportsToolCalling: true, SupportsTemperature: false, InputModalities: []modelpb.Modality{modelpb.Modality_MODALITY_TEXT}, OutputModalities: []modelpb.Modality{modelpb.Modality_MODALITY_TEXT}, SupportsStreaming: false, RecommendedIndex: 3},
	{ModelID: "o1-mini", DisplayName: "o1 Mini", Description: "Faster, cheaper reasoning model", ProviderType: modelpb.ProviderType_PROVIDER_TYPE_OPENAI, Family: "o-series", ReleaseDate: "2024-09-12", Status: modelpb.ModelStatus_MODEL_STATUS_BETA, InputCostPerMillion: 3.00, OutputCostPerMillion: 12.00, ContextWindow: 128000, MaxInputTokens: 128000, MaxOutputTokens: 65536, SupportsReasoning: true, SupportsToolCalling: true, SupportsTemperature: false, InputModalities: []modelpb.Modality{modelpb.Modality_MODALITY_TEXT}, OutputModalities: []modelpb.Modality{modelpb.Modality_MODALITY_TEXT}, SupportsStreaming: false, RecommendedIndex: 4},
	{ModelID: "claude-3-5-sonnet-20241022", DisplayName: "Claude 3.5 Sonnet", Description: "Most intelligent Claude model with enhanced reasoning", ProviderType: modelpb.ProviderType_PROVIDER_TYPE_ANTHROPIC, Family: "Claude-3.5", ReleaseDate: "2024-10-22", Status: modelpb.ModelStatus_MODEL_STATUS_ACTIVE, InputCostPerMillion: 3.00, OutputCostPerMillion: 15.00, CacheReadCostPerToken: 0.30, CacheWriteCostPerToken: 3.75, ContextWindow: 200000, MaxInputTokens: 200000, MaxOutputTokens: 8192, SupportsReasoning: true, SupportsToolCalling: true, SupportsTemperature: true, SupportsVision: true, InputModalities: []modelpb.Modality{modelpb.Modality_MODALITY_TEXT, modelpb.Modality_MODALITY_IMAGE_INPUT}, OutputModalities: []modelpb.Modality{modelpb.Modality_MODALITY_TEXT}, SupportsStreaming: false, RecommendedIndex: 1},
	{ModelID: "claude-3-5-haiku-20241022", DisplayName: "Claude 3.5 Haiku", Description: "Fast, affordable Claude model", ProviderType: modelpb.ProviderType_PROVIDER_TYPE_ANTHROPIC, Family: "Claude-3.5", ReleaseDate: "2024-10-22", Status: modelpb.ModelStatus_MODEL_STATUS_ACTIVE, InputCostPerMillion: 0.80, OutputCostPerMillion: 4.00, CacheReadCostPerToken: 0.08, CacheWriteCostPerToken: 1.00, ContextWindow: 200000, MaxInputTokens: 200000, MaxOutputTokens: 8192, SupportsReasoning: true, SupportsToolCalling: true, SupportsTemperature: true, SupportsVision: true, InputModalities: []modelpb.Modality{modelpb.Modality_MODALITY_TEXT, modelpb.Modality_MODALITY_IMAGE_INPUT}, OutputModalities: []modelpb.Modality{modelpb.Modality_MODALITY_TEXT}, SupportsStreaming: false, RecommendedIndex: 2},
	{ModelID: "claude-opus-4-20250514", DisplayName: "Claude Opus 4", Description: "Most capable Claude model for complex tasks", ProviderType: modelpb.ProviderType_PROVIDER_TYPE_ANTHROPIC, Family: "Claude-4", ReleaseDate: "2025-05-14", Status: modelpb.ModelStatus_MODEL_STATUS_ACTIVE, InputCostPerMillion: 15.00, OutputCostPerMillion: 75.00, CacheReadCostPerToken: 1.50, CacheWriteCostPerToken: 18.75, ContextWindow: 200000, MaxInputTokens: 200000, MaxOutputTokens: 8192, SupportsReasoning: true, SupportsToolCalling: true, SupportsTemperature: true, SupportsVision: true, InputModalities: []modelpb.Modality{modelpb.Modality_MODALITY_TEXT, modelpb.Modality_MODALITY_IMAGE_INPUT}, OutputModalities: []modelpb.Modality{modelpb.Modality_MODALITY_TEXT}, SupportsStreaming: false, RecommendedIndex: 3},
	{ModelID: "gemini-2.0-flash", DisplayName: "Gemini 2.0 Flash", Description: "Fast, efficient Google model with strong multimodal support", ProviderType: modelpb.ProviderType_PROVIDER_TYPE_GOOGLE, Family: "Gemini-2.0", ReleaseDate: "2024-12-11", Status: modelpb.ModelStatus_MODEL_STATUS_ACTIVE, InputCostPerMillion: 0.10, OutputCostPerMillion: 0.40, ContextWindow: 1000000, MaxInputTokens: 1000000, MaxOutputTokens: 8192, SupportsReasoning: true, SupportsToolCalling: true, SupportsTemperature: true, SupportsVision: true, InputModalities: []modelpb.Modality{modelpb.Modality_MODALITY_TEXT, modelpb.Modality_MODALITY_IMAGE_INPUT, modelpb.Modality_MODALITY_AUDIO_INPUT, modelpb.Modality_MODALITY_VIDEO_INPUT}, OutputModalities: []modelpb.Modality{modelpb.Modality_MODALITY_TEXT, modelpb.Modality_MODALITY_AUDIO_OUTPUT}, SupportsStreaming: true, RecommendedIndex: 1},
	{ModelID: "gemini-2.5-pro-preview-06-05", DisplayName: "Gemini 2.5 Pro", Description: "Google's most capable model with thinking capabilities", ProviderType: modelpb.ProviderType_PROVIDER_TYPE_GOOGLE, Family: "Gemini-2.5", ReleaseDate: "2025-06-05", Status: modelpb.ModelStatus_MODEL_STATUS_BETA, InputCostPerMillion: 1.25, OutputCostPerMillion: 10.00, ContextWindow: 2000000, MaxInputTokens: 2000000, MaxOutputTokens: 32768, SupportsReasoning: true, SupportsToolCalling: true, SupportsTemperature: true, SupportsVision: true, InputModalities: []modelpb.Modality{modelpb.Modality_MODALITY_TEXT, modelpb.Modality_MODALITY_IMAGE_INPUT, modelpb.Modality_MODALITY_VIDEO_INPUT}, OutputModalities: []modelpb.Modality{modelpb.Modality_MODALITY_TEXT}, SupportsStreaming: true, RecommendedIndex: 2},
	{ModelID: "llama-3.1-70b-versatile", DisplayName: "Llama 3.1 70B Versatile", Description: "Fast open-source model via Groq", ProviderType: modelpb.ProviderType_PROVIDER_TYPE_GROQ, Family: "Llama-3.1", ReleaseDate: "2024-07-23", Status: modelpb.ModelStatus_MODEL_STATUS_ACTIVE, IsFree: false, InputCostPerMillion: 0.59, OutputCostPerMillion: 0.79, ContextWindow: 128000, MaxInputTokens: 128000, MaxOutputTokens: 32768, SupportsToolCalling: true, SupportsTemperature: true, InputModalities: []modelpb.Modality{modelpb.Modality_MODALITY_TEXT}, OutputModalities: []modelpb.Modality{modelpb.Modality_MODALITY_TEXT}, SupportsStreaming: true, RecommendedIndex: 1},
	{ModelID: "mixtral-8x7b-32768", DisplayName: "Mixtral 8x7B", Description: "Fast mixture-of-experts model via Groq", ProviderType: modelpb.ProviderType_PROVIDER_TYPE_GROQ, Family: "Mixtral", ReleaseDate: "2024-01-18", Status: modelpb.ModelStatus_MODEL_STATUS_ACTIVE, IsFree: false, InputCostPerMillion: 0.24, OutputCostPerMillion: 0.24, ContextWindow: 32768, MaxInputTokens: 32768, MaxOutputTokens: 32768, SupportsToolCalling: true, SupportsTemperature: true, InputModalities: []modelpb.Modality{modelpb.Modality_MODALITY_TEXT}, OutputModalities: []modelpb.Modality{modelpb.Modality_MODALITY_TEXT}, SupportsStreaming: true, RecommendedIndex: 2},
}

type Lookup struct {
	byID   map[string]*ModelConfig
	byType map[modelpb.ProviderType][]*ModelConfig
}

func NewLookup() *Lookup {
	l := &Lookup{
		byID:   make(map[string]*ModelConfig),
		byType: make(map[modelpb.ProviderType][]*ModelConfig),
	}
	for i := range Models {
		m := &Models[i]
		l.byID[m.ModelID] = m
		l.byType[m.ProviderType] = append(l.byType[m.ProviderType], m)
	}
	return l
}

func (l *Lookup) Get(id string) (*ModelConfig, bool) {
	m, ok := l.byID[id]
	return m, ok
}

func (l *Lookup) ByProvider(t modelpb.ProviderType) []*ModelConfig {
	return l.byType[t]
}

func (l *Lookup) GetProvider(t modelpb.ProviderType) *ProviderConfig {
	for i := range Providers {
		p := &Providers[i]
		if p.Type == t {
			return p
		}
	}
	return nil
}

func (c *ModelConfig) ToProto() *modelpb.PredefinedModel {
	return &modelpb.PredefinedModel{
		ModelId:      c.ModelID,
		DisplayName:  &c.DisplayName,
		Description:  &c.Description,
		ProviderType: c.ProviderType,
		Family:       &c.Family,
		ReleaseDate:  &c.ReleaseDate,
		Status:       c.Status,
		IsFree:       &c.IsFree,
		Cost: &modelpb.ModelCost{
			InputPerMillion:    c.InputCostPerMillion,
			OutputPerMillion:   c.OutputCostPerMillion,
			CacheReadPerToken:  c.CacheReadCostPerToken,
			CacheWritePerToken: c.CacheWriteCostPerToken,
		},
		ContextLimit: &modelpb.ModelContextLimit{
			ContextWindow:   c.ContextWindow,
			MaxInputTokens:  c.MaxInputTokens,
			MaxOutputTokens: c.MaxOutputTokens,
		},
		Capabilities: &modelpb.ModelCapabilities{
			SupportsReasoning:   c.SupportsReasoning,
			SupportsToolCalling: c.SupportsToolCalling,
			SupportsTemperature: c.SupportsTemperature,
			SupportsVision:      c.SupportsVision,
			SupportsStreaming:   c.SupportsStreaming,
			InputModalities:     c.InputModalities,
			OutputModalities:    c.OutputModalities,
		},
		RecommendedIndex: &c.RecommendedIndex,
	}
}

func (p *ProviderConfig) ToProto() *modelpb.PredefinedModelProvider {
	return &modelpb.PredefinedModelProvider{
		Type:              p.Type,
		Name:              &p.Name,
		BaseUrl:           &p.BaseURL,
		ApiKeyEnvVar:      &p.APikeyEnvVar,
		DefaultTimeoutMs:  p.DefaultTimeoutMs,
		DocumentationUrl:  &p.DocumentationURL,
		SupportsStreaming: p.SupportsStreaming,
	}
}

var GlobalLookup = NewLookup()
