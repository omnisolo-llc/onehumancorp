package orchestration

import (
	"context"
	"testing"
	"github.com/onehumancorp/mono/srcs/server/db"
)

func TestTasksDB_ClaimTask(t *testing.T) {
	// Use the test provider from db package to get a clean SQLite database
	dbProvider := db.NewTestProvider(t)
	database := &db.DB{Provider: dbProvider}
	var err error



	// Create table manually for test
	_, err = database.Exec(context.Background(), `
		CREATE TABLE IF NOT EXISTS shared_tasks_v2 (
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
	`)
	if err != nil {
		t.Fatalf("failed to create test table: %v", err)
	}

	tasksDB := NewTasksDB(database)
	ctx := context.Background()

	// Try claiming when empty
	_, err = tasksDB.ClaimTask(ctx, "org1", "agent1")
	if err != ErrNoTasksAvailable {
		t.Errorf("expected ErrNoTasksAvailable, got: %v", err)
	}

	// Create a task
	task := &SharedTask{
		ID:             "task1",
		OrganizationID: "org1",
		Title:          "Test Task",
		Status:         "PENDING",
	}
	err = tasksDB.CreateTask(ctx, task)
	if err != nil {
		t.Fatalf("failed to create task: %v", err)
	}

	// Claim it
	claimed, err := tasksDB.ClaimTask(ctx, "org1", "agent1")
	if err != nil {
		t.Fatalf("failed to claim task: %v", err)
	}
	if claimed.ID != "task1" {
		t.Errorf("expected task1, got %s", claimed.ID)
	}
	if claimed.Status != "ASSIGNED" {
		t.Errorf("expected ASSIGNED, got %s", claimed.Status)
	}
	if claimed.AssignedAgentID != "agent1" {
		t.Errorf("expected agent1, got %v", claimed.AssignedAgentID)
	}

	// Try claiming again, should be empty since we claimed it
	_, err = tasksDB.ClaimTask(ctx, "org1", "agent2")
	if err != ErrNoTasksAvailable {
		t.Errorf("expected ErrNoTasksAvailable, got: %v", err)
	}
}
