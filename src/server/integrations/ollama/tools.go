package ollama

import (
	"github.com/onehumancorp/mono/src/server/telemetry"
	"context"
	"encoding/json"
	"fmt"
	"io"
	"net/http"
	"strings"
	"time"
)

// MCP Tool definitions

// Tool represents an MCP tool definition.
type Tool struct {
	Name        string          `json:"name"`
	Description string          `json:"description"`
	InputSchema json.RawMessage `json:"inputSchema"`
}

// OllamaTool represents the MCP tools for managing local Ollama instances.
type OllamaTool struct {
	Client *http.Client
}

// NewOllamaTool initializes the Ollama MCP tool handler.
func NewOllamaTool() *OllamaTool {
	return &OllamaTool{
		Client: &http.Client{Timeout: 10 * time.Second},
	}
}

// ModelList represents the response from the Ollama /api/tags endpoint.
type ModelList struct {
	Models []struct {
		Name       string `json:"name"`
		ModifiedAt string `json:"modified_at"`
		Size       int64  `json:"size"`
	} `json:"models"`
}

// ListOllamaModels queries the local Ollama instance for downloaded models.
func (t *OllamaTool) ListOllamaModels(ctx context.Context, url string) (*ModelList, error) {
	if url == "" {
		url = "http://localhost:11434"
	}
	endpoint := fmt.Sprintf("%s/api/tags", strings.TrimRight(url, "/"))

	req, err := http.NewRequestWithContext(ctx, http.MethodGet, endpoint, nil)
	if err != nil {
		return nil, fmt.Errorf("failed to create request: %w", err)
	}

	resp, err := t.Client.Do(req)
	if err != nil {
		return nil, fmt.Errorf("failed to connect to ollama: %w", err)
	}
	defer resp.Body.Close()

	if resp.StatusCode != http.StatusOK {
		return nil, fmt.Errorf("ollama returned status %d", resp.StatusCode)
	}

	body, err := io.ReadAll(resp.Body)
	if err != nil {
		return nil, fmt.Errorf("failed to read response: %w", err)
	}

	var models ModelList
	if err := json.Unmarshal(body, &models); err != nil {
		return nil, fmt.Errorf("failed to parse ollama models: %w", err)
	}

	return &models, nil
}

// PullPayload represents the request body for pulling a model.
type PullPayload struct {
	Name   string `json:"name"`
	Stream bool   `json:"stream"`
}

// PullOllamaModel requests the Ollama instance to download a model.
func (t *OllamaTool) PullOllamaModel(ctx context.Context, url, modelName string) error {
	if url == "" {
		url = "http://localhost:11434"
	}
	if modelName == "" {
		return fmt.Errorf("model name is required")
	}

	endpoint := fmt.Sprintf("%s/api/pull", strings.TrimRight(url, "/"))

	payload := PullPayload{
		Name:   modelName,
		Stream: false,
	}

	jsonData, err := json.Marshal(telemetry.RedactInterfacePII(payload))
	if err != nil {
		return fmt.Errorf("failed to marshal payload: %w", err)
	}

	req, err := http.NewRequestWithContext(ctx, http.MethodPost, endpoint, strings.NewReader(string(jsonData)))
	if err != nil {
		return fmt.Errorf("failed to create request: %w", err)
	}
	req.Header.Set("Content-Type", "application/json")

	// Pulling can take a while, use a longer timeout locally
	client := &http.Client{Timeout: 5 * time.Minute}
	resp, err := client.Do(req)
	if err != nil {
		return fmt.Errorf("failed to connect to ollama: %w", err)
	}
	defer resp.Body.Close()

	if resp.StatusCode != http.StatusOK {
		body, _ := io.ReadAll(resp.Body)
		return fmt.Errorf("ollama returned status %d: %s", resp.StatusCode, string(body))
	}

	return nil
}

// GeneratePayload represents a raw generation request.
type GeneratePayload struct {
	Model  string `json:"model"`
	Prompt string `json:"prompt"`
	Stream bool   `json:"stream"`
}

// CheckOllamaHealth performs a raw generation check to ensure the model is functioning.
func (t *OllamaTool) CheckOllamaHealth(ctx context.Context, url, modelName string) (bool, error) {
	if url == "" {
		url = "http://localhost:11434"
	}
	if modelName == "" {
		return false, fmt.Errorf("model name is required for health check")
	}

	endpoint := fmt.Sprintf("%s/api/generate", strings.TrimRight(url, "/"))

	payload := GeneratePayload{
		Model:  modelName,
		Prompt: "Hello",
		Stream: false,
	}

	jsonData, err := json.Marshal(telemetry.RedactInterfacePII(payload))
	if err != nil {
		return false, fmt.Errorf("failed to marshal payload: %w", err)
	}

	req, err := http.NewRequestWithContext(ctx, http.MethodPost, endpoint, strings.NewReader(string(jsonData)))
	if err != nil {
		return false, fmt.Errorf("failed to create request: %w", err)
	}
	req.Header.Set("Content-Type", "application/json")

	resp, err := t.Client.Do(req)
	if err != nil {
		return false, fmt.Errorf("health check failed: %w", err)
	}
	defer resp.Body.Close()

	if resp.StatusCode == http.StatusOK {
		return true, nil
	}

	return false, fmt.Errorf("health check returned status %d", resp.StatusCode)
}
