package statemachine

import (
	"context"
	"strings"
	"testing"
	"database/sql"

	"github.com/onehumancorp/mono/srcs/server/db"
	_ "modernc.org/sqlite"
)

// Helper to create an in-memory test provider
func setupTestDB(t *testing.T) db.Provider {
	sqliteDB, err := sql.Open("sqlite", "file::memory:?cache=shared")
	if err != nil {
		t.Fatalf("Failed to open sqlite db: %v", err)
	}

	dbProvider := db.NewSqliteProvider(sqliteDB)

	ctx := context.Background()
	tx, err := dbProvider.Begin(ctx)
	if err != nil {
		t.Fatalf("Failed to begin tx: %v", err)
	}

	_, err = tx.Exec(ctx, `
		CREATE TABLE shared_tasks (
			id TEXT PRIMARY KEY,
			organization_id TEXT NOT NULL,
			parent_plan_id TEXT,
			title TEXT NOT NULL,
			description TEXT,
			payload TEXT NOT NULL,
			status TEXT NOT NULL DEFAULT 'PENDING',
			priority INTEGER DEFAULT 0,
			agent_id TEXT,
			locked_until DATETIME,
			created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
			updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
		);
		CREATE TABLE state_machine_transitions (
			id TEXT PRIMARY KEY,
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
		t.Fatalf("Failed to create tables: %v", err)
	}

	err = tx.Commit(ctx)
	if err != nil {
		t.Fatalf("Failed to commit tables: %v", err)
	}

	return dbProvider
}

func TestStateMachine_Transition(t *testing.T) {
	ctx := context.Background()
	dbProvider := setupTestDB(t)
	defer dbProvider.Close()

	// Insert initial test task
	taskID := generateID()
	tx, _ := dbProvider.Begin(ctx)
	_, err := tx.Exec(ctx, `INSERT INTO shared_tasks (id, organization_id, title, payload, status) VALUES ($1, 'org1', 'Test Task', '{}', 'PENDING')`, taskID)
	tx.Commit(ctx)
	if err != nil {
		t.Fatalf("Failed to insert task: %v", err)
	}

	sm := NewStateMachine(dbProvider, nil, nil) // Passing nil Hub and nil Redis for tests

	// 1. Test Valid Transition: PENDING -> IN_PROGRESS
	err = sm.Transition(ctx, taskID, "SHARED_TASK", StateInProgress, "agent1", "Starting task")
	if err != nil {
		t.Errorf("Expected transition to succeed, got: %v", err)
	}

	// Verify DB state
	var currentStatus string
	tx, _ = dbProvider.Begin(ctx)
	err = tx.QueryRow(ctx, "SELECT status FROM shared_tasks WHERE id = $1", taskID).Scan(&currentStatus)
	tx.Commit(ctx)
	if err != nil || currentStatus != StateInProgress {
		t.Errorf("Expected status IN_PROGRESS, got: %s (err: %v)", currentStatus, err)
	}

	// Verify transition log
	var fromState, toState string
	tx, _ = dbProvider.Begin(ctx)
	err = tx.QueryRow(ctx, "SELECT from_state, to_state FROM state_machine_transitions WHERE entity_id = $1 ORDER BY occurred_at DESC LIMIT 1", taskID).Scan(&fromState, &toState)
	tx.Commit(ctx)
	if err != nil || fromState != StatePending || toState != StateInProgress {
		t.Errorf("Expected audit log PENDING -> IN_PROGRESS, got: %s -> %s (err: %v)", fromState, toState, err)
	}

	// 2. Test Invalid Transition: IN_PROGRESS -> ASSIGNED
	err = sm.Transition(ctx, taskID, "SHARED_TASK", StateAssigned, "agent1", "Invalid move")
	if err == nil || !strings.Contains(err.Error(), "invalid transition from IN_PROGRESS to ASSIGNED") {
		t.Errorf("Expected invalid transition error, got: %v", err)
	}

	// 3. Test Entity Not Found
	err = sm.Transition(ctx, "nonexistent", "SHARED_TASK", StateInProgress, "agent1", "Should fail")
	if err == nil {
		t.Errorf("Expected entity not found error")
	}

	// 4. Test Unsupported Entity Type
	err = sm.Transition(ctx, taskID, "UNKNOWN_TYPE", StateInProgress, "agent1", "Should fail")
	if err == nil {
		t.Errorf("Expected unsupported entity type error")
	}

	// 5. Test Same State Transition (No-op)
	err = sm.Transition(ctx, taskID, "SHARED_TASK", StateInProgress, "agent1", "Noop")
	if err != nil {
		t.Errorf("Expected no-op to succeed, got: %v", err)
	}
}
