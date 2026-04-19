package repository

import (
	"context"
	"database/sql"
	"testing"
	"time"

	_ "modernc.org/sqlite"
)

func setupDB(t *testing.T) *sql.DB {
	db, err := sql.Open("sqlite", ":memory:")
	if err != nil {
		t.Fatalf("failed to open database: %v", err)
	}

	schema := `
	CREATE TABLE tasks (
		id TEXT PRIMARY KEY,
		organization_id TEXT NOT NULL,
		parent_task_id TEXT,
		title TEXT NOT NULL,
		description TEXT,
		status TEXT NOT NULL DEFAULT 'PENDING',
		assigned_agent_role TEXT,
		created_at DATETIME,
		updated_at DATETIME
	);
	CREATE TABLE task_dependencies (
		task_id TEXT NOT NULL,
		depends_on_task_id TEXT NOT NULL,
		PRIMARY KEY (task_id, depends_on_task_id),
		FOREIGN KEY (task_id) REFERENCES tasks(id),
		FOREIGN KEY (depends_on_task_id) REFERENCES tasks(id)
	);
	`
	_, err = db.Exec(schema)
	if err != nil {
		t.Fatalf("failed to create schema: %v", err)
	}

	return db
}

func TestTaskRepository_CreateTask(t *testing.T) {
	db := setupDB(t)
	defer db.Close()

	repo := NewTaskRepository(db)
	ctx := context.WithValue(context.Background(), orgIDKey, "org-123")

	task := &Task{
		Title:             "Test Task",
		Description:       "Test Description",
		Status:            "PENDING",
		AssignedAgentRole: "tester",
	}

	err := repo.CreateTask(ctx, task)
	if err != nil {
		t.Fatalf("failed to create task: %v", err)
	}

	if task.ID == "" {
		t.Errorf("expected task ID to be set")
	}
	if task.OrganizationID != "org-123" {
		t.Errorf("expected organization_id to be org-123, got %s", task.OrganizationID)
	}
	if task.CreatedAt.IsZero() {
		t.Errorf("expected created_at to be set")
	}
	if task.UpdatedAt.IsZero() {
		t.Errorf("expected updated_at to be set")
	}

	// Test missing organization ID in context
	ctxNoOrg := context.Background()
	err = repo.CreateTask(ctxNoOrg, &Task{Title: "Should Fail"})
	if err == nil {
		t.Errorf("expected error when context has no organization_id")
	}
}

func TestTaskRepository_GetTasksByOrg(t *testing.T) {
	db := setupDB(t)
	defer db.Close()

	repo := NewTaskRepository(db)
	ctxOrg1 := context.WithValue(context.Background(), orgIDKey, "org-1")
	ctxOrg2 := context.WithValue(context.Background(), orgIDKey, "org-2")

	// Create tasks for org-1
	_ = repo.CreateTask(ctxOrg1, &Task{Title: "Task 1"})
	_ = repo.CreateTask(ctxOrg1, &Task{Title: "Task 2"})

	// Create tasks for org-2
	_ = repo.CreateTask(ctxOrg2, &Task{Title: "Task 3"})

	// Test getting tasks for org-1
	tasksOrg1, err := repo.GetTasksByOrg(ctxOrg1)
	if err != nil {
		t.Fatalf("failed to get tasks: %v", err)
	}
	if len(tasksOrg1) != 2 {
		t.Errorf("expected 2 tasks for org-1, got %d", len(tasksOrg1))
	}

	// Test getting tasks for org-2
	tasksOrg2, err := repo.GetTasksByOrg(ctxOrg2)
	if err != nil {
		t.Fatalf("failed to get tasks: %v", err)
	}
	if len(tasksOrg2) != 1 {
		t.Errorf("expected 1 task for org-2, got %d", len(tasksOrg2))
	}
	if tasksOrg2[0].Title != "Task 3" {
		t.Errorf("expected task title 'Task 3', got %s", tasksOrg2[0].Title)
	}

	// Test missing organization ID
	_, err = repo.GetTasksByOrg(context.Background())
	if err == nil {
		t.Errorf("expected error when context has no organization_id")
	}
}

func TestTaskRepository_UpdateTaskStatus(t *testing.T) {
	db := setupDB(t)
	defer db.Close()

	repo := NewTaskRepository(db)
	ctx := context.WithValue(context.Background(), orgIDKey, "org-123")

	task := &Task{
		Title: "Test Task",
	}
	err := repo.CreateTask(ctx, task)
	if err != nil {
		t.Fatalf("failed to create task: %v", err)
	}

	time.Sleep(10 * time.Millisecond) // Ensure updated_at changes

	err = repo.UpdateTaskStatus(ctx, task.ID, "IN_PROGRESS")
	if err != nil {
		t.Fatalf("failed to update task status: %v", err)
	}

	// Verify the update
	tasks, _ := repo.GetTasksByOrg(ctx)
	if tasks[0].Status != "IN_PROGRESS" {
		t.Errorf("expected status 'IN_PROGRESS', got %s", tasks[0].Status)
	}
	if !tasks[0].UpdatedAt.After(tasks[0].CreatedAt) {
		t.Errorf("expected updated_at (%v) to be after created_at (%v)", tasks[0].UpdatedAt, tasks[0].CreatedAt)
	}

	// Test updating task for different org (should fail)
	ctxOtherOrg := context.WithValue(context.Background(), orgIDKey, "org-other")
	err = repo.UpdateTaskStatus(ctxOtherOrg, task.ID, "DONE")
	if err == nil {
		t.Errorf("expected error when updating task owned by different organization")
	}

	// Test missing organization ID
	err = repo.UpdateTaskStatus(context.Background(), task.ID, "DONE")
	if err == nil {
		t.Errorf("expected error when context has no organization_id")
	}
}
