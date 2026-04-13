package orchestration

import (
	"context"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/auth"
	"github.com/onehumancorp/mono/srcs/server/db"
	"github.com/onehumancorp/mono/srcs/server/telemetry"
)

func TestClaimTask_SQLite(t *testing.T) {
	telemetry.InitTelemetry()
	dbProvider, err := db.NewSqliteProvider("file::memory:?cache=shared")
	if err != nil {
		t.Fatalf("failed to create sqlite provider: %v", err)
	}

	ctx := context.Background()

	// Create tables


	_, err = dbProvider.Exec(ctx, `
		CREATE TABLE IF NOT EXISTS shared_tasks_v4 (
			id TEXT PRIMARY KEY,
			organization_id TEXT NOT NULL,
			parent_plan_id TEXT,
			title TEXT NOT NULL,
			description TEXT,
			status TEXT NOT NULL DEFAULT 'PENDING',
			assigned_agent_id TEXT,
			dependencies JSONB,
			created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
			updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
		)
	`)
	if err != nil {
		t.Fatalf("failed to create swarm_tasks: %v", err)
	}

	_, err = dbProvider.Exec(ctx, `
		CREATE TABLE IF NOT EXISTS state_machine_transitions (
			id TEXT PRIMARY KEY,
			entity_id TEXT NOT NULL,
			entity_type TEXT NOT NULL,
			from_state TEXT NOT NULL,
			to_state TEXT NOT NULL,
			agent_id TEXT,
			reason TEXT,
			occurred_at DATETIME DEFAULT CURRENT_TIMESTAMP
		)
	`)
	if err != nil {
		t.Fatalf("failed to create state_machine_transitions: %v", err)
	}

	// Insert test tasks. task-2 depends on task-1 (which is COMPLETED)
	_, err = dbProvider.Exec(ctx, `
		INSERT INTO shared_tasks_v4 (id, organization_id, parent_plan_id, title, status, dependencies)
		VALUES ('task-1', 'org-1', 'p-1', 'Test Task 1', 'COMPLETED', '[]')
	`)
	if err != nil {
		t.Fatalf("failed to insert test task 1: %v", err)
	}

	_, err = dbProvider.Exec(ctx, `
		INSERT INTO shared_tasks_v4 (id, organization_id, parent_plan_id, title, status, dependencies)
		VALUES ('task-2', 'org-1', 'p-1', 'Test Task 2', 'PENDING', '["task-1"]')
	`)
	if err != nil {
		t.Fatalf("failed to insert test task 2: %v", err)
	}

	// task-3 depends on task-2 (which is PENDING)
	_, err = dbProvider.Exec(ctx, `
		INSERT INTO shared_tasks_v4 (id, organization_id, parent_plan_id, title, status, dependencies)
		VALUES ('task-3', 'org-1', 'p-1', 'Test Task 3', 'PENDING', '["task-2"]')
	`)
	if err != nil {
		t.Fatalf("failed to insert test task 3: %v", err)
	}

	claims := &auth.Claims{
		OrganizationID: "org-1",
	}
	ctxWithClaims := context.WithValue(ctx, auth.ClaimsContextKeyForTest, claims)

	to := NewSharedTaskOrchestrator(dbProvider)

	// Claim task-2 (since task-1 is COMPLETED)
	task, err := to.ClaimTask(ctxWithClaims, "agent-1")
	if err != nil {
		t.Fatalf("ClaimTask failed: %v", err)
	}

	if task == nil {
		t.Fatalf("expected to claim task-2, got nil")
	}

	if task.ID != "task-2" {
		t.Errorf("expected task ID 'task-2', got '%s'", task.ID)
	}

	if task.Status != "ASSIGNED" {
		t.Errorf("expected status 'ASSIGNED', got '%s'", task.Status)
	}

	// Try to claim task-3, should return nil because task-2 is not COMPLETED
	task3, err := to.ClaimTask(ctxWithClaims, "agent-2")
	if err != nil {
		t.Fatalf("ClaimTask failed: %v", err)
	}

	if task3 != nil {
		t.Fatalf("expected nil task for task-3, got %v", task3)
	}

	// Transition task-2
	err = to.TransitionTask(ctxWithClaims, "task-2", "agent-1", "ASSIGNED", "EXECUTING", "Starting work")
	if err != nil {
		t.Fatalf("TransitionTask failed: %v", err)
	}
}

func TestClaimTask_Postgres(t *testing.T) {
	telemetry.InitTelemetry()
	// We simulate the Postgres method by passing a SQLite provider and utilizing the db layer's `convertBindVars`
	// which implicitly handles FOR UPDATE SKIP LOCKED compatibility when run against SQLite.
	dbProvider, err := db.NewSqliteProvider("file::memory:?cache=shared")
	if err != nil {
		t.Fatalf("failed to create sqlite provider: %v", err)
	}

	ctx := context.Background()



	_, err = dbProvider.Exec(ctx, `
		CREATE TABLE IF NOT EXISTS shared_tasks_v4 (
			id TEXT PRIMARY KEY,
			organization_id TEXT NOT NULL,
			parent_plan_id TEXT,
			title TEXT NOT NULL,
			description TEXT,
			status TEXT NOT NULL DEFAULT 'PENDING',
			assigned_agent_id TEXT,
			dependencies JSONB,
			created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
			updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
		)
	`)
	if err != nil {
		t.Fatalf("failed to create swarm_tasks: %v", err)
	}

	_, err = dbProvider.Exec(ctx, `
		CREATE TABLE IF NOT EXISTS state_machine_transitions (
			id TEXT PRIMARY KEY,
			entity_id TEXT NOT NULL,
			entity_type TEXT NOT NULL,
			from_state TEXT NOT NULL,
			to_state TEXT NOT NULL,
			agent_id TEXT,
			reason TEXT,
			occurred_at DATETIME DEFAULT CURRENT_TIMESTAMP
		)
	`)
	if err != nil {
		t.Fatalf("failed to create state_machine_transitions: %v", err)
	}

	_, err = dbProvider.Exec(ctx, "INSERT INTO shared_tasks_v4 (id, organization_id, parent_plan_id, title, status, dependencies) VALUES ('task-1', 'org-1', 'p-1', 'Test Task 1', 'COMPLETED', '[]')")
	if err != nil {
		t.Fatalf("failed to insert: %v", err)
	}

	_, err = dbProvider.Exec(ctx, `INSERT INTO shared_tasks_v4 (id, organization_id, parent_plan_id, title, status, dependencies) VALUES ('task-2', 'org-1', 'p-1', 'Test Task 2', 'PENDING', '["task-1"]')`)
	if err != nil {
		t.Fatalf("failed to insert: %v", err)
	}

	to := NewSharedTaskOrchestrator(dbProvider)

	// Direct call to Postgres branch logic, SQLite provider ignores FOR UPDATE SKIP LOCKED
	task, err := to.claimTaskPostgres(ctx, "org-1", "agent-pg")
	if err != nil {
		t.Fatalf("claimTaskPostgres failed: %v", err)
	}

	if task == nil {
		t.Fatalf("expected to claim task-2, got nil")
	}

	if task.ID != "task-2" {
		t.Errorf("expected task ID 'task-2', got '%s'", task.ID)
	}

	if task.Status != "ASSIGNED" {
		t.Errorf("expected status 'ASSIGNED', got '%s'", task.Status)
	}
}
