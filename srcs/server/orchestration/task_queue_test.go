package orchestration_test

import (
	"context"
	"testing"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
	"github.com/onehumancorp/mono/srcs/server/models"
	"github.com/onehumancorp/mono/srcs/server/orchestration"
)

type mockMinimaxClient struct {
	called bool
}

func (m *mockMinimaxClient) GenerateEmbedding(ctx context.Context, text string) ([]float32, error) {
	m.called = true
	return []float32{0.1, 0.2, 0.3}, nil
}
func (m *mockMinimaxClient) Reason(ctx context.Context, prompt string) (string, error) {
	return "mock reason", nil
}

func TestTaskOrchestrator_DAG_Standalone(t *testing.T) {
	// Setup in-memory DB as required by skeptical memory
	t.Setenv("DATABASE_URL", "sqlite://file::memory:?mode=memory")
	ctx := context.Background()
	provDB, err := db.New(context.Background())
	if err != nil {
		t.Fatalf("failed to create db: %v", err)
	}
	prov := provDB.Provider

	_, err = prov.Exec(context.Background(), `
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
			embedding TEXT,
			source_mission_id TEXT,
			consolidated_at DATETIME DEFAULT CURRENT_TIMESTAMP
		);
	`)
	if err != nil {
		t.Fatalf("failed to run migrations: %v", err)
	}

	mockMinimax := &mockMinimaxClient{}
	// Nil redisClient and hub for standalone
	orchestrator := orchestration.NewTaskOrchestrator(prov, nil, nil, mockMinimax)

	// Create Task A
	taskA := models.Task{
		Title:       "Task A",
		Description: "Base task",
		Priority:    "P1",
	}
	enqTaskA, err := orchestrator.EnqueueTask(ctx, taskA, nil)
	if err != nil {
		t.Fatalf("failed to enqueue Task A: %v", err)
	}

	// Create Task B dependent on Task A
	taskB := models.Task{
		Title:       "Task B",
		Description: "Dependent task",
		Priority:    "P1",
	}
	enqTaskB, err := orchestrator.EnqueueTask(ctx, taskB, []string{enqTaskA.ID})
	if err != nil {
		t.Fatalf("failed to enqueue Task B: %v", err)
	}

	// Try to acquire Task B - should fail or get Task A instead because Task B is blocked
	agentID := "agent-123"
	readyTask, err := orchestrator.AcquireReadyTask(ctx, agentID, nil)
	if err != nil {
		t.Fatalf("failed to acquire task: %v", err)
	}
	if readyTask == nil {
		t.Fatalf("expected to acquire Task A, got nil")
	}
	if readyTask.ID != enqTaskA.ID {
		t.Fatalf("expected Task A (ID %s), got ID %s", enqTaskA.ID, readyTask.ID)
	}

	// Try to acquire again - should return nil because Task B is not ready and Task A is IN_PROGRESS
	readyTask2, err := orchestrator.AcquireReadyTask(ctx, agentID, nil)
	if err != nil {
		t.Fatalf("failed to acquire task 2: %v", err)
	}
	if readyTask2 != nil {
		t.Fatalf("expected no ready tasks, but got ID %s", readyTask2.ID)
	}

	// Complete Task A
	err = orchestrator.CompleteTask(ctx, enqTaskA.ID, agentID, "Done with A")
	if err != nil {
		t.Fatalf("failed to complete Task A: %v", err)
	}

	// Wait a moment for async hook
	time.Sleep(100 * time.Millisecond)

	if !mockMinimax.called {
		t.Errorf("expected AutoDream hook to call GenerateEmbedding")
	}

	// Now Task B should be READY
	readyTask3, err := orchestrator.AcquireReadyTask(ctx, agentID, nil)
	if err != nil {
		t.Fatalf("failed to acquire task 3: %v", err)
	}
	if readyTask3 == nil {
		t.Fatalf("expected to acquire Task B, got nil")
	}
	if readyTask3.ID != enqTaskB.ID {
		t.Fatalf("expected Task B (ID %s), got ID %s", enqTaskB.ID, readyTask3.ID)
	}

	// Complete Task B
	err = orchestrator.CompleteTask(ctx, enqTaskB.ID, agentID, "Done with B")
	if err != nil {
		t.Fatalf("failed to complete Task B: %v", err)
	}
}
