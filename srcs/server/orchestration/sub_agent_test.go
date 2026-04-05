package orchestration

import (
	"context"
	"testing"
	"time"

	"github.com/onehumancorp/mono/srcs/server/auth"
	"github.com/onehumancorp/mono/srcs/server/db"
)

func TestSubAgentSpawner(t *testing.T) {
	// Create an in-memory SQLite database using the test provider logic
	prov := db.NewTestProvider(t)
	defer prov.Close()

	// Apply necessary schema manually for the test
	_, err := prov.Exec(context.Background(), `
		CREATE TABLE IF NOT EXISTS shared_tasks (
			id TEXT PRIMARY KEY,
			organization_id TEXT NOT NULL,
			parent_plan_id TEXT,
			dependencies JSONB NOT NULL DEFAULT '[]',
			title TEXT NOT NULL,
			description TEXT,
			status TEXT NOT NULL DEFAULT 'PENDING',
			agent_id TEXT,
			priority TEXT NOT NULL DEFAULT 'P2',
			payload TEXT NOT NULL DEFAULT '{}',
			locked_until DATETIME,
			created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
			updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
		);

		CREATE TABLE IF NOT EXISTS task_dependencies (
			task_id TEXT NOT NULL,
			depends_on_task_id TEXT NOT NULL,
			PRIMARY KEY (task_id, depends_on_task_id)
		);
	`)
	if err != nil {
		t.Fatalf("failed to create tables: %v", err)
	}

	tm := NewTaskManager(prov, nil)
	spawner := NewDefaultSubAgentSpawner(nil, tm)

	ctx := auth.ContextWithClaims(context.Background(), &auth.Claims{OrganizationID: "org-test"})

	// Create a task
	task, err := tm.CreateTask(ctx, "org-test", "Test Delegate", "Test Description", "DELEGATED")
	if err != nil {
		t.Fatalf("failed to create task: %v", err)
	}

	// Manually set status to IN_PROGRESS so it can be completed by the spawner
	_, err = prov.Exec(ctx, "UPDATE shared_tasks SET status = 'IN_PROGRESS', agent_id = 'spawner-worker' WHERE id = ?", task.ID)
	if err != nil {
		t.Fatalf("failed to manually update task to IN_PROGRESS: %v", err)
	}

	err = spawner.Spawn(ctx, task)
	if err != nil {
		t.Fatalf("spawner failed: %v", err)
	}

	// Give the mock goroutine time to complete the task
	time.Sleep(1 * time.Second)

	// Check if the task was completed
	var status string
	err = prov.QueryRow(ctx, "SELECT status FROM shared_tasks WHERE id = ?", task.ID).Scan(&status)
	if err != nil {
		t.Fatalf("failed to query task status: %v", err)
	}

	if status != "COMPLETED" {
		t.Errorf("expected task status to be COMPLETED, got %s", status)
	}
}
