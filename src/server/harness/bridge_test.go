package harness

import (
	"context"
	"encoding/json"
	"net/http"
	"net/http/httptest"
	"strings"
	"testing"
	"time"

	"github.com/gorilla/websocket"
)

var upgrader = websocket.Upgrader{}

func TestWebSocketBridge_RequestPermission(t *testing.T) {
	// Start mock websocket server
	server := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		c, err := upgrader.Upgrade(w, r, nil)
		if err != nil {
			return
		}
		defer c.Close()

		for {
			mt, message, err := c.ReadMessage()
			if err != nil {
				break
			}
			var req PermissionRequest
			if err := json.Unmarshal(message, &req); err != nil {
				continue
			}

			// Mock authorization logic
			var resp AuthorizationResponse
			if req.Command == "allowed_tool" {
				resp = AuthorizationResponse{Authorized: true}
			} else {
				resp = AuthorizationResponse{Authorized: false, Reason: "tool not allowed"}
			}

			respData, _ := json.Marshal(resp)
			err = c.WriteMessage(mt, respData)
			if err != nil {
				break
			}
		}
	}))
	defer server.Close()

	// Convert http:// to ws://
	wsURL := "ws" + strings.TrimPrefix(server.URL, "http")
	bridge := NewWebSocketBridge(wsURL)

	t.Run("authorized request", func(t *testing.T) {
		ctx, cancel := context.WithTimeout(context.Background(), 2*time.Second)
		defer cancel()

		resp, err := bridge.RequestPermission(ctx, PermissionRequest{Command: "allowed_tool"})
		if err != nil {
			t.Fatalf("expected no error, got %v", err)
		}
		if !resp.Authorized {
			t.Errorf("expected authorized to be true")
		}
	})

	t.Run("denied request", func(t *testing.T) {
		ctx, cancel := context.WithTimeout(context.Background(), 2*time.Second)
		defer cancel()

		resp, err := bridge.RequestPermission(ctx, PermissionRequest{Command: "blocked_tool"})
		if err != nil {
			t.Fatalf("expected no error, got %v", err)
		}
		if resp.Authorized {
			t.Errorf("expected authorized to be false")
		}
		if resp.Reason != "tool not allowed" {
			t.Errorf("expected reason 'tool not allowed', got '%s'", resp.Reason)
		}
	})
}
