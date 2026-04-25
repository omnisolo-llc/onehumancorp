package harness

import (
	"bytes"
	"context"
	"encoding/json"
	"fmt"
	"net/http"
	"time"
)

// PermissionRequest represents a request sent to the cloud orchestrator asking for permission to execute a tool.
type PermissionRequest struct {
	Command string `json:"command"`
}

// AuthorizationResponse represents the response from the cloud orchestrator granting or denying permission.
type AuthorizationResponse struct {
	Authorized bool   `json:"authorized"`
	Reason     string `json:"reason,omitempty"`
}

// BridgeTransport defines the interface for communicating with the remote OHC Hub (Cloud).
type BridgeTransport interface {
	// RequestPermission sends a permission request to the cloud and waits for an authorization response.
	RequestPermission(ctx context.Context, req PermissionRequest) (*AuthorizationResponse, error)
}

// WebSocketBridge implements BridgeTransport using an HTTP POST request (simplifying bidirectional stream for now).
// In a full implementation, this would establish a WebSocket or SSE connection.
type WebSocketBridge struct {
	Endpoint string
	Client   *http.Client
}

// NewWebSocketBridge creates a new WebSocketBridge connected to the specified endpoint.
func NewWebSocketBridge(endpoint string) *WebSocketBridge {
	return &WebSocketBridge{
		Endpoint: endpoint,
		Client: &http.Client{
			Timeout: 10 * time.Second,
		},
	}
}

// RequestPermission sends the request to the configured cloud endpoint.
func (b *WebSocketBridge) RequestPermission(ctx context.Context, req PermissionRequest) (*AuthorizationResponse, error) {
	payload, err := json.Marshal(req)
	if err != nil {
		return nil, fmt.Errorf("failed to marshal permission request: %w", err)
	}

	httpReq, err := http.NewRequestWithContext(ctx, "POST", b.Endpoint, bytes.NewReader(payload))
	if err != nil {
		return nil, fmt.Errorf("failed to create request: %w", err)
	}
	httpReq.Header.Set("Content-Type", "application/json")

	resp, err := b.Client.Do(httpReq)
	if err != nil {
		return nil, fmt.Errorf("failed to send permission request: %w", err)
	}
	defer resp.Body.Close()

	if resp.StatusCode != http.StatusOK {
		return nil, fmt.Errorf("cloud orchestrator returned non-200 status: %d", resp.StatusCode)
	}

	var authResp AuthorizationResponse
	if err := json.NewDecoder(resp.Body).Decode(&authResp); err != nil {
		return nil, fmt.Errorf("failed to decode authorization response: %w", err)
	}

	return &authResp, nil
}

// MockBridge is a simple mock implementation of BridgeTransport for testing.
type MockBridge struct {
	Response *AuthorizationResponse
	Err      error
}

// RequestPermission returns the preconfigured response or error.
func (m *MockBridge) RequestPermission(ctx context.Context, req PermissionRequest) (*AuthorizationResponse, error) {
	if m.Err != nil {
		return nil, m.Err
	}
	if m.Response != nil {
		return m.Response, nil
	}
	// Default allow
	return &AuthorizationResponse{Authorized: true}, nil
}
