package queue

import (
	"bytes"
	"context"
	"encoding/json"
	"net/http"
	"net/http/httptest"
	"testing"

	orchqueue "github.com/onehumancorp/mono/srcs/server/orchestration/queue"
)

type mockQueue struct {
	payloads []*orchqueue.SubAgentTaskQueuePayload
}

func (m *mockQueue) Enqueue(ctx context.Context, payload *orchqueue.SubAgentTaskQueuePayload) error {
	if payload.QueueName == "error-q" {
		return context.DeadlineExceeded
	}
	m.payloads = append(m.payloads, payload)
	return nil
}

func (m *mockQueue) Process(ctx context.Context, queueName string) (*orchqueue.SubAgentTaskQueuePayload, error) { return nil, nil }
func (m *mockQueue) Complete(ctx context.Context, jobID string, queueName string) error { return nil }
func (m *mockQueue) Fail(ctx context.Context, jobID string, queueName string, reason string) error { return nil }

func TestHandleSpawn_MethodNotAllowed(t *testing.T) {
	handler := HandleSpawn(func() orchqueue.SubAgentTaskQueue { return &mockQueue{} })
	req, _ := http.NewRequest(http.MethodGet, "/api/queue/spawn", nil)
	rr := httptest.NewRecorder()
	handler.ServeHTTP(rr, req)
	if rr.Code != http.StatusMethodNotAllowed {
		t.Errorf("Expected 405, got %d", rr.Code)
	}
}

func TestHandleSpawn_BadRequest(t *testing.T) {
	handler := HandleSpawn(func() orchqueue.SubAgentTaskQueue { return &mockQueue{} })
	req, _ := http.NewRequest(http.MethodPost, "/api/queue/spawn", bytes.NewBuffer([]byte("{invalid-json")))
	rr := httptest.NewRecorder()
	handler.ServeHTTP(rr, req)
	if rr.Code != http.StatusBadRequest {
		t.Errorf("Expected 400, got %d", rr.Code)
	}
}

func TestHandleSpawn_NilQueue(t *testing.T) {
	handler := HandleSpawn(func() orchqueue.SubAgentTaskQueue { return nil })
	reqBody := SpawnRequest{JobID: "1", QueueName: "q"}
	b, _ := json.Marshal(reqBody)
	req, _ := http.NewRequest(http.MethodPost, "/api/queue/spawn", bytes.NewBuffer(b))
	rr := httptest.NewRecorder()
	handler.ServeHTTP(rr, req)
	if rr.Code != http.StatusInternalServerError {
		t.Errorf("Expected 500, got %d", rr.Code)
	}
}

func TestHandleSpawn_EnqueueError(t *testing.T) {
	handler := HandleSpawn(func() orchqueue.SubAgentTaskQueue { return &mockQueue{} })
	reqBody := SpawnRequest{JobID: "1", QueueName: "error-q"}
	b, _ := json.Marshal(reqBody)
	req, _ := http.NewRequest(http.MethodPost, "/api/queue/spawn", bytes.NewBuffer(b))
	rr := httptest.NewRecorder()
	handler.ServeHTTP(rr, req)
	if rr.Code != http.StatusInternalServerError {
		t.Errorf("Expected 500, got %d", rr.Code)
	}
}

func TestHandleQueueSpawn(t *testing.T) {
	q := &mockQueue{}
	handler := HandleSpawn(func() orchqueue.SubAgentTaskQueue { return q })

	reqBody := SpawnRequest{JobID: "job-123", QueueName: "agent-queue", Data: orchqueue.SubAgentTaskData{IssueRef: "ref-1"}}
	b, _ := json.Marshal(reqBody)
	req, _ := http.NewRequest(http.MethodPost, "/api/queue/spawn", bytes.NewBuffer(b))
	rr := httptest.NewRecorder()
	handler.ServeHTTP(rr, req)

	if rr.Code != http.StatusAccepted {
		t.Errorf("Expected status %v, got %v", http.StatusAccepted, rr.Code)
	}
}
