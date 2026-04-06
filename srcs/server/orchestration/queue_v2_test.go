package orchestration_test

import (
	"context"
	"testing"
	"database/sql"
	_ "modernc.org/sqlite"

	"github.com/onehumancorp/mono/srcs/server/db"
	"github.com/onehumancorp/mono/srcs/server/orchestration"
)

func TestSqliteQueue(t *testing.T) {
	dbConn, err := sql.Open("sqlite", ":memory:")
	if err != nil {
		t.Fatalf("failed to open db: %v", err)
	}
	provider := db.NewSqliteProvider(dbConn)

	queue := orchestration.NewSqliteQueue(provider)
	ctx := context.Background()

	// Test Enqueue
	payload := map[string]interface{}{"key": "value"}
	id, err := queue.Enqueue(ctx, "task-123", payload)
	if err != nil {
		t.Fatalf("failed to enqueue: %v", err)
	}
	if id == "" {
		t.Fatal("expected non-empty id")
	}

	// Test Dequeue
	task, err := queue.Dequeue(ctx)
	if err != nil {
		t.Fatalf("failed to dequeue: %v", err)
	}
	if task == nil {
		t.Fatal("expected task, got nil")
	}
	if task.ID != id {
		t.Errorf("expected id %q, got %q", id, task.ID)
	}
	if task.ParentTaskID != "task-123" {
		t.Errorf("expected parent task id 'task-123', got %q", task.ParentTaskID)
	}

	// Test Complete
	err = queue.Complete(ctx, id)
	if err != nil {
		t.Fatalf("failed to complete: %v", err)
	}
}
