package ollama

import (
	"bytes"
	"context"
	"encoding/json"
	"fmt"
	"net/http"

	"onehumancorp/srcs/server/integrations"
)

type OllamaProvider struct {
	BaseURL string
	Client  *http.Client
}

func NewOllamaProvider(baseURL string) *OllamaProvider {
	if baseURL == "" {
		baseURL = "http://localhost:11434"
	}
	return &OllamaProvider{
		BaseURL: baseURL,
		Client:  &http.Client{},
	}
}

func (p *OllamaProvider) ID() string {
	return "ollama_mcp"
}

func (p *OllamaProvider) Initialize() error {
	return nil
}

func (p *OllamaProvider) Tools() []string {
	return []string{
		"ListOllamaModels",
		"PullOllamaModel",
		"CheckOllamaHealth",
	}
}

// OllamaTagsResponse represents the response from /api/tags
type OllamaTagsResponse struct {
	Models []struct {
		Name string `json:"name"`
	} `json:"models"`
}

func (p *OllamaProvider) ListOllamaModels(ctx context.Context) ([]string, error) {
	req, err := http.NewRequestWithContext(ctx, http.MethodGet, fmt.Sprintf("%s/api/tags", p.BaseURL), nil)
	if err != nil {
		return nil, fmt.Errorf("failed to create request: %w", err)
	}

	resp, err := p.Client.Do(req)
	if err != nil {
		return nil, fmt.Errorf("failed to fetch models: %w", err)
	}
	defer resp.Body.Close()

	if resp.StatusCode != http.StatusOK {
		return nil, fmt.Errorf("unexpected status code: %d", resp.StatusCode)
	}

	var tagsResp OllamaTagsResponse
	if err := json.NewDecoder(resp.Body).Decode(&tagsResp); err != nil {
		return nil, fmt.Errorf("failed to parse response: %w", err)
	}

	var models []string
	for _, m := range tagsResp.Models {
		models = append(models, m.Name)
	}
	return models, nil
}

func (p *OllamaProvider) PullOllamaModel(ctx context.Context, model string) error {
	payload := map[string]interface{}{
		"name": model,
		// Stream false so the request blocks until done
		"stream": false,
	}

	bodyBytes, _ := json.Marshal(payload)

	req, err := http.NewRequestWithContext(ctx, http.MethodPost, fmt.Sprintf("%s/api/pull", p.BaseURL), bytes.NewBuffer(bodyBytes))
	if err != nil {
		return fmt.Errorf("failed to create request: %w", err)
	}
	req.Header.Set("Content-Type", "application/json")

	resp, err := p.Client.Do(req)
	if err != nil {
		return fmt.Errorf("failed to pull model: %w", err)
	}
	defer resp.Body.Close()

	if resp.StatusCode != http.StatusOK {
		return fmt.Errorf("unexpected status code pulling model: %d", resp.StatusCode)
	}

	return nil
}

func (p *OllamaProvider) CheckOllamaHealth(ctx context.Context) (bool, error) {
	// Simple API check
	req, err := http.NewRequestWithContext(ctx, http.MethodGet, p.BaseURL, nil)
	if err != nil {
		return false, fmt.Errorf("failed to create request: %w", err)
	}

	resp, err := p.Client.Do(req)
	if err != nil {
		// Connection refused or other error means it's not healthy
		return false, nil
	}
	defer resp.Body.Close()

	if resp.StatusCode == http.StatusOK {
		return true, nil
	}
	return false, nil
}

func init() {
	integrations.Register(NewOllamaProvider(""))
}
