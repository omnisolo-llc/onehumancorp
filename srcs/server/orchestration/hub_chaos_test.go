package orchestration

import (
	"context"
	"database/sql"
	"fmt"
	"path/filepath"
	"sync"
	"testing"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
)

// TestSqliteHubRepository_Chaos simulates high-concurrency message event ingestion
// across the backend to stress-test the SQLite lock contention.
func TestSqliteHubRepository_Chaos(t *testing.T) {
	// 1. Setup SQLite Database
	tmpDir := t.TempDir()
	dbPath := filepath.Join(tmpDir, "hub_chaos.db")

	// Create table manually for the test
	sqlDB, err := sql.Open("sqlite", dbPath+"?_pragma=journal_mode(WAL)&_pragma=busy_timeout(10000)")
	if err != nil {
		t.Fatalf("Failed to open sqlite: %v", err)
	}
	defer sqlDB.Close()

	_, err = sqlDB.Exec(`
		CREATE TABLE IF NOT EXISTS agents (
			id              TEXT PRIMARY KEY,
			name            TEXT NOT NULL,
			role            TEXT NOT NULL,
			organization_id TEXT NOT NULL DEFAULT '',
			status          TEXT NOT NULL DEFAULT 'IDLE',
			provider_type   TEXT NOT NULL DEFAULT '',
			region          TEXT NOT NULL DEFAULT '',
			registered_at   DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
		);
		CREATE TABLE IF NOT EXISTS agent_inbox (
			seq         INTEGER PRIMARY KEY AUTOINCREMENT,
			agent_id    TEXT NOT NULL,
			message_id  TEXT NOT NULL,
			from_agent  TEXT NOT NULL,
			to_agent    TEXT NOT NULL DEFAULT '',
			type        TEXT NOT NULL,
			content     TEXT NOT NULL DEFAULT '',
			meeting_id  TEXT NOT NULL DEFAULT '',
			occurred_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
		);
	`)
	if err != nil {
		t.Fatalf("Failed to create tables: %v", err)
	}

	repo := NewSqliteHubRepository(sqlDB)

	// Register an agent to receive messages
	agentID := "target-agent"
	err = repo.RegisterAgent(context.Background(), Agent{
		ID: agentID, Name: "Target", Role: "WORKER", Status: StatusIdle,
	})
	if err != nil {
		t.Fatalf("Failed to register agent: %v", err)
	}

	// 2. High-concurrency PushMessage ingestion
	var wg sync.WaitGroup
	numWorkers := 50
	messagesPerWorker := 20
	errs := make(chan error, numWorkers*messagesPerWorker)

	start := time.Now()
	for i := 0; i < numWorkers; i++ {
		wg.Add(1)
		go func(workerIdx int) {
			defer wg.Done()
			for j := 0; j < messagesPerWorker; j++ {
				msg := Message{
					ID:         fmt.Sprintf("msg-%d-%d", workerIdx, j),
					FromAgent:  fmt.Sprintf("sender-%d", workerIdx),
					ToAgent:    agentID,
					Type:       EventTask,
					Content:    "Chaos message",
					OccurredAt: time.Now().UTC(),
				}

				// Implement a small local retry for busy locks as SQLite will throw "database is locked" under heavy concurrent writes
				// In a real app, `withRetry` or connection pool settings handle this, but here we test the repository directly.
				var pushErr error
				for retries := 0; retries < 50; retries++ {
					pushErr = repo.PushMessage(context.Background(), agentID, msg)
					if pushErr == nil {
						break
					}
					time.Sleep(100 * time.Millisecond)
				}

				if pushErr != nil {
					errs <- fmt.Errorf("worker %d failed to push message %d: %v", workerIdx, j, pushErr)
				}
			}
		}(i)
	}

	wg.Wait()
	close(errs)

	for err := range errs {
		t.Errorf("Concurrency error: %v", err)
	}

	t.Logf("Ingested %d messages concurrently into SQLite in %v", numWorkers*messagesPerWorker, time.Since(start))

	// 3. Verify messages
	msgs, err := repo.PopMessages(context.Background(), agentID)
	if err != nil {
		t.Fatalf("Failed to pop messages: %v", err)
	}

	if len(msgs) != numWorkers*messagesPerWorker {
		t.Errorf("Expected %d messages, got %d", numWorkers*messagesPerWorker, len(msgs))
	} else {
		t.Logf("Successfully retrieved and popped %d messages", len(msgs))
	}
}

// TestPgHubRepository_Chaos tests the PostgreSQL implementation.
// This requires a real Postgres instance, so we skip it if DATABASE_URL is not set.
func TestPgHubRepository_Chaos(t *testing.T) {
	ctx := context.Background()
	provider, err := db.NewProvider(ctx)
	if err != nil || provider == nil || provider.Type != "postgres" {
		t.Skip("Skipping PgHubRepository chaos test: Postgres not configured")
	}
	defer provider.Close()

	if err := provider.RunMigrations(ctx); err != nil {
		t.Fatalf("Failed to run migrations: %v", err)
	}

	repo := NewPgHubRepository(provider.PgPool)

	// Register an agent
	agentID := "pg-target-agent"
	err = repo.RegisterAgent(ctx, Agent{
		ID: agentID, Name: "PG Target", Role: "WORKER", Status: StatusIdle,
	})
	if err != nil {
		t.Fatalf("Failed to register agent: %v", err)
	}

	// High-concurrency PushMessage ingestion
	var wg sync.WaitGroup
	numWorkers := 100
	messagesPerWorker := 50
	errs := make(chan error, numWorkers*messagesPerWorker)

	start := time.Now()
	for i := 0; i < numWorkers; i++ {
		wg.Add(1)
		go func(workerIdx int) {
			defer wg.Done()
			for j := 0; j < messagesPerWorker; j++ {
				msg := Message{
					ID:         fmt.Sprintf("pg-msg-%d-%d", workerIdx, j),
					FromAgent:  fmt.Sprintf("pg-sender-%d", workerIdx),
					ToAgent:    agentID,
					Type:       EventTask,
					Content:    "PG Chaos message",
					OccurredAt: time.Now().UTC(),
				}

				if err := repo.PushMessage(ctx, agentID, msg); err != nil {
					errs <- fmt.Errorf("worker %d failed to push pg message %d: %v", workerIdx, j, err)
				}
			}
		}(i)
	}

	wg.Wait()
	close(errs)

	for err := range errs {
		t.Errorf("PG Concurrency error: %v", err)
	}

	t.Logf("Ingested %d messages concurrently into Postgres in %v", numWorkers*messagesPerWorker, time.Since(start))

	// Verify messages
	msgs, err := repo.PopMessages(ctx, agentID)
	if err != nil {
		t.Fatalf("Failed to pop pg messages: %v", err)
	}

	if len(msgs) != numWorkers*messagesPerWorker {
		t.Errorf("Expected %d pg messages, got %d", numWorkers*messagesPerWorker, len(msgs))
	} else {
		t.Logf("Successfully retrieved and popped %d pg messages", len(msgs))
	}

	// Cleanup
	_ = repo.RemoveAgent(ctx, agentID)
}
