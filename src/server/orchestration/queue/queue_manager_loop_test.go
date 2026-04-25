package queue

import (
	"context"
	"errors"
	"github.com/onehumancorp/mono/src/server/db"
	"testing"
	"strings"
	"time"
)

func TestQueueManagerLoop(t *testing.T) {
	provider := db.NewTestProvider(t)
	ctx := context.Background()
	pollCtx, cancelPoll := context.WithCancel(ctx)
	defer cancelPoll()

	schema := `
	CREATE TABLE IF NOT EXISTS sub_agent_queue (
		id TEXT PRIMARY KEY,
		organization_id TEXT NOT NULL,
		tenant_id TEXT NOT NULL DEFAULT '',
		parent_task_id TEXT NOT NULL,
		payload TEXT,
		status TEXT NOT NULL DEFAULT 'QUEUED',
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

	// Keep a connection alive to prevent memory DB from disappearing in cache=shared mode
	txKeepAlive, _ := provider.Begin(ctx)
	if txKeepAlive != nil {
		defer txKeepAlive.Rollback(ctx)
	}

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

	go qm.StartPolling(pollCtx, "worker-1", 10*time.Millisecond, handler)

	// Wait for jobs to be processed
	for i := 0; i < 20; i++ {
		time.Sleep(100 * time.Millisecond)
		if len(processedJobs) >= 2 {
			break
		}
	}
	cancelPoll() // stop polling
	time.Sleep(50 * time.Millisecond)

	if len(processedJobs) < 2 {
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
	if status2 != "FAILED" {
		t.Fatalf("Expected job-2 status FAILED, got %s", status2)
	}
}
