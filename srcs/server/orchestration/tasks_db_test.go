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

	_, err = dbProvider.Exec(ctx, `
		CREATE TABLE IF NOT EXISTS task_dependencies (
			task_id TEXT NOT NULL,
			depends_on_task_id TEXT NOT NULL
		)
	`)
	if err != nil {
		t.Fatalf("failed to create table: %v", err)
	}

	// Insert a test task with unsatisfied dependency
	_, err = dbProvider.Exec(ctx, `
		INSERT INTO shared_tasks (id, organization_id, title, status)
		VALUES ('task-1', 'org-1', 'Test Task', 'PENDING'),
               ('task-dep', 'org-1', 'Dep Task', 'PENDING')
	`)
	if err != nil {
		t.Fatalf("failed to insert test task: %v", err)
	}

	_, err = dbProvider.Exec(ctx, `
		INSERT INTO task_dependencies (task_id, depends_on_task_id)
		VALUES ('task-1', 'task-dep')
	`)
	if err != nil {
		t.Fatalf("failed to insert test task dep: %v", err)
	}

	claims := &auth.Claims{
		OrganizationID: "org-1",
	}
	ctxWithClaims := context.WithValue(ctx, auth.ClaimsContextKeyForTest, claims)

	to := NewSharedTaskOrchestrator(dbProvider)

	// Try to claim task-1, should fail because dependency task-dep is PENDING
	task, err := to.ClaimTask(ctxWithClaims, "agent-1")
	if err != nil {
		t.Fatalf("ClaimTask failed: %v", err)
	}

	// 'task-dep' should be claimed instead since it has no dependencies
	if task == nil {
		t.Fatalf("expected to claim task-dep, got nil")
	}
	if task.ID != "task-dep" {
		t.Errorf("expected task ID 'task-dep', got '%s'", task.ID)
	}

	// Complete task-dep
	_, err = dbProvider.Exec(ctx, `
		UPDATE shared_tasks SET status = 'COMPLETED' WHERE id = 'task-dep'
	`)
	if err != nil {
		t.Fatalf("failed to complete task-dep: %v", err)
	}

	// Try to claim task-1 again, should succeed
	task2, err := to.ClaimTask(ctxWithClaims, "agent-1")
	if err != nil {
		t.Fatalf("ClaimTask failed: %v", err)
	}

	if task2 == nil {
		t.Fatalf("expected to claim task-1, got nil")
	}

	if task2.ID != "task-1" {
		t.Errorf("expected task ID 'task-1', got '%s'", task2.ID)
	}

	if task2.Status != "ASSIGNED" {
		t.Errorf("expected status 'ASSIGNED', got '%s'", task2.Status)
	}

	if task2.AssignedAgentID == nil || *task2.AssignedAgentID != "agent-1" {
		t.Errorf("expected assigned agent 'agent-1', got '%v'", task2.AssignedAgentID)
	}

	// Try to claim another task, should return nil
	task3, err := to.ClaimTask(ctxWithClaims, "agent-2")
	if err != nil {
		t.Fatalf("ClaimTask failed: %v", err)
	}

	if task3 != nil {
		t.Fatalf("expected nil task, got %v", task3)
	}
}

func TestClaimTask_Postgres(t *testing.T) {
	// Skip Postgres since it requires running instance for this pure db-layer test
	t.Skip("Postgres requires a running instance, skip for basic unit test")
}
