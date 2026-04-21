package tasks

import (
	"context"
	"testing"
	"github.com/onehumancorp/mono/srcs/server/db"
)

// Using test provider that applies convertBindVars for SQLite fallback mode
func TestQueueClaimAndComplete(t *testing.T) {
	provider := db.NewTestProvider(t)
	defer provider.Close()

	_, err := provider.Exec(context.Background(), `CREATE TABLE IF NOT EXISTS shared_tasks (
		id VARCHAR PRIMARY KEY,
		title VARCHAR NOT NULL,
		status VARCHAR NOT NULL DEFAULT 'PENDING',
		assignee VARCHAR,
		created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
		updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
	)`)
	if err != nil {
		t.Fatalf("failed to create table: %v", err)
	}

	_, err = provider.Exec(context.Background(), `INSERT INTO shared_tasks (id, title) VALUES ('1', 'Test Task')`)
	if err != nil {
		t.Fatalf("failed to insert: %v", err)
	}

	queue := NewTaskQueue(provider)

	id, err := queue.ClaimTask(context.Background(), "agent-1")
	if err != nil {
		t.Fatalf("failed to claim task: %v", err)
	}
	if id != "1" {
		t.Errorf("expected id '1', got '%s'", id)
	}

	err = queue.CompleteTask(context.Background(), id)
	if err != nil {
		t.Fatalf("failed to complete task: %v", err)
	}
}

// Test claiming when queue is empty
func TestQueueClaimEmpty(t *testing.T) {
	provider := db.NewTestProvider(t)
	defer provider.Close()

	_, err := provider.Exec(context.Background(), `CREATE TABLE IF NOT EXISTS shared_tasks (
		id VARCHAR PRIMARY KEY,
		title VARCHAR NOT NULL,
		status VARCHAR NOT NULL DEFAULT 'PENDING',
		assignee VARCHAR,
		created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
		updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
	)`)
	if err != nil {
		t.Fatalf("failed to create table: %v", err)
	}

	queue := NewTaskQueue(provider)

	id, err := queue.ClaimTask(context.Background(), "agent-1")
	if err != nil {
		t.Fatalf("failed to claim task: %v", err)
	}
	if id != "" {
		t.Errorf("expected empty id, got '%s'", id)
	}
}
