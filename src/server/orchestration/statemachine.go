package orchestration

import (
	"context"
	"fmt"
	"time"

	"github.com/google/uuid"
	"github.com/onehumancorp/mono/src/server/db"
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
func (sm *StateMachine) Transition(ctx context.Context, taskID, agentID, fromState, toState, reason string) error {
	// 1. Acquire distributed lock
	var lock DistributedLock
	if sm.lockProvider != nil {
		lock = sm.lockProvider.NewLock(fmt.Sprintf("task:%s", taskID))
		if err := lock.Lock(ctx, 30*time.Second); err != nil {
			return fmt.Errorf("failed to acquire distributed lock for task %s: %w", taskID, err)
		}
		defer lock.Unlock(ctx) // Always unlock when done
	}

	// 2. Validate transition
	validTransitions, ok := KAIROSValidTransitions[fromState]
	if !ok {
		return fmt.Errorf("invalid fromState: %s", fromState)
	}
	isValid := false
	for _, s := range validTransitions {
		if s == toState {
			isValid = true
			break
		}
	}
	if !isValid {
		return fmt.Errorf("invalid transition from %s to %s", fromState, toState)
	}

	// 3. Database operation
	tx, err := sm.dbProvider.Begin(ctx)
	if err != nil {
		return fmt.Errorf("failed to begin transaction: %w", err)
	}
	defer tx.Rollback(ctx)

	// In cloud mode, use FOR UPDATE SKIP LOCKED to prevent split-brain if lock fails or falls back
	var currentState string
	var query string
	if sm.dbProvider.IsSQLite() {
		query = `SELECT status FROM shared_tasks_master WHERE id = $1`
	} else {
		query = `SELECT status FROM shared_tasks_master WHERE id = $1 FOR UPDATE SKIP LOCKED`
	}
	if err := tx.QueryRow(ctx, query, taskID).Scan(&currentState); err != nil {
		return fmt.Errorf("failed to get task %s: %w", taskID, err)
	}

	if currentState != fromState {
		return fmt.Errorf("task %s is not in expected state %s (actual: %s)", taskID, fromState, currentState)
	}

	updateQuery := `UPDATE shared_tasks_master SET status = $1, assigned_agent_id = $2, updated_at = CURRENT_TIMESTAMP WHERE id = $3`
	if _, err := tx.Exec(ctx, updateQuery, toState, agentID, taskID); err != nil {
		return fmt.Errorf("failed to update task %s: %w", taskID, err)
	}

	// Record audit transition
	transitionID := uuid.New().String()
	auditQuery := `
		INSERT INTO state_machine_transitions (id, entity_id, entity_type, from_state, to_state, agent_id, reason)
		VALUES ($1, $2, 'task', $3, $4, $5, $6)
	`
	if _, err := tx.Exec(ctx, auditQuery, transitionID, taskID, fromState, toState, agentID, reason); err != nil {
		return fmt.Errorf("failed to record transition audit log: %w", err)
	}

	if err := tx.Commit(ctx); err != nil {
		return fmt.Errorf("failed to commit transaction: %w", err)
	}

	// 4. Publish to Teammate Mesh
	if sm.mesh != nil {
		payload := []byte(fmt.Sprintf(`{"task_id":"%s","agent_id":"%s","from_state":"%s","to_state":"%s","reason":"%s"}`, taskID, agentID, fromState, toState, reason))
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
