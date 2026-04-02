package orchestration

import (
	"context"
	"os"
	"testing"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
)

// MockMinimaxClient is a simple mock for testing
type MockMinimaxClient struct{}

func (m *MockMinimaxClient) Reason(ctx context.Context, prompt string) (string, error) {
	return "mock reason response", nil
}

func (m *MockMinimaxClient) GenerateEmbedding(ctx context.Context, text string) ([]float32, error) {
	return []float32{0.1, 0.2, 0.3}, nil
}

func TestTaskOrchestratorDependencyResolution(t *testing.T) {
	t.Setenv("DATABASE_URL", "sqlite://file::memory:?mode=memory")
	ctx := context.Background()

	dbProvider, err := db.New(ctx)
	if err != nil {
		t.Fatalf("failed to init db: %v", err)
	}

	// Create required schema manually since we don't run full migrations in this unit test
	_, err = dbProvider.Exec(ctx, `
		CREATE TABLE IF NOT EXISTS swarm_tasks (
			id TEXT PRIMARY KEY,
			mission_id TEXT,
			parent_plan_id TEXT,
			title TEXT,
			description TEXT,
			priority TEXT,
			status TEXT,
			assigned_agent_id TEXT,
			locked_until DATETIME,
			payload TEXT,
			dependencies TEXT,
			created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
			updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
		);
		CREATE TABLE IF NOT EXISTS task_dependencies (
			task_id TEXT,
			depends_on_task_id TEXT,
			PRIMARY KEY (task_id, depends_on_task_id)
		);
		CREATE TABLE IF NOT EXISTS swarm_long_term_memory (
			id TEXT PRIMARY KEY DEFAULT (lower(hex(randomblob(16)))),
			topic TEXT,
			summary TEXT,
			embedding TEXT,
			created_at DATETIME DEFAULT CURRENT_TIMESTAMP
		);
	`)
	if err != nil {
		t.Fatalf("failed to create schema: %v", err)
	}

	mockMinimax := &MockMinimaxClient{}
	orchestrator := NewTaskOrchestrator(dbProvider, nil, nil, nil, mockMinimax)

	// Create Task A
	taskA := &SharedTask{
		ID:        "task-a",
		MissionID: "mission-1",
		Title:     "Task A",
		Status:    "PENDING",
	}
	err = orchestrator.EnqueueTask(ctx, taskA)
	if err != nil {
		t.Fatalf("failed to enqueue task A: %v", err)
	}

	// Create Task B dependent on Task A
	taskB := &SharedTask{
		ID:           "task-b",
		MissionID:    "mission-1",
		Title:        "Task B",
		Status:       "PENDING",
		Dependencies: []string{"task-a"},
	}
	err = orchestrator.EnqueueTask(ctx, taskB)
	if err != nil {
		t.Fatalf("failed to enqueue task B: %v", err)
	}

	// 1. Try to acquire a ready task. We should only get task A since task B is blocked.
	claimedA, err := orchestrator.AcquireReadyTask(ctx, "agent-1", nil)
	if err != nil {
		t.Fatalf("failed to acquire task: %v", err)
	}
	if claimedA == nil {
		t.Fatalf("expected to claim task A, got nil")
	}
	if claimedA.ID != "task-a" {
		t.Fatalf("expected task-a, got %s", claimedA.ID)
	}

	// 2. Try to acquire again. Should get nil because A is IN_PROGRESS and B is blocked by A.
	claimedNone, err := orchestrator.AcquireReadyTask(ctx, "agent-2", nil)
	if err != nil {
		t.Fatalf("failed to acquire task: %v", err)
	}
	if claimedNone != nil {
		t.Fatalf("expected no tasks to be ready, but got %s", claimedNone.ID)
	}

	// 3. Complete Task A
	err = orchestrator.CompleteTask(ctx, "task-a", "success")
	if err != nil {
		t.Fatalf("failed to complete task A: %v", err)
	}

	// Let the AutoDream background goroutine execute
	time.Sleep(100 * time.Millisecond)

	// 4. Try to acquire again. Now Task B should be ready.
	claimedB, err := orchestrator.AcquireReadyTask(ctx, "agent-3", nil)
	if err != nil {
		t.Fatalf("failed to acquire task: %v", err)
	}
	if claimedB == nil {
		t.Fatalf("expected to claim task B, got nil")
	}
	if claimedB.ID != "task-b" {
		t.Fatalf("expected task-b, got %s", claimedB.ID)
	}
}
