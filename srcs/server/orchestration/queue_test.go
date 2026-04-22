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


func TestSQLiteTaskQueue_ConcurrentPoll(t *testing.T) {
	os.Setenv("OHC_MULTITENANT", "false")
	defer os.Unsetenv("OHC_MULTITENANT")

	prov := db.NewTestProvider(t)
	defer prov.Close()

	q := NewTaskQueue(prov, nil)
	ctx := context.Background()

	// Enqueue a single job
	_, err := q.Enqueue(ctx, "test_queue", map[string]interface{}{"data": "value"})
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}

	// Concurrently attempt to poll
	errCh := make(chan error, 10)
	taskCh := make(chan *QueuedTask, 10)

	for i := 0; i < 10; i++ {
		go func() {
			task, err := q.Poll(ctx, "test_queue")
			if err != nil {
				errCh <- err
				return
			}
			if task != nil {
				taskCh <- task
			}
			errCh <- nil
		}()
	}

	tasksAcquired := 0
	for i := 0; i < 10; i++ {
		err := <-errCh
		if err != nil {
			t.Fatalf("unexpected error during poll: %v", err)
		}
	}

	close(taskCh)
	for range taskCh {
		tasksAcquired++
	}

	// Verify only exactly 1 consumer acquired it
	if tasksAcquired != 1 {
		t.Fatalf("expected exactly 1 task acquired, got %d", tasksAcquired)
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
