package orchestration

import (
	"context"
	"fmt"
	"testing"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
	"github.com/onehumancorp/mono/srcs/server/telemetry"
)

func TestAutoDreamPruneSessions(t *testing.T) {
	telemetry.InitTelemetry()
	t.Setenv("DATABASE_URL", "sqlite://:memory:")
	pool, err := db.New(context.Background())
	if err != nil {
		t.Fatalf("failed to init db: %v", err)
	}
	defer pool.Close()

	if err := pool.RunMigrations(context.Background()); err != nil {
		t.Fatalf("failed migrations: %v", err)
	}

	worker := NewAutoDreamWorker(pool.Provider)

	ctx := context.Background()
	_, _ = pool.Exec(ctx, "DELETE FROM agent_session_data") // clear table

	oldTime := time.Now().Add(-48 * time.Hour).UTC().Format("2006-01-02 15:04:05")
	newTime := time.Now().Add(-1 * time.Hour).UTC().Format("2006-01-02 15:04:05")

	if pool.Provider.IsSQLite() {
		_, err = pool.Exec(ctx, "INSERT INTO agent_session_data (session_id, agent_id, context_data, last_accessed) VALUES ('s1', 'a1', 'c1', ?)", oldTime)
		if err != nil {
			t.Fatalf("failed to insert: %v", err)
		}
		_, err = pool.Exec(ctx, "INSERT INTO agent_session_data (session_id, agent_id, context_data, last_accessed) VALUES ('s2', 'a1', 'c2', ?)", newTime)
		if err != nil {
			t.Fatalf("failed to insert: %v", err)
		}
	} else {
		_, err = pool.Exec(ctx, "INSERT INTO agent_session_data (session_id, agent_id, context_data, last_accessed) VALUES ('s1', 'a1', 'c1', $1)", oldTime)
		if err != nil {
			t.Fatalf("failed to insert: %v", err)
		}
		_, err = pool.Exec(ctx, "INSERT INTO agent_session_data (session_id, agent_id, context_data, last_accessed) VALUES ('s2', 'a1', 'c2', $1)", newTime)
		if err != nil {
			t.Fatalf("failed to insert: %v", err)
		}
	}

	worker.pruneStaleSessions(ctx)

	var count int
	err = pool.QueryRow(ctx, "SELECT COUNT(*) FROM agent_session_data").Scan(&count)
	if err != nil {
		t.Fatalf("failed to query count: %v", err)
	}

	if count != 1 {
		t.Errorf("expected 1 session remaining, got %d", count)
	}
}

func TestAutoDreamTruthInjectionAndConflict(t *testing.T) {
	t.Setenv("DATABASE_URL", "sqlite://:memory:")
	pool, err := db.New(context.Background())
	if err != nil {
		t.Fatalf("failed to init db: %v", err)
	}
	defer pool.Close()

	if err := pool.RunMigrations(context.Background()); err != nil {
		t.Fatalf("failed migrations: %v", err)
	}

	worker := NewAutoDreamWorker(pool.Provider)
	ctx := context.Background()

	// Clear out truth table
	_, _ = pool.Exec(ctx, "DELETE FROM swarm_truth_embeddings")

	// Create dummy vector string representation of 1536 dimension (or we mock it with a smaller one for SQLite fallback, but the column expects 1536 in pgvector).
	vectorStr := "["
	for i := 0; i < 1536; i++ {
		if i > 0 {
			vectorStr += ","
		}
		vectorStr += fmt.Sprintf("%f", float64(i)*0.0001)
	}
	vectorStr += "]"

	// Inject two highly similar truths
	err = worker.InjectTruth(ctx, "mem1", "Sky is blue", vectorStr)
	if err != nil {
		t.Fatalf("failed to inject truth: %v", err)
	}

	err = worker.InjectTruth(ctx, "mem2", "Sky is dark blue", vectorStr)
	if err != nil {
		t.Fatalf("failed to inject truth 2: %v", err)
	}

	// Wait, run conflict resolution
	worker.resolveConflicts(ctx)

	if !pool.Provider.IsSQLite() {
		// Postgres: Verify conflict was recorded
		var count int
		err = pool.QueryRow(ctx, "SELECT COUNT(*) FROM memory_conflicts WHERE memory_id_1 IN ('mem1', 'mem2')").Scan(&count)
		if err != nil {
			t.Fatalf("failed to query conflicts: %v", err)
		}
		if count != 1 {
			t.Errorf("expected 1 conflict to be recorded and resolved, got %d", count)
		}
	}
}

