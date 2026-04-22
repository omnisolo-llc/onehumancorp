package orchestration

import (
	"context"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/db"
)

func TestSharedTaskOrchestrator(t *testing.T) {
	provider := db.NewTestProvider(t)
	ctx := context.Background()

	_, err := provider.Exec(ctx, `
		CREATE TABLE IF NOT EXISTS shared_tasks_v4 (
			id VARCHAR PRIMARY KEY,
			organization_id VARCHAR NOT NULL,
			title VARCHAR NOT NULL,
			description TEXT,
			status VARCHAR NOT NULL DEFAULT 'PENDING',
			agent_id VARCHAR,
			priority VARCHAR NOT NULL DEFAULT 'P2',
			payload TEXT,
			parent_plan_id TEXT,
			dependencies TEXT NOT NULL DEFAULT '[]',
			created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
			updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
		)
	`)
	if err != nil {
		t.Fatalf("failed to create table: %v", err)
	}

	orch := NewSharedTaskOrchestrator(provider)

	task := &SharedTask{
		OrganizationID: "org-1",
		Title:          "Test Task",
		Description:    "Test description",
		Status:         "PENDING",
		Priority:       "P1",
		Dependencies:   []string{"dep-1", "dep-2"},
	}

	err = orch.CreateTask(ctx, task)
	if err != nil {
		t.Fatalf("failed to create task: %v", err)
	}

	if task.ID == "" {
		t.Fatal("task ID should be populated")
	}

	fetchedTask, err := orch.GetTask(ctx, task.ID)
	if err != nil {
		t.Fatalf("failed to get task: %v", err)
	}

	if fetchedTask.Title != task.Title {
		t.Errorf("expected title %q, got %q", task.Title, fetchedTask.Title)
	}

	if len(fetchedTask.Dependencies) != 2 {
		t.Errorf("expected 2 dependencies, got %d", len(fetchedTask.Dependencies))
	}

	err = orch.UpdateTaskStatus(ctx, task.ID, "CLAIMED")
	if err != nil {
		t.Fatalf("failed to update task status: %v", err)
	}

	fetchedTask, err = orch.GetTask(ctx, task.ID)
	if err != nil {
		t.Fatalf("failed to get task: %v", err)
	}

	if fetchedTask.Status != "CLAIMED" {
		t.Errorf("expected status CLAIMED, got %q", fetchedTask.Status)
	}
}
