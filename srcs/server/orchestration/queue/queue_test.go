package queue

import (
	"context"
	"database/sql"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/db"
	_ "modernc.org/sqlite"
)

func newTestProvider(t *testing.T) db.Provider {
	t.Helper()
	d, err := sql.Open("sqlite", ":memory:")
	if err != nil {
		t.Fatalf("failed to open test sqlite db: %v", err)
	}
	if err := d.PingContext(context.Background()); err != nil {
		t.Fatalf("failed to ping test sqlite db: %v", err)
	}
	t.Cleanup(func() {
		d.Close()
	})
	return db.NewSqliteProvider(d)
}

func TestSQLiteTaskQueue(t *testing.T) {
	provider := newTestProvider(t)

	ctx := context.Background()

	// Apply migrations or schema
	schema := `
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
	CREATE INDEX IF NOT EXISTS idx_jobs_runnable ON sub_agent_jobs (status, run_after) WHERE status = 'QUEUED';
	`
	_, err := provider.Exec(ctx, schema)
	if err != nil {
		t.Fatalf("failed to create schema: %v", err)
	}

	q := NewSQLiteTaskQueue(provider)

	job := &Job{
		ID:           "test-job-1",
		ParentTaskID: "task-1",
		AgentRole:    "tester",
		Payload:      "{}",
		MaxAttempts:  3,
	}

	if err := q.Enqueue(ctx, job); err != nil {
		t.Fatalf("Enqueue failed: %v", err)
	}

	dequeued, err := q.Dequeue(ctx, []string{"tester"})
	if err != nil {
		t.Fatalf("Dequeue failed: %v", err)
	}
	if dequeued == nil {
		t.Fatal("Expected to dequeue job, got nil")
	}
	if dequeued.ID != "test-job-1" {
		t.Fatalf("Expected job ID test-job-1, got %s", dequeued.ID)
	}
	if dequeued.Status != "RUNNING" {
		t.Fatalf("Expected job status RUNNING, got %s", dequeued.Status)
	}

	// Test Complete
	if err := q.Complete(ctx, "test-job-1"); err != nil {
		t.Fatalf("Complete failed: %v", err)
	}

	// Verify completion
	var status string
	err = provider.QueryRow(ctx, "SELECT status FROM sub_agent_jobs WHERE id = 'test-job-1'").Scan(&status)
	if err != nil {
		t.Fatalf("Failed to query status: %v", err)
	}
	if status != "COMPLETED" {
		t.Fatalf("Expected status COMPLETED, got %s", status)
	}

	// Test Fail
	job2 := &Job{
		ID:           "test-job-2",
		ParentTaskID: "task-1",
		AgentRole:    "tester",
		Payload:      "{}",
		MaxAttempts:  3,
	}
	q.Enqueue(ctx, job2)
	q.Dequeue(ctx, []string{"tester"})

	if err := q.Fail(ctx, "test-job-2", "some error"); err != nil {
		t.Fatalf("Fail failed: %v", err)
	}

	var attempts int
	err = provider.QueryRow(ctx, "SELECT status, attempts FROM sub_agent_jobs WHERE id = 'test-job-2'").Scan(&status, &attempts)
	if err != nil {
		t.Fatalf("Failed to query job: %v", err)
	}
	if status != "QUEUED" { // Should be requeued
		t.Fatalf("Expected status QUEUED, got %s", status)
	}
	if attempts != 1 {
		t.Fatalf("Expected 1 attempt, got %d", attempts)
	}
}

func TestSQLiteTaskQueue_DequeueMiss(t *testing.T) {
	provider := newTestProvider(t)
	ctx := context.Background()

	schema := `
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
	`
	provider.Exec(ctx, schema)
	q := NewSQLiteTaskQueue(provider)

	// Dequeue from empty queue
	dequeued, err := q.Dequeue(ctx, []string{"tester"})
	if err != nil {
		t.Fatalf("Expected nil err, got %v", err)
	}
	if dequeued != nil {
		t.Fatal("Expected nil job")
	}
}

func TestSQLiteTaskQueue_FailMaxAttempts(t *testing.T) {
	provider := newTestProvider(t)
	ctx := context.Background()

	schema := `
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
	`
	provider.Exec(ctx, schema)
	q := NewSQLiteTaskQueue(provider)

	job := &Job{
		ID:           "test-job-fail-max",
		ParentTaskID: "task-1",
		AgentRole:    "tester",
		Payload:      "{}",
		MaxAttempts:  1,
	}
	q.Enqueue(ctx, job)
	q.Dequeue(ctx, []string{"tester"})

	if err := q.Fail(ctx, "test-job-fail-max", "final error"); err != nil {
		t.Fatalf("Fail failed: %v", err)
	}

	var status string
	var attempts int
	err := provider.QueryRow(ctx, "SELECT status, attempts FROM sub_agent_jobs WHERE id = 'test-job-fail-max'").Scan(&status, &attempts)
	if err != nil {
		t.Fatalf("Failed to query job: %v", err)
	}
	if status != "FAILED" {
		t.Fatalf("Expected status FAILED, got %s", status)
	}
}
