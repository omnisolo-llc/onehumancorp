package orchestration

import (
	"context"
	"testing"
	"github.com/onehumancorp/mono/srcs/server/db"
)

func TestSqliteQueue_EnqueueDequeue(t *testing.T) {
	prov := db.NewTestProvider(t)
	defer prov.Close()

	q := NewSqliteQueue(prov)
	ctx := context.Background()

	parentTaskID := "parent-1"
	payload := map[string]interface{}{"key": "value"}

	id, err := q.Enqueue(ctx, parentTaskID, payload)
	if err != nil {
		t.Fatalf("Enqueue failed: %v", err)
	}
	if id == "" {
		t.Fatalf("Expected non-empty id")
	}

	task, err := q.Dequeue(ctx)
	if err != nil {
		t.Fatalf("Dequeue failed: %v", err)
	}
	if task == nil {
		t.Fatalf("Expected to dequeue a task, got nil")
	}
	if task.ID != id {
		t.Errorf("Expected id %s, got %s", id, task.ID)
	}
	if task.Payload["key"] != "value" {
		t.Errorf("Expected value, got %v", task.Payload["key"])
	}

	err = q.Complete(ctx, id)
	if err != nil {
		t.Fatalf("Complete failed: %v", err)
	}
}
