package orchestration

import (
	"context"
	"fmt"
	"time"

	"github.com/google/uuid"
	"github.com/onehumancorp/mono/srcs/server/db"
)

// Valid transitions for KAIROS
var KAIROSValidTransitions = map[string][]string{
	"PENDING":     {"READY", "IN_PROGRESS"},
	"READY":       {"IN_PROGRESS"},
	"IN_PROGRESS": {"COMPLETED", "BLOCKED", "FAILED"},
	"BLOCKED":     {"IN_PROGRESS", "FAILED"},
}

// StateMachine for KAIROS task orchestration
type StateMachine struct {
	dbProvider   db.Provider
	lockProvider DistributedLockProvider
	mesh         MeshTransport
}

// NewStateMachine returns a new StateMachine
func NewStateMachine(dbProvider db.Provider, lockProvider DistributedLockProvider, mesh MeshTransport) *StateMachine {
	return &StateMachine{
		dbProvider:   dbProvider,
		lockProvider: lockProvider,
		mesh:         mesh,
	}
}

// Transition performs a state transition for a task, respecting distributed locks and allowed transitions.
func (sm *StateMachine) Transition(ctx context.Context, taskID, agentID, expectedFromState, toState, reason string) error {
	// 1. Acquire distributed lock
	var lock DistributedLock
	if sm.lockProvider != nil {
		lock = sm.lockProvider.NewLock(fmt.Sprintf("task:%s", taskID))
		if err := lock.Lock(ctx, 30*time.Second); err != nil {
			return fmt.Errorf("failed to acquire distributed lock for task %s: %w", taskID, err)
		}
		defer lock.Unlock(ctx) // Always unlock when done
	}

	// 3. Database operation
	tx, err := sm.dbProvider.Begin(ctx)
	if err != nil {
		return fmt.Errorf("failed to begin transaction: %w", err)
	}
	defer tx.Rollback(ctx)

	var currentState string
	var query string
	if sm.dbProvider.IsSQLite() {
		query = `SELECT status FROM shared_tasks WHERE id = $1`
	} else {
		query = `SELECT status FROM shared_tasks WHERE id = $1 FOR UPDATE SKIP LOCKED`
	}
	if err := tx.QueryRow(ctx, query, taskID).Scan(&currentState); err != nil {
		return fmt.Errorf("failed to get task %s: %w", taskID, err)
	}

	if currentState == toState {
		return nil
	}

	validTransitions, ok := KAIROSValidTransitions[currentState]
	if !ok {
		return fmt.Errorf("invalid fromState: %s", currentState)
	}
	isValid := false
	for _, s := range validTransitions {
		if s == toState {
			isValid = true
			break
		}
	}
	if !isValid {
		return fmt.Errorf("invalid transition from %s to %s", currentState, toState)
	}

	updateQuery := `UPDATE shared_tasks SET status = $1, agent_id = $2, updated_at = CURRENT_TIMESTAMP WHERE id = $3`
	if _, err := tx.Exec(ctx, updateQuery, toState, agentID, taskID); err != nil {
		return fmt.Errorf("failed to update task %s: %w", taskID, err)
	}

	transitionID := uuid.New().String()
	auditQuery := `
		INSERT INTO state_machine_transitions (id, entity_id, entity_type, from_state, to_state, agent_id, reason)
		VALUES ($1, $2, 'task', $3, $4, $5, $6)
	`
	if _, err := tx.Exec(ctx, auditQuery, transitionID, taskID, currentState, toState, agentID, reason); err != nil {
		return fmt.Errorf("failed to record transition audit log: %w", err)
	}

	if err := tx.Commit(ctx); err != nil {
		return fmt.Errorf("failed to commit transaction: %w", err)
	}

	if sm.mesh != nil {
		payload := []byte(fmt.Sprintf(`{"task_id":"%s","agent_id":"%s","from_state":"%s","to_state":"%s","reason":"%s"}`, taskID, agentID, currentState, toState, reason))
		_ = sm.mesh.BroadcastMeshEvent(ctx, "tasks", payload)
	}

	return nil
}

