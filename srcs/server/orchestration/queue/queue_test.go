package queue_test

import (
	"context"
	"testing"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
	"github.com/onehumancorp/mono/srcs/server/orchestration/queue"
)

// TestSQLiteQueue tests the SQLite implementation of the TaskQueue.
func TestSQLiteQueue(t *testing.T) {
	provider := db.NewTestProvider(t)

	ctx := context.Background()

	// Ensure table exists for tests using the provider
	_, err := provider.Exec(ctx, `
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
		);
	`)
	if err != nil {
		t.Fatalf("failed to create table: %v", err)
	}

	q := queue.NewSQLiteTaskQueue(provider)

	// 1. Test Enqueue
	job := &queue.Job{
		ID:           "job-1",
		ParentTaskID: "task-1",
		AgentRole:    "cost-engineer",
		Payload:      `{"action": "optimize"}`,
		MaxAttempts:  3,
		RunAfter:     time.Now().Add(-1 * time.Minute), // Run immediately
	}

	err = q.Enqueue(ctx, job)
	if err != nil {
		t.Fatalf("failed to enqueue: %v", err)
	}

	// 2. Test Dequeue
	dequeued, err := q.Dequeue(ctx, []string{"cost-engineer"})
	if err != nil {
		t.Fatalf("failed to dequeue: %v", err)
	}
	if dequeued == nil {
		t.Fatalf("expected job, got nil")
	}
	if dequeued.ID != "job-1" {
		t.Errorf("expected job ID job-1, got %s", dequeued.ID)
	}
	if dequeued.Status != "RUNNING" {
		t.Errorf("expected status RUNNING, got %s", dequeued.Status)
	}

	// 3. Test Fail (requeue)
	err = q.Fail(ctx, "job-1", "temp error")
	if err != nil {
		t.Fatalf("failed to fail job: %v", err)
	}

	// Wait a moment for backoff? Backoff is 1 minute, so it shouldn't be immediately available
	dequeued2, err := q.Dequeue(ctx, []string{"cost-engineer"})
	if err != nil {
		t.Fatalf("failed to dequeue: %v", err)
	}
	if dequeued2 != nil {
		t.Errorf("expected nil job due to backoff, got %v", dequeued2.ID)
	}

	// Force it to be available again
	_, _ = provider.Exec(ctx, "UPDATE sub_agent_jobs SET run_after = datetime(CURRENT_TIMESTAMP, '-1 minute') WHERE id = 'job-1'")
	dequeued3, err := q.Dequeue(ctx, []string{"cost-engineer"})
	if err != nil {
		t.Fatalf("failed to dequeue: %v", err)
	}
	if dequeued3 == nil {
		t.Fatalf("expected job after forced backoff reset")
	}

	// 4. Test Complete
	err = q.Complete(ctx, "job-1")
	if err != nil {
		t.Fatalf("failed to complete job: %v", err)
	}

	// Ensure no jobs available
	dequeued4, err := q.Dequeue(ctx, []string{"cost-engineer"})
	if err != nil {
		t.Fatalf("failed to dequeue: %v", err)
	}
	if dequeued4 != nil {
		t.Errorf("expected no jobs after completion")
	}
}
