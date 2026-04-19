package tasks

import (
	"context"
	"database/sql"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/db"
	_ "modernc.org/sqlite"
)

func TestQueue(t *testing.T) {
	database, err := sql.Open("sqlite", ":memory:")
	if err != nil {
		t.Fatalf("Failed to open db: %v", err)
	}
	defer database.Close()

	_, err = database.Exec(`
		CREATE TABLE shared_tasks (
			id INTEGER PRIMARY KEY AUTOINCREMENT,
			organization_id TEXT NOT NULL,
			title TEXT NOT NULL,
			description TEXT,
			status TEXT NOT NULL DEFAULT 'PENDING',
			assignee TEXT,
			created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
			updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
		);
	`)
	if err != nil {
		t.Fatalf("Failed to create table: %v", err)
	}

	provider := db.NewSqliteProvider(database)
	q := NewQueue(provider)
	ctx := context.Background()

	task := &Task{
		OrganizationID: "org-1",
		Title:          "Test Task",
		Description:    "Test Description",
	}

	err = q.AddTask(ctx, task)
	if err != nil {
		t.Fatalf("Failed to add task: %v", err)
	}

	claimed, err := q.ClaimTask(ctx, "org-1", "agent-1")
	if err != nil {
		t.Fatalf("Failed to claim task: %v", err)
	}
	if claimed == nil {
		t.Fatalf("Expected to claim a task")
	}
	if claimed.Assignee != "agent-1" {
		t.Fatalf("Expected assignee agent-1, got %s", claimed.Assignee)
	}
	if claimed.Status != "IN_PROGRESS" {
		t.Fatalf("Expected status IN_PROGRESS, got %s", claimed.Status)
	}

	claimed2, err := q.ClaimTask(ctx, "org-1", "agent-2")
	if err != nil {
		t.Fatalf("Failed to claim task: %v", err)
	}
	if claimed2 != nil {
		t.Fatalf("Expected no task to be claimed, got %v", claimed2)
	}
}
