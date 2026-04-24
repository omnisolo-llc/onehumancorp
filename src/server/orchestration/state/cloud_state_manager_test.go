package state

import (
	"context"
	"encoding/json"
	"testing"

	"github.com/onehumancorp/mono/src/server/db"
	_ "modernc.org/sqlite"
)

func TestCloudStateManager(t *testing.T) {
	provider := db.NewTestProvider(t)
	defer provider.Close()

	ctx := context.Background()

	// Apply migrations or create tables directly
	tx, err := provider.Begin(ctx)
	if err != nil {
		t.Fatal(err)
	}

	_, err = tx.Exec(ctx, `
		CREATE TABLE IF NOT EXISTS swarm_tasks (
			id TEXT PRIMARY KEY DEFAULT (lower(hex(randomblob(16)))),
			mission_id TEXT NOT NULL,
			parent_plan_id TEXT,
			dependencies JSON NOT NULL DEFAULT '[]',
			title TEXT NOT NULL,
			status TEXT NOT NULL DEFAULT 'PENDING',
			assigned_agent_id TEXT,
			payload JSON,
			locked_until DATETIME,
			created_at DATETIME DEFAULT CURRENT_TIMESTAMP
		);

		CREATE TABLE IF NOT EXISTS state_machine_transitions (
			id TEXT PRIMARY KEY DEFAULT (lower(hex(randomblob(16)))),
			entity_id TEXT NOT NULL,
			entity_type TEXT NOT NULL,
			from_state TEXT NOT NULL,
			to_state TEXT NOT NULL,
			agent_id TEXT,
			reason TEXT,
			occurred_at DATETIME DEFAULT CURRENT_TIMESTAMP
		);
	`)
	if err != nil {
		t.Fatal(err)
	}
	tx.Commit(ctx)

	// Since we mock CloudStateManager with SQLite for test, ClaimTask will fail
	// gracefully or have syntax errors if it runs real PostgreSQL syntax on SQLite.
	// But let's test it using NewCloudStateManager

	sm := NewCloudStateManager(provider, nil)

	tx, _ = provider.Begin(ctx)
	_, err = tx.Exec(ctx, "INSERT INTO swarm_tasks (id, mission_id, title, status) VALUES ('c_task1', 'm1', 'Task 1', 'PENDING')")
	if err != nil {
		t.Fatal(err)
	}

	_, err = tx.Exec(ctx, "INSERT INTO swarm_tasks (id, mission_id, title, status) VALUES ('c_parent_task', 'm1', 'Parent Task', 'COMPLETED')")
	if err != nil {
		t.Fatal(err)
	}

	deps, _ := json.Marshal([]string{"c_parent_task"})
	_, err = tx.Exec(ctx, "INSERT INTO swarm_tasks (id, mission_id, title, status, dependencies) VALUES ('c_task2', 'm1', 'Task 2', 'PENDING', $1)", string(deps))
	if err != nil {
		t.Fatal(err)
	}
	tx.Commit(ctx)

	// TransitionState without deps
	err = sm.TransitionState(ctx, "c_task1", "agent1", "PENDING", "EXECUTING", "start")
	if err != nil {
		t.Fatalf("TransitionState failed: %v", err)
	}

	// TransitionState with deps
	err = sm.TransitionState(ctx, "c_task2", "agent1", "PENDING", "EXECUTING", "start")
	if err != nil {
		t.Fatalf("TransitionState with deps failed: %v", err)
	}

	status, err := sm.GetTaskStatus(ctx, "c_task1")
	if err != nil || status != "EXECUTING" {
		t.Fatalf("Expected EXECUTING, got %s", status)
	}

	err = sm.MarkTaskCompleted(ctx, "c_task1")
	if err != nil {
		t.Fatalf("MarkTaskCompleted failed: %v", err)
	}

	status, _ = sm.GetTaskStatus(ctx, "c_task1")
	if status != "COMPLETED" {
		t.Fatalf("Expected COMPLETED, got %s", status)
	}
}
