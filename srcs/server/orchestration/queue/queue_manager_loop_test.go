package queue

import (
	"context"
	"errors"
	"github.com/onehumancorp/mono/srcs/server/db"
	"testing"
	"strings"
	"time"
)

func TestQueueManagerLoop(t *testing.T) {
	provider := db.NewTestProvider(t)
	ctx, cancel := context.WithCancel(context.Background())
	defer cancel()

	schema := `
	CREATE TABLE IF NOT EXISTS sub_agent_queue (
		id TEXT PRIMARY KEY,
		organization_id TEXT NOT NULL,
		parent_task_id TEXT NOT NULL,
		payload TEXT,
		status TEXT NOT NULL DEFAULT 'QUEUED',
		attempts INTEGER DEFAULT 0,
		max_attempts INTEGER DEFAULT 3,
		run_after TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
		worker_id TEXT,
		created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
		updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
	);
	`
	_, err := provider.Exec(ctx, schema)
	if err != nil {
		t.Fatalf("failed to create schema: %v", err)
	}

	qm := NewQueueManager(provider)

	job1 := &SubAgentJob{
		ID:             "job-1",
		OrganizationID: "org-1",
		ParentTaskID:   "task-1",
		Payload:        map[string]interface{}{"key": "val1"},
	}
	job2 := &SubAgentJob{
		ID:             "job-2",
		OrganizationID: "org-1",
		ParentTaskID:   "task-1",
		Payload:        map[string]interface{}{"key": "val2"},
	}

	err = qm.Enqueue(ctx, job1)
	if err != nil {
		t.Fatalf("failed to enqueue: %v", err)
	}
	err = qm.Enqueue(ctx, job2)
	if err != nil {
		t.Fatalf("failed to enqueue: %v", err)
	}

	processedJobs := make([]string, 0)
	handler := func(ctx context.Context, job *SubAgentJob) error {
		processedJobs = append(processedJobs, job.ID)
		if job.ID == "job-2" {
			return errors.New("simulated failure")
		}
		return nil
	}

	go qm.StartPolling(ctx, "worker-1", 10*time.Millisecond, handler)

	time.Sleep(100 * time.Millisecond)
	cancel() // stop polling
	time.Sleep(50 * time.Millisecond)

	if len(processedJobs) != 2 {
		t.Fatalf("Expected 2 jobs to be processed, got %d", len(processedJobs))
	}

	// Verify status in DB
	var status1, status2 string
	// Retry loop for SQLITE_BUSY
	for i := 0; i < 5; i++ {
		err = provider.QueryRow(context.TODO(), "SELECT status FROM sub_agent_queue WHERE id = 'job-1'").Scan(&status1)
		if err == nil || (err != nil && !strings.Contains(err.Error(), "database is locked")) {
			break
		}
		time.Sleep(50 * time.Millisecond)
	}
	if err != nil {
		t.Fatalf("Failed to query status: %v", err)
	}
	if status1 != "COMPLETED" {
		t.Fatalf("Expected job-1 status COMPLETED, got %s", status1)
	}

	for i := 0; i < 5; i++ {
		err = provider.QueryRow(context.TODO(), "SELECT status FROM sub_agent_queue WHERE id = 'job-2'").Scan(&status2)
		if err == nil || (err != nil && !strings.Contains(err.Error(), "database is locked")) {
			break
		}
		time.Sleep(50 * time.Millisecond)
	}
	if err != nil {
		t.Fatalf("Failed to query status: %v", err)
	}
	// Expected job-2 to be QUEUED because of retry backoff
	if status2 != "QUEUED" {
		t.Fatalf("Expected job-2 status QUEUED due to backoff, got %s", status2)
	}
}

