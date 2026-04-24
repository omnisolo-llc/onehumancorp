package orchestration

import (
	"context"
	"strings"
	"sync"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/db"
	"github.com/stretchr/testify/assert"
	"github.com/stretchr/testify/require"
)

func TestStateMachine_Transitions(t *testing.T) {
	ctx := context.Background()
	provider := db.NewTestProvider(t)

	// Create tables
	tx, _ := provider.Begin(ctx)
	tx.Exec(ctx, `
		CREATE TABLE IF NOT EXISTS distributed_locks (lock_key TEXT PRIMARY KEY, owner_id TEXT NOT NULL, expires_at DATETIME NOT NULL);
		CREATE TABLE IF NOT EXISTS distributed_locks (lock_key TEXT PRIMARY KEY, owner_id TEXT NOT NULL, expires_at DATETIME NOT NULL);
		CREATE TABLE IF NOT EXISTS state_machine_transitions (id TEXT PRIMARY KEY, entity_id TEXT NOT NULL, entity_type TEXT NOT NULL, from_state TEXT NOT NULL, to_state TEXT NOT NULL, agent_id TEXT, reason TEXT, occurred_at DATETIME DEFAULT CURRENT_TIMESTAMP);
		CREATE TABLE shared_tasks (
			id TEXT PRIMARY KEY,
			organization_id TEXT NOT NULL,
			title TEXT NOT NULL,
			status TEXT NOT NULL DEFAULT 'PENDING',
			agent_id TEXT,
			updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
		)
	`)
	tx.Exec(ctx, `
		CREATE TABLE state_machine_transitions (
			id TEXT PRIMARY KEY,
			entity_id TEXT NOT NULL,
			entity_type TEXT NOT NULL,
			from_state TEXT NOT NULL,
			to_state TEXT NOT NULL,
			agent_id TEXT,
			reason TEXT,
			occurred_at DATETIME DEFAULT CURRENT_TIMESTAMP
		)
	`)
	tx.Exec(ctx, `INSERT INTO shared_tasks (id, organization_id, title, status) VALUES ('task1', 'org1', 'Test', 'PENDING')`)
	tx.Commit(ctx)

	lockProvider, _ := NewDistributedLockProvider(ctx, provider, nil)
	sm := NewStateMachine(provider, lockProvider, nil)

	// Valid transition PENDING -> READY
	err := sm.TransitionToReady(ctx, "task1")
	require.NoError(t, err)

	// Valid transition READY -> IN_PROGRESS
	err = sm.TransitionToInProgress(ctx, "task1", "agent1")
	require.NoError(t, err)

	// Verify state
	var status, assignedAgent string
	provider.QueryRow(ctx, "SELECT status, agent_id FROM shared_tasks WHERE id = 'task1'").Scan(&status, &assignedAgent)
	assert.Equal(t, "IN_PROGRESS", status)
	assert.Equal(t, "agent1", assignedAgent)

	// Invalid transition IN_PROGRESS -> PENDING
	err = sm.Transition(ctx, "task1", "agent1", "IN_PROGRESS", "PENDING", "Invalid")
	require.Error(t, err)
	assert.True(t, strings.Contains(err.Error(), "invalid transition"))

	// Valid transition IN_PROGRESS -> COMPLETED
	err = sm.TransitionToCompleted(ctx, "task1", "agent1")
	require.NoError(t, err)
}

