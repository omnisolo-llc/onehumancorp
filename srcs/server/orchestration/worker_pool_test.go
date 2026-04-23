package orchestration

import (
	"context"
	"os"
	"sync/atomic"
	"testing"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
)

func TestWorkerPool_StartStop(t *testing.T) {
	os.Setenv("OHC_MULTITENANT", "false")
	defer os.Unsetenv("OHC_MULTITENANT")

	prov := db.NewTestProvider(t)
	defer prov.Close()

	_, err := prov.Exec(context.Background(), `
		CREATE TABLE IF NOT EXISTS sub_agent_queue (
			id TEXT PRIMARY KEY,
			organization_id TEXT NOT NULL,
			parent_task_id TEXT NOT NULL,
			payload TEXT,
			status TEXT NOT NULL DEFAULT 'QUEUED',
			worker_id TEXT,
			created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
			updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
		);
	`)
	if err != nil {
		t.Fatalf("failed to create schema: %v", err)
	}

	q := NewTaskQueue(prov)

	handler := func(ctx context.Context, task *Task) error {
		return nil
	}

	wp := NewWorkerPool(q, 3, handler)

	ctx, cancel := context.WithCancel(context.Background())
	defer cancel()

	wp.Start(ctx)

	// Let it run for a bit
	time.Sleep(50 * time.Millisecond)

	wp.Stop()
	// If it doesn't hang, StartStop is working properly.
}

func TestWorkerPool_TaskProcessing(t *testing.T) {
	os.Setenv("OHC_MULTITENANT", "false")
	defer os.Unsetenv("OHC_MULTITENANT")

	prov := db.NewTestProvider(t)
	defer prov.Close()

	_, err := prov.Exec(context.Background(), `
		CREATE TABLE IF NOT EXISTS sub_agent_queue (
			id TEXT PRIMARY KEY,
			organization_id TEXT NOT NULL,
			parent_task_id TEXT NOT NULL,
			payload TEXT,
			status TEXT NOT NULL DEFAULT 'QUEUED',
			worker_id TEXT,
			created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
			updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
		);
	`)
	if err != nil {
		t.Fatalf("failed to create schema: %v", err)
	}

	q := NewTaskQueue(prov)
	ctx := context.Background()

	// Enqueue a few tasks
	q.Enqueue(ctx, &Task{OrganizationID: "o", ParentTaskID: "p", Payload: map[string]interface{}{"id": 1}})
	q.Enqueue(ctx, &Task{OrganizationID: "o", ParentTaskID: "p", Payload: map[string]interface{}{"id": 2}})
	q.Enqueue(ctx, &Task{OrganizationID: "o", ParentTaskID: "p", Payload: map[string]interface{}{"id": 3}})

	var processedCount int32
	handler := func(ctx context.Context, task *Task) error {
		atomic.AddInt32(&processedCount, 1)
		return nil
	}

	wp := NewWorkerPool(q, 2, handler)

	wpCtx, cancel := context.WithCancel(context.Background())
	defer cancel()

	wp.Start(wpCtx)

	// Wait for tasks to be processed
	time.Sleep(250 * time.Millisecond)

	wp.Stop()

	if atomic.LoadInt32(&processedCount) != 3 {
		t.Fatalf("expected 3 tasks processed, got %d", processedCount)
	}

	// Verify they are acknowledged
	var pendingCount int
	prov.QueryRow(ctx, "SELECT COUNT(*) FROM sub_agent_queue WHERE status = 'QUEUED'").Scan(&pendingCount)
	if pendingCount != 0 {
		t.Fatalf("expected 0 pending tasks, got %d", pendingCount)
	}

	var completedCount int
	prov.QueryRow(ctx, "SELECT COUNT(*) FROM sub_agent_queue WHERE status = 'COMPLETED'").Scan(&completedCount)
	if completedCount != 3 {
		t.Fatalf("expected 3 completed tasks, got %d", completedCount)
	}
}

// Postgres DB test
func TestPostgresTaskQueue(t *testing.T) {
	// For 100% test coverage we test IsSQLite() mock functionality
	mockProvider := &mockPostgresProvider{Provider: db.NewTestProvider(t)}

	q := NewTaskQueue(mockProvider)

	_, ok := q.(*PostgresTaskQueue)
	if !ok {
		t.Fatalf("expected PostgresTaskQueue")
	}
}

type mockPostgresProvider struct {
	db.Provider
}

func (m *mockPostgresProvider) IsSQLite() bool {
	return false
}
