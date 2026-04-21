package orchestration

import (
	"bytes"
	"net/http"
	"net/http/httptest"
	"testing"
)

func TestCentrifugeNode_HandleV1Broadcast(t *testing.T) {
	node, err := NewCentrifugeNode()
	if err != nil {
		t.Fatalf("failed to create node: %v", err)
	}
	defer node.Close()

	req := httptest.NewRequest(http.MethodPost, "/api/v1/mesh/broadcast", bytes.NewBuffer([]byte(`{"task_id":"123"}`)))
	w := httptest.NewRecorder()

	node.HandleV1Broadcast(w, req)

	if w.Code != http.StatusOK {
		t.Errorf("Expected status OK, got %v", w.Code)
	}
}
