package orchestration

import (
	"context"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/auth"
	"github.com/onehumancorp/mono/srcs/server/db"
)

func TestClaimTask_SQLite(t *testing.T) {
	dbProvider, err := db.NewSqliteProvider("file::memory:?cache=shared")
	if err != nil {
		t.Fatalf("failed to create sqlite provider: %v", err)
	}

	ctx := context.Background()

	// Create table manually since we might not run migrations in this test or wait for them
	_, err = dbProvider.Exec(ctx, `

		CREATE TABLE IF NOT EXISTS task_dependencies (
			task_id TEXT NOT NULL,
			depends_on_task_id TEXT NOT NULL,
			PRIMARY KEY (task_id, depends_on_task_id)
		)
	`)

	if err != nil {
		t.Fatalf("failed to create task_dependencies table: %v", err)
	}

	_, err = dbProvider.Exec(ctx, `
		CREATE TABLE IF NOT EXISTS shared_tasks (
			id TEXT PRIMARY KEY,
			organization_id TEXT NOT NULL,
			parent_plan_id TEXT,
			title TEXT NOT NULL,
			description TEXT,
			status TEXT NOT NULL DEFAULT 'PENDING',
			agent_id TEXT,
			dependencies JSONB,
			created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
			updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
		)
	`)
	if err != nil {
		t.Fatalf("failed to create table: %v", err)
	}

	// Insert a test task
	_, err = dbProvider.Exec(ctx, `
		INSERT INTO shared_tasks (id, organization_id, title, status)
		VALUES ('task-1', 'org-1', 'Test Task', 'PENDING')
	`)
	if err != nil {
		t.Fatalf("failed to insert test task: %v", err)
	}

	claims := &auth.Claims{
		OrganizationID: "org-1",
	}
	ctxWithClaims := context.WithValue(ctx, auth.ClaimsContextKeyForTest, claims)

	to := NewSharedTaskOrchestrator(dbProvider)

	// Claim the task
	task, err := to.ClaimTask(ctxWithClaims, "agent-1")
	if err != nil {
		t.Fatalf("ClaimTask failed: %v", err)
	}

	if task == nil {
		t.Fatalf("expected to claim a task, got nil")
	}

	if task.ID != "task-1" {
		t.Errorf("expected task ID 'task-1', got '%s'", task.ID)
	}

	if task.Status != "ASSIGNED" {
		t.Errorf("expected status 'ASSIGNED', got '%s'", task.Status)
	}

	if task.AssignedAgentID == nil || *task.AssignedAgentID != "agent-1" {
		t.Errorf("expected assigned agent 'agent-1', got '%v'", task.AssignedAgentID)
	}

	// Try to claim another task, should return nil
	task2, err := to.ClaimTask(ctxWithClaims, "agent-2")
	if err != nil {
		t.Fatalf("ClaimTask failed: %v", err)
	}

	if task2 != nil {
		t.Fatalf("expected nil task, got %v", task2)
	}
}

func TestClaimTask_Postgres(t *testing.T) {
	// Skip Postgres since it requires running instance for this pure db-layer test
	t.Skip("Postgres requires a running instance, skip for basic unit test")
}
