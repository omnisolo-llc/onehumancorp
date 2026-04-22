package tasks_test

import (
	"bytes"
	"context"
	"encoding/json"
	"net/http"
	"net/http/httptest"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/api/tasks"
	"github.com/onehumancorp/mono/srcs/server/db"
	"github.com/onehumancorp/mono/srcs/server/db/models"
)

type mockSharedTaskRepo struct {
	tasks map[string]*models.SharedTask
}

func (m *mockSharedTaskRepo) AcquireTask(ctx context.Context, organizationID, agentID string) (*db.TaskRecord, error) {
	return nil, nil
}

func (m *mockSharedTaskRepo) CreateSharedTask(ctx context.Context, task *models.SharedTask) error {
	if task.ID == "" {
		task.ID = "test-id"
	}
	task.Status = "PENDING"
	m.tasks[task.ID] = task
	return nil
}

func (m *mockSharedTaskRepo) GetSharedTasks(ctx context.Context, organizationID string) ([]*models.SharedTask, error) {
	return nil, nil
}

func (m *mockSharedTaskRepo) ClaimSharedTask(ctx context.Context, taskID, agentID string) (bool, error) {
	task, ok := m.tasks[taskID]
	if !ok {
		return false, nil
	}
	if task.Status != "PENDING" {
		return false, nil
	}
	task.Status = "IN_PROGRESS"
	return true, nil
}

func TestHandleEnqueueTask(t *testing.T) {
	repo := &mockSharedTaskRepo{tasks: make(map[string]*models.SharedTask)}
	server := tasks.NewServer(repo)

	reqBody := `{"organization_id": "org1", "title": "Test Task"}`
	req := httptest.NewRequest(http.MethodPost, "/api/queue/subagent", bytes.NewBufferString(reqBody))
	req.Header.Set("Content-Type", "application/json")
	w := httptest.NewRecorder()

	server.HandleEnqueueTask(w, req)

	res := w.Result()
	if res.StatusCode != http.StatusCreated {
		t.Errorf("expected status %d, got %d", http.StatusCreated, res.StatusCode)
	}

	var responseTask models.SharedTask
	if err := json.NewDecoder(res.Body).Decode(&responseTask); err != nil {
		t.Fatalf("failed to decode response: %v", err)
	}

	if responseTask.Title != "Test Task" {
		t.Errorf("expected title 'Test Task', got '%s'", responseTask.Title)
	}
}

func TestHandleClaimTask(t *testing.T) {
	repo := &mockSharedTaskRepo{tasks: make(map[string]*models.SharedTask)}
	repo.tasks["task1"] = &models.SharedTask{ID: "task1", Status: "PENDING"}

	server := tasks.NewServer(repo)

	reqBody := `{"task_id": "task1", "agent_id": "agent1"}`
	req := httptest.NewRequest(http.MethodPost, "/api/v1/tasks/claim", bytes.NewBufferString(reqBody))
	req.Header.Set("Content-Type", "application/json")
	w := httptest.NewRecorder()

	server.HandleClaimTask(w, req)

	res := w.Result()
	if res.StatusCode != http.StatusOK {
		t.Errorf("expected status %d, got %d", http.StatusOK, res.StatusCode)
	}

	// Try claiming again
	req2 := httptest.NewRequest(http.MethodPost, "/api/v1/tasks/claim", bytes.NewBufferString(reqBody))
	req2.Header.Set("Content-Type", "application/json")
	w2 := httptest.NewRecorder()

	server.HandleClaimTask(w2, req2)

	res2 := w2.Result()
	if res2.StatusCode != http.StatusConflict {
		t.Errorf("expected status %d, got %d", http.StatusConflict, res2.StatusCode)
	}
}
