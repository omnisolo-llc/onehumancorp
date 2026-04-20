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

type mockHarnessForInterceptor struct {
	executedCmd string
}

func (m *mockHarnessForInterceptor) Execute(ctx context.Context, command string) (Result, error) {
	m.executedCmd = command
	return Result{Stdout: "success"}, nil
}

func TestPermissionInterceptor(t *testing.T) {
	upgrader := websocket.Upgrader{}

	// Create a test server that acts as the Cloud orchestrator
	server := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		c, err := upgrader.Upgrade(w, r, nil)
		if err != nil {
			return
		}
		defer c.Close()

		for {
			_, message, err := c.ReadMessage()
			if err != nil {
				break
			}

			var req PermissionRequest
			if err := json.Unmarshal(message, &req); err != nil {
				continue
			}

			resp := AuthorizationResponse{
				RequestID: req.RequestID,
				Command: req.Command,
			}

			// Simple mock logic
			if strings.Contains(req.Command, "safe_tool") {
				resp.Allowed = true
			} else {
				resp.Allowed = false
				resp.ErrorMsg = "unsafe tool"
			}

			respMsg, _ := json.Marshal(resp)
			c.WriteMessage(websocket.TextMessage, respMsg)
		}
	}))
	defer server.Close()

	// Convert http:// to ws://
	wsURL := "ws" + strings.TrimPrefix(server.URL, "http")

	bridge, err := NewWebSocketBridge(wsURL)
	if err != nil {
		t.Fatalf("Failed to create bridge: %v", err)
	}
	defer bridge.Close()

	harness := &mockHarnessForInterceptor{}
	interceptor := NewPermissionInterceptor(bridge)

	t.Run("Allowed tool", func(t *testing.T) {
		ctx, cancel := context.WithTimeout(context.Background(), 2*time.Second)
		defer cancel()

		err := interceptor.CheckPermission(ctx, "safe_tool --arg")
		if err != nil {
			t.Fatalf("Expected success, got error: %v", err)
		}
		res, err := harness.Execute(ctx, "safe_tool --arg")
		if err != nil {
			t.Fatalf("Expected execute success, got error: %v", err)
		}
		if res.Stdout != "success" {
			t.Errorf("Expected stdout 'success', got '%s'", res.Stdout)
		}
		if harness.executedCmd != "safe_tool --arg" {
			t.Errorf("Expected command 'safe_tool --arg', got '%s'", harness.executedCmd)
		}
	})

	t.Run("Denied tool", func(t *testing.T) {
		ctx, cancel := context.WithTimeout(context.Background(), 2*time.Second)
		defer cancel()

		err := interceptor.CheckPermission(ctx, "rm -rf /")
		if err == nil {
			t.Fatal("Expected error for denied tool, got nil")
		}
		if !strings.Contains(err.Error(), "permission denied by bridge") {
			t.Errorf("Expected permission denied error, got: %v", err)
		}
		if !strings.Contains(err.Error(), "unsafe tool") {
			t.Errorf("Expected error to contain 'unsafe tool', got: %v", err)
		}
	})
}
