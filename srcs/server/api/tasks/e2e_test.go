package tasks

import (
	"context"
	"net/http"
	"net/http/httptest"
	"strings"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/db"
)

func TestTasksEndToEnd(t *testing.T) {
	provider := db.NewTestProvider(t)
	defer provider.Close()

	_, err := provider.Exec(context.Background(), `CREATE TABLE IF NOT EXISTS shared_tasks (
		id VARCHAR PRIMARY KEY,
		title VARCHAR NOT NULL,
		status VARCHAR NOT NULL DEFAULT 'PENDING',
		assignee VARCHAR,
		created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
		updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
	)`)
	if err != nil {
		t.Fatalf("failed to create table: %v", err)
	}

	queue := NewTaskQueue(provider)
	mux := http.NewServeMux()
	RegisterRoutes(mux, queue)
	server := httptest.NewServer(mux)
	defer server.Close()

	// 1. Create a task
	req, _ := http.NewRequest(http.MethodPost, server.URL+"/api/tasks/create", strings.NewReader(`{"id": "e2e-1", "title": "E2E Test Task"}`))
	req.Header.Set("Content-Type", "application/json")
	resp, err := http.DefaultClient.Do(req)
	if err != nil {
		t.Fatalf("failed to create task: %v", err)
	}
	if resp.StatusCode != http.StatusCreated {
		t.Fatalf("expected status 201, got %d", resp.StatusCode)
	}

	// 2. Claim the task
	req, _ = http.NewRequest(http.MethodGet, server.URL+"/api/tasks/claim?agent_id=agent-e2e", nil)
	resp, err = http.DefaultClient.Do(req)
	if err != nil {
		t.Fatalf("failed to claim task: %v", err)
	}
	if resp.StatusCode != http.StatusOK {
		t.Fatalf("expected status 200, got %d", resp.StatusCode)
	}

	// 3. List tasks
	req, _ = http.NewRequest(http.MethodGet, server.URL+"/api/tasks/list", nil)
	resp, err = http.DefaultClient.Do(req)
	if err != nil {
		t.Fatalf("failed to list tasks: %v", err)
	}
	if resp.StatusCode != http.StatusOK {
		t.Fatalf("expected status 200, got %d", resp.StatusCode)
	}
}
