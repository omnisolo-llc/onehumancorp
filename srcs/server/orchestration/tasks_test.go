package orchestration

import (
	"context"
	"os"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/db"
)

func TestTaskManager_Standalone(t *testing.T) {
	os.Setenv("OHC_MULTITENANT", "false")
	// Use an in-memory SQLite DB
	os.Setenv("DATABASE_URL", "sqlite://file::memory:?cache=shared")

	ctx := context.Background()
	provider, err := db.New(ctx)
	if err != nil {
		t.Fatalf("failed to init db: %v", err)
	}
	defer provider.Close()

	if err := provider.RunMigrations(ctx); err != nil {
		t.Fatalf("failed to run migrations: %v", err)
	}

	tm := NewTaskManager(provider)

	// Create task
	task, err := tm.CreateTask(ctx, "mission-1", "Test Task", "Desc", "P1")
	if err != nil {
		t.Fatalf("failed to create task: %v", err)
	}

	// Get pending tasks
	pending, err := tm.GetPendingTasks(ctx)
	if err != nil {
		t.Fatalf("failed to get pending tasks: %v", err)
	}
	if len(pending) != 1 || pending[0].ID != task.ID {
		t.Errorf("expected 1 pending task with matching ID, got %d", len(pending))
	}

	// Claim task
	err = tm.ClaimTask(ctx, task.ID, "agent-1")
	if err != nil {
		t.Fatalf("failed to claim task: %v", err)
	}

	// Claim again should fail
	err = tm.ClaimTask(ctx, task.ID, "agent-2")
	if err != ErrTaskLocked {
		t.Errorf("expected ErrTaskLocked, got %v", err)
	}

	// Update status
	err = tm.UpdateTaskStatus(ctx, task.ID, "COMPLETED")
	if err != nil {
		t.Fatalf("failed to update task status: %v", err)
	}
}
