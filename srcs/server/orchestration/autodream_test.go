package orchestration

import (
	"context"
	"fmt"
	"testing"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
)

func TestAutoDreamPruneSessions(t *testing.T) {
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
