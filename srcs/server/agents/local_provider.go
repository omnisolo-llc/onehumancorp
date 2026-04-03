package agents

import (
	"bytes"
	"context"
	"encoding/json"
	"errors"
	"fmt"
	"io"
	"net/http"
	"time"

	"github.com/onehumancorp/mono/srcs/server/orchestration"
)

// LocalLLMProvider implements orchestration.MinimaxClient for local embedded LLMs.
type LocalLLMProvider struct {
	Endpoint  string
	ModelName string
	client    *http.Client
}

// NewLocalLLMProvider creates a new LocalLLMProvider.
func NewLocalLLMProvider(endpoint, modelName string) *LocalLLMProvider {
	if endpoint == "" {
		endpoint = "http://127.0.0.1:11434/api/generate"
	}
	if modelName == "" {
		modelName = "llama2"
	}
	return &LocalLLMProvider{
		Endpoint:  endpoint,
		ModelName: modelName,
		client: &http.Client{
			Timeout: 60 * time.Second,
		},
	}
}

// Reason generates a response from the local LLM.
func (p *LocalLLMProvider) Reason(ctx context.Context, prompt string) (string, error) {
	reqBody, err := json.Marshal(map[string]interface{}{
		"model":  p.ModelName,
		"prompt": prompt,
		"stream": false,
	})
	if err != nil {
		return "", err
	}

	req, err := http.NewRequestWithContext(ctx, "POST", p.Endpoint, bytes.NewReader(reqBody))
	if err != nil {
		return "", err
	}
	req.Header.Set("Content-Type", "application/json")

	resp, err := p.client.Do(req)
	if err != nil {
		return "", err
	}
	defer resp.Body.Close()

	if resp.StatusCode != http.StatusOK {
		return "", fmt.Errorf("local LLM returned status %d", resp.StatusCode)
	}

	respBody, err := io.ReadAll(resp.Body)
	if err != nil {
		return "", err
	}

	var result struct {
		Response string `json:"response"`
	}
	if err := json.Unmarshal(respBody, &result); err != nil {
		return "", err
	}

	return result.Response, nil
}

// GenerateEmbedding generates embeddings using the local LLM.
func (p *LocalLLMProvider) GenerateEmbedding(ctx context.Context, text string) ([]float32, error) {
	// For local provider, we might hit an embeddings endpoint
	// e.g., Ollama /api/embeddings
	endpoint := p.Endpoint
	// simple hack to replace generate with embeddings if it matches
	if len(endpoint) > 9 && endpoint[len(endpoint)-9:] == "/generate" {
		endpoint = endpoint[:len(endpoint)-9] + "/embeddings"
	}

	reqBody, err := json.Marshal(map[string]interface{}{
		"model":  p.ModelName,
		"prompt": text,
	})
	if err != nil {
		return nil, err
	}

	req, err := http.NewRequestWithContext(ctx, "POST", endpoint, bytes.NewReader(reqBody))
	if err != nil {
		return nil, err
	}
	req.Header.Set("Content-Type", "application/json")

	resp, err := p.client.Do(req)
	if err != nil {
		return nil, err
	}
	defer resp.Body.Close()

	if resp.StatusCode != http.StatusOK {
		return nil, fmt.Errorf("local LLM returned status %d", resp.StatusCode)
	}

	respBody, err := io.ReadAll(resp.Body)
	if err != nil {
		return nil, err
	}

	var result struct {
		Embedding []float32 `json:"embedding"`
	}
	if err := json.Unmarshal(respBody, &result); err != nil {
		return nil, err
	}

	return result.Embedding, nil
}

// ResilientProvider implements orchestration.MinimaxClient by wrapping a primary cloud provider and a fallback local provider.
type ResilientProvider struct {
	primary  orchestration.MinimaxClient
	fallback orchestration.MinimaxClient
}

// NewResilientProvider creates a new ResilientProvider.
func NewResilientProvider(primary, fallback orchestration.MinimaxClient) *ResilientProvider {
	return &ResilientProvider{
		primary:  primary,
		fallback: fallback,
	}
}

// Reason attempts to use the primary provider, and falls back if it encounters network or timeout errors.
func (p *ResilientProvider) Reason(ctx context.Context, prompt string) (string, error) {
	res, err := p.primary.Reason(ctx, prompt)
	if err != nil {
		// Log error, fallback to local provider
		// Ideally we check if error is network/timeout related, but for now we fallback on any error from primary
		res, fallbackErr := p.fallback.Reason(ctx, prompt)
		if fallbackErr != nil {
			return "", errors.Join(err, fallbackErr)
		}
		return res, nil
	}
	return res, nil
}

// GenerateEmbedding attempts to use the primary provider, and falls back if it encounters network or timeout errors.
func (p *ResilientProvider) GenerateEmbedding(ctx context.Context, text string) ([]float32, error) {
	res, err := p.primary.GenerateEmbedding(ctx, text)
	if err != nil {
		res, fallbackErr := p.fallback.GenerateEmbedding(ctx, text)
		if fallbackErr != nil {
			return nil, errors.Join(err, fallbackErr)
		}
		return res, nil
	}
	return res, nil
}
