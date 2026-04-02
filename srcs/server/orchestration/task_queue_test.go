package orchestration

import (
	"context"
	"testing"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
)

func TestTaskOrchestratorDependencies(t *testing.T) {
	t.Setenv("DATABASE_URL", "sqlite://file::memory:?mode=memory")
	ctx := context.Background()

	// Initialize DB
	provider, err := db.New(ctx)
	if err != nil {
		t.Fatalf("failed to init db: %v", err)
	}

	// Setup schema manually for in-memory sqlite without migrations
	_, err = provider.Exec(ctx, `
		CREATE TABLE shared_tasks (
			id TEXT PRIMARY KEY,
			mission_id TEXT,
			title TEXT,
			description TEXT,
			assigned_agent_id TEXT,
			status TEXT,
			priority TEXT,
			created_at DATETIME,
			updated_at DATETIME
		);
		CREATE TABLE task_dependencies (
			task_id TEXT NOT NULL,
			depends_on_task_id TEXT NOT NULL,
			PRIMARY KEY (task_id, depends_on_task_id)
		);
		CREATE TABLE autodream_memories (
			id TEXT PRIMARY KEY,
			content TEXT,
			embedding TEXT,
			source_mission_id TEXT,
			consolidated_at DATETIME
		);
	`)
	if err != nil {
		t.Fatalf("failed to setup schema: %v", err)
	}

	mockLLM := &mockMinimax{}
	orchestrator := NewTaskOrchestrator(provider.Provider, nil, nil, mockLLM)

	// Create Task A (No dependencies)
	taskA := &SharedTask{
		ID:          "task-A",
		MissionID:   "mission-1",
		Title:       "Task A",
		Description: "Base Task",
		Priority:    "P1",
	}
	err = orchestrator.EnqueueTask(ctx, taskA, nil)
	if err != nil {
		t.Fatalf("enqueue task A failed: %v", err)
	}

	// Create Task B (Depends on Task A)
	taskB := &SharedTask{
		ID:          "task-B",
		MissionID:   "mission-1",
		Title:       "Task B",
		Description: "Dependent Task",
		Priority:    "P1",
	}
	err = orchestrator.EnqueueTask(ctx, taskB, []string{"task-A"})
	if err != nil {
		t.Fatalf("enqueue task B failed: %v", err)
	}

	// Verify Task A is READY, Task B is PENDING
	var statusA, statusB string
	provider.QueryRow(ctx, "SELECT status FROM shared_tasks WHERE id = 'task-A'").Scan(&statusA)
	provider.QueryRow(ctx, "SELECT status FROM shared_tasks WHERE id = 'task-B'").Scan(&statusB)
	if statusA != "READY" {
		t.Errorf("expected Task A to be READY, got %s", statusA)
	}
	if statusB != "PENDING" {
		t.Errorf("expected Task B to be PENDING, got %s", statusB)
	}

	// Agent 1 claims Task A
	acquiredTask, err := orchestrator.AcquireReadyTask(ctx, "agent-1", nil)
	if err != nil {
		t.Fatalf("acquire task failed: %v", err)
	}
	if acquiredTask == nil || acquiredTask.ID != "task-A" {
		t.Fatalf("expected to acquire task-A, got %v", acquiredTask)
	}

	// Agent 1 completes Task A
	err = orchestrator.CompleteTask(ctx, "task-A", "agent-1", "Done")
	if err != nil {
		t.Fatalf("complete task A failed: %v", err)
	}

	// Task B should now be READY
	provider.QueryRow(ctx, "SELECT status FROM shared_tasks WHERE id = 'task-B'").Scan(&statusB)
	if statusB != "READY" {
		t.Errorf("expected Task B to be READY after A is completed, got %s", statusB)
	}

	// Agent 2 claims Task B
	acquiredTask2, err := orchestrator.AcquireReadyTask(ctx, "agent-2", nil)
	if err != nil {
		t.Fatalf("acquire task B failed: %v", err)
	}
	if acquiredTask2 == nil || acquiredTask2.ID != "task-B" {
		t.Fatalf("expected to acquire task-B, got %v", acquiredTask2)
	}

	// Agent 2 completes Task B
	err = orchestrator.CompleteTask(ctx, "task-B", "agent-2", "Done")
	if err != nil {
		t.Fatalf("complete task B failed: %v", err)
	}

	// Allow some time for background AutoDream hook to execute
	time.Sleep(50 * time.Millisecond)

	// Check if AutoDream memory was inserted
	var memCount int
	provider.QueryRow(ctx, "SELECT COUNT(*) FROM autodream_memories").Scan(&memCount)
	if memCount != 2 {
		t.Errorf("expected 2 AutoDream memories, got %d", memCount)
	}
}
