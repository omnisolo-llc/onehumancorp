package orchestration

import (
	"context"
	"os"
	"testing"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
	"github.com/onehumancorp/mono/srcs/server/orchestration/queue"
)

func TestTaskManager_InitializationFallback(t *testing.T) {
	provider := db.NewTestProvider(t)

	// Test bad URL
	t.Setenv("OHC_MULTITENANT", "true")
	t.Setenv("REDIS_URL", "redis://bad-url:0")

	// Should fallback cleanly and not panic
	tm := NewTaskManager(provider, nil, nil)
	if tm == nil {
		t.Fatalf("expected tm")
	}

	// Wait a bit and stop to hit stopWorkerLoop and StartWorkerLoop branches
	time.Sleep(10 * time.Millisecond)
	tm.StopWorkerLoop()
	// Duplicate stop to test idempotency if applicable
	tm.StopWorkerLoop()
}

func TestTaskManager_SettersAndGetters(t *testing.T) {
	provider := db.NewTestProvider(t)
	tm := NewTaskManager(provider, nil, nil)

	mockHub := &CentrifugeNode{}
	tm.SetHub(mockHub)
	if tm.hub != mockHub {
		t.Fatalf("expected mockHub")
	}

	mockMesh := &MemoryMeshTransport{}
	tm.SetMeshTransport(mockMesh)
}

func TestTaskManager_TaskDelegationAndLifecycle(t *testing.T) {
	provider := db.NewTestProvider(t)
	tm := NewTaskManager(provider, nil, nil)

	ctx := context.Background()

	// Should fail with generic payload mapping error if no queue
	err := tm.DelegateSubTask(ctx, "task-1", "agent-role", map[string]interface{}{"key": "val"})
	if err == nil {
		t.Fatalf("expected error without initialized queue")
	}

	// Let's set up a mock queue to pass delegation
	// This hits queue paths in tasks.go DelegateSubTask
	tm.taskQueue = queue.NewMemoryJobQueue()
	err = tm.DelegateSubTask(ctx, "task-1", "agent-role", map[string]interface{}{"key": "val"})
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}

	// Hit task Review logic
	// Requires an actual task.
	st, _ := tm.CreateTask(ctx, "org-1", "Task to review", "desc", "P1")
	tm.db.Exec(ctx, "UPDATE shared_tasks SET status = 'IN_PROGRESS' WHERE id = $1", st.ID)

	err = tm.ReviewTask(ctx, st.ID, "reviewer-1")
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}

	// Verify Review mutation
	var status string
	tm.db.QueryRow(ctx, "SELECT status FROM shared_tasks WHERE id = $1", st.ID).Scan(&status)
	if status != "VERIFYING" {
		t.Fatalf("expected VERIFYING, got %v", status)
	}
}

func TestTaskManager_EdgeCases(t *testing.T) {
	provider := db.NewTestProvider(t)
	tm := NewTaskManager(provider, nil, nil)

	ctx := context.Background()

	// Hit missing circular dependency paths
	st1, _ := tm.CreateTask(ctx, "org-1", "T1", "", "")
	st2, _ := tm.CreateTask(ctx, "org-1", "T2", "", "")

	// Insert mutual dependency to trigger cycle error
	tm.db.Exec(ctx, "INSERT INTO task_dependencies (task_id, depends_on_task_id) VALUES ($1, $2)", st1.ID, st2.ID)
	tm.db.Exec(ctx, "INSERT INTO task_dependencies (task_id, depends_on_task_id) VALUES ($1, $2)", st2.ID, st1.ID)

	err := tm.CheckCircularDependency(ctx, st1.ID, []string{st2.ID})
	if err == nil {
		t.Fatalf("expected circular dependency error")
	}

	id := generateID()
	if id == "" {
		t.Fatalf("generateID returned empty")
	}
}
