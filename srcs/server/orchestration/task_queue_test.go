package orchestration

import (
	"context"
	"testing"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
	"github.com/onehumancorp/mono/srcs/server/models"
)

func setupTestTaskOrchestrator(t *testing.T) (*TaskOrchestrator, db.Provider) {
	t.Setenv("DATABASE_URL", "sqlite://file::memory:?mode=memory")
	ctx := context.Background()

	provider, err := db.New(ctx)
	if err != nil {
		t.Fatalf("Failed to initialize db: %v", err)
	}

	// Apply schema migrations specific to tests
	queries := []string{
		`CREATE TABLE IF NOT EXISTS shared_tasks (
			id TEXT PRIMARY KEY,
			mission_id TEXT,
			title TEXT,
			description TEXT,
			assigned_agent_id TEXT,
			status TEXT,
			priority TEXT,
			payload TEXT,
			locked_until DATETIME,
			created_at DATETIME,
			updated_at DATETIME
		)`,
		`CREATE TABLE IF NOT EXISTS task_dependencies (
			task_id TEXT,
			depends_on_task_id TEXT,
			PRIMARY KEY(task_id, depends_on_task_id)
		)`,
		`CREATE TABLE IF NOT EXISTS autodream_memories (
			id TEXT PRIMARY KEY,
			content TEXT,
			embedding BLOB,
			source_mission_id TEXT,
			consolidated_at DATETIME
		)`,
	}

	for _, q := range queries {
		if _, err := provider.Exec(ctx, q); err != nil {
			t.Fatalf("Failed to execute query %q: %v", q, err)
		}
	}

	return NewTaskOrchestrator(provider, nil), provider
}

func TestTaskOrchestrator_EnqueueTask(t *testing.T) {
	to, _ := setupTestTaskOrchestrator(t)

	task := models.Task{
		ID:          "task-1",
		MissionID:   "m-1",
		Title:       "Test Task",
		Description: "A simple task",
		Status:      "READY",
		Priority:    "P1",
	}

	err := to.EnqueueTask(context.Background(), task)
	if err != nil {
		t.Fatalf("Failed to enqueue task: %v", err)
	}

	// Try acquiring it
	acquired, err := to.AcquireReadyTask(context.Background(), "agent-1", nil)
	if err != nil {
		t.Fatalf("Failed to acquire task: %v", err)
	}

	if acquired.ID != "task-1" {
		t.Errorf("Expected task ID task-1, got %s", acquired.ID)
	}
	if acquired.Status != "IN_PROGRESS" {
		t.Errorf("Expected status IN_PROGRESS, got %s", acquired.Status)
	}
	if acquired.AssignedAgentID != "agent-1" {
		t.Errorf("Expected assigned agent agent-1, got %s", acquired.AssignedAgentID)
	}
}

func TestTaskOrchestrator_Dependencies(t *testing.T) {
	to, provider := setupTestTaskOrchestrator(t)

	taskA := models.Task{
		ID:        "task-A",
		MissionID: "m-1",
		Title:     "Task A",
		Status:    "READY",
	}

	taskB := models.Task{
		ID:        "task-B",
		MissionID: "m-1",
		Title:     "Task B",
	}

	// Enqueue Task A (no deps)
	err := to.EnqueueTask(context.Background(), taskA)
	if err != nil {
		t.Fatalf("Failed to enqueue task A: %v", err)
	}

	// Enqueue Task B (depends on Task A)
	err = to.EnqueueTaskWithDependencies(context.Background(), taskB, []string{"task-A"})
	if err != nil {
		t.Fatalf("Failed to enqueue task B: %v", err)
	}

	// At this point, Task A is READY, Task B is PENDING
	var statusB string
	err = provider.QueryRow(context.Background(), "SELECT status FROM shared_tasks WHERE id = 'task-B'").Scan(&statusB)
	if err != nil {
		t.Fatalf("Failed to get status B: %v", err)
	}
	if statusB != "PENDING" {
		t.Errorf("Expected Task B to be PENDING, got %s", statusB)
	}

	// Acquire and complete Task A
	acquired, err := to.AcquireReadyTask(context.Background(), "agent-1", nil)
	if err != nil {
		t.Fatalf("Failed to acquire task A: %v", err)
	}
	if acquired.ID != "task-A" {
		t.Fatalf("Expected acquired task to be A")
	}

	err = to.CompleteTask(context.Background(), "task-A", "done")
	if err != nil {
		t.Fatalf("Failed to complete task A: %v", err)
	}

	// Allow background AutoDream hook to run
	time.Sleep(100 * time.Millisecond)

	// Now Task B should be READY
	err = provider.QueryRow(context.Background(), "SELECT status FROM shared_tasks WHERE id = 'task-B'").Scan(&statusB)
	if err != nil {
		t.Fatalf("Failed to get status B after A completed: %v", err)
	}
	if statusB != "READY" {
		t.Errorf("Expected Task B to be READY, got %s", statusB)
	}

	// Acquire Task B
	acquiredB, err := to.AcquireReadyTask(context.Background(), "agent-2", nil)
	if err != nil {
		t.Fatalf("Failed to acquire task B: %v", err)
	}
	if acquiredB.ID != "task-B" {
		t.Fatalf("Expected acquired task to be B")
	}
}

func TestTaskOrchestrator_FailAndBlock(t *testing.T) {
	to, provider := setupTestTaskOrchestrator(t)
	task := models.Task{ID: "task-f", MissionID: "m", Title: "F"}
	to.EnqueueTask(context.Background(), task)

	to.FailTask(context.Background(), "task-f", "error")
	var status string
	provider.QueryRow(context.Background(), "SELECT status FROM shared_tasks WHERE id = 'task-f'").Scan(&status)
	if status != "FAILED" {
		t.Errorf("Expected FAILED, got %s", status)
	}

	task2 := models.Task{ID: "task-b", MissionID: "m", Title: "B"}
	to.EnqueueTask(context.Background(), task2)

	to.BlockTask(context.Background(), "task-b", "blocked")
	provider.QueryRow(context.Background(), "SELECT status FROM shared_tasks WHERE id = 'task-b'").Scan(&status)
	if status != "BLOCKED" {
		t.Errorf("Expected BLOCKED, got %s", status)
	}
}
