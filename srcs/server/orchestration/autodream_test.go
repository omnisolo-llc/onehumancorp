package orchestration

import (
	"context"
	"fmt"
	"os"
	"path/filepath"
	"testing"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
)

func TestAutoDreamPruneSessions(t *testing.T) {
	t.Setenv("DATABASE_URL", "sqlite://file:autodream-prune-test?mode=memory&cache=shared")
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
	t.Setenv("DATABASE_URL", "sqlite://file:autodream-truth-test?mode=memory&cache=shared")
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
	t.Setenv("DATABASE_URL", "sqlite://file:autodream-session-compression-test?mode=memory&cache=shared")
	pool, err := db.New(context.Background())
	if err != nil {
		t.Fatalf("failed to create db: %v", err)
	}
	defer pool.Close()

	ctx := context.Background()

	if err := pool.RunMigrations(ctx); err != nil {
		t.Fatalf("failed migrations: %v", err)
	}

	// Delete tables as they might have been created by another test
	_, _ = pool.Provider.Exec(ctx, "DELETE FROM agent_session_data")
	_, _ = pool.Provider.Exec(ctx, "DELETE FROM autodream_memories")

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
	err = pool.Provider.QueryRow(ctx, "SELECT COUNT(*) FROM autodream_memories WHERE source_mission_id = 'sess-1'").Scan(&count)
	if err != nil {
		t.Fatalf("failed to query count: %v", err)
	}
	if count != 1 {
		t.Errorf("expected 1 memory inserted, got %d", count)
	}
}

func TestAutoDreamWorker_MemoryIngestion(t *testing.T) {
	t.Setenv("DATABASE_URL", "sqlite://file:autodream-memory-ingestion-test?mode=memory&cache=shared")
	pool, err := db.New(context.Background())
	if err != nil {
		t.Fatalf("failed to create db: %v", err)
	}
	defer pool.Close()

	ctx := context.Background()

	if err := pool.RunMigrations(ctx); err != nil {
		t.Fatalf("failed migrations: %v", err)
	}

	_, _ = pool.Provider.Exec(ctx, "DELETE FROM autodream_memories")

	worker := NewAutoDreamWorker(pool.Provider)

	// Create mock test file in .agent-task/memory

	err = os.MkdirAll(".agent-task/memory", 0755)
	if err != nil {
		t.Fatalf("failed to create memory directory: %v", err)
	}
	defer os.RemoveAll(".agent-task/memory")

	testFilePath := filepath.Join(".agent-task/memory", "test_memory.yml")
	err = os.WriteFile(testFilePath, []byte("Test memory context"), 0644)
	if err != nil {
		t.Fatalf("failed to write mock memory file: %v", err)
	}

	t.Setenv("MCPANY_DEBUG", "1")
	worker.ingestAgentMemories(ctx)

	// Verify the file was deleted
	if _, err := os.Stat(testFilePath); !os.IsNotExist(err) {
		t.Errorf("expected memory file to be deleted, but it still exists")
	}

	// Verify the memory was inserted into the database
	var count int
	err = pool.Provider.QueryRow(ctx, "SELECT COUNT(*) FROM autodream_memories WHERE source_mission_id = 'mem-test_memory.yml'").Scan(&count)
	if err != nil {
		t.Fatalf("failed to query count: %v", err)
	}
	if count != 1 {
		t.Errorf("expected 1 memory inserted, got %d", count)
	}
}

func TestAutoDreamConsolidateEpoch(t *testing.T) {
	t.Setenv("DATABASE_URL", "sqlite://file:autodream-consolidate-epoch-test?mode=memory&cache=shared")
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
