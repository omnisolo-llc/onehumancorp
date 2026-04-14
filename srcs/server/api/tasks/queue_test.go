package tasks

import (
	"database/sql"
	"context"
	"testing"
	_ "modernc.org/sqlite"
	"github.com/onehumancorp/mono/srcs/server/db"
)

func TestTaskQueue(t *testing.T) {
	sqldb, err := sql.Open("sqlite", ":memory:")
	if err != nil {
		t.Fatalf("failed to open sqlite db: %v", err)
	}
	defer sqldb.Close()
	provider := db.NewSqliteProvider(sqldb)

	_, err = provider.Exec(context.Background(), `
		CREATE TABLE IF NOT EXISTS shared_tasks (
			id TEXT PRIMARY KEY,
			title TEXT NOT NULL,
			status TEXT NOT NULL DEFAULT 'PENDING',
			assignee TEXT,
			created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
			updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
		)
	`)
	if err != nil {
		t.Fatalf("Failed to create table: %v", err)
	}

	queue := NewTaskQueue(provider)

	_, err = provider.Exec(context.Background(), "INSERT INTO shared_tasks (id, title, status) VALUES ('1', 'Task 1', 'PENDING')")
	if err != nil {
		t.Fatalf("Failed to insert task: %v", err)
	}

	tasks, err := queue.ListTasks(context.Background())
	if err != nil {
		t.Fatalf("Failed to list tasks: %v", err)
	}
	if len(tasks) != 1 {
		t.Fatalf("Expected 1 task, got %d", len(tasks))
	}

	task, err := queue.ClaimTask(context.Background(), "agent-1")
	if err != nil {
		t.Fatalf("Failed to claim task: %v", err)
	}

	if task == nil {
		t.Fatalf("Expected to claim a task")
	}

	if task.ID != "1" || task.Status != "ASSIGNED" || task.Assignee != "agent-1" {
		t.Fatalf("Task not assigned correctly: %+v", task)
	}

	err = queue.CompleteTask(context.Background(), "1")
	if err != nil {
		t.Fatalf("Failed to complete task: %v", err)
	}
}
