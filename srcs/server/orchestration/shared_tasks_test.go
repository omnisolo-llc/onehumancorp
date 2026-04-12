package orchestration_test

import (
	"context"
	"database/sql"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/db"
	"github.com/onehumancorp/mono/srcs/server/orchestration"
	_ "modernc.org/sqlite"
)

func setupSharedTasksTestDB(t *testing.T) db.Provider {
	t.Helper()
	sqldb, err := sql.Open("sqlite", ":memory:")
	if err != nil {
		t.Fatalf("failed to open test sqlite db: %v", err)
	}

	if err := sqldb.PingContext(context.Background()); err != nil {
		t.Fatalf("failed to ping test sqlite db: %v", err)
	}

	t.Cleanup(func() {
		sqldb.Close()
	})

	provider := db.NewSqliteProvider(sqldb)

	ctx := context.Background()
	_, err = provider.Exec(ctx, `
		CREATE TABLE IF NOT EXISTS shared_tasks (
			id TEXT PRIMARY KEY,
			organization_id TEXT NOT NULL,
			parent_plan_id TEXT,
			title TEXT NOT NULL,
			description TEXT,
			status TEXT NOT NULL DEFAULT 'PENDING',
			agent_id TEXT,
			dependencies TEXT NOT NULL DEFAULT '[]',
			created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
			updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
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

	return provider
}

func TestClaimTask(t *testing.T) {
	provider := setupSharedTasksTestDB(t)
	orch := orchestration.NewOrchestrator(provider)
	ctx := context.Background()

	// Insert task with no dependencies
	_, err := provider.Exec(ctx, `
		INSERT INTO shared_tasks (id, organization_id, title, status, dependencies)
		VALUES ('task1', 'org1', 'Task 1', 'PENDING', '[]')
	`)
	if err != nil {
		t.Fatalf("insert task failed: %v", err)
	}

	// Insert dependent task
	_, err = provider.Exec(ctx, `
		INSERT INTO shared_tasks (id, organization_id, title, status, dependencies)
		VALUES ('task2', 'org1', 'Task 2', 'PENDING', '[]')
	`)
	if err != nil {
		t.Fatalf("insert task failed: %v", err)
	}

	_, err = provider.Exec(ctx, `
		INSERT INTO task_dependencies (task_id, depends_on_task_id)
		VALUES ('task2', 'task1')
	`)
	if err != nil {
		t.Fatalf("insert dependency failed: %v", err)
	}

	// Claiming should only return task1
	claimed, err := orch.ClaimTask(ctx, "agent1")
	if err != nil {
		t.Fatalf("claim failed: %v", err)
	}
	if claimed == nil || claimed.ID != "task1" {
		t.Fatalf("expected task1, got %v", claimed)
	}

	// Second claim should return nil since task2 is blocked by task1
	claimed2, err := orch.ClaimTask(ctx, "agent2")
	if err != nil {
		t.Fatalf("claim failed: %v", err)
	}
	if claimed2 != nil {
		t.Fatalf("expected no task, got %v", claimed2)
	}

	// Mark task1 as COMPLETED
	_, err = provider.Exec(ctx, `UPDATE shared_tasks SET status = 'COMPLETED' WHERE id = 'task1'`)
	if err != nil {
		t.Fatalf("update failed: %v", err)
	}

	// Claiming should now return task2
	claimed3, err := orch.ClaimTask(ctx, "agent3")
	if err != nil {
		t.Fatalf("claim failed: %v", err)
	}
	if claimed3 == nil || claimed3.ID != "task2" {
		t.Fatalf("expected task2, got %v", claimed3)
	}
}
