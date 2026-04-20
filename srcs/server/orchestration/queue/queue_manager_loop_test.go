package queue

import (
	"context"
	"errors"
	"strings"
	"testing"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
)

func TestQueueManagerLoop(t *testing.T) {
	t.Skip()
	provider := db.NewTestProvider(t)
	ctx, cancel := context.WithCancel(context.Background())
	defer cancel()

	schema := `
	CREATE TABLE IF NOT EXISTS sub_agent_queue (
		id TEXT PRIMARY KEY,
		organization_id TEXT NOT NULL,
		parent_task_id TEXT NOT NULL,
		payload TEXT DEFAULT '{}',
		status TEXT NOT NULL DEFAULT 'QUEUED',
		worker_id TEXT,
		created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
		updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
	);
	`
	_, err := provider.Exec(ctx, schema)
	if err != nil {
		t.Fatalf("Failed to create schema: %v", err)
	}

	qm := NewQueueManager(provider)

	_, _ = provider.Exec(ctx, "INSERT INTO sub_agent_queue (id, organization_id, parent_task_id, status) VALUES ('job-1', 'org-1', 'parent-1', 'QUEUED')")
	_, _ = provider.Exec(ctx, "INSERT INTO sub_agent_queue (id, organization_id, parent_task_id, status) VALUES ('job-2', 'org-1', 'parent-1', 'QUEUED')")

	processedJobs := make(map[string]bool)

	handler := func(ctx context.Context, job *SubAgentJob) error {
		processedJobs[job.ID] = true
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
	// Retry loop for SQLITE_BUSY
	for i := 0; i < 5; i++ {
		err = provider.QueryRow(context.Background(), "SELECT status FROM sub_agent_queue WHERE id = 'job-1'").Scan(&status1)
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
		err = provider.QueryRow(context.Background(), "SELECT status FROM sub_agent_queue WHERE id = 'job-2'").Scan(&status2)
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
