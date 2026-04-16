package statemachine

import (
	"context"
	"crypto/rand"
	"encoding/hex"
	"fmt"
	"strings"

	"github.com/onehumancorp/mono/srcs/server/db"
)

// State constants
const (
	StatePending           = "PENDING"
	StateAssigned          = "ASSIGNED"
	StateExecuting         = "EXECUTING"
	StateWaitingDelegation = "WAITING_DELEGATION"
	StateReview            = "REVIEW"
	StateSuccess           = "SUCCESS"
	StateTerminatedError   = "TERMINATED_ERROR"

	StateInProgress = "IN_PROGRESS"
	StateCompleted  = "COMPLETED"
	StateFailed     = "FAILED"
)

// Valid transitions
var ValidTransitions = map[string][]string{
	StatePending:           {StateInProgress, StateAssigned},
	StateAssigned:          {StateExecuting, StateWaitingDelegation, StateTerminatedError},
	StateInProgress:        {StateReview, StateCompleted, StateFailed, StateTerminatedError, StateWaitingDelegation},
	StateExecuting:         {StateReview, StateSuccess, StateTerminatedError},
	StateWaitingDelegation: {StateExecuting, StateTerminatedError},
	StateReview:            {StateCompleted, StateSuccess, StateTerminatedError, StateExecuting, StateInProgress},
}

// StateMachine manages state transitions for entities
type StateMachine struct {
	dbProvider db.Provider
	broadcast  func(string, map[string]interface{})
}

// NewStateMachine creates a new state machine
func NewStateMachine(dbProvider db.Provider, broadcast func(string, map[string]interface{})) *StateMachine {
	return &StateMachine{
		dbProvider: dbProvider,
		broadcast:  broadcast,
	}
}

// IsValidTransition checks if the transition from fromState to toState is valid
func IsValidTransition(fromState, toState string) bool {
	if fromState == toState {
		return true // No-op transition
	}
	validNextStates, ok := ValidTransitions[fromState]
	if !ok {
		return false
	}
	for _, state := range validNextStates {
		if state == toState {
			return true
		}
	}
	return false
}

// Transition performs a state transition for the given entity
func (sm *StateMachine) Transition(ctx context.Context, entityID, entityType, toState, agentID, reason string) error {
	tx, err := sm.dbProvider.Begin(ctx)
	if err != nil {
		return fmt.Errorf("failed to begin transaction: %w", err)
	}
	defer tx.Rollback(ctx)

	broadcastFunc, err := sm.TransitionWithTx(ctx, tx, entityID, entityType, toState, agentID, reason)
	if err != nil {
		return err
	}

	if err := tx.Commit(ctx); err != nil {
		return fmt.Errorf("failed to commit transaction: %w", err)
	}

	// Broadcast transition
	if broadcastFunc != nil {
		broadcastFunc()
	}

	return nil
}

// TransitionWithTx performs a state transition using an existing transaction and returns a broadcast closure and an error.
// The caller MUST call the returned function after successfully committing the transaction to ensure broadcasts
// are not sent for rolled-back transactions.
func (sm *StateMachine) TransitionWithTx(ctx context.Context, tx db.Tx, entityID, entityType, toState, agentID, reason string) (func(), error) {
	// Acquire lock and read current state
	var currentState string
	var query string

	if entityType == "SHARED_TASK" {
		if sm.dbProvider.IsSQLite() {
			query = `SELECT status FROM shared_tasks WHERE id = $1`
		} else {
			query = `SELECT status FROM shared_tasks WHERE id = $1 FOR UPDATE`
		}

		err := tx.QueryRow(ctx, query, entityID).Scan(&currentState)
		if err != nil {
			if strings.Contains(err.Error(), "no rows in result set") {
				return nil, fmt.Errorf("entity not found: %s", entityID)
			}
			return nil, fmt.Errorf("failed to read current state: %w", err)
		}
	} else {
		return nil, fmt.Errorf("unsupported entity type: %s", entityType)
	}

	// Validate transition
	if !IsValidTransition(currentState, toState) {
		return nil, fmt.Errorf("invalid transition from %s to %s", currentState, toState)
	}

	if currentState == toState {
		return func() {}, nil // No change needed
	}

	// Update entity state
	if entityType == "SHARED_TASK" {
		updateQuery := `UPDATE shared_tasks SET status = $1, agent_id = $2, updated_at = CURRENT_TIMESTAMP WHERE id = $3`
		_, err := tx.Exec(ctx, updateQuery, toState, agentID, entityID)
		if err != nil {
			return nil, fmt.Errorf("failed to update entity state: %w", err)
		}
	}

	// Record audit log
	transitionID := generateID()
	auditQuery := `
		INSERT INTO state_machine_transitions (id, entity_id, entity_type, from_state, to_state, agent_id, reason)
		VALUES ($1, $2, $3, $4, $5, $6, $7)
	`
	_, err := tx.Exec(ctx, auditQuery, transitionID, entityID, entityType, currentState, toState, agentID, reason)
	if err != nil {
		return nil, fmt.Errorf("failed to record transition audit log: %w", err)
	}

	broadcastFunc := func() {
		if sm.broadcast != nil {
			payload := map[string]interface{}{
				"entity_id":   entityID,
				"entity_type": entityType,
				"from_state":  currentState,
				"to_state":    toState,
				"agent_id":    agentID,
				"reason":      reason,
			}
			sm.broadcast(entityID, payload)
		}
	}

	return broadcastFunc, nil
}

// generateID generates a pseudo-uuid for SQLite compatibility.
func generateID() string {
	b := make([]byte, 16)
	_, _ = rand.Read(b)
	return hex.EncodeToString(b[0:4]) + "-" + hex.EncodeToString(b[4:6]) + "-" + hex.EncodeToString(b[6:8]) + "-" + hex.EncodeToString(b[8:10]) + "-" + hex.EncodeToString(b[10:])
}
