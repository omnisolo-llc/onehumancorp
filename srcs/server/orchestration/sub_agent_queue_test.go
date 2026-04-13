package orchestration

import (
	"context"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/db"
)

func TestSQLiteSubAgentQueue(t *testing.T) {
	provider := db.NewTestProvider(t)
	// No defer Close, provider implementation handles its own lifecycle in testing

	ctx := context.Background()

	// Ensure table exists for test
	_, err := provider.Exec(ctx, `
		CREATE TABLE IF NOT EXISTS sub_agent_queue (
			id TEXT PRIMARY KEY,
			organization_id TEXT NOT NULL,
			parent_task_id TEXT NOT NULL,
			payload TEXT,
			status TEXT NOT NULL DEFAULT 'QUEUED',
			worker_id TEXT,
			created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
			updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
		)
	`)
	if err != nil {
		t.Fatalf("Failed to create table: %v", err)
	}

	queue := NewSubAgentQueue(provider, nil, "test_queue")

	job := Job{
		ID:             "job-1",
		OrganizationID: "org-1",
		ParentTaskID:   "task-1",
		Payload:        map[string]interface{}{"foo": "bar"},
	}

	if err := queue.Enqueue(ctx, job); err != nil {
		t.Fatalf("Enqueue failed: %v", err)
	}

	dequeued, err := queue.Dequeue(ctx)
	if err != nil {
		t.Fatalf("Dequeue failed: %v", err)
	}
	if dequeued == nil {
		t.Fatal("Expected a job, got nil")
	}
	if dequeued.ID != job.ID {
		t.Errorf("Expected job ID %s, got %s", job.ID, dequeued.ID)
	}

	if err := queue.Ack(ctx, job.ID); err != nil {
		t.Fatalf("Ack failed: %v", err)
	}

	dequeued2, err := queue.Dequeue(ctx)
	if err != nil {
		t.Fatalf("Dequeue failed: %v", err)
	}
	if dequeued2 != nil {
		t.Fatal("Expected nil, got a job")
	}

	queue.Enqueue(ctx, Job{ID: "job-2", OrganizationID: "org-1", ParentTaskID: "task-1"})
	dq, _ := queue.Dequeue(ctx)
	if err := queue.Nack(ctx, dq.ID); err != nil {
		t.Fatalf("Nack failed: %v", err)
	}
	dqAgain, _ := queue.Dequeue(ctx)
	if dqAgain == nil || dqAgain.ID != "job-2" {
		t.Fatal("Expected job-2 to be back in queue after Nack")
	}
}

func TestRedisSubAgentQueue_Fallback(t *testing.T) {
	queue := NewSubAgentQueue(nil, nil, "test_queue")
	if queue == nil {
		t.Fatal("Expected fallback to SQLiteSubAgentQueue")
	}
}
