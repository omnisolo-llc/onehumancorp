package queue

import (
	"database/sql"
	"time"

	"context"
	"testing"

	"github.com/onehumancorp/mono/src/server/db"
	_ "modernc.org/sqlite"
)


func TestSQLiteTaskQueue(t *testing.T) {
	provider := db.NewTestProvider(t)

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

func TestQueueManager(t *testing.T) {
	provider := db.NewTestProvider(t)
	ctx := context.Background()

	schema := `
	CREATE TABLE IF NOT EXISTS sub_agent_queue (
		id TEXT PRIMARY KEY,
		organization_id TEXT NOT NULL,
		parent_task_id TEXT NOT NULL,
		payload JSONB,
		status TEXT NOT NULL DEFAULT 'QUEUED',
		worker_id TEXT,
		created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
		updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
	);
	`
	_, err := provider.Exec(ctx, schema)
	if err != nil {
		t.Fatalf("failed to create schema: %v", err)
	}

	qm := NewQueueManager(provider)

	job := &SubAgentJob{
		ID:             "job-1",
		OrganizationID: "org-1",
		ParentTaskID:   "task-1",
		Payload:        map[string]interface{}{"key": "value"},
	}

	err = qm.Enqueue(ctx, job)
	if err != nil {
		t.Fatalf("failed to enqueue: %v", err)
	}

	polledJob, err := qm.Acquire(ctx, "worker-1")
	if err != nil {
		t.Fatalf("failed to poll: %v", err)
	}
	if polledJob == nil {
		t.Fatalf("expected job, got nil")
	}

	if polledJob.ID != "job-1" {
		t.Errorf("expected job ID job-1, got %s", polledJob.ID)
	}
	if polledJob.Status != "RUNNING" {
		t.Errorf("expected status RUNNING, got %s", polledJob.Status)
	}

	polledJob2, err := qm.Acquire(ctx, "worker-2")
	if err != nil {
		t.Fatalf("failed to poll second time: %v", err)
	}
	if polledJob2 != nil {
		t.Fatalf("expected nil job, got %v", polledJob2)
	}
}


// added for Sub-Agent Orchestration Queue - issue_id: 4240

type mockPostgresProvider struct {
	db.Provider
}

func (m *mockPostgresProvider) IsSQLite() bool {
	return false
}

type mockRow struct {
	err error
	scanFunc func(dest ...any) error
}

func (m *mockRow) Scan(dest ...any) error {
	if m.err != nil {
		return m.err
	}
	if m.scanFunc != nil {
		return m.scanFunc(dest...)
	}
	return nil
}

type mockPostgresProviderPoll struct {
	db.Provider
	err error
	scanFunc func(dest ...any) error
}

func (m *mockPostgresProviderPoll) IsSQLite() bool {
	return false
}

func (m *mockPostgresProviderPoll) QueryRow(ctx context.Context, query string, args ...any) db.Row {
	return &mockRow{err: m.err, scanFunc: m.scanFunc}
}

func TestQueueManager_Postgres_Poll_Success(t *testing.T) {
	provider := db.NewTestProvider(t)
	mockProvider := &mockPostgresProviderPoll{
		Provider: provider,
		scanFunc: func(dest ...any) error {
			*dest[0].(*string) = "pg-id"
			*dest[1].(*string) = "org"
			*dest[2].(*string) = "task"
			*dest[3].(*string) = `{"key":"pg-val"}`
			*dest[4].(*string) = "RUNNING"
			*dest[5].(*sql.NullString) = sql.NullString{String: "worker-1", Valid: true}
			*dest[6].(*time.Time) = time.Now()
			*dest[7].(*time.Time) = time.Now()
			return nil
		},
	}

	qm := NewQueueManager(mockProvider)
	ctx := context.Background()

	polledJob, err := qm.Acquire(ctx, "worker-1")
	if err != nil {
		t.Fatalf("expected success, got %v", err)
	}
	if polledJob == nil {
		t.Fatalf("expected job, got nil")
	}
	if polledJob.ID != "pg-id" {
		t.Fatalf("expected pg-id, got %v", polledJob.ID)
	}
}

func TestQueueManager_Postgres_Poll_NoRows(t *testing.T) {
	provider := db.NewTestProvider(t)
	mockProvider := &mockPostgresProviderPoll{
		Provider: provider,
		err: sql.ErrNoRows,
	}

	qm := NewQueueManager(mockProvider)
	ctx := context.Background()

	polledJob, err := qm.Acquire(ctx, "worker-1")
	if err != nil {
		t.Fatalf("expected nil err, got %v", err)
	}
	if polledJob != nil {
		t.Fatalf("expected nil job, got %v", polledJob)
	}
}
