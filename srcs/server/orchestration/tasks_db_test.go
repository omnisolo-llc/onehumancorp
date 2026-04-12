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

func TestClaimTask_Postgres(t *testing.T) {
	// Skip Postgres since it requires running instance for this pure db-layer test
	t.Skip("Postgres requires a running instance, skip for basic unit test")
}

func TestUpdateSwarmTaskStatus(t *testing.T) {
	dbProvider, err := db.NewSqliteProvider("file::memory:?cache=shared")
	if err != nil {
		t.Fatalf("failed to create sqlite provider: %v", err)
	}
	defer dbProvider.Close()

	ctx := context.Background()

	// Create table manually
	_, err = dbProvider.Exec(ctx, `
		CREATE TABLE IF NOT EXISTS swarm_tasks (
			id TEXT PRIMARY KEY,
			status TEXT NOT NULL,
			assigned_agent_id TEXT,
			updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
		)
	`)
	if err != nil {
		t.Fatalf("failed to create swarm_tasks table: %v", err)
	}

	_, err = dbProvider.Exec(ctx, `
		CREATE TABLE IF NOT EXISTS state_machine_transitions (
			id TEXT PRIMARY KEY,
			entity_id TEXT NOT NULL,
			entity_type TEXT NOT NULL,
			from_state TEXT NOT NULL,
			to_state TEXT NOT NULL,
			agent_id TEXT,
			reason TEXT
		)
	`)
	if err != nil {
		t.Fatalf("failed to create state_machine_transitions table: %v", err)
	}

	// Insert test data
	taskID := "task-1"
	_, err = dbProvider.Exec(ctx, `
		INSERT INTO swarm_tasks (id, status) VALUES ($1, 'PENDING')
	`, taskID)
	if err != nil {
		t.Fatalf("failed to insert test task: %v", err)
	}

	to := NewSharedTaskOrchestrator(dbProvider)

	// Valid transition
	err = to.UpdateSwarmTaskStatus(ctx, taskID, "agent-1", "PENDING", "IN_PROGRESS", "started work")
	if err != nil {
		t.Errorf("expected success on valid transition, got error: %v", err)
	}

	// Invalid transition
	err = to.UpdateSwarmTaskStatus(ctx, taskID, "agent-1", "PENDING", "COMPLETED", "done")
	if err == nil {
		t.Errorf("expected error on invalid transition, got success")
	}

	// Another valid transition
	err = to.UpdateSwarmTaskStatus(ctx, taskID, "agent-1", "IN_PROGRESS", "COMPLETED", "done")
	if err != nil {
		t.Errorf("expected success on valid transition, got error: %v", err)
	}
}
