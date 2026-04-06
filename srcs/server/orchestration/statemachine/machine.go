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


// TransitionWithTx performs a state transition using an existing transaction
func (sm *StateMachine) TransitionWithTx(ctx context.Context, tx db.Tx, entityID, entityType, toState, agentID, reason string) error {
	// Read current state
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
				return fmt.Errorf("entity not found: %s", entityID)
			}
			return fmt.Errorf("failed to read current state: %w", err)
		}
	} else {
		return fmt.Errorf("unsupported entity type: %s", entityType)
	}

	// Validate transition
	if !IsValidTransition(currentState, toState) {
		return fmt.Errorf("invalid transition from %s to %s", currentState, toState)
	}

	if currentState == toState {
		return nil // No change needed
	}

	// Update entity state
	if entityType == "SHARED_TASK" {
		var err error
		if toState == StateInProgress || toState == StateAssigned {
			updateQuery := `UPDATE shared_tasks SET status = $1, agent_id = $2, updated_at = CURRENT_TIMESTAMP WHERE id = $3`
			_, err = tx.Exec(ctx, updateQuery, toState, agentID, entityID)
		} else {
			updateQuery := `UPDATE shared_tasks SET status = $1, updated_at = CURRENT_TIMESTAMP WHERE id = $2`
			_, err = tx.Exec(ctx, updateQuery, toState, entityID)
		}
		if err != nil {
			return fmt.Errorf("failed to update entity state: %w", err)
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
		return fmt.Errorf("failed to record transition audit log: %w", err)
	}

	// Broadcast transition
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

	return nil
}

// Transition performs a state transition for the given entity
func (sm *StateMachine) Transition(ctx context.Context, entityID, entityType, toState, agentID, reason string) error {
	tx, err := sm.dbProvider.Begin(ctx)
	if err != nil {
		return fmt.Errorf("failed to begin transaction: %w", err)
	}
	defer tx.Rollback(ctx)
	err = sm.TransitionWithTx(ctx, tx, entityID, entityType, toState, agentID, reason)
	if err != nil {
		return err
	}
	return tx.Commit(ctx)
}

// generateID generates a pseudo-uuid for SQLite compatibility.
func generateID() string {
	b := make([]byte, 16)
	_, _ = rand.Read(b)
	return hex.EncodeToString(b[0:4]) + "-" + hex.EncodeToString(b[4:6]) + "-" + hex.EncodeToString(b[6:8]) + "-" + hex.EncodeToString(b[8:10]) + "-" + hex.EncodeToString(b[10:])
}


// BulkTransitionWithTx performs a state transition for multiple entities using an existing transaction
func (sm *StateMachine) BulkTransitionWithTx(ctx context.Context, tx db.Tx, entityIDs []string, entityType, toState, agentID, reason string) error {
	if len(entityIDs) == 0 {
		return nil
	}

	if entityType != "SHARED_TASK" {
		return fmt.Errorf("unsupported entity type: %s", entityType)
	}

	// For bulk operations, we assume the caller has already verified the transition is valid
	// and holds the necessary locks, as querying each individual state in bulk could be complex.
	// In PollTasks, the caller selects tasks that are PENDING, and we know PENDING -> IN_PROGRESS is valid.

	placeholders := make([]string, len(entityIDs))
	args := []interface{}{}

	if toState == StateInProgress || toState == StateAssigned {
		args = append(args, toState, agentID)
		for i, id := range entityIDs {
			placeholders[i] = fmt.Sprintf("$%d", i+3)
			args = append(args, id)
		}

		updateQuery := fmt.Sprintf(`UPDATE shared_tasks SET status = $1, agent_id = $2, updated_at = CURRENT_TIMESTAMP WHERE id IN (%s)`, strings.Join(placeholders, ", "))
		_, err := tx.Exec(ctx, updateQuery, args...)
		if err != nil {
			return fmt.Errorf("failed to bulk update entity state: %w", err)
		}
	} else {
		args = append(args, toState)
		for i, id := range entityIDs {
			placeholders[i] = fmt.Sprintf("$%d", i+2)
			args = append(args, id)
		}

		updateQuery := fmt.Sprintf(`UPDATE shared_tasks SET status = $1, updated_at = CURRENT_TIMESTAMP WHERE id IN (%s)`, strings.Join(placeholders, ", "))
		_, err := tx.Exec(ctx, updateQuery, args...)
		if err != nil {
			return fmt.Errorf("failed to bulk update entity state: %w", err)
		}
	}

	// Insert audit logs in bulk if possible or loop
	for _, id := range entityIDs {
		transitionID := generateID()
		auditQuery := `
			INSERT INTO state_machine_transitions (id, entity_id, entity_type, from_state, to_state, agent_id, reason)
			VALUES ($1, $2, $3, 'PENDING', $4, $5, $6)
		`
		// We hardcode 'PENDING' for from_state here because BulkTransition is primarily used for claiming new tasks
		// If needed for other states, we would need to pass in fromState or read them all.
		_, err := tx.Exec(ctx, auditQuery, transitionID, id, entityType, toState, agentID, reason)
		if err != nil {
			return fmt.Errorf("failed to record transition audit log: %w", err)
		}

		if sm.broadcast != nil {
			payload := map[string]interface{}{
				"entity_id":   id,
				"entity_type": entityType,
				"from_state":  "PENDING",
				"to_state":    toState,
				"agent_id":    agentID,
				"reason":      reason,
			}
			sm.broadcast(id, payload)
		}
	}

	return nil
}
