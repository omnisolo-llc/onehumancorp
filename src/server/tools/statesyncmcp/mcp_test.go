package statesyncmcp

import (
	"context"

	"net/http"
	"net/http/httptest"
	"testing"
)

func TestListTools(t *testing.T) {
	client := NewMCPClient("http://localhost")
	tools := client.ListTools()
	if len(tools) != 2 {
		t.Errorf("Expected 2 tools, got %d", len(tools))
	}
	if tools[0].Name != "crdt_push" || tools[1].Name != "crdt_pull" {
		t.Errorf("Unexpected tools: %v", tools)
	}
}

func TestCallToolUnknown(t *testing.T) {
	client := NewMCPClient("http://localhost")
	_, err := client.CallTool(context.Background(), "unknown", nil)
	if err == nil {
		t.Error("Expected error for unknown tool")
	}
}

func TestCRDTPushSuccess(t *testing.T) {
	server := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		if r.URL.Path != "/api/v1/sync/mcp-deltas" {
			t.Errorf("Unexpected path: %s", r.URL.Path)
		}
		w.WriteHeader(http.StatusOK)
	}))
	defer server.Close()

	client := NewMCPClient(server.URL)
	args := map[string]interface{}{
		"deltas": []map[string]interface{}{
			{"id": "1", "entity_id": "e1", "data": "d1", "updated_at": "2023-01-01T00:00:00Z"},
		},
	}
	res, err := client.CallTool(context.Background(), "crdt_push", args)
	if err != nil {
		t.Errorf("Unexpected error: %v", err)
	}
	if res != "success" {
		t.Errorf("Unexpected response: %v", res)
	}
}

func TestCRDTPull(t *testing.T) {
	client := NewMCPClient("http://localhost")
	res, err := client.CallTool(context.Background(), "crdt_pull", nil)
	if err != nil {
		t.Errorf("Unexpected error: %v", err)
	}
	payload, ok := res.(Payload)
	if !ok {
		t.Errorf("Unexpected response type")
	}
	if len(payload.Deltas) != 0 {
		t.Errorf("Expected 0 deltas, got %d", len(payload.Deltas))
	}
}
