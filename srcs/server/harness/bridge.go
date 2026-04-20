package harness

import (
	"context"
	"encoding/json"
	"fmt"
	"net/url"
	"sync"
	"time"

	"github.com/gorilla/websocket"
)

// PermissionRequest represents a request to execute a tool.
type PermissionRequest struct {
	RequestID string `json:"requestId"`
	Command   string `json:"command"`
}

// AuthorizationResponse represents the cloud's response to a PermissionRequest.
type AuthorizationResponse struct {
	RequestID string `json:"requestId"`
	Command   string `json:"command"`
	Allowed   bool   `json:"allowed"`
	ErrorMsg  string `json:"errorMsg,omitempty"`
}

// BridgeTransport handles bidirectional communication with the orchestrator.
type BridgeTransport interface {
	SendRequest(req PermissionRequest) error
	ReceiveResponse(ctx context.Context, requestID string) (AuthorizationResponse, error)
	Close() error
}

// WebSocketBridge implements BridgeTransport using Gorilla WebSocket.
type WebSocketBridge struct {
	conn       *websocket.Conn
	writeMu    sync.Mutex
	mu         sync.Mutex
	pending    map[string]chan AuthorizationResponse
	ctx        context.Context
	cancel     context.CancelFunc
	readErr    error
}

// NewWebSocketBridge connects to the specified URL and returns a BridgeTransport.
func NewWebSocketBridge(u string) (*WebSocketBridge, error) {
	parsedURL, err := url.Parse(u)
	if err != nil {
		return nil, fmt.Errorf("invalid url: %w", err)
	}

	conn, _, err := websocket.DefaultDialer.Dial(parsedURL.String(), nil)
	if err != nil {
		return nil, fmt.Errorf("failed to connect to bridge %s: %w", u, err)
	}

	ctx, cancel := context.WithCancel(context.Background())
	bridge := &WebSocketBridge{
		conn:    conn,
		pending: make(map[string]chan AuthorizationResponse),
		ctx:     ctx,
		cancel:  cancel,
	}

	go bridge.readLoop()

	return bridge, nil
}

func (b *WebSocketBridge) readLoop() {
	defer b.cancel()
	for {
		_, msg, err := b.conn.ReadMessage()
		if err != nil {
			b.mu.Lock()
			b.readErr = err
			b.mu.Unlock()
			// Close all pending channels
			b.mu.Lock()
			for _, ch := range b.pending {
				close(ch)
			}
			b.mu.Unlock()
			return
		}

		var resp AuthorizationResponse
		if err := json.Unmarshal(msg, &resp); err != nil {
			continue // skip invalid messages
		}

		b.mu.Lock()
		ch, ok := b.pending[resp.RequestID]
		b.mu.Unlock()
		if ok {
			select {
			case ch <- resp:
			default:
				// Avoid blocking if channel is full (e.g. duplicate response)
			}
		}
	}
}


// SendRequest sends a PermissionRequest over the websocket.
func (b *WebSocketBridge) SendRequest(req PermissionRequest) error {
	b.mu.Lock()
	if b.readErr != nil {
		err := b.readErr
		b.mu.Unlock()
		return fmt.Errorf("bridge connection closed: %w", err)
	}
	// Register pending channel
	ch := make(chan AuthorizationResponse, 1)
	b.pending[req.RequestID] = ch
	b.mu.Unlock()

	data, err := json.Marshal(req)
	if err != nil {
		b.cleanupPending(req.RequestID)
		return err
	}

	b.writeMu.Lock()
	defer b.writeMu.Unlock()
	if err := b.conn.WriteMessage(websocket.TextMessage, data); err != nil {
		b.cleanupPending(req.RequestID)
		return err
	}
	return nil
}

func (b *WebSocketBridge) cleanupPending(reqID string) {
	b.mu.Lock()
	defer b.mu.Unlock()
	delete(b.pending, reqID)
}

// ReceiveResponse blocks and waits for an AuthorizationResponse, respecting the provided context.
func (b *WebSocketBridge) ReceiveResponse(ctx context.Context, requestID string) (AuthorizationResponse, error) {
	b.mu.Lock()
	ch, ok := b.pending[requestID]
	b.mu.Unlock()

	if !ok {
		return AuthorizationResponse{}, fmt.Errorf("no pending request for ID %s", requestID)
	}

	defer b.cleanupPending(requestID)

	select {
	case resp, ok := <-ch:
		if !ok {
			b.mu.Lock()
			err := b.readErr
			b.mu.Unlock()
			if err == nil {
				err = fmt.Errorf("connection closed")
			}
			return AuthorizationResponse{}, fmt.Errorf("failed to receive response: %w", err)
		}
		return resp, nil
	case <-ctx.Done():
		return AuthorizationResponse{}, ctx.Err()
	case <-b.ctx.Done():
		return AuthorizationResponse{}, fmt.Errorf("bridge closed")
	}
}

// Close closes the underlying connection.
func (b *WebSocketBridge) Close() error {
	b.cancel()
	b.writeMu.Lock()
	defer b.writeMu.Unlock()

	err := b.conn.WriteControl(websocket.CloseMessage, websocket.FormatCloseMessage(websocket.CloseNormalClosure, ""), time.Now().Add(time.Second))
	b.conn.Close()
	return err
}
