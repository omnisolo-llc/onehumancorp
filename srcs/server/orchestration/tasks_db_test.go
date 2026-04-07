package orchestration

import (
	"context"
	"os"
	"testing"
	"time"

	"github.com/onehumancorp/mono/srcs/server/auth"
	"github.com/onehumancorp/mono/srcs/server/db"
)

func setupTasksDBTest(t *testing.T) (*SharedTaskDB, func()) {
	t.Helper()
	prov := db.NewTestProvider(t)

	_, err := prov.Exec(context.Background(), `
		CREATE TABLE IF NOT EXISTS shared_tasks (
			id TEXT PRIMARY KEY,
			organization_id TEXT NOT NULL,
			parent_plan_id TEXT,
			dependencies JSONB NOT NULL DEFAULT '[]',
			title TEXT NOT NULL,
			description TEXT,
			status TEXT NOT NULL DEFAULT 'PENDING',
			assigned_agent_id TEXT,
			priority TEXT NOT NULL DEFAULT 'P2',
			payload TEXT NOT NULL DEFAULT '{}',
			locked_until DATETIME,
			created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
			updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
		);
	`)
	if err != nil {
		t.Fatalf("failed to create table: %v", err)
	}

	tdb := NewSharedTaskDB(prov)

	return tdb, func() {
		prov.Close()
	}
}

func TestSharedTaskDB_ClaimTask(t *testing.T) {
	t.Setenv("OHC_STANDALONE", "true")

	tdb, cleanup := setupTasksDBTest(t)
	defer cleanup()

	ctx := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, &auth.Claims{OrganizationID: "org-1"})

	// Attempt to claim when empty
	_, err := tdb.ClaimTask(ctx, "agent-1")
	if err == nil {
		t.Fatalf("expected error claiming from empty DB, got nil")
	}

	// Insert task
	_, err = tdb.db.Exec(ctx, `
		INSERT INTO shared_tasks (id, organization_id, title, status)
		VALUES ('task-1', 'org-1', 'Test Task', 'PENDING')
	`)
	if err != nil {
		t.Fatalf("failed to insert task: %v", err)
	}

	// Claim task
	task, err := tdb.ClaimTask(ctx, "agent-1")
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}
	if task == nil {
		t.Fatalf("expected task, got nil")
	}
	if task.Status != "ASSIGNED" {
		t.Errorf("expected Status 'ASSIGNED', got %s", task.Status)
	}
	if task.AssignedAgentID != "agent-1" {
		t.Errorf("expected AssignedAgentID 'agent-1', got %s", task.AssignedAgentID)
	}

	// Attempt to claim again (should be none pending)
	_, err = tdb.ClaimTask(ctx, "agent-2")
	if err == nil {
		t.Fatalf("expected error when no pending tasks, got nil")
	}
}
