package models

import "time"

type ModelProviderRow struct {
	ID             string            `json:"id"`
	OrganizationID string            `json:"organization_id"`
	Type           string            `json:"type"`
	Name           string            `json:"name"`
	APIKeyEnvVar   string            `json:"api_key_env_var"`
	BaseURL        string            `json:"base_url"`
	TimeoutMs      int               `json:"timeout_ms"`
	ChunkTimeoutMs int               `json:"chunk_timeout_ms"`
	Headers        map[string]string `json:"headers"`
	Options        map[string]string `json:"options"`
	Enabled        bool              `json:"enabled"`
	EnvVars        []string          `json:"env_vars"`
	NPMPackage     string            `json:"npm_package"`
	CreatedAt      time.Time         `json:"created_at"`
	UpdatedAt      time.Time         `json:"updated_at"`
}

type ModelInstanceRow struct {
	ID                  string            `json:"id"`
	OrganizationID      string            `json:"organization_id"`
	ProviderID          string            `json:"provider_id"`
	Name                string            `json:"name"`
	ModelID             string            `json:"model_id"`
	DisplayName         string            `json:"display_name"`
	Description         string            `json:"description"`
	IconURL             string            `json:"icon_url"`
	IconColor           string            `json:"icon_color"`
	InputCostPerToken   float64           `json:"input_cost_per_token"`
	OutputCostPerToken  float64           `json:"output_cost_per_token"`
	CacheReadPerToken   float64           `json:"cache_read_per_token"`
	CacheWritePerToken  float64           `json:"cache_write_per_token"`
	ContextWindow       int               `json:"context_window"`
	MaxInputTokens      int               `json:"max_input_tokens"`
	MaxOutputTokens     int               `json:"max_output_tokens"`
	SupportsReasoning   bool              `json:"supports_reasoning"`
	SupportsToolCalling bool              `json:"supports_tool_calling"`
	SupportsTemperature bool              `json:"supports_temperature"`
	InputModalities     []string          `json:"input_modalities"`
	OutputModalities    []string          `json:"output_modalities"`
	Status              string            `json:"status"`
	IsFree              bool              `json:"is_free"`
	ReleaseDate         string            `json:"release_date"`
	Family              string            `json:"family"`
	Metadata            map[string]string `json:"metadata"`
	CreatedAt           time.Time         `json:"created_at"`
	UpdatedAt           time.Time         `json:"updated_at"`
}

type ModelBindingRow struct {
	ID              string    `json:"id"`
	OrganizationID  string    `json:"organization_id"`
	AgentID         string    `json:"agent_id"`
	ModelInstanceID string    `json:"model_instance_id"`
	IsDefault       bool      `json:"is_default"`
	Priority        int       `json:"priority"`
	CreatedAt       time.Time `json:"created_at"`
	UpdatedAt       time.Time `json:"updated_at"`
}
