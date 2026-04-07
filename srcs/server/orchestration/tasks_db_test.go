package orchestration

import (
	"context"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/auth"
	"github.com/onehumancorp/mono/srcs/server/db"
)

func setupTestDB(t *testing.T) db.Provider {
    d, err := db.NewTestProvider(":memory:")
    if err != nil {
        t.Fatalf("failed to create test provider: %v", err)
    }

	query := `
CREATE TABLE IF NOT EXISTS shared_tasks (
    id TEXT PRIMARY KEY,
    organization_id TEXT NOT NULL,
    parent_plan_id TEXT,
    title TEXT NOT NULL,
    description TEXT,
    status TEXT NOT NULL DEFAULT 'PENDING',
    agent_id TEXT,
    dependencies JSONB,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);
	`
	_, err = d.Exec(context.Background(), query)
	if err != nil {
		t.Fatalf("failed to create table: %v", err)
	}

	return d
}

func TestClaimTask_SQLite(t *testing.T) {
	provider := setupTestDB(t)
	defer provider.Close()

	tasksDB := NewTasksDB(provider)

	// inject claims
	claims := &auth.Claims{
		OrganizationID: "org-1",
		UserID:         "agent-1",
		Role:           "system",
	}
	ctx := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, claims)

	// Insert task
	insertQuery := `
		INSERT INTO shared_tasks (id, organization_id, title, status)
		VALUES ('task-1', 'org-1', 'Test Task', 'PENDING')
	`
	_, err := provider.Exec(ctx, insertQuery)
	if err != nil {
		t.Fatalf("failed to insert test data: %v", err)
	}

	// Attempt claim
	task, err := tasksDB.ClaimTask(ctx, "org-1", "agent-1")
	if err != nil {
		t.Fatalf("ClaimTask failed: %v", err)
	}

	if task == nil {
		t.Fatalf("expected task to be claimed, got nil")
	}

	if task.ID != "task-1" {
		t.Errorf("expected task-1, got %s", task.ID)
	}
	if task.Status != "ASSIGNED" {
		t.Errorf("expected ASSIGNED, got %s", task.Status)
	}
	if task.AssignedAgentID.String != "agent-1" {
		t.Errorf("expected agent-1, got %s", task.AssignedAgentID.String)
	}

	// Attempt another claim, should be nil since no PENDING tasks left
	task2, err := tasksDB.ClaimTask(ctx, "org-1", "agent-2")
	if err != nil {
		t.Fatalf("ClaimTask 2 failed: %v", err)
	}

	if task2 != nil {
		t.Errorf("expected nil task, got %v", task2)
	}
}

func TestClaimTask_NotFound(t *testing.T) {
	provider := setupTestDB(t)
	defer provider.Close()

	tasksDB := NewTasksDB(provider)
	ctx := context.Background()

	task, err := tasksDB.ClaimTask(ctx, "org-1", "agent-1")
	if err != nil {
		t.Fatalf("ClaimTask failed: %v", err)
	}
	if task != nil {
		t.Errorf("expected nil, got %v", task)
	}
}
