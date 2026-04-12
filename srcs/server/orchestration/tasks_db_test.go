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

func TestClaimTask_DAG_Dependencies(t *testing.T) {
	dbProvider, err := db.NewSqliteProvider("file::memory:?cache=shared")
	if err != nil {
		t.Fatalf("failed to create sqlite provider: %v", err)
	}

	ctx := context.Background()

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
		t.Fatalf("failed to create shared_tasks table: %v", err)
	}

	_, err = dbProvider.Exec(ctx, `
		CREATE TABLE IF NOT EXISTS task_dependencies (
			task_id TEXT REFERENCES shared_tasks(id) ON DELETE CASCADE,
			depends_on_task_id TEXT REFERENCES shared_tasks(id) ON DELETE CASCADE,
			PRIMARY KEY (task_id, depends_on_task_id)
		)
	`)
	if err != nil {
		t.Fatalf("failed to create task_dependencies table: %v", err)
	}

	_, err = dbProvider.Exec(ctx, `
		INSERT INTO shared_tasks (id, organization_id, title, status)
		VALUES ('task-1', 'org-1', 'Test Task 1', 'PENDING'),
		       ('task-2', 'org-1', 'Test Task 2', 'PENDING')
	`)
	if err != nil {
		t.Fatalf("failed to insert test tasks: %v", err)
	}

	// task-2 depends on task-1
	_, err = dbProvider.Exec(ctx, `
		INSERT INTO task_dependencies (task_id, depends_on_task_id)
		VALUES ('task-2', 'task-1')
	`)
	if err != nil {
		t.Fatalf("failed to insert task dependencies: %v", err)
	}

	claims := &auth.Claims{
		OrganizationID: "org-1",
	}
	ctxWithClaims := context.WithValue(ctx, auth.ClaimsContextKeyForTest, claims)

	to := NewSharedTaskOrchestrator(dbProvider)

	// Attempt to claim tasks. Should claim task-1 first because task-2 depends on task-1 and task-1 is not COMPLETED.
	taskA, err := to.ClaimTask(ctxWithClaims, "agent-1")
	if err != nil {
		t.Fatalf("ClaimTask failed: %v", err)
	}

	if taskA == nil {
		t.Fatalf("expected to claim task-1, got nil")
	}

	if taskA.ID != "task-1" {
		t.Errorf("expected task ID 'task-1', got '%s'", taskA.ID)
	}

	// Attempt to claim another task. Should return nil because task-2 is blocked.
	taskB, err := to.ClaimTask(ctxWithClaims, "agent-2")
	if err != nil {
		t.Fatalf("ClaimTask failed: %v", err)
	}

	if taskB != nil {
		t.Fatalf("expected nil task, got %v", taskB)
	}

	// Mark task-1 as COMPLETED
	_, err = dbProvider.Exec(ctx, `
		UPDATE shared_tasks SET status = 'COMPLETED' WHERE id = 'task-1'
	`)
	if err != nil {
		t.Fatalf("failed to complete task-1: %v", err)
	}

	// Now task-2 should be claimable
	taskC, err := to.ClaimTask(ctxWithClaims, "agent-2")
	if err != nil {
		t.Fatalf("ClaimTask failed: %v", err)
	}

	if taskC == nil {
		t.Fatalf("expected to claim task-2, got nil")
	}

	if taskC.ID != "task-2" {
		t.Errorf("expected task ID 'task-2', got '%s'", taskC.ID)
	}
}

func TestClaimTask_Postgres(t *testing.T) {
	// Skip Postgres since it requires running instance for this pure db-layer test
	t.Skip("Postgres requires a running instance, skip for basic unit test")
}
