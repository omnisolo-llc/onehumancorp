package orchestration

import (
	"context"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/db"
)

func TestCheckoutTask(t *testing.T) {
	ctx := context.Background()
	provider := db.NewSqliteProvider(db.SetupTestDB(t))

	// Create table
	_, err := provider.Exec(ctx, `
		CREATE TABLE agent_tasks (
			id TEXT PRIMARY KEY,
			mission_id TEXT,
			parent_task_id TEXT,
			title TEXT NOT NULL,
			description TEXT,
			status TEXT NOT NULL,
			assigned_agent_id TEXT,
			dependencies TEXT,
			created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
			updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
		)
	`)
	if err != nil {
		t.Fatalf("failed to create table: %v", err)
	}

	// Need a dummy hub or nil for test
	svc := NewTaskService(provider, nil)

	task := &Task{
		ID: "task-1",
		MissionID: "mission-1",
		Title: "Test Task",
		Status: TaskStatusPending,
		Dependencies: []string{},
	}

	err = svc.CreateTask(ctx, task)
	if err != nil {
		t.Fatalf("failed to create task: %v", err)
	}

	checkedOut, err := svc.CheckoutTask(ctx, "agent-1")
	if err != nil {
		t.Fatalf("failed to checkout task: %v", err)
	}

	if checkedOut.AssignedAgentID != "agent-1" {
		t.Errorf("expected assigned agent id agent-1, got %s", checkedOut.AssignedAgentID)
	}

	if checkedOut.Status != TaskStatusInProgress {
		t.Errorf("expected status IN_PROGRESS, got %s", checkedOut.Status)
	}
}
