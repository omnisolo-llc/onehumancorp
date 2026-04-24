package queue

import (
	"github.com/onehumancorp/mono/src/server/db"

	"context"
	"testing"
)

func TestSQLiteSubAgentTaskQueue(t *testing.T) {
	provider := db.NewTestProvider(t)
	ctx := context.Background()

	schema := `
	CREATE TABLE IF NOT EXISTS sub_agent_tasks (
		job_id TEXT PRIMARY KEY,
		queue_name TEXT NOT NULL,
		payload TEXT NOT NULL,
		status TEXT NOT NULL DEFAULT 'QUEUED',
		created_at DATETIME DEFAULT CURRENT_TIMESTAMP
	);
	`
	if _, err := provider.Exec(ctx, schema); err != nil {
		t.Fatalf("failed to create schema: %v", err)
	}

	q := NewSQLiteSubAgentTaskQueue(provider)
	payload := &SubAgentTaskQueuePayload{
		JobID:     "worker-task-77",
		QueueName: "l5-implementers",
		Data: SubAgentTaskData{
			IssueRef:             "github-issue-123",
			RepositoryStateHash:  "sha256-abc",
			ExecutionTimeoutMs:   3600000,
		},
	}

	if err := q.Enqueue(ctx, payload); err != nil {
		t.Fatalf("Enqueue failed: %v", err)
	}

	p, err := q.Process(ctx, "l5-implementers")
	if err != nil {
		t.Fatalf("Process failed: %v", err)
	}
	if p == nil {
		t.Fatalf("Expected job, got nil")
	}
	if p.JobID != "worker-task-77" {
		t.Fatalf("Expected job ID worker-task-77, got %s", p.JobID)
	}
}
