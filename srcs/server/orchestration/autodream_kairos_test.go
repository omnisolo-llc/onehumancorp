package orchestration

import (
	"context"
	"os"
	"testing"

	)

func TestAutoDreamWorker_ProcessCompletedTasks(t *testing.T) {
	os.Setenv("OHC_STANDALONE", "true")
	defer os.Unsetenv("OHC_STANDALONE")

	prov := NewTestProvider(t)
	defer prov.Close()

	ctx := context.Background()

	// Ensure tables exist
	_, _ = prov.Exec(ctx, `
		CREATE TABLE IF NOT EXISTS swarm_tasks (
			id TEXT PRIMARY KEY,
			mission_id TEXT NOT NULL,
			title TEXT NOT NULL,
			status TEXT NOT NULL DEFAULT 'PENDING',
			payload TEXT NOT NULL DEFAULT '{}'
		);
	`)

	_, _ = prov.Exec(ctx, `
		CREATE TABLE IF NOT EXISTS autodream_memories (
			id TEXT PRIMARY KEY DEFAULT (lower(hex(randomblob(16)))),
			content TEXT NOT NULL,
			embedding TEXT,
			source_mission_id TEXT,
			consolidated_at DATETIME DEFAULT CURRENT_TIMESTAMP
		);
	`)

	_, _ = prov.Exec(ctx, "INSERT INTO swarm_tasks (id, mission_id, title, status, payload) VALUES ('t1', 'm1', 'Task 1', 'COMPLETED', '{\"foo\":\"bar\"}')")
	_, _ = prov.Exec(ctx, "INSERT INTO swarm_tasks (id, mission_id, title, status, payload) VALUES ('t2', 'm2', 'Task 2', 'PENDING', '{\"foo\":\"bar\"}')")

	worker := NewAutoDreamWorker(prov)

	err := worker.ProcessCompletedTasks(ctx)
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}

	// Verify t1 was processed and deleted
	var count int
	_ = prov.QueryRow(ctx, "SELECT COUNT(*) FROM swarm_tasks WHERE id = 't1'").Scan(&count)
	if count != 0 {
		t.Errorf("expected task t1 to be deleted")
	}

	// Verify t2 is still there
	_ = prov.QueryRow(ctx, "SELECT COUNT(*) FROM swarm_tasks WHERE id = 't2'").Scan(&count)
	if count != 1 {
		t.Errorf("expected task t2 to remain")
	}

	// Verify autodream memory was inserted
	_ = prov.QueryRow(ctx, "SELECT COUNT(*) FROM autodream_memories WHERE source_mission_id = 'm1'").Scan(&count)
	if count != 1 {
		t.Errorf("expected 1 autodream_memory for m1, got %d", count)
	}
}
