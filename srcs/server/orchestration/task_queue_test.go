package orchestration

import (
	"context"
	"os"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/db"
	"github.com/onehumancorp/mono/srcs/server/models"
)

func setupQueueTestDB(t *testing.T) (*TaskOrchestrator, func()) {
	t.Helper()
	prov := db.NewTestProvider(t)

	// Create tables
	_, err := prov.Exec(context.Background(), `
		CREATE TABLE IF NOT EXISTS swarm_tasks (
			id TEXT PRIMARY KEY,
			mission_id TEXT NOT NULL,
			title TEXT NOT NULL,
			status TEXT NOT NULL DEFAULT 'PENDING',
			assigned_agent_id TEXT,
			locked_until DATETIME,
			payload TEXT NOT NULL DEFAULT '{}',
			created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
			updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
		);
		CREATE TABLE IF NOT EXISTS task_dependencies (
			task_id TEXT NOT NULL,
			depends_on_task_id TEXT NOT NULL,
			PRIMARY KEY (task_id, depends_on_task_id),
			FOREIGN KEY (task_id) REFERENCES swarm_tasks(id) ON DELETE CASCADE,
			FOREIGN KEY (depends_on_task_id) REFERENCES swarm_tasks(id) ON DELETE CASCADE
		);
	`)
	if err != nil {
		t.Fatalf("failed to create tables: %v", err)
	}

	to := NewTaskOrchestrator(prov, nil)

	return to, func() {
		prov.Close()
	}
}

func TestTaskOrchestrator_EnqueueTask(t *testing.T) {
	os.Setenv("OHC_STANDALONE", "true")
	defer os.Unsetenv("OHC_STANDALONE")

	to, cleanup := setupQueueTestDB(t)
	defer cleanup()

	ctx := context.Background()

	// 1. Task without dependencies should be READY
	taskA := &models.Task{
		ID:        "task-a",
		MissionID: "mission-1",
		Title:     "Task A",
	}
	err := to.EnqueueTask(ctx, taskA, nil)
	if err != nil {
		t.Fatalf("failed to enqueue Task A: %v", err)
	}

	var statusA string
	_ = to.db.QueryRow(ctx, "SELECT status FROM swarm_tasks WHERE id = 'task-a'").Scan(&statusA)
	if statusA != "READY" {
		t.Errorf("expected Task A to be READY, got %s", statusA)
	}

	// 2. Task with unresolved dependencies should be PENDING
	taskB := &models.Task{
		ID:        "task-b",
		MissionID: "mission-1",
		Title:     "Task B",
	}
	err = to.EnqueueTask(ctx, taskB, []string{"task-a"})
	if err != nil {
		t.Fatalf("failed to enqueue Task B: %v", err)
	}

	var statusB string
	_ = to.db.QueryRow(ctx, "SELECT status FROM swarm_tasks WHERE id = 'task-b'").Scan(&statusB)
	if statusB != "PENDING" {
		t.Errorf("expected Task B to be PENDING, got %s", statusB)
	}
}

func TestTaskOrchestrator_DependencyResolution(t *testing.T) {
	os.Setenv("OHC_STANDALONE", "true")
	defer os.Unsetenv("OHC_STANDALONE")

	to, cleanup := setupQueueTestDB(t)
	defer cleanup()

	ctx := context.Background()

	taskA := &models.Task{ID: "task-a", MissionID: "mission-1", Title: "Task A"}
	taskB := &models.Task{ID: "task-b", MissionID: "mission-1", Title: "Task B"}

	_ = to.EnqueueTask(ctx, taskA, nil)
	_ = to.EnqueueTask(ctx, taskB, []string{"task-a"})

	// Acquire Task A
	claimedTaskA, err := to.AcquireReadyTask(ctx, "agent-1", nil)
	if err != nil {
		t.Fatalf("failed to acquire Task A: %v", err)
	}
	if claimedTaskA == nil || claimedTaskA.ID != "task-a" {
		t.Fatalf("expected to acquire Task A, got %v", claimedTaskA)
	}

	// Task B should not be claimable yet
	claimedTaskB, _ := to.AcquireReadyTask(ctx, "agent-2", nil)
	if claimedTaskB != nil {
		t.Fatalf("expected Task B to not be ready")
	}

	// Complete Task A
	err = to.CompleteTask(ctx, "task-a", "agent-1", "result-a")
	if err != nil {
		t.Fatalf("failed to complete Task A: %v", err)
	}

	// Task B should now be READY and claimable
	claimedTaskB, err = to.AcquireReadyTask(ctx, "agent-2", nil)
	if err != nil {
		t.Fatalf("failed to acquire Task B: %v", err)
	}
	if claimedTaskB == nil || claimedTaskB.ID != "task-b" {
		t.Fatalf("expected to acquire Task B, got %v", claimedTaskB)
	}
}