func TestStateMachine_ConcurrentTransitions(t *testing.T) {
	ctx := context.Background()
	provider := db.NewTestProvider(t)

	// Create tables
	tx, _ := provider.Begin(ctx)
	tx.Exec(ctx, `
		CREATE TABLE IF NOT EXISTS distributed_locks (lock_key TEXT PRIMARY KEY, owner_id TEXT NOT NULL, expires_at DATETIME NOT NULL);
		CREATE TABLE IF NOT EXISTS distributed_locks (lock_key TEXT PRIMARY KEY, owner_id TEXT NOT NULL, expires_at DATETIME NOT NULL);
		CREATE TABLE IF NOT EXISTS state_machine_transitions (id TEXT PRIMARY KEY, entity_id TEXT NOT NULL, entity_type TEXT NOT NULL, from_state TEXT NOT NULL, to_state TEXT NOT NULL, agent_id TEXT, reason TEXT, occurred_at DATETIME DEFAULT CURRENT_TIMESTAMP);
		CREATE TABLE shared_tasks (
			id TEXT PRIMARY KEY,
			organization_id TEXT NOT NULL,
			title TEXT NOT NULL,
			status TEXT NOT NULL DEFAULT 'PENDING',
			agent_id TEXT,
			updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
		)
	`)
	tx.Exec(ctx, `
		CREATE TABLE state_machine_transitions (
			id TEXT PRIMARY KEY,
			entity_id TEXT NOT NULL,
			entity_type TEXT NOT NULL,
			from_state TEXT NOT NULL,
			to_state TEXT NOT NULL,
			agent_id TEXT,
			reason TEXT,
			occurred_at DATETIME DEFAULT CURRENT_TIMESTAMP
		)
	`)
	tx.Exec(ctx, `INSERT INTO shared_tasks (id, organization_id, title, status) VALUES ('task2', 'org1', 'Test', 'READY')`)
	tx.Commit(ctx)

	lockProvider, _ := NewDistributedLockProvider(ctx, provider, nil)
	sm := NewStateMachine(provider, lockProvider, nil)

	var wg sync.WaitGroup
	errs := make(chan error, 10)

	// Try to start the task concurrently 10 times
	for i := 0; i < 10; i++ {
		wg.Add(1)
		go func(agentID string) {
			defer wg.Done()
			err := sm.TransitionToInProgress(ctx, "task2", agentID)
			errs <- err
		}("agent" + string(rune('A'+i)))
	}

	wg.Wait()
	close(errs)

	successCount := 0
	for err := range errs {
		if err == nil {
			successCount++
		} else { t.Log("Error:", err) }
	}

	assert.Equal(t, 1, successCount)
}

func TestStateMachine_AdditionalTransitions(t *testing.T) {
	ctx := context.Background()
	provider := db.NewTestProvider(t)

	tx, _ := provider.Begin(ctx)
	tx.Exec(ctx, `
		CREATE TABLE IF NOT EXISTS distributed_locks (lock_key TEXT PRIMARY KEY, owner_id TEXT NOT NULL, expires_at DATETIME NOT NULL);
		CREATE TABLE IF NOT EXISTS distributed_locks (lock_key TEXT PRIMARY KEY, owner_id TEXT NOT NULL, expires_at DATETIME NOT NULL);
		CREATE TABLE IF NOT EXISTS state_machine_transitions (id TEXT PRIMARY KEY, entity_id TEXT NOT NULL, entity_type TEXT NOT NULL, from_state TEXT NOT NULL, to_state TEXT NOT NULL, agent_id TEXT, reason TEXT, occurred_at DATETIME DEFAULT CURRENT_TIMESTAMP);
		CREATE TABLE shared_tasks (
			id TEXT PRIMARY KEY,
			organization_id TEXT NOT NULL,
			title TEXT NOT NULL,
			status TEXT NOT NULL DEFAULT 'PENDING',
			agent_id TEXT,
			updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
		)
	`)
	tx.Exec(ctx, `
		CREATE TABLE state_machine_transitions (
			id TEXT PRIMARY KEY,
			entity_id TEXT NOT NULL,
			entity_type TEXT NOT NULL,
			from_state TEXT NOT NULL,
			to_state TEXT NOT NULL,
			agent_id TEXT,
			reason TEXT,
			occurred_at DATETIME DEFAULT CURRENT_TIMESTAMP
		)
	`)
	tx.Exec(ctx, `INSERT INTO shared_tasks (id, organization_id, title, status) VALUES ('task3', 'org1', 'Test', 'IN_PROGRESS')`)
	tx.Exec(ctx, `INSERT INTO shared_tasks (id, organization_id, title, status) VALUES ('task4', 'org1', 'Test', 'IN_PROGRESS')`)
	tx.Exec(ctx, `INSERT INTO shared_tasks (id, organization_id, title, status) VALUES ('task5', 'org1', 'Test', 'BLOCKED')`)
	tx.Commit(ctx)

	lockProvider, _ := NewDistributedLockProvider(ctx, provider, nil)
	sm := NewStateMachine(provider, lockProvider, nil)

	err := sm.TransitionToBlocked(ctx, "task3", "agent1")
	require.NoError(t, err)

	var status string
	provider.QueryRow(ctx, "SELECT status FROM shared_tasks WHERE id = 'task3'").Scan(&status)
	assert.Equal(t, "BLOCKED", status)

	err = sm.TransitionToFailed(ctx, "task4", "agent1")
	require.NoError(t, err)

	provider.QueryRow(ctx, "SELECT status FROM shared_tasks WHERE id = 'task4'").Scan(&status)
	assert.Equal(t, "FAILED", status)

	err = sm.Transition(ctx, "task5", "agent1", "BLOCKED", "IN_PROGRESS", "Resuming")
	require.NoError(t, err)

	provider.QueryRow(ctx, "SELECT status FROM shared_tasks WHERE id = 'task5'").Scan(&status)
	assert.Equal(t, "IN_PROGRESS", status)
}
