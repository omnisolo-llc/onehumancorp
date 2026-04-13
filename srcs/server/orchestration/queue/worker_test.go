package queue

import (
	"context"
	"errors"
	"testing"
	"time"
)

func TestWorkerLoop(t *testing.T) {
	provider := newTestProvider(t)
	ctx, cancel := context.WithCancel(context.Background())
	defer cancel()

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

	qm := NewQueueManager(provider)

	job1 := &Job{
		ID:           "job-worker-1",
		ParentTaskID: "task-1",
		AgentRole:    "worker",
		Payload:      "{}",
		MaxAttempts:  3,
	}
	job2 := &Job{
		ID:           "job-worker-2",
		ParentTaskID: "task-1",
		AgentRole:    "worker",
		Payload:      "{}",
		MaxAttempts:  3,
	}

	qm.Enqueue(ctx, job1)
	qm.Enqueue(ctx, job2)

	processed := make(chan string, 2)

	handler := func(c context.Context, job *Job) error {
		if job.ID == "job-worker-2" {
			return errors.New("simulated error")
		}
		processed <- job.ID
		return nil
	}

	go WorkerLoop(ctx, qm, []string{"worker"}, handler)

	select {
	case id := <-processed:
		if id != "job-worker-1" {
			t.Errorf("expected job-worker-1 to be processed, got %s", id)
		}
	case <-time.After(2 * time.Second):
		t.Fatal("timeout waiting for job processing")
	}

	// wait for job 2 to fail and be requeued
	time.Sleep(500 * time.Millisecond)

	var status string
	var attempts int
	err = provider.QueryRow(ctx, "SELECT status, attempts FROM sub_agent_jobs WHERE id = 'job-worker-2'").Scan(&status, &attempts)
	if err != nil {
		t.Fatalf("failed to query job-worker-2: %v", err)
	}
	if attempts != 1 {
		t.Errorf("expected 1 attempt, got %d", attempts)
	}
	if status != "QUEUED" { // it should be requeued
		t.Errorf("expected status QUEUED, got %s", status)
	}
}
