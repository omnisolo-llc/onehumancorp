package orchestration

import (
	"context"
	"database/sql"
	"testing"

	_ "github.com/mattn/go-sqlite3"
)

type claimsContextKey string

const ClaimsContextKeyForTest claimsContextKey = "ClaimsContextKeyForTest"

func TestClaimTaskSQLite(t *testing.T) {
	db, err := sql.Open("sqlite3", ":memory:")
	if err != nil {
		t.Fatalf("failed to open sqlite db: %v", err)
	}
	defer db.Close()

	_, err = db.Exec(`
		CREATE TABLE shared_tasks (
			id TEXT PRIMARY KEY,
			organization_id TEXT NOT NULL,
			parent_plan_id TEXT,
			title TEXT NOT NULL,
			description TEXT,
			status TEXT NOT NULL DEFAULT 'PENDING',
			assigned_agent_id TEXT,
			dependencies JSONB DEFAULT '[]',
			created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
			updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
		);
	`)
	if err != nil {
		t.Fatalf("failed to create table: %v", err)
	}

	_, err = db.Exec(`
		INSERT INTO shared_tasks (id, organization_id, title, status)
		VALUES ('task1', 'org1', 'Test Task 1', 'PENDING')
	`)
	if err != nil {
		t.Fatalf("failed to insert mock task: %v", err)
	}

	tasksDB := NewTasksDB(db, true)

	ctx := context.WithValue(context.Background(), ClaimsContextKeyForTest, "test_claims")

	task, err := tasksDB.ClaimTask(ctx, "org1", "agent1")
	if err != nil {
		t.Fatalf("ClaimTask failed: %v", err)
	}

	if task == nil {
		t.Fatalf("expected to claim a task, got nil")
	}

	if task.ID != "task1" {
		t.Errorf("expected task ID 'task1', got '%s'", task.ID)
	}
	if task.Status != "ASSIGNED" {
		t.Errorf("expected status 'ASSIGNED', got '%s'", task.Status)
	}
	if task.AssignedAgentID == nil || *task.AssignedAgentID != "agent1" {
		t.Errorf("expected assigned agent 'agent1', got '%v'", task.AssignedAgentID)
	}

	// Try claiming again, should get nil
	task2, err := tasksDB.ClaimTask(ctx, "org1", "agent2")
	if err != nil {
		t.Fatalf("ClaimTask 2 failed: %v", err)
	}
	if task2 != nil {
		t.Errorf("expected nil task, got '%v'", task2)
	}
}
