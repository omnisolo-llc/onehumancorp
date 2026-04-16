package db

import (
	"context"
	"fmt"
	"os"
	"testing"

	_ "modernc.org/sqlite"
)

// To run benchmarks:
// bazelisk test //srcs/server/db:db_test --test_arg=-test.bench=.

func setupBenchmarkSchema(t *testing.B, provider Provider) {
	ctx := context.Background()
	schema := `
	CREATE TABLE IF NOT EXISTS shared_tasks_decomposition (
		id TEXT PRIMARY KEY,
		organization_id TEXT NOT NULL,
		title TEXT NOT NULL,
		description TEXT,
		status TEXT NOT NULL DEFAULT 'PENDING',
		assigned_agent_id TEXT,
		priority TEXT NOT NULL DEFAULT 'P2',
		payload TEXT,
		parent_plan_id TEXT,
		dependencies TEXT NOT NULL DEFAULT '[]',
		locked_until TIMESTAMP,
		created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
		updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
	);`

	_, err := provider.Exec(ctx, schema)
	if err != nil {
		t.Fatalf("Failed to create schema: %v", err)
	}

	// Delete existing tasks to avoid primary key collisions
	_, err = provider.Exec(ctx, "DELETE FROM shared_tasks_decomposition")
	if err != nil {
		t.Fatalf("Failed to delete existing tasks: %v", err)
	}

	// Insert test tasks
	tx, err := provider.Begin(ctx)
	if err != nil {
		t.Fatalf("Failed to begin tx: %v", err)
	}
	for i := 0; i < 1000; i++ {
		insertQuery := fmt.Sprintf(`
			INSERT INTO shared_tasks_decomposition (id, organization_id, title, status, created_at, updated_at)
			VALUES ('task-%d', 'org-1', 'Task %d', 'PENDING', '2026-04-15 10:00:00', '2026-04-15 10:00:00')
		`, i, i)
		_, err = tx.Exec(ctx, insertQuery)
		if err != nil {
			t.Fatalf("Failed to insert task: %v", err)
		}
	}
	err = tx.Commit(ctx)
	if err != nil {
		t.Fatalf("Failed to commit: %v", err)
	}
}

func BenchmarkAcquireTaskSQLite(b *testing.B) {
	b.StopTimer()
	b.Setenv("DATABASE_URL", "sqlite://file::memory:?mode=memory&cache=shared")

	dbp, err := New(context.Background())
	if err != nil {
		b.Fatalf("Failed to initialize standalone db: %v", err)
	}
	defer dbp.Close()

	provider := dbp.Provider
	setupBenchmarkSchema(b, provider)
	repo := NewSharedTaskRepository(provider)
	ctx := context.Background()

	b.StartTimer()
	for i := 0; i < b.N; i++ {
		// Stop timer while we re-insert a pending task if needed to keep going
		b.StopTimer()
		if i > 0 && i%1000 == 0 {
			setupBenchmarkSchema(b, provider)
		}
		b.StartTimer()

		_, err := repo.AcquireTask(ctx, "org-1", "agent-x")
		if err != nil {
			b.Fatalf("Failed to acquire task: %v", err)
		}
	}
}

func BenchmarkAcquireTaskPostgres(b *testing.B) {
	pgDSN := os.Getenv("OHC_TEST_PG_DSN")
	if pgDSN == "" {
		b.Skip("Skipping postgres benchmark because OHC_TEST_PG_DSN is not set")
	}
	b.StopTimer()
	b.Setenv("DATABASE_URL", pgDSN)

	dbp, err := New(context.Background())
	if err != nil {
		b.Fatalf("Failed to initialize postgres db: %v", err)
	}
	defer dbp.Close()

	provider := dbp.Provider
	setupBenchmarkSchema(b, provider)
	repo := NewSharedTaskRepository(provider)
	ctx := context.Background()

	b.StartTimer()
	for i := 0; i < b.N; i++ {
		b.StopTimer()
		if i > 0 && i%1000 == 0 {
			setupBenchmarkSchema(b, provider)
		}
		b.StartTimer()

		_, err := repo.AcquireTask(ctx, "org-1", "agent-x")
		if err != nil {
			b.Fatalf("Failed to acquire task: %v", err)
		}
	}
}
