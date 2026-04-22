package db_test

import (
	"context"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/db"
	"github.com/onehumancorp/mono/srcs/server/db/models"
)

func TestCreateAndClaimSharedTask(t *testing.T) {
	provider := db.NewTestProvider(t)
	repo := db.NewSharedTaskRepository(provider)

	ctx := context.Background()

	task := &models.SharedTask{
		OrganizationID: "org1",
		Title:          "Test Task",
	}

	err := repo.CreateSharedTask(ctx, task)
	if err != nil {
		t.Fatalf("failed to create task: %v", err)
	}

	if task.ID == "" {
		t.Fatalf("expected task ID to be generated")
	}

	tasks, err := repo.GetSharedTasks(ctx, "org1")
	if err != nil {
		t.Fatalf("failed to get tasks: %v", err)
	}

	if len(tasks) == 0 {
		t.Fatalf("expected to find tasks")
	}

	claimed, err := repo.ClaimSharedTask(ctx, task.ID, "agent1")
	if err != nil {
		t.Fatalf("failed to claim task: %v", err)
	}

	if !claimed {
		t.Fatalf("expected to successfully claim task")
	}

	// Try claiming again
	claimed2, err := repo.ClaimSharedTask(ctx, task.ID, "agent2")
	if err != nil {
		t.Fatalf("failed to claim task again: %v", err)
	}

	if claimed2 {
		t.Fatalf("expected task to already be claimed")
	}
}
