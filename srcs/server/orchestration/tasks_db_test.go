package orchestration

import (
	"context"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/auth"
	"github.com/onehumancorp/mono/srcs/server/db"
)

func setupTasksDBTest(t *testing.T) (*TasksDB, func()) {
	t.Helper()
	prov := db.NewTestProvider(t)

	_, err := prov.Exec(context.Background(), `
		CREATE TABLE IF NOT EXISTS shared_tasks (
			id TEXT PRIMARY KEY,
			organization_id TEXT NOT NULL,
			title TEXT NOT NULL,
			description TEXT,
			status TEXT NOT NULL DEFAULT 'PENDING',
			agent_id TEXT,
			priority TEXT NOT NULL DEFAULT 'P2',
			payload JSONB,
			created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
			updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
		);
	`)
	if err != nil {
		t.Fatalf("failed to create table: %v", err)
	}

	tdb := NewTasksDB(prov)

	return tdb, func() {
		prov.Close()
	}
}

func TestTasksDB_ClaimTask(t *testing.T) {
	t.Setenv("OHC_STANDALONE", "true")

	tdb, cleanup := setupTasksDBTest(t)
	defer cleanup()

	claims := &auth.Claims{OrganizationID: "org-1"}
	ctx := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, claims)

	// Claim when empty
	task, err := tdb.ClaimTask(ctx, "agent-1")
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}
	if task != nil {
		t.Fatalf("expected nil task when empty, got %v", task)
	}

	// Insert task
	_, err = tdb.db.Exec(ctx, `INSERT INTO shared_tasks (id, organization_id, title) VALUES ('task-1', 'org-1', 'Test Task')`)
	if err != nil {
		t.Fatalf("failed to insert task: %v", err)
	}

	// Claim task
	claimedTask, err := tdb.ClaimTask(ctx, "agent-1")
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}
	if claimedTask == nil {
		t.Fatalf("expected task, got nil")
	}
	if claimedTask.Status != "IN_PROGRESS" {
		t.Errorf("expected Status 'IN_PROGRESS', got %s", claimedTask.Status)
	}
	if claimedTask.AssignedAgentID != "agent-1" {
		t.Errorf("expected AssignedAgentID 'agent-1', got %s", claimedTask.AssignedAgentID)
	}

	// Claim another (should be empty because the only task is IN_PROGRESS)
	task3, err := tdb.ClaimTask(ctx, "agent-2")
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}
	if task3 != nil {
		t.Fatalf("expected nil task, got %v", task3)
	}
}
