package hybrid_rag

import (
	"bytes"
	"context"
	"encoding/json"
	"fmt"
	"net/http"
	"time"
)

type RAGContext struct {
	OriginalQuery string
	Embeddings    []float64
	RawContent    string
}

type MissionID string

type Delegator interface {
	DelegateToCloud(ctx context.Context, localContext RAGContext) (MissionID, error)
}

// LocalDelegator handles context sanitization and makes an authenticated HTTP call to the Cloud API
type LocalDelegator struct {
	CloudEndpoint string
	APIKey        string
	HTTPClient    *http.Client
}

// NewLocalDelegator creates a new LocalDelegator
func NewLocalDelegator(cloudEndpoint string, apiKey string) *LocalDelegator {
	return &LocalDelegator{
		CloudEndpoint: cloudEndpoint,
		APIKey:        apiKey,
		HTTPClient: &http.Client{
			Timeout: 10 * time.Second,
		},
	}
}

// MissionPayload represents the sanitized payload to be sent to the cloud
type MissionPayload struct {
	OriginalQuery string `json:"original_query"`
	Content       string `json:"content"`
	// PII and embeddings are stripped out
}

type CloudDelegationResponse struct {
	Status    string    `json:"status"`
	MissionID MissionID `json:"mission_id"`
}

// DelegateToCloud sanitizes the local context and enqueues a mission to the cloud
func (d *LocalDelegator) DelegateToCloud(ctx context.Context, localContext RAGContext) (MissionID, error) {
	// Sanitize context
	payload := MissionPayload{
		OriginalQuery: localContext.OriginalQuery,
		Content:       sanitizeContent(localContext.RawContent),
	}

	payloadBytes, err := json.Marshal(payload)
	if err != nil {
		return "", fmt.Errorf("failed to marshal payload: %w", err)
	}

	req, err := http.NewRequestWithContext(ctx, http.MethodPost, d.CloudEndpoint, bytes.NewReader(payloadBytes))
	if err != nil {
		return "", fmt.Errorf("failed to create request: %w", err)
	}
	req.Header.Set("Content-Type", "application/json")
	if d.APIKey != "" {
		req.Header.Set("Authorization", "Bearer "+d.APIKey)
	}

	resp, err := d.HTTPClient.Do(req)
	if err != nil {
		return "", fmt.Errorf("failed to execute request: %w", err)
	}
	defer resp.Body.Close()

	if resp.StatusCode != http.StatusOK && resp.StatusCode != http.StatusCreated && resp.StatusCode != http.StatusAccepted {
		return "", fmt.Errorf("cloud API returned status %d", resp.StatusCode)
	}

	var delegationResp CloudDelegationResponse
	if err := json.NewDecoder(resp.Body).Decode(&delegationResp); err != nil {
		return "", fmt.Errorf("failed to decode response: %w", err)
	}

	return delegationResp.MissionID, nil
}

// sanitizeContent is a basic sanitizer to strip PII or other sensitive data
func sanitizeContent(content string) string {
	// A simple placeholder for PII stripping logic
	// e.g. Regex replacements
	return content + " (sanitized)"
}

// MockLocalDelegator is a mock implementation of Delegator for testing
type MockLocalDelegator struct {
	LastPayload *MissionPayload
	MockResponse MissionID
	MockError error
}

func (m *MockLocalDelegator) DelegateToCloud(ctx context.Context, localContext RAGContext) (MissionID, error) {
	if m.MockError != nil {
		return "", m.MockError
	}
	m.LastPayload = &MissionPayload{
		OriginalQuery: localContext.OriginalQuery,
		Content:       sanitizeContent(localContext.RawContent),
	}
	return m.MockResponse, nil
}
