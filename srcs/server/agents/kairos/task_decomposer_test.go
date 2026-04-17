package kairos

import (
	"context"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/db"
)

func createTestTable(t *testing.T, provider db.Provider) {
	t.Helper()
	_, err := provider.Exec(context.Background(), `
CREATE TABLE IF NOT EXISTS shared_tasks_decomposition (
    id TEXT PRIMARY KEY,
    organization_id TEXT NOT NULL,
    title TEXT NOT NULL,
    description TEXT,
    status TEXT NOT NULL DEFAULT 'PENDING',
    assigned_agent_id TEXT,
    priority TEXT NOT NULL DEFAULT 'P2',
    payload TEXT,
    parent_plan_id TEXT,
    dependencies TEXT NOT NULL DEFAULT '[]',
    locked_until DATETIME,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
);
	`)
	if err != nil {
		t.Fatalf("failed to create table: %v", err)
	}
}

func TestTaskDecomposerCircularDependency(t *testing.T) {
	provider := db.NewTestProvider(t)

	td := NewTaskDecomposer(provider)
	ctx := context.Background()

	tasks := []*Task{
		{ID: "task1", OrganizationID: "org1", Title: "Task 1", Dependencies: []string{"task2"}},
		{ID: "task2", OrganizationID: "org1", Title: "Task 2", Dependencies: []string{"task1"}},
	}

	err := td.CreateTasks(ctx, tasks)
	if err != ErrCircularDependency {
		t.Fatalf("expected circular dependency error, got %v", err)
	}
}

func TestTaskDecomposerAcquire(t *testing.T) {
	provider := db.NewTestProvider(t)
	createTestTable(t, provider)

	td := NewTaskDecomposer(provider)
	ctx := context.Background()

	tasks := []*Task{
		{ID: "taskA", OrganizationID: "orgAcquire", Title: "Task A", Dependencies: []string{}},
		{ID: "taskB", OrganizationID: "orgAcquire", Title: "Task B", Dependencies: []string{"taskA"}},
	}

	err := td.CreateTasks(ctx, tasks)
	if err != nil {
		t.Fatalf("failed to create tasks: %v", err)
	}

	// Try to acquire, should get Task A because B depends on A
	task, err := td.AcquirePendingTask(ctx, "orgAcquire", "agent1")
	if err != nil {
		t.Fatalf("failed to acquire task: %v", err)
	}
	if task == nil {
		t.Fatalf("expected task, got nil")
	}
	if task.ID != "taskA" {
		t.Fatalf("expected taskA, got %s", task.ID)
	}
	if task.Status != "IN_PROGRESS" {
		t.Fatalf("expected task status IN_PROGRESS, got %s", task.Status)
	}

	// Try to acquire again, should fail because B still depends on A (which is IN_PROGRESS)
	task2, err := td.AcquirePendingTask(ctx, "orgAcquire", "agent1")
	if err != nil {
		t.Fatalf("expected nil error, got %v", err)
	}
	if task2 != nil {
		t.Fatalf("expected no task to acquire, got %v", task2.ID)
	}

	// Complete Task A
	err = td.UpdateTaskStatus(ctx, "taskA", "COMPLETED")
	if err != nil {
		t.Fatalf("failed to complete task A: %v", err)
	}

	// Acquire again, should get Task B
	task3, err := td.AcquirePendingTask(ctx, "orgAcquire", "agent1")
	if err != nil {
		t.Fatalf("failed to acquire task: %v", err)
	}
	if task3 == nil {
		t.Fatalf("expected task, got nil")
	}
	if task3.ID != "taskB" {
		t.Fatalf("expected taskB, got %s", task3.ID)
	}
}
