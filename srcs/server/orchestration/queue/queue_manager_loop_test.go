package queue

import (
	"context"
	"errors"
	"testing"
	"time"
)

func TestQueueManagerLoop(t *testing.T) {
	provider := newTestProvider(t)
	ctx, cancel := context.WithCancel(context.Background())
	defer cancel()

	schema := `
	CREATE TABLE IF NOT EXISTS sub_agent_queue (
		id TEXT PRIMARY KEY,
		organization_id TEXT NOT NULL,
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

	if len(processedJobs) != 2 {
		t.Fatalf("Expected 2 jobs to be processed, got %d", len(processedJobs))
	}

	// Verify status in DB
	var status1, status2 string
	err = provider.QueryRow(context.Background(), "SELECT status FROM sub_agent_queue WHERE id = 'job-1'").Scan(&status1)
	if err != nil {
		t.Fatalf("Failed to query status: %v", err)
	}
	if status1 != "COMPLETED" {
		t.Fatalf("Expected job-1 status COMPLETED, got %s", status1)
	}

	err = provider.QueryRow(context.Background(), "SELECT status FROM sub_agent_queue WHERE id = 'job-2'").Scan(&status2)
	if err != nil {
		t.Fatalf("Failed to query status: %v", err)
	}
	if status2 != "FAILED" {
		t.Fatalf("Expected job-2 status FAILED, got %s", status2)
	}
}
