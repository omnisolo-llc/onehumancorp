package kairos

import (
		"context"
			"encoding/json"
	"net/http"
	"net/http/httptest"
		"testing"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
	)


type MockMutex struct{}

func (m *MockMutex) Lock(ctx context.Context, ttl time.Duration) error { return nil }
func (m *MockMutex) Unlock(ctx context.Context) error                  { return nil }

type MockMutexProvider struct{}

func (p *MockMutexProvider) NewMutex(key string) Mutex { return &MockMutex{} }



func TestApprovalAPI_GetApprovals(t *testing.T) {
	ctx := context.Background()
	provider := db.NewTestProvider(t)

	_, err := provider.Exec(ctx, `
		CREATE TABLE IF NOT EXISTS shared_tasks (
			id TEXT PRIMARY KEY,
			agent_id TEXT,
			status TEXT,
			payload TEXT,
			action_risk TEXT,
			approval_status TEXT,
			proposed_content TEXT,
			created_at DATETIME
		);
	`)
	if err != nil {
		t.Fatalf("failed to create table: %v", err)
	}

	mesh := NewMemoryMesh()
	mutexProvider := &MockMutexProvider{}
	api := NewApprovalAPI(provider, mesh, mutexProvider)

	// Insert test data
	repo := NewSharedTaskRepo(provider)
	task := &SharedTask{
		ID:             "test-task-1",
		AgentID:        "agent-1",
		Status:         "PENDING",
		Payload:        []byte(`{}`),
		ActionRisk:     "High",
		ApprovalStatus: "PENDING",
		CreatedAt:      time.Now(),
	}
	_ = repo.Insert(ctx, task)

	t.Run("RequiresmTLS", func(t *testing.T) {
		req := createMockTLSRequest(http.MethodGet, "/api/kairos/approvals", nil, false)
		w := httptest.NewRecorder()
		api.HandleGetApprovals(w, req)
		if w.Code != http.StatusForbidden {
			t.Errorf("expected 403, got %d", w.Code)
		}
	})

	t.Run("ReturnsPendingApprovals", func(t *testing.T) {
		req := createMockTLSRequest(http.MethodGet, "/api/kairos/approvals", nil, true)
		w := httptest.NewRecorder()
		api.HandleGetApprovals(w, req)

		if w.Code != http.StatusOK {
			t.Errorf("expected 200, got %d", w.Code)
		}

		var resp []SharedTask
		if err := json.NewDecoder(w.Body).Decode(&resp); err != nil {
			t.Fatalf("failed to decode response: %v", err)
		}

		if len(resp) != 1 {
			t.Fatalf("expected 1 task, got %d", len(resp))
		}
		if resp[0].ID != "test-task-1" {
			t.Errorf("expected task id test-task-1, got %s", resp[0].ID)
		}
	})
}

func TestApprovalAPI_DecideApproval(t *testing.T) {
	ctx := context.Background()
	provider := db.NewTestProvider(t)

	_, err := provider.Exec(ctx, `
		CREATE TABLE IF NOT EXISTS shared_tasks (
			id TEXT PRIMARY KEY,
			agent_id TEXT,
			status TEXT,
			payload TEXT,
			action_risk TEXT,
			approval_status TEXT,
			proposed_content TEXT,
			created_at DATETIME
		);
	`)
	if err != nil {
		t.Fatalf("failed to create table: %v", err)
	}

	mesh := NewMemoryMesh()
	mutexProvider := &MockMutexProvider{}
	api := NewApprovalAPI(provider, mesh, mutexProvider)

	repo := NewSharedTaskRepo(provider)
	task := &SharedTask{
		ID:             "test-task-2",
		AgentID:        "agent-1",
		Status:         "PENDING",
		Payload:        []byte(`{}`),
		ActionRisk:     "High",
		ApprovalStatus: "PENDING",
		CreatedAt:      time.Now(),
	}
	_ = repo.Insert(ctx, task)

	t.Run("RequiresmTLS", func(t *testing.T) {
		req := createMockTLSRequest(http.MethodPost, "/api/kairos/approvals/decide", nil, false)
		w := httptest.NewRecorder()
		api.HandleDecideApproval(w, req)
		if w.Code != http.StatusForbidden {
			t.Errorf("expected 403, got %d", w.Code)
		}
	})

	t.Run("ApproveTask", func(t *testing.T) {
		reqBody := `{"task_id": "test-task-2", "decision": "APPROVED"}`
		req := createMockTLSRequest(http.MethodPost, "/api/kairos/approvals/decide", []byte(reqBody), true)
		w := httptest.NewRecorder()

		api.HandleDecideApproval(w, req)

		if w.Code != http.StatusOK {
			t.Errorf("expected 200, got %d", w.Code)
		}

		updatedTask, _ := repo.Get(ctx, "test-task-2")
		if updatedTask.ApprovalStatus != "APPROVED" {
			t.Errorf("expected APPROVED, got %s", updatedTask.ApprovalStatus)
		}
	})

	t.Run("AlreadyApproved", func(t *testing.T) {
		reqBody := `{"task_id": "test-task-2", "decision": "APPROVED"}`
		req := createMockTLSRequest(http.MethodPost, "/api/kairos/approvals/decide", []byte(reqBody), true)
		w := httptest.NewRecorder()

		api.HandleDecideApproval(w, req)

		if w.Code != http.StatusConflict {
			t.Errorf("expected 409 Conflict, got %d", w.Code)
		}
	})
}