func TestQueueManager_QuotaEnforcement(t *testing.T) {
	provider := db.NewTestProvider(t)
	ctx := context.Background()

	schema := `
	CREATE TABLE IF NOT EXISTS sub_agent_queue (
		id TEXT PRIMARY KEY,
		organization_id TEXT NOT NULL,
		parent_task_id TEXT NOT NULL,
		payload TEXT,
		status TEXT NOT NULL DEFAULT 'QUEUED',
		attempts INTEGER DEFAULT 0,
		max_attempts INTEGER DEFAULT 3,
		run_after TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
		worker_id TEXT,
		created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
		updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
	);
	`
	_, err := provider.Exec(ctx, schema)
	if err != nil {
		t.Fatalf("failed to create schema: %v", err)
	}

	qm := NewQueueManager(provider)

	// Simulate 10 running agents
	for i := 1; i <= 10; i++ {
		_, err := provider.Exec(ctx, "INSERT INTO sub_agent_queue (id, organization_id, parent_task_id, payload, status, run_after) VALUES (?, 'org-1', 'task-1', '{}', 'RUNNING', ?)", "running-"+string(rune(i)), time.Now())
		if err != nil {
			t.Fatalf("failed to insert running job: %v", err)
		}
	}

	// Insert 1 pending job
	job := &SubAgentJob{
		ID:             "job-11",
		OrganizationID: "org-1",
		ParentTaskID:   "task-1",
		Payload:        map[string]interface{}{},
		RunAfter:       time.Now(),
	}
	err = qm.Enqueue(ctx, job)
	if err != nil {
		t.Fatalf("failed to enqueue: %v", err)
	}

	// Attempt to poll. It should return nil because 10 are already running
	polledJob, err := qm.Poll(ctx, "worker-1")
	if err != nil {
		t.Fatalf("poll failed: %v", err)
	}
	if polledJob != nil {
		t.Fatalf("Expected nil job due to quota, got %v", polledJob.ID)
	}
}

func TestQueueManager_ExponentialBackoff(t *testing.T) {
	provider := db.NewTestProvider(t)
	ctx := context.Background()

	schema := `
	CREATE TABLE IF NOT EXISTS sub_agent_queue (
		id TEXT PRIMARY KEY,
		organization_id TEXT NOT NULL,
		parent_task_id TEXT NOT NULL,
		payload TEXT,
		status TEXT NOT NULL DEFAULT 'QUEUED',
		attempts INTEGER DEFAULT 0,
		max_attempts INTEGER DEFAULT 3,
		run_after TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
		worker_id TEXT,
		created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
		updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
	);
	`
	_, err := provider.Exec(ctx, schema)
	if err != nil {
		t.Fatalf("failed to create schema: %v", err)
	}

	qm := NewQueueManager(provider)

	// Insert job with 2 attempts
	_, err = provider.Exec(ctx, "INSERT INTO sub_agent_queue (id, organization_id, parent_task_id, payload, status, attempts, max_attempts) VALUES ('job-failed', 'org-1', 'task-1', '{}', 'RUNNING', 2, 3)")
	if err != nil {
		t.Fatalf("failed to insert running job: %v", err)
	}

	err = qm.MarkFailed(ctx, "job-failed", "reason")
	if err != nil {
		t.Fatalf("MarkFailed returned error: %v", err)
	}

	var status string
	var attempts int
	err = provider.QueryRow(ctx, "SELECT status, attempts FROM sub_agent_queue WHERE id = 'job-failed'").Scan(&status, &attempts)
	if err != nil {
		t.Fatalf("failed to fetch job: %v", err)
	}

	if status != "QUEUED" {
		t.Fatalf("expected status QUEUED, got %s", status)
	}
	if attempts != 3 {
		t.Fatalf("expected attempts 3, got %d", attempts)
	}

	// Fail again -> should hit max_attempts and become FAILED
	err = qm.MarkFailed(ctx, "job-failed", "reason")
	if err != nil {
		t.Fatalf("MarkFailed returned error: %v", err)
	}

	err = provider.QueryRow(ctx, "SELECT status FROM sub_agent_queue WHERE id = 'job-failed'").Scan(&status)
	if err != nil {
		t.Fatalf("failed to fetch job: %v", err)
	}

	if status != "FAILED" {
		t.Fatalf("expected status FAILED after hitting max attempts, got %s", status)
	}
}
