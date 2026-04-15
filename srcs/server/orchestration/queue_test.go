package orchestration

import (
	"context"
	"os"
	"testing"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
)

func TestSQLiteTaskQueue(t *testing.T) {
	os.Setenv("OHC_MULTITENANT", "false")
	defer os.Unsetenv("OHC_MULTITENANT")

	prov := db.NewTestProvider(t)
	defer prov.Close()

	q := NewTaskQueue(prov, nil)

	ctx := context.Background()

	// Enqueue
	id, err := q.Enqueue(ctx, "test_queue", map[string]interface{}{"data": "value"})
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}
	if id == "" {
		t.Fatal("expected non-empty id")
	}

	// Poll
	task, err := q.Poll(ctx, "test_queue")
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}
	if task == nil {
		t.Fatal("expected task")
	}
	if task.ID != id {
		t.Errorf("expected id %s, got %s", id, task.ID)
	}
	if task.Payload["data"] != "value" {
		t.Errorf("expected value, got %v", task.Payload["data"])
	}

	// Complete
	err = q.Complete(ctx, "test_queue", id)
	if err != nil {
		t.Fatalf("expected no error on complete, got %v", err)
	}

	// Poll again (should be empty)
	task2, err := q.Poll(ctx, "test_queue")
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}
	if task2 != nil {
		t.Fatal("expected nil task, got", task2)
	}
}

func TestSQLiteTaskQueue_Delayed(t *testing.T) {
	os.Setenv("OHC_MULTITENANT", "false")
	defer os.Unsetenv("OHC_MULTITENANT")

	prov := db.NewTestProvider(t)
	defer prov.Close()

	q := NewTaskQueue(prov, nil)

	ctx := context.Background()

	id, err := q.EnqueueDelayed(ctx, "test_queue", map[string]interface{}{"data": "value"}, 100*time.Millisecond)
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}

	// Poll immediately (should be empty)
	task, err := q.Poll(ctx, "test_queue")
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}
	if task != nil {
		t.Fatal("expected nil task immediately")
	}

	// Wait and poll again
	time.Sleep(150 * time.Millisecond)
	task, err = q.Poll(ctx, "test_queue")
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}
	if task == nil {
		t.Fatal("expected task after delay")
	}
	if task.ID != id {
		t.Errorf("expected id %s, got %s", id, task.ID)
	}
}
