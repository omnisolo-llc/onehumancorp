package orchestration

import (
	"context"
	"database/sql"
	"testing"
	"time"

	_ "github.com/mattn/go-sqlite3"
)

func TestSqliteQueue(t *testing.T) {
	db, err := sql.Open("sqlite3", ":memory:")
	if err != nil {
		t.Fatalf("Failed to open db: %v", err)
	}
	defer db.Close()

	_, err = db.Exec(`
		CREATE TABLE sub_agent_queue (
			id TEXT PRIMARY KEY,
			parent_task_id TEXT,
			payload TEXT,
			status TEXT,
			scheduled_at DATETIME,
			completed_at DATETIME
		)
	`)
	if err != nil {
		t.Fatalf("Failed to create table: %v", err)
	}

	queue := NewSqliteQueue(db)
	ctx := context.Background()

	job := SubAgentJob{
		ID:           "job-1",
		ParentTaskID: "task-1",
		Payload:      []byte(`{"data":"test"}`),
		ScheduledAt:  time.Now().Add(-1 * time.Minute),
	}

	if err := queue.Enqueue(ctx, job); err != nil {
		t.Fatalf("Enqueue failed: %v", err)
	}

	dequeued, err := queue.Dequeue(ctx)
	if err != nil {
		t.Fatalf("Dequeue failed: %v", err)
	}
	if dequeued == nil {
		t.Fatal("Expected to dequeue a job, got nil")
	}
	if dequeued.ID != job.ID {
		t.Errorf("Expected job ID %s, got %s", job.ID, dequeued.ID)
	}

	err = queue.Complete(ctx, dequeued.ID)
	if err != nil {
		t.Fatalf("Complete failed: %v", err)
	}
}
