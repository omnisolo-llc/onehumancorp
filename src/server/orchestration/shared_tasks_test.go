package orchestration

import (
	"context"
	"database/sql"
	"testing"

	_ "github.com/mattn/go-sqlite3"
)

func TestSharedTaskOrchestrator(t *testing.T) {
	db, err := sql.Open("sqlite3", ":memory:")
	if err != nil {
		t.Fatalf("Failed to open db: %v", err)
	}
	defer db.Close()

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
		)
	`)
	if err != nil {
		t.Fatalf("Failed to create table: %v", err)
	}

	orchestrator := NewSharedTaskOrchestrator(db)
	ctx := context.Background()

	task := &SharedTaskV4{
		OrganizationID: "org-1",
		Title:          "Test Task",
		Description:    "Test Description",
		Status:         "PENDING",
		Priority:       "P1",
		Dependencies:   "[]",
	}

	createdTask, err := orchestrator.CreateTask(ctx, task)
	if err != nil {
		t.Fatalf("Failed to create task: %v", err)
	}
	if createdTask.ID == "" {
		t.Errorf("Expected ID to be populated")
	}

	fetchedTask, err := orchestrator.GetTask(ctx, createdTask.ID)
	if err != nil {
		t.Fatalf("Failed to fetch task: %v", err)
	}
	if fetchedTask.Title != "Test Task" {
		t.Errorf("Expected title 'Test Task', got '%s'", fetchedTask.Title)
	}
}
