package dashboard

import (
	"bytes"
	"encoding/json"
	"net/http"
	"net/http/httptest"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/orchestration"
)

func TestMeshBroadcast(t *testing.T) {
	tm := orchestration.NewTaskManager(nil, nil)
	srv := &Server{
		hub: orchestration.NewHub(nil, tm, nil),
	}

	payload := map[string]interface{}{
		"channel":  "mesh:tasks",
		"agent_id": "test_agent",
		"action":   "TEST",
		"status":   "OK",
	}
	body, _ := json.Marshal(payload)

	req := httptest.NewRequest("POST", "/api/mesh/broadcast", bytes.NewReader(body))
	w := httptest.NewRecorder()

	srv.handleMeshBroadcast(w, req)

	if w.Code != http.StatusOK {
		t.Fatalf("Expected 200, got %d", w.Code)
	}
}

func TestMeshDirect(t *testing.T) {
	tm := orchestration.NewTaskManager(nil, nil)
	srv := &Server{
		hub: orchestration.NewHub(nil, tm, nil),
	}

	payload := map[string]interface{}{
		"toAgent": "test_agent_2",
		"payload": "{}",
	}
	body, _ := json.Marshal(payload)

	req := httptest.NewRequest("POST", "/api/mesh/direct", bytes.NewReader(body))
	w := httptest.NewRecorder()

	srv.handleMeshDirect(w, req)

	if w.Code != http.StatusOK {
		t.Fatalf("Expected 200, got %d", w.Code)
	}
}

func TestMeshMailbox(t *testing.T) {
	tm := orchestration.NewTaskManager(nil, nil)
	srv := &Server{
		hub: orchestration.NewHub(nil, tm, nil),
	}

	req := httptest.NewRequest("GET", "/api/mesh/mailbox?agent_id=test_agent", nil)
	w := httptest.NewRecorder()

	srv.handleMeshMailbox(w, req)

	if w.Code != http.StatusOK {
		t.Fatalf("Expected 200, got %d", w.Code)
	}

	var resp struct {
		Messages []orchestration.Message `json:"messages"`
	}
	if err := json.Unmarshal(w.Body.Bytes(), &resp); err != nil {
		t.Fatalf("Failed to unmarshal response: %v", err)
	}

	if len(resp.Messages) != 0 {
		t.Fatalf("Expected 0 messages, got %d", len(resp.Messages))
	}
}
