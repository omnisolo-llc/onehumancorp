package repositories

import (
	"context"
	"database/sql"
	"testing"

	"github.com/google/uuid"
	"github.com/onehumancorp/mono/srcs/server/db"
	"github.com/onehumancorp/mono/srcs/server/db/models"
	_ "modernc.org/sqlite"
)

func setupTestDB(t *testing.T) db.Provider {
	sqlDB, err := sql.Open("sqlite", ":memory:")
	if err != nil {
		t.Fatalf("Failed to open sqlite db: %v", err)
	}

	// Create tables
	schema := `
	CREATE TABLE swarm_tasks (
		id TEXT PRIMARY KEY,
		title TEXT NOT NULL,
		description TEXT,
		status TEXT NOT NULL DEFAULT 'PENDING',
		priority TEXT NOT NULL DEFAULT 'P0',
		agent_id TEXT,
		created_at DATETIME NOT NULL,
		updated_at DATETIME NOT NULL
	);

	CREATE TABLE state_machine_transitions (
		id TEXT PRIMARY KEY,
		task_id TEXT NOT NULL,
		from_state TEXT,
		to_state TEXT NOT NULL,
		triggered_by TEXT,
		transitioned_at DATETIME NOT NULL,
		FOREIGN KEY (task_id) REFERENCES swarm_tasks(id) ON DELETE CASCADE
	);

	CREATE TABLE task_dependencies (
		task_id TEXT NOT NULL,
		depends_on_task_id TEXT NOT NULL,
		PRIMARY KEY (task_id, depends_on_task_id),
		FOREIGN KEY (task_id) REFERENCES swarm_tasks(id) ON DELETE CASCADE,
		FOREIGN KEY (depends_on_task_id) REFERENCES swarm_tasks(id) ON DELETE CASCADE
	);
	`
	_, err = sqlDB.Exec(schema)
	if err != nil {
		t.Fatalf("Failed to create schema: %v", err)
	}

	return db.NewSqliteProvider(sqlDB)
}

func TestTaskRepository_CreateAndGetPendingTasks(t *testing.T) {
	provider := setupTestDB(t)
	repo := NewTaskRepository(provider)
	ctx := context.Background()

	task1 := &models.SwarmTask{
		Title:       "Test Task 1",
		Description: "Desc 1",
	}
	err := repo.CreateTask(ctx, task1)
	if err != nil {
		t.Fatalf("Failed to create task 1: %v", err)
	}

	task2 := &models.SwarmTask{
		Title:       "Test Task 2",
		Description: "Desc 2",
		Status:      "DONE",
	}
	err = repo.CreateTask(ctx, task2)
	if err != nil {
		t.Fatalf("Failed to create task 2: %v", err)
	}

	pending, err := repo.GetPendingTasks(ctx)
	if err != nil {
		t.Fatalf("Failed to get pending tasks: %v", err)
	}

	if len(pending) != 1 {
		t.Errorf("Expected 1 pending task, got %d", len(pending))
	}
	if pending[0].Title != "Test Task 1" {
		t.Errorf("Expected 'Test Task 1', got '%s'", pending[0].Title)
	}
}

func TestTaskRepository_ClaimTask(t *testing.T) {
	provider := setupTestDB(t)
	repo := NewTaskRepository(provider)
	ctx := context.Background()

	taskID := uuid.New().String()
	task := &models.SwarmTask{
		ID:          taskID,
		Title:       "Task to claim",
		Description: "Desc",
	}
	err := repo.CreateTask(ctx, task)
	if err != nil {
		t.Fatalf("Failed to create task: %v", err)
	}

	agentID := uuid.New().String()
	claimed, err := repo.ClaimTask(ctx, taskID, agentID)
	if err != nil {
		t.Fatalf("Failed to claim task: %v", err)
	}
	if !claimed {
		t.Errorf("Expected task to be claimed")
	}

	// Try claiming again
	claimed2, err := repo.ClaimTask(ctx, taskID, uuid.New().String())
	if err != nil {
		t.Fatalf("Error during second claim attempt: %v", err)
	}
	if claimed2 {
		t.Errorf("Expected second claim to fail")
	}
}

func TestTaskRepository_CompleteTask(t *testing.T) {
	provider := setupTestDB(t)
	repo := NewTaskRepository(provider)
	ctx := context.Background()

	taskID := uuid.New().String()
	task := &models.SwarmTask{
		ID:          taskID,
		Title:       "Task to complete",
		Description: "Desc",
	}
	err := repo.CreateTask(ctx, task)
	if err != nil {
		t.Fatalf("Failed to create task: %v", err)
	}

	err = repo.CompleteTask(ctx, taskID)
	if err != nil {
		t.Fatalf("Failed to complete task: %v", err)
	}

	pending, err := repo.GetPendingTasks(ctx)
	if err != nil {
		t.Fatalf("Failed to get pending tasks: %v", err)
	}
	if len(pending) != 0 {
		t.Errorf("Expected 0 pending tasks, got %d", len(pending))
	}
}

func TestTaskRepository_TaskDependencies(t *testing.T) {
	provider := setupTestDB(t)
	repo := NewTaskRepository(provider)
	ctx := context.Background()

	task1ID := uuid.New().String()
	task2ID := uuid.New().String()

	// Create tasks
	repo.CreateTask(ctx, &models.SwarmTask{ID: task1ID, Title: "Task 1"})
	repo.CreateTask(ctx, &models.SwarmTask{ID: task2ID, Title: "Task 2"})

	_, err := provider.Exec(ctx, "INSERT INTO task_dependencies (task_id, depends_on_task_id) VALUES (?, ?)", task1ID, task2ID)
	if err != nil {
		t.Fatalf("Failed to insert dependency: %v", err)
	}

	deps, err := repo.GetTaskDependencies(ctx, task1ID)
	if err != nil {
		t.Fatalf("Failed to get dependencies: %v", err)
	}
	if len(deps) != 1 {
		t.Errorf("Expected 1 dependency, got %d", len(deps))
	}
	if deps[0] != task2ID {
		t.Errorf("Expected dependency ID %s, got %s", task2ID, deps[0])
	}
}
