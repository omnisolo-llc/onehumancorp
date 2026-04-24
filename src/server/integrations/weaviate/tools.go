package weaviate

import (
	"bytes"
	"context"
	"encoding/json"
	"fmt"
	"io"
	"net/http"
	"strings"
	"time"
)

// WeaviateTool represents the MCP tools for managing Weaviate instances.
type WeaviateTool struct {
	Client *http.Client
}

// NewWeaviateTool initializes the Weaviate MCP tool handler.
func NewWeaviateTool() *WeaviateTool {
	return &WeaviateTool{
		Client: &http.Client{Timeout: 30 * time.Second},
	}
}

// QueryPayload represents the GraphQL query body.
type QueryPayload struct {
	Query string `json:"query"`
}

// WeaviateQuery performs a GraphQL query against the Weaviate instance.
func (t *WeaviateTool) WeaviateQuery(ctx context.Context, url, apiKey, query string) (map[string]interface{}, error) {
	if url == "" {
		url = "http://localhost:8080"
	}
	if query == "" {
		return nil, fmt.Errorf("query is required")
	}

	endpoint := fmt.Sprintf("%s/v1/graphql", strings.TrimRight(url, "/"))

	payload := QueryPayload{
		Query: query,
	}

	jsonData, err := json.Marshal(payload)
	if err != nil {
		return nil, fmt.Errorf("failed to marshal payload: %w", err)
	}

	req, err := http.NewRequestWithContext(ctx, http.MethodPost, endpoint, bytes.NewReader(jsonData))
	if err != nil {
		return nil, fmt.Errorf("failed to create request: %w", err)
	}
	req.Header.Set("Content-Type", "application/json")
	if apiKey != "" {
		req.Header.Set("Authorization", fmt.Sprintf("Bearer %s", apiKey))
	}

	resp, err := t.Client.Do(req)
	if err != nil {
		return nil, fmt.Errorf("failed to connect to weaviate: %w", err)
	}
	defer resp.Body.Close()

	body, err := io.ReadAll(resp.Body)
	if err != nil {
		return nil, fmt.Errorf("failed to read response: %w", err)
	}

	if resp.StatusCode != http.StatusOK {
		return nil, fmt.Errorf("weaviate returned status %d: %s", resp.StatusCode, string(body))
	}

	var result map[string]interface{}
	if err := json.Unmarshal(body, &result); err != nil {
		return nil, fmt.Errorf("failed to parse weaviate response: %w", err)
	}

	return result, nil
}

// InsertPayload represents the object insertion body.
type InsertPayload struct {
	Class      string                 `json:"class"`
	Properties map[string]interface{} `json:"properties"`
	Vector     []float32              `json:"vector,omitempty"`
}

// WeaviateInsert inserts a new object into a Weaviate collection.
func (t *WeaviateTool) WeaviateInsert(ctx context.Context, url, apiKey, class string, properties map[string]interface{}, vector []float32) (map[string]interface{}, error) {
	if url == "" {
		url = "http://localhost:8080"
	}
	if class == "" {
		return nil, fmt.Errorf("class is required")
	}

	endpoint := fmt.Sprintf("%s/v1/objects", strings.TrimRight(url, "/"))

	payload := InsertPayload{
		Class:      class,
		Properties: properties,
		Vector:     vector,
	}

	jsonData, err := json.Marshal(payload)
	if err != nil {
		return nil, fmt.Errorf("failed to marshal payload: %w", err)
	}

	req, err := http.NewRequestWithContext(ctx, http.MethodPost, endpoint, bytes.NewReader(jsonData))
	if err != nil {
		return nil, fmt.Errorf("failed to create request: %w", err)
	}
	req.Header.Set("Content-Type", "application/json")
	if apiKey != "" {
		req.Header.Set("Authorization", fmt.Sprintf("Bearer %s", apiKey))
	}

	resp, err := t.Client.Do(req)
	if err != nil {
		return nil, fmt.Errorf("failed to connect to weaviate: %w", err)
	}
	defer resp.Body.Close()

	body, err := io.ReadAll(resp.Body)
	if err != nil {
		return nil, fmt.Errorf("failed to read response: %w", err)
	}

	if resp.StatusCode != http.StatusOK {
		return nil, fmt.Errorf("weaviate returned status %d: %s", resp.StatusCode, string(body))
	}

	var result map[string]interface{}
	if err := json.Unmarshal(body, &result); err != nil {
		return nil, fmt.Errorf("failed to parse weaviate response: %w", err)
	}

	return result, nil
}

// WeaviateSchema fetches the current schema from Weaviate.
func (t *WeaviateTool) WeaviateSchema(ctx context.Context, url, apiKey string) (map[string]interface{}, error) {
	if url == "" {
		url = "http://localhost:8080"
	}

	endpoint := fmt.Sprintf("%s/v1/schema", strings.TrimRight(url, "/"))

	req, err := http.NewRequestWithContext(ctx, http.MethodGet, endpoint, nil)
	if err != nil {
		return nil, fmt.Errorf("failed to create request: %w", err)
	}
	if apiKey != "" {
		req.Header.Set("Authorization", fmt.Sprintf("Bearer %s", apiKey))
	}

	resp, err := t.Client.Do(req)
	if err != nil {
		return nil, fmt.Errorf("failed to connect to weaviate: %w", err)
	}
	defer resp.Body.Close()

	body, err := io.ReadAll(resp.Body)
	if err != nil {
		return nil, fmt.Errorf("failed to read response: %w", err)
	}

	if resp.StatusCode != http.StatusOK {
		return nil, fmt.Errorf("weaviate returned status %d: %s", resp.StatusCode, string(body))
	}

	var result map[string]interface{}
	if err := json.Unmarshal(body, &result); err != nil {
		return nil, fmt.Errorf("failed to parse weaviate response: %w", err)
	}

	return result, nil
}
