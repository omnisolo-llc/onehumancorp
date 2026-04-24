package state

import (
	"context"
	"testing"
	"encoding/json"

	"github.com/onehumancorp/mono/src/server/db"
	_ "modernc.org/sqlite"
)

func TestStandaloneStateManager(t *testing.T) {
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

	sm := NewStandaloneStateManager(provider)

	// Insert test data
	tx, _ = provider.Begin(ctx)
	_, err = tx.Exec(ctx, "INSERT INTO swarm_tasks (id, mission_id, title, status) VALUES ('task1', 'm1', 'Task 1', 'PENDING')")
	if err != nil {
		t.Fatal(err)
	}

	// parent task
	_, err = tx.Exec(ctx, "INSERT INTO swarm_tasks (id, mission_id, title, status) VALUES ('parent_task', 'm1', 'Parent Task', 'COMPLETED')")
	if err != nil {
		t.Fatal(err)
	}

	// task with dependencies
	deps, _ := json.Marshal([]string{"parent_task"})
	_, err = tx.Exec(ctx, "INSERT INTO swarm_tasks (id, mission_id, title, status, dependencies) VALUES ('task2', 'm1', 'Task 2', 'PENDING', $1)", string(deps))
	if err != nil {
		t.Fatal(err)
	}
	tx.Commit(ctx)

	// Test ClaimTask
	task, err := sm.ClaimTask(ctx, "agent1")
	if err != nil {
		t.Fatalf("ClaimTask failed: %v", err)
	}
	if task.ID != "task1" && task.ID != "task2" {
		t.Fatalf("Expected task1 or task2, got %s", task.ID)
	}

	// Test TransitionState without deps
	err = sm.TransitionState(ctx, "task1", "agent1", "PENDING", "EXECUTING", "start")
	if err != nil {
		t.Fatalf("TransitionState failed: %v", err)
	}

	// Test TransitionState with deps
	err = sm.TransitionState(ctx, "task2", "agent1", "PENDING", "EXECUTING", "start")
	if err != nil {
		t.Fatalf("TransitionState with deps failed: %v", err)
	}

	status, err := sm.GetTaskStatus(ctx, "task1")
	if err != nil || status != "EXECUTING" {
		t.Fatalf("Expected EXECUTING, got %s, err: %v", status, err)
	}

	// Test MarkTaskCompleted
	err = sm.MarkTaskCompleted(ctx, "task1")
	if err != nil {
		t.Fatalf("MarkTaskCompleted failed: %v", err)
	}

	status, _ = sm.GetTaskStatus(ctx, "task1")
	if status != "COMPLETED" {
		t.Fatalf("Expected COMPLETED, got %s", status)
	}
}
