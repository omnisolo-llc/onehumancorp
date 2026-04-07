package orchestration


import (
	"context"
	"encoding/json"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/auth"
	"github.com/onehumancorp/mono/srcs/server/db"
)

func setupTestTasksDB(t *testing.T) (db.Provider, *TasksDB) {
	dbProvider, err := db.NewTestProvider()
	if err != nil {
		t.Fatalf("failed to create test provider: %v", err)
	}

	dbWrapper := dbProvider
	tasksDB := NewTasksDB(dbWrapper)

	query := `
CREATE TABLE IF NOT EXISTS shared_tasks (
    id TEXT PRIMARY KEY,
    organization_id TEXT NOT NULL,
    parent_plan_id TEXT,
    title TEXT NOT NULL,
    description TEXT,
    status TEXT NOT NULL DEFAULT 'PENDING',
    assigned_agent_id TEXT,
    dependencies JSONB,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);
`
	if _, err := dbProvider.Exec(context.Background(), query); err != nil {
		t.Fatalf("failed to create shared_tasks table: %v", err)
	}

	return dbWrapper, tasksDB
}

func TestTasksDB_ClaimTask(t *testing.T) {
	dbWrapper, tasksDB := setupTestTasksDB(t)
	defer dbWrapper.Close()

	ctx := context.Background()
	claims := &auth.Claims{
		UserID:         "test-user-123",
		OrganizationID: "test-org-123",
		Role:           "agent",
	}
	ctx = context.WithValue(ctx, auth.ClaimsContextKeyForTest, claims)

	deps, _ := json.Marshal([]string{"dep1"})
	_, err := dbWrapper.Exec(ctx, `
		INSERT INTO shared_tasks (id, organization_id, title, status, dependencies)
		VALUES ('task-123', 'test-org-123', 'Test Task', 'PENDING', $1)
	`, deps)
	if err != nil {
		t.Fatalf("failed to insert test task: %v", err)
	}

	task, err := tasksDB.ClaimTask(ctx, "agent-1")
	if err != nil {
		t.Fatalf("expected no error claiming task, got %v", err)
	}

	if task == nil {
		t.Fatalf("expected task to be claimed, got nil")
	}

	if task.ID != "task-123" {
		t.Errorf("expected task ID to be 'task-123', got '%s'", task.ID)
	}

	if task.Status != "ASSIGNED" {
		t.Errorf("expected task status to be 'ASSIGNED', got '%s'", task.Status)
	}

	if task.AssignedAgentID == nil || *task.AssignedAgentID != "agent-1" {
		t.Errorf("expected assigned agent ID to be 'agent-1', got %v", task.AssignedAgentID)
	}

	task2, err := tasksDB.ClaimTask(ctx, "agent-2")
	if err != nil {
		t.Fatalf("expected no error claiming non-existent task, got %v", err)
	}

	if task2 != nil {
		t.Fatalf("expected no task to be returned, got %v", task2)
	}
}

func TestTasksDB_ClaimTask_NoPending(t *testing.T) {
	dbWrapper, tasksDB := setupTestTasksDB(t)
	defer dbWrapper.Close()

	ctx := context.Background()
	task, err := tasksDB.ClaimTask(ctx, "agent-1")
	if err != nil {
		t.Fatalf("expected no error claiming when no tasks exist, got %v", err)
	}

	if task != nil {
		t.Fatalf("expected no task to be returned, got %v", task)
	}
}
