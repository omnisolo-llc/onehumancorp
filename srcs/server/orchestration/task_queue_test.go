package orchestration

import (
	"context"
	"os"
	"testing"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
)

func setupTestQueueDB(t *testing.T) (*QueueManager, func()) {
	t.Helper()
	t.Setenv("DATABASE_URL", "sqlite://file::memory:?mode=memory")
	prov := db.NewTestProvider(t)

	ctx := context.Background()

	// Create tables
	_, err := prov.Exec(ctx, `
		CREATE TABLE IF NOT EXISTS shared_tasks (
			id TEXT PRIMARY KEY,
			mission_id TEXT NOT NULL,
			title TEXT NOT NULL,
			description TEXT,
			assigned_agent_id TEXT,
			status TEXT NOT NULL DEFAULT 'PENDING',
			priority TEXT NOT NULL DEFAULT 'P2',
			created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
			updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
		);

		CREATE TABLE IF NOT EXISTS task_dependencies (
			task_id TEXT NOT NULL,
			depends_on_task_id TEXT NOT NULL,
			PRIMARY KEY (task_id, depends_on_task_id),
			FOREIGN KEY (task_id) REFERENCES shared_tasks(id) ON DELETE CASCADE,
			FOREIGN KEY (depends_on_task_id) REFERENCES shared_tasks(id) ON DELETE CASCADE
		);
	`)
	if err != nil {
		t.Fatalf("failed to create table: %v", err)
	}

	qm := NewQueueManager(prov, nil, nil)

	return qm, func() {
		prov.Close()
	}
}

func TestQueueManager_EnqueueTask(t *testing.T) {
	qm, cleanup := setupTestQueueDB(t)
	defer cleanup()
	ctx := context.Background()

	taskA := &SharedTask{
		MissionID:   "m1",
		Title:       "Task A",
		Description: "Base Task",
		Priority:    "P1",
	}

	taskA, err := qm.EnqueueTask(ctx, taskA, nil)
	if err != nil {
		t.Fatalf("unexpected error enqueueing Task A: %v", err)
	}
	if taskA.Status != "READY" {
		t.Fatalf("Task A should be READY, got: %s", taskA.Status)
	}

	taskB := &SharedTask{
		MissionID:   "m1",
		Title:       "Task B",
		Description: "Dependent Task",
		Priority:    "P1",
	}

	taskB, err = qm.EnqueueTask(ctx, taskB, []string{taskA.ID})
	if err != nil {
		t.Fatalf("unexpected error enqueueing Task B: %v", err)
	}
	if taskB.Status != "PENDING" {
		t.Fatalf("Task B should be PENDING, got: %s", taskB.Status)
	}
}

func TestQueueManager_DependencyResolution(t *testing.T) {
	qm, cleanup := setupTestQueueDB(t)
	defer cleanup()
	ctx := context.Background()

	// Enqueue A
	taskA, _ := qm.EnqueueTask(ctx, &SharedTask{MissionID: "m1", Title: "Task A", Priority: "P1"}, nil)

	// Enqueue B depending on A
	taskB, _ := qm.EnqueueTask(ctx, &SharedTask{MissionID: "m1", Title: "Task B", Priority: "P1"}, []string{taskA.ID})

	// Acquire A
	acquiredTask, err := qm.AcquireReadyTask(ctx, "agent-1", nil)
	if err != nil {
		t.Fatalf("failed to acquire task: %v", err)
	}
	if acquiredTask.ID != taskA.ID {
		t.Fatalf("expected to acquire Task A, got %v", acquiredTask.Title)
	}

	// Try acquiring B, should get nil because B is PENDING
	acquiredB, err := qm.AcquireReadyTask(ctx, "agent-2", nil)
	if err != nil {
		t.Fatalf("error attempting to acquire: %v", err)
	}
	if acquiredB != nil {
		t.Fatalf("expected nil when acquiring before dependencies resolved, got: %v", acquiredB.Title)
	}

	// Complete A
	err = qm.CompleteTask(ctx, taskA.ID, "agent-1", "Done")
	if err != nil {
		t.Fatalf("failed to complete Task A: %v", err)
	}

	// Now B should be READY, acquire B
	acquiredB2, err := qm.AcquireReadyTask(ctx, "agent-2", nil)
	if err != nil {
		t.Fatalf("failed to acquire Task B: %v", err)
	}
	if acquiredB2 == nil {
		t.Fatalf("expected to acquire Task B after A completed, got nil")
	}
	if acquiredB2.ID != taskB.ID {
		t.Fatalf("expected Task B, got %v", acquiredB2.Title)
	}

	// Complete B
	err = qm.CompleteTask(ctx, acquiredB2.ID, "agent-2", "Done")
	if err != nil {
		t.Fatalf("failed to complete Task B: %v", err)
	}
}

func TestQueueManager_MultipleDependencies(t *testing.T) {
	qm, cleanup := setupTestQueueDB(t)
	defer cleanup()
	ctx := context.Background()

	taskA, _ := qm.EnqueueTask(ctx, &SharedTask{MissionID: "m1", Title: "Task A", Priority: "P1"}, nil)
	taskB, _ := qm.EnqueueTask(ctx, &SharedTask{MissionID: "m1", Title: "Task B", Priority: "P1"}, nil)

	taskC, _ := qm.EnqueueTask(ctx, &SharedTask{MissionID: "m1", Title: "Task C", Priority: "P1"}, []string{taskA.ID, taskB.ID})

	// Acquire and complete A
	acquiredA, _ := qm.AcquireReadyTask(ctx, "agent-1", nil)
	qm.CompleteTask(ctx, acquiredA.ID, "agent-1", "Done")

	// Task C should still be PENDING
	var statusC string
	err := qm.db.QueryRow(ctx, "SELECT status FROM shared_tasks WHERE id = $1", taskC.ID).Scan(&statusC)
	if err != nil {
		t.Fatalf("failed to get status for C: %v", err)
	}
	if statusC != "PENDING" {
		t.Fatalf("expected C to be PENDING, got %s", statusC)
	}

	// Acquire and complete B
	acquiredB, _ := qm.AcquireReadyTask(ctx, "agent-2", nil)
	qm.CompleteTask(ctx, acquiredB.ID, "agent-2", "Done")

	// Task C should now be READY
	err = qm.db.QueryRow(ctx, "SELECT status FROM shared_tasks WHERE id = $1", taskC.ID).Scan(&statusC)
	if err != nil {
		t.Fatalf("failed to get status for C: %v", err)
	}
	if statusC != "READY" {
		t.Fatalf("expected C to be READY, got %s", statusC)
	}

	acquiredC, err := qm.AcquireReadyTask(ctx, "agent-3", nil)
	if err != nil || acquiredC == nil || acquiredC.ID != taskC.ID {
		t.Fatalf("failed to acquire Task C: %v", err)
	}
}
