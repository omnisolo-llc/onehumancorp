package orchestration

import (
	"context"
	"database/sql"
	"encoding/json"
	"sync"
	"sync/atomic"
	"testing"
	"time"

	"github.com/google/uuid"
	_ "github.com/mattn/go-sqlite3"
)

// MockAgentHarness is a mock harness for testing WorkerPool.
type MockAgentHarness struct {
	ExecutionCount int32
	mu             sync.Mutex
	tasks          []*Task
}

func (m *MockAgentHarness) Execute(ctx context.Context, task *Task) error {
	atomic.AddInt32(&m.ExecutionCount, 1)
	m.mu.Lock()
	m.tasks = append(m.tasks, task)
	m.mu.Unlock()
	// Simulate work
	time.Sleep(10 * time.Millisecond)
	return nil
}

func setupQueueTestDB(t *testing.T) *sql.DB {
	// Use an in-memory SQLite database but with a unique shared cache per test
	// so that connections see the same tables.
	db, err := sql.Open("sqlite3", "file:"+uuid.New().String()+"?mode=memory&cache=shared&_foreign_keys=on")
	if err != nil {
		t.Fatalf("Failed to open test db: %v", err)
	}

	// Create table exactly matching the migration schema
	createTable := `
	CREATE TABLE sub_agent_queue (
		id TEXT PRIMARY KEY,
		organization_id VARCHAR NOT NULL,
		parent_task_id TEXT NOT NULL,
		payload JSONB,
		status VARCHAR NOT NULL DEFAULT 'QUEUED',
		worker_id VARCHAR,
		created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
		updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
	);`

	_, err = db.Exec(createTable)
	if err != nil {
		t.Fatalf("Failed to create test table: %v", err)
	}

	return db
}

func TestDBTaskQueue(t *testing.T) {
	db := setupQueueTestDB(t)
	defer db.Close()

	ctx := context.Background()
	queue := NewDBTaskQueue(db, "test-worker-1", true)

	taskID := uuid.New().String()
	parentTaskID := uuid.New().String()
	payload := json.RawMessage(`{"key": "value"}`)

	t.Run("Enqueue", func(t *testing.T) {
		task := &Task{
			ID:             taskID,
			OrganizationID: "org-1",
			ParentTaskID:   parentTaskID,
			Payload:        payload,
		}

		err := queue.Enqueue(ctx, task)
		if err != nil {
			t.Fatalf("Enqueue failed: %v", err)
		}

		// Verify task was inserted
		var status string
		err = db.QueryRow("SELECT status FROM sub_agent_queue WHERE id = ?", taskID).Scan(&status)
		if err != nil {
			t.Fatalf("Failed to verify inserted task: %v", err)
		}
		if status != "QUEUED" {
			t.Errorf("Expected status QUEUED, got %s", status)
		}
	})

	t.Run("Dequeue", func(t *testing.T) {
		task, err := queue.Dequeue(ctx)
		if err != nil {
			t.Fatalf("Dequeue failed: %v", err)
		}
		if task == nil {
			t.Fatal("Expected to dequeue a task, got nil")
		}

		if task.ID != taskID {
			t.Errorf("Expected task ID %s, got %s", taskID, task.ID)
		}
		if task.Status != "PROCESSING" {
			t.Errorf("Expected status PROCESSING, got %s", task.Status)
		}
		if task.WorkerID == nil || *task.WorkerID != "test-worker-1" {
			t.Errorf("Expected worker ID test-worker-1, got %v", task.WorkerID)
		}
		if string(task.Payload) != `{"key": "value"}` {
			t.Errorf("Expected payload, got %s", string(task.Payload))
		}

		// Try dequeueing again, should return nil since no QUEUED tasks
		emptyTask, err := queue.Dequeue(ctx)
		if err != nil {
			t.Fatalf("Second Dequeue failed: %v", err)
		}
		if emptyTask != nil {
			t.Fatal("Expected nil task since queue should be empty, got a task")
		}
	})

	t.Run("Acknowledge", func(t *testing.T) {
		err := queue.Acknowledge(ctx, taskID)
		if err != nil {
			t.Fatalf("Acknowledge failed: %v", err)
		}

		// Verify task is completed
		var status string
		err = db.QueryRow("SELECT status FROM sub_agent_queue WHERE id = ?", taskID).Scan(&status)
		if err != nil {
			t.Fatalf("Failed to verify task status: %v", err)
		}
		if status != "COMPLETED" {
			t.Errorf("Expected status COMPLETED, got %s", status)
		}
	})
}

func TestWorkerPool(t *testing.T) {
	db := setupQueueTestDB(t)
	defer db.Close()

	ctx := context.Background()
	queue := NewDBTaskQueue(db, "test-worker-pool", true)
	harness := &MockAgentHarness{}

	// Enqueue 5 test tasks
	for i := 0; i < 5; i++ {
		task := &Task{
			ID:             uuid.New().String(),
			OrganizationID: "org-2",
			ParentTaskID:   uuid.New().String(),
			Payload:        json.RawMessage(`{}`),
		}
		err := queue.Enqueue(ctx, task)
		if err != nil {
			t.Fatalf("Failed to enqueue task %d: %v", i, err)
		}
	}

	// Create and start worker pool
	pool := NewWorkerPool(queue, harness, 3)
	poolCtx, cancel := context.WithCancel(ctx)
	defer cancel() // ensure cleanup

	pool.Start(poolCtx)

	// Wait for processing
	deadline := time.Now().Add(2 * time.Second)
	for time.Now().Before(deadline) {
		if atomic.LoadInt32(&harness.ExecutionCount) == 5 {
			var count int
			db.QueryRow("SELECT COUNT(*) FROM sub_agent_queue WHERE status = 'COMPLETED'").Scan(&count)
			if count == 5 {
				break
			}
		}
		time.Sleep(50 * time.Millisecond)
	}

	// Stop pool
	pool.Stop()
	// Allow a little time for goroutines to fully exit
	time.Sleep(100 * time.Millisecond)

	if atomic.LoadInt32(&harness.ExecutionCount) != 5 {
		t.Errorf("Expected 5 task executions, got %d", harness.ExecutionCount)
	}

	// Verify all tasks in DB are acknowledged
	var count int
	err := db.QueryRow("SELECT COUNT(*) FROM sub_agent_queue WHERE status = 'COMPLETED'").Scan(&count)
	if err != nil {
		t.Fatalf("Failed to count completed tasks: %v", err)
	}
	if count != 5 {
		t.Errorf("Expected 5 completed tasks in DB, got %d", count)
	}
}