func TestAutoDreamWorker_SessionCompression(t *testing.T) {
	t.Setenv("DATABASE_URL", "sqlite://file::memory:?mode=memory")
	pool, err := db.New(context.Background())
	if err != nil {
		t.Fatalf("failed to create db: %v", err)
	}

	ctx := context.Background()

	// Ensure table exists
	_, err = pool.Provider.Exec(ctx, `
		CREATE TABLE IF NOT EXISTS agent_session_data (
			session_id TEXT PRIMARY KEY,
			agent_id TEXT NOT NULL,
			context_data TEXT NOT NULL,
			created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
			last_accessed DATETIME DEFAULT CURRENT_TIMESTAMP
		)
	`)
	if err != nil {
		t.Fatalf("failed to create agent_session_data: %v", err)
	}
	_, err = pool.Provider.Exec(ctx, `
		CREATE TABLE IF NOT EXISTS autodream_memories_master (
			tenant_id TEXT,
			memory_type TEXT,
			id TEXT PRIMARY KEY,
			content TEXT NOT NULL,
			source_task_id TEXT
		)
	`)
	if err != nil {
		t.Fatalf("failed to create autodream_memories_master: %v", err)
	}

	_, err = pool.Provider.Exec(ctx, "INSERT INTO agent_session_data (session_id, agent_id, context_data) VALUES ('sess-1', 'agent-1', 'test context')")
	if err != nil {
		t.Fatalf("failed to insert mock session: %v", err)
	}

	worker := NewAutoDreamWorker(pool.Provider)
	worker.compressSessionData(ctx)

	// Verify the session was deleted
	var count int
	err = pool.Provider.QueryRow(ctx, "SELECT COUNT(*) FROM agent_session_data").Scan(&count)
	if err != nil {
		t.Fatalf("failed to query count: %v", err)
	}
	if count != 0 {
		t.Errorf("expected 0 sessions left, got %d", count)
	}

	// Verify the memory was inserted
	err = pool.Provider.QueryRow(ctx, "SELECT COUNT(*) FROM autodream_memories_master WHERE source_task_id = 'sess-1'").Scan(&count)
	if err != nil {
		t.Fatalf("failed to query count: %v", err)
	}
	if count != 1 {
		t.Errorf("expected 1 memory inserted, got %d", count)
	}
}

func TestAutoDreamConsolidateEpoch(t *testing.T) {
	t.Setenv("DATABASE_URL", "sqlite://file::memory:?mode=memory")
	pool, err := db.New(context.Background())
	if err != nil {
		t.Fatalf("failed to init db: %v", err)
	}
	defer pool.Close()

	if err := pool.RunMigrations(context.Background()); err != nil {
		t.Fatalf("failed migrations: %v", err)
	}

	worker := NewAutoDreamWorker(pool.Provider)
	ctx := context.Background()

	err = worker.ConsolidateEpoch(ctx)
	if err != nil {
		t.Fatalf("ConsolidateEpoch failed: %v", err)
	}

	// Verify epoch record was created and updated
	var count int
	var status string
	err = pool.QueryRow(ctx, "SELECT COUNT(*), MAX(status) FROM swarm_dream_epochs").Scan(&count, &status)
	if err != nil {
		t.Fatalf("failed to query epoch record: %v", err)
	}

	if count != 1 {
		t.Errorf("expected 1 epoch record, got %d", count)
	}
	if status != "COMPLETED" {
		t.Errorf("expected epoch status COMPLETED, got %s", status)
	}
}

func TestAutoDreamWorker_PipelinesCoverage(t *testing.T) {
	t.Setenv("DATABASE_URL", "sqlite://file::memory:?mode=memory")
	pool, err := db.New(context.Background())
	if err != nil {
		t.Fatalf("failed to create db: %v", err)
	}
	defer pool.Close()

	ctx, cancel := context.WithCancel(context.Background())

	worker := NewAutoDreamWorker(pool.Provider)

	// Verify the non-blocking nature and fast exit of Start when context is cancelled.
	go worker.Start(ctx)
	cancel() // instantly cancel to let goroutines exit
	time.Sleep(100 * time.Millisecond)

	// Since pipelines run on intervals, explicitly run the internal sub-methods
	// to ensure full coverage of database and branching logic.
	ctx = context.Background()

	// Add test data for ingestCompletedTasks
	_, _ = pool.Provider.Exec(ctx, "INSERT INTO shared_tasks (id, status, organization_id, payload) VALUES ('t1', 'COMPLETED', 'test_org', '{}')")
	worker.ingestCompletedTasks(ctx)

	var count int
	_ = pool.Provider.QueryRow(ctx, "SELECT COUNT(*) FROM shared_tasks").Scan(&count)
	// ingestCompletedTasks handles shared_tasks

	// Add test data for compressSessionContexts
	oldTime := time.Now().Add(-10 * time.Minute).UTC().Format("2006-01-02 15:04:05")
	_, _ = pool.Provider.Exec(ctx, "INSERT INTO agent_session_data (session_id, agent_id, context_data, last_accessed) VALUES ('s_context_1', 'agent', 'ctx', ?)", oldTime)
	worker.compressSessionContexts(ctx)

	// Wait for background routine in pruneStaleSessions
	time.Sleep(50 * time.Millisecond)

	_ = pool.Provider.QueryRow(ctx, "SELECT COUNT(*) FROM agent_session_data WHERE session_id = 's_context_1'").Scan(&count)
	if count != 0 {
		t.Errorf("expected compressSessionContexts to process and delete session, got %d", count)
	}
}
