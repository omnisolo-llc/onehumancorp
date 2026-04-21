package orchestration

import (
	"context"
	"database/sql"
	"testing"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
	_ "modernc.org/sqlite"
)

func newTaskRepositoryTestProvider(t *testing.T) db.Provider {
	t.Helper()
	sqlDB, err := sql.Open("sqlite", ":memory:")
	if err != nil {
		t.Fatalf("failed to open test sqlite db: %v", err)
	}
	if err := sqlDB.PingContext(context.Background()); err != nil {
		t.Fatalf("failed to ping test sqlite db: %v", err)
	}
	t.Cleanup(func() {
		sqlDB.Close()
	})

	provider := db.NewSqliteProvider(sqlDB)

	// Create table
	_, err = provider.Exec(context.Background(), `
		CREATE TABLE ohc_tasks (
			id VARCHAR PRIMARY KEY,
			title TEXT NOT NULL,
			description TEXT,
			status TEXT DEFAULT 'PENDING',
			assigned_agent_id TEXT,
			priority INTEGER DEFAULT 0,
			created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
			updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
		);
	`)
	if err != nil {
		t.Fatalf("failed to create ohc_tasks table: %v", err)
	}

	return provider
}

func TestTaskRepository(t *testing.T) {
	provider := newTaskRepositoryTestProvider(t)
	repo := NewTaskRepository(provider)
	ctx := context.Background()

	task := &TaskEntity{
		ID:          "task-1",
		Title:       "Test Task",
		Description: "A task for testing",
		Status:      "PENDING",
		Priority:    1,
		CreatedAt:   time.Now().UTC(),
		UpdatedAt:   time.Now().UTC(),
	}

	err := repo.CreateTask(ctx, task)
	if err != nil {
		t.Fatalf("CreateTask failed: %v", err)
	}

	tasks, err := repo.ListTasks(ctx)
	if err != nil {
		t.Fatalf("ListTasks failed: %v", err)
	}
	if len(tasks) != 1 {
		t.Fatalf("expected 1 task, got %d", len(tasks))
	}

	// Test claiming
	claimed, err := repo.ClaimTask(ctx, "task-1", "agent-1")
	if err != nil {
		t.Fatalf("ClaimTask failed: %v", err)
	}
	if !claimed {
		t.Fatal("expected task to be claimed")
	}

	// Test claiming already claimed task
	claimedAgain, err := repo.ClaimTask(ctx, "task-1", "agent-2")
	if err != nil {
		t.Fatalf("ClaimTask on already claimed task failed: %v", err)
	}
	if claimedAgain {
		t.Fatal("expected task to not be claimed again")
	}

	// Verify status
	updatedTask, err := repo.GetTask(ctx, "task-1")
	if err != nil {
		t.Fatalf("GetTask failed: %v", err)
	}
	if updatedTask.Status != "IN_PROGRESS" {
		t.Fatalf("expected status IN_PROGRESS, got %s", updatedTask.Status)
	}
	if updatedTask.AssignedAgentID == nil || *updatedTask.AssignedAgentID != "agent-1" {
		t.Fatalf("expected assigned agent agent-1, got %v", updatedTask.AssignedAgentID)
	}
}

func TestUpdateTaskStatus(t *testing.T) {
	provider := newTaskRepositoryTestProvider(t)
	repo := NewTaskRepository(provider)
	ctx := context.Background()

	task := &TaskEntity{
		ID:          "task-2",
		Title:       "Test Task 2",
		Description: "A task for testing status update",
		Status:      "PENDING",
		Priority:    1,
		CreatedAt:   time.Now().UTC(),
		UpdatedAt:   time.Now().UTC(),
	}

	err := repo.CreateTask(ctx, task)
	if err != nil {
		t.Fatalf("CreateTask failed: %v", err)
	}

	err = repo.UpdateTaskStatus(ctx, "task-2", "DONE")
	if err != nil {
		t.Fatalf("UpdateTaskStatus failed: %v", err)
	}

	updatedTask, err := repo.GetTask(ctx, "task-2")
	if err != nil {
		t.Fatalf("GetTask failed: %v", err)
	}
	if updatedTask.Status != "DONE" {
		t.Fatalf("expected status DONE, got %s", updatedTask.Status)
	}
}
