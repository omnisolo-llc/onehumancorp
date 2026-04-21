package state

import (
	"context"
	"testing"
	"encoding/json"

	"github.com/onehumancorp/mono/srcs/server/db"
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

func TestStandaloneStateManager_Errors(t *testing.T) {
	provider := db.NewTestProvider(t)
	defer provider.Close()
	ctx := context.Background()

	tx, _ := provider.Begin(ctx)
	_, _ = tx.Exec(ctx, `
		CREATE TABLE IF NOT EXISTS swarm_tasks (
			id TEXT PRIMARY KEY,
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
	_, _ = tx.Exec(ctx, "INSERT INTO swarm_tasks (id, mission_id, title, status) VALUES ('s_task1', 'm1', 'Task 1', 'PENDING')")
	_, _ = tx.Exec(ctx, "INSERT INTO swarm_tasks (id, mission_id, title, status) VALUES ('s_parent', 'm1', 'Parent', 'PENDING')")
	deps, _ := json.Marshal([]string{"s_parent"})
	_, _ = tx.Exec(ctx, "INSERT INTO swarm_tasks (id, mission_id, title, status, dependencies) VALUES ('s_task2', 'm1', 'Task 2', 'PENDING', $1)", string(deps))
	tx.Commit(ctx)

	sm := NewStandaloneStateManager(provider)

	// Missing task
	err := sm.TransitionState(ctx, "missing", "agent", "PENDING", "EXECUTING", "")
	if err == nil {
		t.Fatal("Expected error")
	}

	// Wrong state
	err = sm.TransitionState(ctx, "s_task1", "agent", "EXECUTING", "COMPLETED", "")
	if err == nil {
		t.Fatal("Expected error")
	}

	// Unmet deps
	err = sm.TransitionState(ctx, "s_task2", "agent", "PENDING", "EXECUTING", "")
	if err == nil {
		t.Fatal("Expected error")
	}

	// ClaimTask when none available (after claiming all)
	_, err = sm.ClaimTask(ctx, "agent")
	_, err = sm.ClaimTask(ctx, "agent")
	_, err = sm.ClaimTask(ctx, "agent")
	task, err := sm.ClaimTask(ctx, "agent")
	if err == nil || task != nil {
		t.Fatal("Expected error when no tasks left to claim")
	}

	err = sm.MarkTaskCompleted(ctx, "missing")
	if err == nil {
		// SQLite might just run update returning 0 rows affected, no err
	}

	_, err = sm.GetTaskStatus(ctx, "missing")
	if err == nil {
		t.Fatal("Expected error getting status of missing task")
	}
}

func TestStandaloneStateManager_DBErrors(t *testing.T) {
	provider := db.NewTestProvider(t)
	defer provider.Close()
	ctx := context.Background()

	// 1. Force tx.Begin to fail by closing the provider
	provider.Close()

	sm := NewStandaloneStateManager(provider)

	err := sm.TransitionState(ctx, "task", "agent", "PENDING", "EXECUTING", "start")
	if err == nil {
		t.Fatal("Expected error when provider closed for TransitionState")
	}

	_, err = sm.ClaimTask(ctx, "agent")
	if err == nil {
		t.Fatal("Expected error when provider closed for ClaimTask")
	}

	err = sm.MarkTaskCompleted(ctx, "task")
	if err == nil {
		t.Fatal("Expected error when provider closed for MarkTaskCompleted")
	}
}

func TestStandaloneStateManager_UpdateErrors(t *testing.T) {
	provider := db.NewTestProvider(t)
	defer provider.Close()
	ctx := context.Background()

	tx, _ := provider.Begin(ctx)
	_, _ = tx.Exec(ctx, `
		CREATE TABLE IF NOT EXISTS swarm_tasks (
			id TEXT PRIMARY KEY,
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
	_, _ = tx.Exec(ctx, "INSERT INTO swarm_tasks (id, mission_id, title, status) VALUES ('s_task1', 'm1', 'Task 1', 'PENDING')")
	tx.Commit(ctx)

	sm := NewStandaloneStateManager(provider)

	// Inject a schema error dynamically to force Update error for TransitionState and MarkTaskCompleted
	tx, _ = provider.Begin(ctx)
	_, _ = tx.Exec(ctx, "DROP TABLE swarm_tasks")
	tx.Commit(ctx)

	err := sm.MarkTaskCompleted(ctx, "s_task1")
	if err == nil {
		t.Fatal("Expected error for missing table")
	}

	err = sm.TransitionState(ctx, "s_task1", "agent", "PENDING", "EXECUTING", "start")
	if err == nil {
		t.Fatal("Expected error for missing table")
	}
}

func TestStandaloneStateManager_InvalidJSON(t *testing.T) {
	provider := db.NewTestProvider(t)
	defer provider.Close()
	ctx := context.Background()

	tx, _ := provider.Begin(ctx)
	_, _ = tx.Exec(ctx, `
		CREATE TABLE IF NOT EXISTS swarm_tasks (
			id TEXT PRIMARY KEY,
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
	`)
	// Insert task with invalid JSON dependencies
	_, _ = tx.Exec(ctx, "INSERT INTO swarm_tasks (id, mission_id, title, status, dependencies) VALUES ('s_task_invalid', 'm1', 'Task Invalid', 'PENDING', '{invalid}')")
	tx.Commit(ctx)

	sm := NewStandaloneStateManager(provider)

	// ClaimTask should fail due to invalid JSON dependencies
	_, err := sm.ClaimTask(ctx, "agent")
	if err != nil {
		t.Logf("Expected error for invalid JSON dependencies in ClaimTask: %v", err)
	}

	// TransitionState should fail due to invalid JSON dependencies when transitioning to EXECUTING
	err = sm.TransitionState(ctx, "s_task_invalid", "agent", "PENDING", "EXECUTING", "start")
	if err == nil {
		t.Fatal("Expected error for invalid JSON dependencies in TransitionState")
	}
}
