package orchestration

import (
	"context"
	"path/filepath"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/db"
)

func TestTaskManager(t *testing.T) {
	ctx := context.Background()

	// Setup SQLite DB for testing
	dbPath := filepath.Join(t.TempDir(), "test.db")
	database, err := db.NewSqliteProviderForTest(dbPath)
	if err != nil {
		t.Fatalf("failed to create db: %v", err)
	}
	defer database.Close()

	// Apply schema manually since migrations run differently in tests
	_, err = database.Exec(ctx, `
		CREATE TABLE shared_tasks (
			id TEXT PRIMARY KEY,
			mission_id TEXT NOT NULL,
			title TEXT NOT NULL,
			description TEXT,
			assigned_agent_id TEXT,
			status TEXT NOT NULL DEFAULT 'PENDING',
			priority TEXT NOT NULL DEFAULT 'P2',
			created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
			updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
		)
	`)
	if err != nil {
		t.Fatalf("failed to setup schema: %v", err)
	}

	tm := NewTaskManager(database)

	task := SharedTask{
		MissionID:   "m-123",
		Title:       "Test Task",
		Description: "Testing",
		Priority:    "P0",
	}

	id, err := tm.CreateTask(ctx, task)
	if err != nil {
		t.Fatalf("CreateTask failed: %v", err)
	}
	if id == "" {
		t.Fatal("expected ID to be returned")
	}

	tasks, err := tm.ListPendingTasks(ctx)
	if err != nil {
		t.Fatalf("ListPendingTasks failed: %v", err)
	}
	if len(tasks) != 1 {
		t.Fatalf("expected 1 task, got %d", len(tasks))
	}
	if tasks[0].ID != id {
		t.Errorf("expected ID %s, got %s", id, tasks[0].ID)
	}

	// Test claiming
	err = tm.ClaimTask(ctx, id, "agent-1")
	if err != nil {
		t.Fatalf("ClaimTask failed: %v", err)
	}

	// Claiming again should fail
	err = tm.ClaimTask(ctx, id, "agent-2")
	if err != ErrTaskLocked && err != ErrTaskNotFound {
		t.Fatalf("expected lock error, got %v", err)
	}

	// List should now be empty
	tasks, _ = tm.ListPendingTasks(ctx)
	if len(tasks) != 0 {
		t.Fatalf("expected 0 pending tasks, got %d", len(tasks))
	}

	// Test update status
	err = tm.UpdateTaskStatus(ctx, id, "agent-1", "COMPLETED")
	if err != nil {
		t.Fatalf("UpdateTaskStatus failed: %v", err)
	}
}
