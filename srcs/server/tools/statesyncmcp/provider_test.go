package statesyncmcp

import (
	"context"
	"encoding/json"
	"net/http"
	"net/http/httptest"
	"onehumancorp/srcs/server/orchestration"
	"testing"
	"time"
)

type mockLocalDB struct {
	tasks      []*orchestration.SharedTask
	getErr     error
	getByIDRes *orchestration.SharedTask
	getByIDErr error
	createErr  error
	updateErr  error
}

func (m *mockLocalDB) ClaimTask(ctx context.Context, organizationID string, agentID string) (*orchestration.SharedTask, error) {
	return nil, nil
}

func (m *mockLocalDB) CreateTask(ctx context.Context, task *orchestration.SharedTask) error {
	return m.createErr
}

func (m *mockLocalDB) GetTask(ctx context.Context, id string) (*orchestration.SharedTask, error) {
	return m.getByIDRes, m.getByIDErr
}

func (m *mockLocalDB) DeleteStuckTasks(ctx context.Context) error {
	return nil
}

func (m *mockLocalDB) UpdateTaskStatus(ctx context.Context, id string, status string) error {
	return m.updateErr
}

func (m *mockLocalDB) GetTasksByOrganization(ctx context.Context, organizationID string) ([]*orchestration.SharedTask, error) {
	return m.tasks, m.getErr
}

func TestDefaultProvider_SyncUp(t *testing.T) {
	server := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		if r.Header.Get("X-Organization-ID") != "org-1" {
			w.WriteHeader(http.StatusUnauthorized)
			return
		}
		w.WriteHeader(http.StatusOK)
	}))
	defer server.Close()

	db := &mockLocalDB{
		tasks: []*orchestration.SharedTask{
			{ID: "task-1", Status: "PENDING"},
			{ID: "task-2", Status: "COMPLETED"},
			{ID: "task-3", Status: "IN_PROGRESS"},
		},
	}

	provider := NewDefaultProvider(db, server.URL)
	count, err := provider.SyncUp(context.Background(), "org-1")
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	// Should sync all 3 using bulk
	if count != 3 {
		t.Errorf("expected 3 tasks synced, got %d", count)
	}
}

func TestDefaultProvider_SyncDown_LWW(t *testing.T) {
	now := time.Now()
	olderTime := now.Add(-1 * time.Hour)
	newerTime := now.Add(1 * time.Hour)

	server := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		tasks := []*orchestration.SharedTask{
			{ID: "task-old", Status: "COMPLETED", UpdatedAt: olderTime},
			{ID: "task-new", Status: "COMPLETED", UpdatedAt: newerTime},
		}
		w.WriteHeader(http.StatusOK)
		json.NewEncoder(w).Encode(tasks)
	}))
	defer server.Close()

	db := &mockLocalDB{
		getByIDRes: &orchestration.SharedTask{
			UpdatedAt: now,
			Status:    "PENDING",
		},
	}

	provider := NewDefaultProvider(db, server.URL)
	count, err := provider.SyncDown(context.Background(), "org-1")
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	if count != 1 {
		t.Errorf("expected 1 task synced, got %d", count)
	}
}

func TestDefaultProvider_GetStatus(t *testing.T) {
	server := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		status := SyncStatus{PendingTasks: 5, Status: "healthy"}
		w.WriteHeader(http.StatusOK)
		json.NewEncoder(w).Encode(status)
	}))
	defer server.Close()

	provider := NewDefaultProvider(&mockLocalDB{}, server.URL)
	status, err := provider.GetStatus(context.Background(), "org-1")
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	if status.PendingTasks != 5 || status.Status != "healthy" {
		t.Errorf("unexpected status: %+v", status)
	}
}

func TestNoOpProvider(t *testing.T) {
	provider := NewNoOpProvider()

	count, err := provider.SyncUp(context.Background(), "org-1")
	if err != nil || count != 0 {
		t.Errorf("expected 0, nil; got %d, %v", count, err)
	}

	count, err = provider.SyncDown(context.Background(), "org-1")
	if err != nil || count != 0 {
		t.Errorf("expected 0, nil; got %d, %v", count, err)
	}

	status, err := provider.GetStatus(context.Background(), "org-1")
	if err != nil || status.Status != "cloud_native" {
		t.Errorf("expected cloud_native, nil; got %v, %v", status, err)
	}
}
