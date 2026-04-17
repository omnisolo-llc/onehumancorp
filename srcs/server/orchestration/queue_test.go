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


func TestJobQueue_MapHighLevelTask(t *testing.T) {
	t.Setenv("OHC_MULTITENANT", "false")
	provider, cleanup := db.SetupTestDB(t)
	defer cleanup()

	jq := &JobQueue{DB: provider}
	ctx := context.Background()
	task := &QueuedTask{
		Payload: map[string]interface{}{
			"title": "test task",
			"organization_id": "test-org",
		},
	}

	err := jq.MapHighLevelTask(ctx, task)
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}
}


func TestTaskQueueService(t *testing.T) {
	os.Setenv("OHC_MULTITENANT", "false")
	defer os.Unsetenv("OHC_MULTITENANT")

	prov := db.NewTestProvider(t)
	defer prov.Close()

	ctx := context.Background()
	db.InitializeSchemas(ctx, prov)

	q := NewTaskQueueService(prov)

	task := SharedTaskDTO{
		ID:            "task1",
		Title:         "Test Task",
		Status:        "PENDING",
		AssignedAgent: nil,
		Payload:       "{}",
	}

	err := q.Push(ctx, task)
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}

	claimedTask, err := q.Claim(ctx, "agent1")
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}
	if claimedTask == nil || claimedTask.ID != "task1" || claimedTask.Status != "IN_PROGRESS" || claimedTask.AssignedAgent == nil || *claimedTask.AssignedAgent != "agent1" {
		t.Fatalf("task claim failed: %+v", claimedTask)
	}

	err = q.Complete(ctx, "task1")
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}

	// Double check nothing else can be claimed
	claimedTask2, err := q.Claim(ctx, "agent2")
	if err == nil {
		t.Fatalf("expected error, got task: %+v", claimedTask2)
	}
}
