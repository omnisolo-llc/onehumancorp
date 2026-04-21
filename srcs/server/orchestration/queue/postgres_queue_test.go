package queue

import (
	"context"
	"testing"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
	_ "modernc.org/sqlite"
)

func TestPostgresQueue_Dequeue(t *testing.T) {
	provider := newTestProvider(t)
	ctx := context.Background()

	// Set up schema
	schema := `
	CREATE TABLE IF NOT EXISTS sub_agent_queue (
		id TEXT PRIMARY KEY,
		organization_id TEXT NOT NULL,
		parent_task_id TEXT NOT NULL,
		payload JSONB,
		status TEXT NOT NULL DEFAULT 'QUEUED',
		scheduled_at TIMESTAMPTZ,
		completed_at TIMESTAMPTZ,
		worker_id TEXT,
		created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
		updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
	);
	`
	_, err := provider.Exec(ctx, schema)
	if err != nil {
		t.Fatalf("failed to create schema: %v", err)
	}

	q := NewPostgresTaskQueue(provider)

	// Since we're using SQLite as our mock provider here, convertBindVars will convert
	// $1, $2, etc into ?1, ?2 so the query will execute properly.
	// Postgres Queue doesn't know it's SQLite behind the scenes unless checked.

	// Test Enqueue
	job := &Job{
		ID:           "test-pg-job-1",
		ParentTaskID: "task-1",
		AgentRole:    "tester",
		Payload:      "{}",
		MaxAttempts:  3,
		RunAfter:     time.Now().Add(-1 * time.Hour), // Ready to run
	}

	if err := q.Enqueue(ctx, job); err != nil {
		t.Fatalf("Enqueue failed: %v", err)
	}

	// Test Dequeue
	dequeued, err := q.Dequeue(ctx, []string{"tester"})
	if err != nil {
		t.Fatalf("Dequeue failed: %v", err)
	}
	if dequeued == nil {
		t.Fatal("Expected to dequeue job, got nil")
	}
	if dequeued.ID != "test-pg-job-1" {
		t.Fatalf("Expected job ID test-pg-job-1, got %s", dequeued.ID)
	}
	if dequeued.Status != "RUNNING" {
		t.Fatalf("Expected job status RUNNING, got %s", dequeued.Status)
	}
}
