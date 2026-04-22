package queue_test

import (
	"context"
	"testing"
	"github.com/onehumancorp/mono/srcs/server/db"
	"github.com/onehumancorp/mono/srcs/server/orchestration/queue"
)

func TestSQLiteSubAgentTaskQueue_E2E(t *testing.T) {
	ctx := context.Background()
	provider := db.NewTestProvider(t)

	_, err := provider.Exec(ctx, "CREATE TABLE sub_agent_tasks (job_id TEXT PRIMARY KEY, queue_name TEXT, payload TEXT, status TEXT, created_at TIMESTAMP)")
	if err != nil {
		t.Fatalf("failed to create table: %v", err)
	}

	opts := queue.QueueOptions{MaxRetries: 3, RateLimitRate: 10, DLQName: "dlq"}
	q := queue.NewSQLiteSubAgentTaskQueue(provider, opts)

	payload := &queue.SubAgentTaskQueuePayload{
		JobID:     "job-e2e",
		QueueName: "agent-q",
		Data: queue.SubAgentTaskData{
			IssueRef: "ref-42",
		},
	}

	if err := q.Enqueue(ctx, payload); err != nil {
		t.Fatalf("failed to enqueue: %v", err)
	}

	processed, err := q.Process(ctx, "agent-q")
	if err != nil {
		t.Fatalf("failed to process: %v", err)
	}

	if processed == nil || processed.JobID != "job-e2e" {
		t.Fatalf("expected job-e2e, got %v", processed)
	}

	if err := q.Fail(ctx, "job-e2e", "agent-q", "test failure"); err != nil {
		t.Fatalf("failed to fail job: %v", err)
	}

	payload2 := &queue.SubAgentTaskQueuePayload{
		JobID:     "job-e2e-2",
		QueueName: "agent-q",
		Data: queue.SubAgentTaskData{
			IssueRef: "ref-43",
		},
	}
	if err := q.Enqueue(ctx, payload2); err != nil {
		t.Fatalf("failed to enqueue: %v", err)
	}

	processed2, _ := q.Process(ctx, "")
	if err := q.Complete(ctx, processed2.JobID, "agent-q"); err != nil {
		t.Fatalf("failed to complete job: %v", err)
	}
}
