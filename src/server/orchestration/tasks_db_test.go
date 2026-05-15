package orchestration

import (
	"context"
	"database/sql"
	"testing"

	_ "github.com/mattn/go-sqlite3"
)

func TestTaskDecomposerEngine(t *testing.T) {
	db, err := sql.Open("sqlite3", ":memory:")
	if err != nil {
		t.Fatalf("Failed to open db: %v", err)
	}
	defer db.Close()

	_, err = db.Exec(`
		CREATE TABLE shared_tasks (
			id TEXT PRIMARY KEY,
			organization_id TEXT NOT NULL,
			title TEXT NOT NULL,
			status TEXT NOT NULL DEFAULT 'PENDING',
			dependencies TEXT DEFAULT '[]'
		)
	`)
	if err != nil {
		t.Fatalf("Failed to create table: %v", err)
	}

	engine := NewTaskDecomposerEngine(db, true)

	// Test case 1: Task without dependencies
	db.Exec("INSERT INTO shared_tasks (id, organization_id, title, status) VALUES ('task1', 'org1', 'Task 1', 'PENDING')")
	task, err := engine.ClaimTask(context.Background(), "org1", "agent1")
	if err != nil {
		t.Fatalf("Expected no error, got %v", err)
	}
	if task == nil || task.ID != "task1" {
		t.Fatalf("Expected task1, got %v", task)
	}

	// Test case 2: Task with unmet dependency
	db.Exec("INSERT INTO shared_tasks (id, organization_id, title, status) VALUES ('task2', 'org1', 'Task 2', 'PENDING')")
	db.Exec("INSERT INTO shared_tasks (id, organization_id, title, status, dependencies) VALUES ('task3', 'org1', 'Task 3', 'PENDING', '[\"task2\"]')")

	task, err = engine.ClaimTask(context.Background(), "org1", "agent1")
	// Expected to claim task2 as task3 has unmet deps
	if err != nil {
		t.Fatalf("Expected no error, got %v", err)
	}
	if task == nil || task.ID != "task2" {
		t.Fatalf("Expected task2, got %v", task)
	}

	// Now no task can be claimed because task3 depends on task2 which is IN_PROGRESS
	task, err = engine.ClaimTask(context.Background(), "org1", "agent1")
	if err != nil {
		t.Fatalf("Expected no error, got %v", err)
	}
	if task != nil {
		t.Fatalf("Expected no task, got %v", task)
	}

	// Test case 3: Task with met dependency
	db.Exec("UPDATE shared_tasks SET status = 'DONE' WHERE id = 'task2'")
	task, err = engine.ClaimTask(context.Background(), "org1", "agent1")
	if err != nil {
		t.Fatalf("Expected no error, got %v", err)
	}
	if task == nil || task.ID != "task3" {
		t.Fatalf("Expected task3, got %v", task)
	}
}
