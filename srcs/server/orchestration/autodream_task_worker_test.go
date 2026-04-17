package orchestration

import (
	"context"
	"os"
	"testing"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
)

func TestAutoDreamTaskWorker(t *testing.T) {
	os.Setenv("OHC_MULTITENANT", "false")
	defer os.Unsetenv("OHC_MULTITENANT")

	prov := db.NewTestProvider(t)
	defer prov.Close()

	ctx := context.Background()

	// Ensure tables exist
	_, err := prov.Exec(ctx, `
		CREATE TABLE IF NOT EXISTS shared_tasks (
			id TEXT PRIMARY KEY,
			title TEXT NOT NULL,
			status TEXT DEFAULT 'PENDING',
			payload TEXT
		)
	`)
	if err != nil {
		t.Fatalf("failed to create tasks table: %v", err)
	}

	_, err = prov.Exec(ctx, `
		CREATE TABLE IF NOT EXISTS autodream_memories (
			id TEXT PRIMARY KEY DEFAULT (lower(hex(randomblob(16)))),
			task_id TEXT,
			content TEXT NOT NULL
		)
	`)
	if err != nil {
		t.Fatalf("failed to create memories table: %v", err)
	}

	// Insert a completed task
	_, err = prov.Exec(ctx, `
		INSERT INTO shared_tasks (id, title, status, payload)
		VALUES ('task-1', 'Test Vectorize', 'COMPLETED', '{}')
	`)
	if err != nil {
		t.Fatalf("failed to insert test task: %v", err)
	}

	// Run worker once
	worker := NewAutoDreamTaskWorker(prov, 1*time.Minute)
	worker.processCompletedTasks(ctx)

	// Check if memory was created
	var count int
	err = prov.QueryRow(ctx, "SELECT COUNT(*) FROM autodream_memories WHERE task_id = 'task-1'").Scan(&count)
	if err != nil {
		t.Fatalf("failed to count memories: %v", err)
	}
	if count != 1 {
		t.Errorf("expected 1 memory, got %d", count)
	}
}
