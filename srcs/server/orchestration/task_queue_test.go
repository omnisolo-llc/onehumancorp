package orchestration

import (
	"context"
	"testing"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
)

func setupTestDB(t *testing.T) db.Provider {
	t.Helper()
	t.Setenv("DATABASE_URL", "sqlite://file::memory:?mode=memory")
	dbWrapper, err := db.New(context.Background())
	if err != nil {
		t.Fatalf("failed to create db: %v", err)
	}

	ctx := context.Background()

	// Setup schemas
	_, err = dbWrapper.Exec(ctx, `
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
			PRIMARY KEY (task_id, depends_on_task_id)
		);
		CREATE TABLE IF NOT EXISTS autodream_memories (
			id TEXT PRIMARY KEY,
			content TEXT NOT NULL,
			source_mission_id TEXT,
			consolidated_at DATETIME DEFAULT CURRENT_TIMESTAMP
		);
	`)
	if err != nil {
		t.Fatalf("failed to create tables: %v", err)
	}

	return dbWrapper.Provider
}

func TestTaskOrchestrator_DependencyResolution(t *testing.T) {
	provider := setupTestDB(t)
	orchestrator := NewTaskManager(provider)
	ctx := context.Background()

	// Task A (No deps)
	taskA := &SharedTask{
		ID:        "task-A",
		MissionID: "mission-1",
		Title:     "Task A",
		Priority:  "P1",
	}
	err := orchestrator.EnqueueTask(ctx, taskA, nil)
	if err != nil {
		t.Fatalf("EnqueueTask A failed: %v", err)
	}

	// Task B (Depends on Task A)
	taskB := &SharedTask{
		ID:        "task-B",
		MissionID: "mission-1",
		Title:     "Task B",
		Priority:  "P1",
	}
	err = orchestrator.EnqueueTask(ctx, taskB, []string{"task-A"})
	if err != nil {
		t.Fatalf("EnqueueTask B failed: %v", err)
	}

	// Task C (Depends on Task B)
	taskC := &SharedTask{
		ID:        "task-C",
		MissionID: "mission-1",
		Title:     "Task C",
		Priority:  "P1",
	}
	err = orchestrator.EnqueueTask(ctx, taskC, []string{"task-B"})
	if err != nil {
		t.Fatalf("EnqueueTask C failed: %v", err)
	}

	// 1. Acquire Task A
	acquiredTaskA, err := orchestrator.AcquireReadyTask(ctx, "agent-1", nil)
	if err != nil {
		t.Fatalf("AcquireReadyTask A failed: %v", err)
	}
	if acquiredTaskA == nil || acquiredTaskA.ID != "task-A" {
		t.Fatalf("Expected to acquire Task A, got %v", acquiredTaskA)
	}

	// 2. Try to acquire Task B (should be nil because it's blocked by A)
	acquiredTaskB, err := orchestrator.AcquireReadyTask(ctx, "agent-2", nil)
	if err != nil {
		t.Fatalf("AcquireReadyTask B failed: %v", err)
	}
	if acquiredTaskB != nil {
		t.Fatalf("Expected nil (Task B is blocked), got %v", acquiredTaskB)
	}

	// 3. Complete Task A
	err = orchestrator.CompleteReadyTask(ctx, "task-A", "Result A")
	if err != nil {
		t.Fatalf("CompleteReadyTask A failed: %v", err)
	}

	// 4. Now Task B should be READY
	acquiredTaskB, err = orchestrator.AcquireReadyTask(ctx, "agent-2", nil)
	if err != nil {
		t.Fatalf("AcquireReadyTask B failed: %v", err)
	}
	if acquiredTaskB == nil || acquiredTaskB.ID != "task-B" {
		t.Fatalf("Expected to acquire Task B, got %v", acquiredTaskB)
	}

	// 5. Task C should still be blocked
	acquiredTaskC, err := orchestrator.AcquireReadyTask(ctx, "agent-3", nil)
	if err != nil {
		t.Fatalf("AcquireReadyTask C failed: %v", err)
	}
	if acquiredTaskC != nil {
		t.Fatalf("Expected nil (Task C is blocked), got %v", acquiredTaskC)
	}

	// 6. Complete Task B
	err = orchestrator.CompleteReadyTask(ctx, "task-B", "Result B")
	if err != nil {
		t.Fatalf("CompleteReadyTask B failed: %v", err)
	}

	// 7. Now Task C should be READY
	acquiredTaskC, err = orchestrator.AcquireReadyTask(ctx, "agent-3", nil)
	if err != nil {
		t.Fatalf("AcquireReadyTask C failed: %v", err)
	}
	if acquiredTaskC == nil || acquiredTaskC.ID != "task-C" {
		t.Fatalf("Expected to acquire Task C, got %v", acquiredTaskC)
	}

	// 8. Complete Task C
	err = orchestrator.CompleteReadyTask(ctx, "task-C", "Result C")
	if err != nil {
		t.Fatalf("CompleteReadyTask C failed: %v", err)
	}

	// Small sleep to let AutoDream hook run if we had set it
	time.Sleep(50 * time.Millisecond)
}
