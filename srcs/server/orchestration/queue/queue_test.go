package queue

import (
	"context"
	"testing"
	"database/sql"

	"github.com/onehumancorp/mono/srcs/server/db"
	_ "modernc.org/sqlite"
)

func setupTestDBSqlite(t *testing.T) db.Provider {
	conn, err := sql.Open("sqlite", ":memory:")
	if err != nil {
		t.Fatalf("failed to open sqlite: %v", err)
	}

	p := db.NewSqliteProvider(conn)
	ctx := context.Background()

	_, err = p.Exec(ctx, `
	CREATE TABLE IF NOT EXISTS sub_agent_jobs (
		id TEXT PRIMARY KEY,
		parent_task_id TEXT,
		agent_role TEXT NOT NULL,
		payload TEXT NOT NULL,
		status TEXT NOT NULL DEFAULT 'QUEUED',
		attempts INTEGER DEFAULT 0,
		max_attempts INTEGER DEFAULT 3,
		run_after DATETIME DEFAULT CURRENT_TIMESTAMP,
		locked_until DATETIME,
		created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
		updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
	);`)
	if err != nil {
		t.Fatalf("failed to create table: %v", err)
	}
	return p
}

func TestSQLiteTaskQueue(t *testing.T) {
	provider := setupTestDBSqlite(t)
	defer provider.Close()

	ctx := context.Background()
	q := NewSQLiteTaskQueue(provider)

	job := &Job{
		ID:          "job-1",
		AgentRole:   "test-agent",
		Payload:     "{}",
		MaxAttempts: 3,
	}

	err := EnqueueJob(ctx, q, job)
	if err != nil {
		t.Fatalf("failed to enqueue: %v", err)
	}

	dequeued, err := DequeueJob(ctx, q, []string{"test-agent"})
	if err != nil {
		t.Fatalf("failed to dequeue: %v", err)
	}
	if dequeued == nil {
		t.Fatal("expected to dequeue a job, got nil")
	}

	if dequeued.ID != "job-1" {
		t.Errorf("expected job ID job-1, got %v", dequeued.ID)
	}

	err = q.Complete(ctx, dequeued.ID)
	if err != nil {
		t.Fatalf("failed to complete: %v", err)
	}
}
