package harness

import (
	"context"
	"encoding/json"
	"fmt"
	"sync"
	"time"

	"github.com/gorilla/websocket"
)

// PermissionRequest represents a request to the Cloud orchestrator to execute a tool.
type PermissionRequest struct {
	Command string `json:"command"`
}

// AuthorizationResponse represents the orchestrator's decision.
type AuthorizationResponse struct {
	Authorized bool   `json:"authorized"`
	Reason     string `json:"reason,omitempty"`
}

// BridgeTransport defines the interface for bidirectional communication with the Cloud.
type BridgeTransport interface {
	RequestPermission(ctx context.Context, req PermissionRequest) (*AuthorizationResponse, error)
}

// WebSocketBridge implements BridgeTransport using a WebSocket connection.
type WebSocketBridge struct {
	url  string
	conn *websocket.Conn
	mu   sync.Mutex
}

// NewWebSocketBridge creates a new WebSocketBridge.
func NewWebSocketBridge(url string) *WebSocketBridge {
	return &WebSocketBridge{url: url}
}

// Close closes the underlying websocket connection.
func (b *WebSocketBridge) Close() error {
	b.mu.Lock()
	defer b.mu.Unlock()
	if b.conn != nil {
		err := b.conn.Close()
		b.conn = nil
		return err
	}
	return nil
}

// RequestPermission connects to the Cloud, sends a PermissionRequest, and waits for an AuthorizationResponse.
func (b *WebSocketBridge) RequestPermission(ctx context.Context, req PermissionRequest) (*AuthorizationResponse, error) {
	b.mu.Lock()
	if b.conn == nil {
		dialer := websocket.Dialer{
			HandshakeTimeout: 10 * time.Second,
		}
		conn, _, err := dialer.DialContext(ctx, b.url, nil)
		if err != nil {
			b.mu.Unlock()
			return nil, fmt.Errorf("failed to dial websocket bridge: %w", err)
		}
		b.conn = conn
	}
	conn := b.conn
	b.mu.Unlock()

	reqData, err := json.Marshal(req)
	if err != nil {
		return nil, fmt.Errorf("failed to marshal permission request: %w", err)
	}

	b.mu.Lock()
	if err := conn.WriteMessage(websocket.TextMessage, reqData); err != nil {
		b.conn.Close()
		b.conn = nil
		b.mu.Unlock()
		return nil, fmt.Errorf("failed to write permission request: %w", err)
	}
	b.mu.Unlock()

	// Wait for response
	errChan := make(chan error, 1)
	respChan := make(chan *AuthorizationResponse, 1)

	go func() {
		_, respData, err := conn.ReadMessage()
		if err != nil {
			b.mu.Lock()
			if b.conn != nil {
				b.conn.Close()
				b.conn = nil
			}
			b.mu.Unlock()
			errChan <- err
			return
		}

		var resp AuthorizationResponse
		if err := json.Unmarshal(respData, &resp); err != nil {
			errChan <- err
			return
		}

		respChan <- &resp
	}()

	select {
	case <-ctx.Done():
		b.Close()
		return nil, ctx.Err()
	case err := <-errChan:
		return nil, fmt.Errorf("failed to read authorization response: %w", err)
	case resp := <-respChan:
		return resp, nil
	}
}
