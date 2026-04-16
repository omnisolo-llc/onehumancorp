package db_test

import (
	"context"
	"os"
	"path/filepath"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/db"
	"github.com/google/uuid"
)

// setupPostgres sets up a Postgres connection for benchmarking if DSN is provided
func setupPostgres(t testing.TB) *db.DB {
	dsn := os.Getenv("POSTGRES_DSN")
	if dsn == "" {
		t.Skip("POSTGRES_DSN not set, skipping postgres benchmark")
	}
	os.Setenv("DATABASE_URL", dsn)
	defer os.Unsetenv("DATABASE_URL")
	ctx := context.Background()
	database, err := db.New(ctx)
	if err != nil {
		t.Fatalf("Failed to connect to Postgres: %v", err)
	}
	if err := database.RunMigrations(ctx); err != nil {
		t.Fatalf("Failed to run migrations: %v", err)
	}
	return database
}

// setupSQLite sets up a temporary SQLite database for benchmarking
func setupSQLite(t testing.TB) *db.DB {
	dir := t.TempDir()
	path := filepath.Join(dir, "bench.db")
	dsn := "sqlite://file:" + path + "?_pragma=journal_mode(WAL)&_pragma=synchronous(NORMAL)&_pragma=busy_timeout(5000)"

	os.Setenv("DATABASE_URL", dsn)
	defer os.Unsetenv("DATABASE_URL")
	ctx := context.Background()
	os.Setenv("OHC_STANDALONE", "true") // Prevent requirement of OHC_SQLITE_KEY
	database, err := db.New(ctx)
	if err != nil {
		t.Fatalf("Failed to connect to SQLite: %v", err)
	}

	if err := database.RunMigrations(ctx); err != nil {
		t.Fatalf("Failed to run migrations: %v", err)
	}

	// Create required tables directly if migrations are missing some SQLite specific tables during tests
	_, err = database.Exec(ctx, `CREATE TABLE IF NOT EXISTS shared_tasks_decomposition (id TEXT PRIMARY KEY, organization_id TEXT NOT NULL, title TEXT NOT NULL, description TEXT, status TEXT NOT NULL DEFAULT 'PENDING', assigned_agent_id TEXT, priority TEXT NOT NULL DEFAULT 'P2', payload TEXT, parent_plan_id TEXT, dependencies TEXT NOT NULL DEFAULT '[]', locked_until DATETIME, created_at DATETIME DEFAULT CURRENT_TIMESTAMP, updated_at DATETIME DEFAULT CURRENT_TIMESTAMP)`)
	if err != nil {
		t.Fatalf("Failed to setup shared_tasks_decomposition: %v", err)
	}

	_, err = database.Exec(ctx, `CREATE TABLE IF NOT EXISTS autodream_memories_master (id TEXT PRIMARY KEY, organization_id TEXT NOT NULL, memory_type TEXT NOT NULL, content TEXT NOT NULL, embedding TEXT, source_task_id TEXT, created_at DATETIME DEFAULT CURRENT_TIMESTAMP)`)
	if err != nil {
		t.Fatalf("Failed to setup autodream_memories_master: %v", err)
	}

	return database
}

func benchmarkAcquireTask(b *testing.B, database *db.DB) {
	ctx := context.Background()
	orgID := uuid.New().String()

	// Pre-seed tasks
	for i := 0; i < 100; i++ {
		_, err := database.Exec(ctx, `
			INSERT INTO shared_tasks_decomposition (id, organization_id, title, status, payload)
			VALUES ($1, $2, $3, 'PENDING', $4)
		`, uuid.New().String(), orgID, "bench_task", "{}")
		if err != nil {
			b.Fatalf("Failed to seed task: %v", err)
		}
	}

	b.ResetTimer()
	b.RunParallel(func(pb *testing.PB) {
		agentID := uuid.New().String()
		for pb.Next() {
			_, err := database.Provider.AcquireTask(ctx, orgID, agentID)
			if err != nil {
				b.Fatalf("Failed to acquire task: %v", err)
			}
		}
	})
}

func benchmarkInsertMemory(b *testing.B, database *db.DB) {
	ctx := context.Background()
	orgID := uuid.New().String()

	b.ResetTimer()
	b.RunParallel(func(pb *testing.PB) {
		for pb.Next() {
			id := uuid.New().String()
			_, err := database.Exec(ctx, `
				INSERT INTO autodream_memories_master (id, organization_id, memory_type, content)
				VALUES ($1, $2, $3, $4)
			`, id, orgID, "bench", "bench memory content")
			if err != nil {
				b.Fatalf("Failed to insert memory: %v", err)
			}
		}
	})
}

func BenchmarkAcquireTask_Postgres(b *testing.B) {
	database := setupPostgres(b)
	defer database.Close()
	benchmarkAcquireTask(b, database)
}

func BenchmarkAcquireTask_SQLite(b *testing.B) {
	database := setupSQLite(b)
	defer database.Close()
	benchmarkAcquireTask(b, database)
}

func BenchmarkInsertMemory_Postgres(b *testing.B) {
	database := setupPostgres(b)
	defer database.Close()
	benchmarkInsertMemory(b, database)
}

func BenchmarkInsertMemory_SQLite(b *testing.B) {
	database := setupSQLite(b)
	defer database.Close()
	benchmarkInsertMemory(b, database)
}
