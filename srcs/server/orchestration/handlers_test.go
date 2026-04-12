package orchestration

import (
	"bytes"
	"net/http"
	"net/http/httptest"
	"testing"
)

// We don't need a real db for all tests if we just want to verify status codes. Let's create a minimal test provider mock.
func TestHandleCreateTask_Unauthorized(t *testing.T) {
	// TaskManager doesn't actually hit DB for auth check
	tm := &TaskManager{}
	req := httptest.NewRequest(http.MethodPost, "/api/orchestration/tasks", bytes.NewBuffer([]byte(`{}`)))
	w := httptest.NewRecorder()

	handleCreateTask(w, req, tm)

	if w.Code != http.StatusUnauthorized {
		t.Errorf("Expected status %d, got %d", http.StatusUnauthorized, w.Code)
	}
}

func TestHandlePollTasks_Unauthorized(t *testing.T) {
	tm := &TaskManager{}
	req := httptest.NewRequest(http.MethodGet, "/api/orchestration/tasks?agent_id=agent1", nil)
	w := httptest.NewRecorder()

	handlePollTasks(w, req, tm)

	if w.Code != http.StatusInternalServerError {
		t.Errorf("Expected status %d, got %d", http.StatusInternalServerError, w.Code)
	}
}

func TestHandleUpdateTaskStatus_Unauthorized(t *testing.T) {
	tm := &TaskManager{}
	body := `{"status": "REVIEW", "agent_id": "agent1"}`
	req := httptest.NewRequest(http.MethodPut, "/api/orchestration/tasks/task-123/status", bytes.NewBuffer([]byte(body)))

	w := httptest.NewRecorder()
	handleUpdateTaskStatus(w, req, tm)

	if w.Code != http.StatusInternalServerError {
		t.Errorf("Expected status %d, got %d", http.StatusInternalServerError, w.Code)
	}
}

func TestRegisterTaskHTTPHandlers(t *testing.T) {
	mux := http.NewServeMux()
	tm := &TaskManager{}
	RegisterTaskHTTPHandlers(mux, tm)

	req := httptest.NewRequest(http.MethodGet, "/api/orchestration/tasks", nil)
	w := httptest.NewRecorder()
	mux.ServeHTTP(w, req)

	if w.Code != http.StatusBadRequest {
		t.Errorf("Expected status %d, got %d", http.StatusBadRequest, w.Code)
	}
}
