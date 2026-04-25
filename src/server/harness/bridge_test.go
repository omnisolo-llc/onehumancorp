package harness

import (
	"context"
	"encoding/json"
	"net/http"
	"net/http/httptest"
	"testing"
)

func TestWebSocketBridge_RequestPermission_Authorized(t *testing.T) {
	server := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		var req PermissionRequest
		if err := json.NewDecoder(r.Body).Decode(&req); err != nil {
			t.Fatalf("failed to decode request: %v", err)
		}

		if req.Command != "echo test" {
			t.Errorf("expected command 'echo test', got %s", req.Command)
		}

		resp := AuthorizationResponse{Authorized: true}
		w.Header().Set("Content-Type", "application/json")
		json.NewEncoder(w).Encode(resp)
	}))
	defer server.Close()

	bridge := NewWebSocketBridge(server.URL)
	req := PermissionRequest{Command: "echo test"}

	resp, err := bridge.RequestPermission(context.Background(), req)
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}

	if !resp.Authorized {
		t.Errorf("expected authorized to be true")
	}
}

func TestWebSocketBridge_RequestPermission_Denied(t *testing.T) {
	server := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		var req PermissionRequest
		if err := json.NewDecoder(r.Body).Decode(&req); err != nil {
			t.Fatalf("failed to decode request: %v", err)
		}

		if req.Command != "rm -rf /" {
			t.Errorf("expected command 'rm -rf /', got %s", req.Command)
		}

		resp := AuthorizationResponse{Authorized: false, Reason: "dangerous command"}
		w.Header().Set("Content-Type", "application/json")
		json.NewEncoder(w).Encode(resp)
	}))
	defer server.Close()

	bridge := NewWebSocketBridge(server.URL)
	req := PermissionRequest{Command: "rm -rf /"}

	resp, err := bridge.RequestPermission(context.Background(), req)
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}

	if resp.Authorized {
		t.Errorf("expected authorized to be false")
	}
	if resp.Reason != "dangerous command" {
		t.Errorf("expected reason 'dangerous command', got %s", resp.Reason)
	}
}
