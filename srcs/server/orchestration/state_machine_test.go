package orchestration

import (
	"context"
	"testing"
	"github.com/onehumancorp/mono/srcs/server/db"
)

func TestTaskStateMachine_ProcessEvent(t *testing.T) {
	// Using memory sqlite
	provider, err := db.NewProvider("sqlite::memory:", 1)
	if err != nil {
		t.Fatalf("Failed to create db: %v", err)
	}

	// Create schema
	_, err = provider.DB().Exec(context.Background(), `
		CREATE TABLE shared_tasks (
			id TEXT PRIMARY KEY,
			organization_id TEXT NOT NULL,
			title TEXT NOT NULL,
			status TEXT NOT NULL DEFAULT 'PENDING',
			parent_task_id TEXT,
			workflow_state TEXT
		)
	`)
	if err != nil {
		t.Fatalf("Failed to create table: %v", err)
	}

	// Insert parent and children
	_, err = provider.DB().Exec(context.Background(), `
		INSERT INTO shared_tasks (id, organization_id, title, status) VALUES ('parent-1', 'org-1', 'Parent', 'EXECUTING')
	`)
	_, err = provider.DB().Exec(context.Background(), `
		INSERT INTO shared_tasks (id, organization_id, title, status, parent_task_id) VALUES ('child-1', 'org-1', 'Child 1', 'DONE', 'parent-1')
	`)
	_, err = provider.DB().Exec(context.Background(), `
		INSERT INTO shared_tasks (id, organization_id, title, status, parent_task_id) VALUES ('child-2', 'org-1', 'Child 2', 'EXECUTING', 'parent-1')
	`)

	sm := NewTaskStateMachine(provider)

	// Test completion of last child
	_, _ = provider.DB().Exec(context.Background(), "UPDATE shared_tasks SET status = 'DONE' WHERE id = 'child-2'")
	err = sm.ProcessEvent(context.Background(), "child-2", EventSubTaskCompleted)
	if err != nil {
		t.Fatalf("ProcessEvent failed: %v", err)
	}

	// Check parent status
	var status string
	err = provider.DB().QueryRow(context.Background(), "SELECT status FROM shared_tasks WHERE id = 'parent-1'").Scan(&status)
	if err != nil {
		t.Fatalf("QueryRow failed: %v", err)
	}
	if status != TaskStateVerifying {
		t.Errorf("Expected parent status %s, got %s", TaskStateVerifying, status)
	}
}
