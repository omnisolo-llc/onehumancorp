package orchestration

import (
	"context"
	"fmt"
	"testing"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
)

func TestAutoDreamWorker_TaskConsolidation(t *testing.T) {
	t.Setenv("DATABASE_URL", "sqlite://file::memory:?mode=memory")
	pool, err := db.New(context.Background())
	if err != nil {
		t.Fatalf("failed to create db: %v", err)
	}
	defer pool.Close()

	if err := pool.RunMigrations(context.Background()); err != nil {
		t.Fatalf("failed to run migrations: %v", err)
	}

	ctx := context.Background()

	// Insert a COMPLETED task
	_, err = pool.Provider.Exec(ctx, `
		INSERT INTO shared_tasks (id, mission_id, description, payload, status)
		VALUES ('task-1', 'mission-1', 'test description', '{"test": "payload"}', 'COMPLETED')
	`)
	if err != nil {
		t.Fatalf("failed to insert completed task: %v", err)
	}

	worker := NewAutoDreamWorker(pool.Provider)
	worker.consolidateCompletedTasks(ctx)

	// Verify task was deleted from shared_tasks
	var count int
	err = pool.Provider.QueryRow(ctx, "SELECT COUNT(*) FROM shared_tasks WHERE id = 'task-1'").Scan(&count)
	if err != nil {
		t.Fatalf("failed to query count: %v", err)
	}
	if count != 0 {
		t.Errorf("expected 0 tasks left, got %d", count)
	}

	// Verify memory was inserted
	err = pool.Provider.QueryRow(ctx, "SELECT COUNT(*) FROM autodream_memories WHERE source_mission_id = 'mission-1'").Scan(&count)
	if err != nil {
		t.Fatalf("failed to query count: %v", err)
	}
	if count != 1 {
		t.Errorf("expected 1 memory inserted, got %d", count)
	}
}