func (sm *StateMachine) TransitionToReady(ctx context.Context, taskID string) error {
	return sm.Transition(ctx, taskID, "", "PENDING", "READY", "Dependencies met")
}

func (sm *StateMachine) TransitionToInProgress(ctx context.Context, taskID, agentID string) error {
	return sm.Transition(ctx, taskID, agentID, "READY", "IN_PROGRESS", "Sub-agent assigned")
}

func (sm *StateMachine) TransitionToCompleted(ctx context.Context, taskID, agentID string) error {
	return sm.Transition(ctx, taskID, agentID, "IN_PROGRESS", "COMPLETED", "Success report")
}

func (sm *StateMachine) TransitionToBlocked(ctx context.Context, taskID, agentID string) error {
	return sm.Transition(ctx, taskID, agentID, "IN_PROGRESS", "BLOCKED", "Teammate Mesh negotiation request")
}

func (sm *StateMachine) TransitionToFailed(ctx context.Context, taskID, agentID string) error {
	return sm.Transition(ctx, taskID, agentID, "IN_PROGRESS", "FAILED", "Unrecoverable error")
}

func (sm *StateMachine) TransitionWithTx(ctx context.Context, tx db.Tx, taskID, entityType, toState, agentID, reason string) (func(), error) {
	// For compatibility with the old interface, this function will extract fromState from db and transition.
	// But we need to use the passed tx!

	// Since we are inside a tx, we shouldn't acquire the distributed lock here because the caller might hold it,
	// or we acquire it with a very short timeout. But `tasks.go` doesn't acquire the distributed lock, it just relies on it.
	// Actually, `tasks.go` used to call `TransitionWithTx(ctx, tx, taskID, "SHARED_TASK", statemachine.StateInProgress, agentID, "...")`.
	// Let's implement it.

	var currentState string
	var query string
	if sm.dbProvider.IsSQLite() {
		query = `SELECT status FROM shared_tasks WHERE id = $1`
	} else {
		query = `SELECT status FROM shared_tasks WHERE id = $1 FOR UPDATE SKIP LOCKED`
	}
	if err := tx.QueryRow(ctx, query, taskID).Scan(&currentState); err != nil {
		return nil, fmt.Errorf("failed to get task %s: %w", taskID, err)
	}

	if currentState == toState {
		return func() {}, nil
	}

	validTransitions, ok := KAIROSValidTransitions[currentState]
	if !ok {
		return nil, fmt.Errorf("invalid fromState: %s", currentState)
	}
	isValid := false
	for _, s := range validTransitions {
		if s == toState {
			isValid = true
			break
		}
	}
	if !isValid {
		return nil, fmt.Errorf("invalid transition from %s to %s", currentState, toState)
	}

	updateQuery := `UPDATE shared_tasks SET status = $1, agent_id = $2, updated_at = CURRENT_TIMESTAMP WHERE id = $3`
	if _, err := tx.Exec(ctx, updateQuery, toState, agentID, taskID); err != nil {
		return nil, fmt.Errorf("failed to update task %s: %w", taskID, err)
	}

	transitionID := uuid.New().String()
	auditQuery := `
		INSERT INTO state_machine_transitions (id, entity_id, entity_type, from_state, to_state, agent_id, reason)
		VALUES ($1, $2, 'task', $3, $4, $5, $6)
	`
	if _, err := tx.Exec(ctx, auditQuery, transitionID, taskID, currentState, toState, agentID, reason); err != nil {
		return nil, fmt.Errorf("failed to record transition audit log: %w", err)
	}

	broadcastFunc := func() {
		if sm.mesh != nil {
			payload := []byte(fmt.Sprintf(`{"task_id":"%s","agent_id":"%s","from_state":"%s","to_state":"%s","reason":"%s"}`, taskID, agentID, currentState, toState, reason))
			_ = sm.mesh.BroadcastMeshEvent(ctx, "tasks", payload)
		}
	}

	return broadcastFunc, nil
}
