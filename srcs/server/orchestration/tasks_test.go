package orchestration

import (
	"context"
	"testing"
	"github.com/onehumancorp/mono/srcs/server/db"
)

func TestClaimTaskStandalone(t *testing.T) {
	ctx := context.Background()
	provider, err := db.New(ctx) // This will create an in-memory SQLite by default without env vars
	if err != nil {
		t.Fatalf("failed to create db: %v", err)
	}
	defer provider.Close()

	err = provider.RunMigrations(ctx)
	if err != nil {
		t.Fatalf("failed to run migrations: %v", err)
	}

	tm := NewTaskQueueManager(provider.Provider, nil)

	// Add a task
	taskID, err := tm.AddTask(ctx, "m-1", "Test Task", `{"key":"value"}`)
	if err != nil {
		t.Fatalf("failed to add task: %v", err)
	}

	// Claim task
	err = tm.ClaimTask(ctx, taskID, "agent-1")
	if err != nil {
		t.Fatalf("failed to claim task: %v", err)
	}

	// Claim again (should fail)
	err = tm.ClaimTask(ctx, taskID, "agent-2")
	if err == nil {
		t.Fatal("expected error when claiming already claimed task, got nil")
	}

	// Complete task
	err = tm.CompleteTask(ctx, taskID, "agent-1")
	if err != nil {
		t.Fatalf("failed to complete task: %v", err)
	}
}
