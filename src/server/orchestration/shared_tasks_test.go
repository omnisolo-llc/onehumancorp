package orchestration

import (
	"context"
	"database/sql"
	"testing"

	_ "github.com/mattn/go-sqlite3"
)

func setupSharedTasksTestDB(t *testing.T) *sql.DB {
	db, err := sql.Open("sqlite3", ":memory:")
	if err != nil {
		t.Fatalf("failed to open memory db: %v", err)
	}

	_, err = db.Exec(`
		CREATE TABLE shared_tasks_v4 (
			id VARCHAR PRIMARY KEY,
			organization_id VARCHAR NOT NULL,
			title VARCHAR NOT NULL,
			description TEXT,
			status VARCHAR NOT NULL DEFAULT 'PENDING',
			agent_id VARCHAR,
			priority VARCHAR NOT NULL DEFAULT 'P2',
			payload TEXT,
			parent_plan_id TEXT,
			dependencies TEXT NOT NULL DEFAULT '[]',
			created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
			updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
		);
	`)
	if err != nil {
		t.Fatalf("failed to create table: %v", err)
	}

	return db
}

func TestSharedTaskOrchestrator_CreateTask(t *testing.T) {
	db := setupSharedTasksTestDB(t)
	defer db.Close()

	orchestrator := NewSharedTaskOrchestrator(db)

	// Happy path
	task := &SharedTask{
		OrganizationID: "org-1",
		Title:          "Test Task",
		Status:         "PENDING",
		Priority:       "P1",
		Dependencies:   []string{"task-a", "task-b"},
	}

	err := orchestrator.CreateTask(context.Background(), task)
	if err != nil {
		t.Fatalf("CreateTask failed: %v", err)
	}

	if task.ID == "" {
		t.Errorf("expected task ID to be generated")
	}

	if task.CreatedAt.IsZero() {
		t.Errorf("expected task CreatedAt to be set")
	}

	// Verify insertion
	var count int
	err = db.QueryRow("SELECT count(*) FROM shared_tasks_v4 WHERE id = ?", task.ID).Scan(&count)
	if err != nil {
		t.Fatalf("failed to query count: %v", err)
	}
	if count != 1 {
		t.Errorf("expected 1 task, got %d", count)
	}

	// Test defaults
	task2 := &SharedTask{
		OrganizationID: "org-2",
		Title:          "Test Task 2",
	}
	err = orchestrator.CreateTask(context.Background(), task2)
	if err != nil {
		t.Fatalf("CreateTask failed: %v", err)
	}
	if task2.Status != "PENDING" {
		t.Errorf("expected Status to be PENDING, got %s", task2.Status)
	}
	if task2.Priority != "P2" {
		t.Errorf("expected Priority to be P2, got %s", task2.Priority)
	}
	if len(task2.Dependencies) != 0 {
		t.Errorf("expected Dependencies to be empty, got %v", task2.Dependencies)
	}
}

func TestSharedTaskOrchestrator_GetTask(t *testing.T) {
	db := setupSharedTasksTestDB(t)
	defer db.Close()

	orchestrator := NewSharedTaskOrchestrator(db)

	task := &SharedTask{
		OrganizationID: "org-1",
		Title:          "Test Task",
	}
	err := orchestrator.CreateTask(context.Background(), task)
	if err != nil {
		t.Fatalf("CreateTask failed: %v", err)
	}

	retrieved, err := orchestrator.GetTask(context.Background(), task.ID)
	if err != nil {
		t.Fatalf("GetTask failed: %v", err)
	}

	if retrieved.ID != task.ID {
		t.Errorf("expected ID %s, got %s", task.ID, retrieved.ID)
	}
	if retrieved.Title != task.Title {
		t.Errorf("expected Title %s, got %s", task.Title, retrieved.Title)
	}

	// Test not found
	_, err = orchestrator.GetTask(context.Background(), "non-existent")
	if err == nil {
		t.Errorf("expected error for non-existent task")
	}
}

func TestSharedTaskOrchestrator_UpdateTaskStatus(t *testing.T) {
	db := setupSharedTasksTestDB(t)
	defer db.Close()

	orchestrator := NewSharedTaskOrchestrator(db)

	task := &SharedTask{
		OrganizationID: "org-1",
		Title:          "Test Task",
	}
	err := orchestrator.CreateTask(context.Background(), task)
	if err != nil {
		t.Fatalf("CreateTask failed: %v", err)
	}

	err = orchestrator.UpdateTaskStatus(context.Background(), task.ID, "COMPLETED")
	if err != nil {
		t.Fatalf("UpdateTaskStatus failed: %v", err)
	}

	retrieved, err := orchestrator.GetTask(context.Background(), task.ID)
	if err != nil {
		t.Fatalf("GetTask failed: %v", err)
	}
	if retrieved.Status != "COMPLETED" {
		t.Errorf("expected Status COMPLETED, got %s", retrieved.Status)
	}

	// Test not found
	err = orchestrator.UpdateTaskStatus(context.Background(), "non-existent", "COMPLETED")
	if err == nil {
		t.Errorf("expected error for non-existent task")
	}
}

func TestSharedTaskOrchestrator_ListTasksByAgent(t *testing.T) {
	db := setupSharedTasksTestDB(t)
	defer db.Close()

	orchestrator := NewSharedTaskOrchestrator(db)

	task1 := &SharedTask{
		OrganizationID: "org-1",
		Title:          "Test Task 1",
		AgentID:        "agent-1",
	}
	err := orchestrator.CreateTask(context.Background(), task1)
	if err != nil {
		t.Fatalf("CreateTask failed: %v", err)
	}

	task2 := &SharedTask{
		OrganizationID: "org-1",
		Title:          "Test Task 2",
		AgentID:        "agent-1",
	}
	err = orchestrator.CreateTask(context.Background(), task2)
	if err != nil {
		t.Fatalf("CreateTask failed: %v", err)
	}

	task3 := &SharedTask{
		OrganizationID: "org-1",
		Title:          "Test Task 3",
		AgentID:        "agent-2",
	}
	err = orchestrator.CreateTask(context.Background(), task3)
	if err != nil {
		t.Fatalf("CreateTask failed: %v", err)
	}

	tasks, err := orchestrator.ListTasksByAgent(context.Background(), "agent-1")
	if err != nil {
		t.Fatalf("ListTasksByAgent failed: %v", err)
	}

	if len(tasks) != 2 {
		t.Errorf("expected 2 tasks, got %d", len(tasks))
	}

	tasks, err = orchestrator.ListTasksByAgent(context.Background(), "agent-3")
	if err != nil {
		t.Fatalf("ListTasksByAgent failed: %v", err)
	}
	if len(tasks) != 0 {
		t.Errorf("expected 0 tasks, got %d", len(tasks))
	}
}

func TestSharedTaskOrchestrator_CreateTask_DBError(t *testing.T) {
	db, err := sql.Open("sqlite3", ":memory:")
	if err != nil {
		t.Fatalf("failed to open memory db: %v", err)
	}
	defer db.Close()

	// Intentionally don't create table to cause error

	orchestrator := NewSharedTaskOrchestrator(db)
	task := &SharedTask{
		OrganizationID: "org-1",
		Title:          "Test Task",
	}

	err = orchestrator.CreateTask(context.Background(), task)
	if err == nil {
		t.Errorf("expected error for db failure")
	}
}
