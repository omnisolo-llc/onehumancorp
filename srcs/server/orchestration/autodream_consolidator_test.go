package orchestration_test

import (
	"context"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/db"
	"github.com/onehumancorp/mono/srcs/server/orchestration"
	"github.com/onehumancorp/mono/srcs/server/lib/llm"
)

func TestAutoDreamConsolidator_ConsolidateCompletedTasks(t *testing.T) {
	ctx := context.Background()

	pool := db.NewTestProvider(t)
	var err error

	// Setup schemas
	_, err = pool.Exec(ctx, `
		CREATE TABLE IF NOT EXISTS shared_tasks (
			id TEXT PRIMARY KEY,
			title TEXT NOT NULL,
			payload TEXT,
			tenant_id TEXT,
			status TEXT
		);
		CREATE TABLE IF NOT EXISTS autodream_memories_master (
			id TEXT PRIMARY KEY,
			organization_id TEXT NOT NULL,
			memory_type TEXT NOT NULL,
			content TEXT NOT NULL,
			embedding BLOB,
			source_task_id TEXT,
			created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
		);
	`)
	if err != nil {
		t.Fatalf("failed to setup schema: %v", err)
	}

	// Insert dummy completed task
	_, err = pool.Exec(ctx, `
		INSERT INTO shared_tasks (id, title, payload, tenant_id, status)
		VALUES ('task-1', 'Test Task', '{"key": "value"}', 'tenant-a', 'COMPLETED')
	`)
	if err != nil {
		t.Fatalf("failed to insert dummy task: %v", err)
	}

	// Add an uncompleted task
	_, err = pool.Exec(ctx, `
		INSERT INTO shared_tasks (id, title, payload, tenant_id, status)
		VALUES ('task-2', 'Unfinished Task', '{}', 'tenant-a', 'PENDING')
	`)
	if err != nil {
		t.Fatalf("failed to insert dummy task: %v", err)
	}

	embedder := llm.NewDefaultEmbedder()
	consolidator := orchestration.NewAutoDreamConsolidator(pool, embedder)

	err = consolidator.ConsolidateCompletedTasks(ctx)
	if err != nil {
		t.Fatalf("unexpected error consolidating tasks: %v", err)
	}

	// Verify insertion into autodream_memories_master
	var count int
	err = pool.QueryRow(ctx, "SELECT count(*) FROM autodream_memories_master WHERE source_task_id = 'task-1'").Scan(&count)
	if err != nil {
		t.Fatalf("failed to query autodream_memories_master: %v", err)
	}
	if count != 1 {
		t.Errorf("expected 1 memory record, got %d", count)
	}

	// Verify status updated to ARCHIVED
	var status string
	err = pool.QueryRow(ctx, "SELECT status FROM shared_tasks WHERE id = 'task-1'").Scan(&status)
	if err != nil {
		t.Fatalf("failed to query task status: %v", err)
	}
	if status != "ARCHIVED" {
		t.Errorf("expected task status to be ARCHIVED, got %s", status)
	}
}
