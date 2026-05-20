package orchestration

import (
	"context"
	"database/sql"
	"testing"

	_ "github.com/mattn/go-sqlite3"
)

func TestClaimTask(t *testing.T) {
	db, err := sql.Open("sqlite3", ":memory:")
	if err != nil {
		t.Fatalf("Failed to open database: %v", err)
	}
	defer db.Close()

	// Create table
	_, err = db.Exec(`
		CREATE TABLE ohc_tasks (
			id TEXT PRIMARY KEY,
			title TEXT NOT NULL,
			description TEXT,
			status TEXT DEFAULT 'PENDING',
			assigned_agent_id TEXT,
			priority INTEGER DEFAULT 0,
			created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
			updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
		)
	`)
	if err != nil {
		t.Fatalf("Failed to create table: %v", err)
	}

	repo := NewTaskRepository(db)

	desc := "This is a test task"
	task := &Task{
		Title:       "Test Task",
		Description: &desc,
	}

	err = repo.CreateTask(context.Background(), task)
	if err != nil {
		t.Fatalf("Failed to create task: %v", err)
	}

	// Claim task
	claimed, err := repo.ClaimTask(context.Background(), task.ID, "agent-1")
	if err != nil {
		t.Fatalf("Failed to claim task: %v", err)
	}
	if !claimed {
		t.Errorf("Expected to successfully claim task")
	}

	// Attempt to claim again
	claimed2, err := repo.ClaimTask(context.Background(), task.ID, "agent-2")
	if err != nil {
		t.Fatalf("Failed to claim task again: %v", err)
	}
	if claimed2 {
		t.Errorf("Expected to fail to claim task again")
	}
}
