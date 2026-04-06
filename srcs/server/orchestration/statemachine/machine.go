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
	StateArchived   = "ARCHIVED"
)

// Valid transitions
var ValidTransitions = map[string][]string{
	StatePending:           {StateInProgress, StateAssigned},
	StateAssigned:          {StateExecuting, StateWaitingDelegation, StateTerminatedError},
	StateInProgress:        {StateReview, StateCompleted, StateFailed, StateTerminatedError, StateWaitingDelegation},
	StateExecuting:         {StateReview, StateSuccess, StateTerminatedError},
	StateWaitingDelegation: {StateExecuting, StateTerminatedError},
	StateReview:            {StateCompleted, StateSuccess, StateTerminatedError, StateExecuting, StateInProgress},
	StateCompleted:         {StateArchived},
	StateFailed:            {StateArchived},
	StateTerminatedError:   {StateArchived},
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

// BulkTransitionWithTx performs a state transition for multiple entities using an existing transaction
func (sm *StateMachine) BulkTransitionWithTx(ctx context.Context, tx db.Tx, entityIDs []string, entityType, toState, agentID, reason string) error {
	if len(entityIDs) == 0 {
		return nil
	}

	if entityType != "SHARED_TASK" {
		return fmt.Errorf("unsupported entity type: %s", entityType)
	}

	// Update entity states in bulk
	var updateQuery string
	var execArgs []interface{}

	placeholders := make([]string, len(entityIDs))
	if sm.dbProvider.IsSQLite() {
		for i := range entityIDs {
			placeholders[i] = "?"
			execArgs = append(execArgs, entityIDs[i])
		}

		if agentID != "" {
			updateQuery = fmt.Sprintf(`UPDATE shared_tasks SET status = ?, agent_id = ?, updated_at = CURRENT_TIMESTAMP WHERE id IN (%s)`, strings.Join(placeholders, ", "))
			execArgs = append([]interface{}{toState, agentID}, execArgs...)
		} else {
			updateQuery = fmt.Sprintf(`UPDATE shared_tasks SET status = ?, updated_at = CURRENT_TIMESTAMP WHERE id IN (%s)`, strings.Join(placeholders, ", "))
			execArgs = append([]interface{}{toState}, execArgs...)
		}
	} else {
		for i := range entityIDs {
			if agentID != "" {
				placeholders[i] = fmt.Sprintf("$%d", i+3)
			} else {
				placeholders[i] = fmt.Sprintf("$%d", i+2)
			}
			execArgs = append(execArgs, entityIDs[i])
		}

		if agentID != "" {
			updateQuery = fmt.Sprintf(`UPDATE shared_tasks SET status = $1, agent_id = $2, updated_at = CURRENT_TIMESTAMP WHERE id IN (%s)`, strings.Join(placeholders, ", "))
			execArgs = append([]interface{}{toState, agentID}, execArgs...)
		} else {
			updateQuery = fmt.Sprintf(`UPDATE shared_tasks SET status = $1, updated_at = CURRENT_TIMESTAMP WHERE id IN (%s)`, strings.Join(placeholders, ", "))
			execArgs = append([]interface{}{toState}, execArgs...)
		}
	}

	_, err := tx.Exec(ctx, updateQuery, execArgs...)
	if err != nil {
		return fmt.Errorf("failed to bulk update entity states: %w", err)
	}

	// Record audit logs
	for _, entityID := range entityIDs {
		transitionID := generateID()
		var auditQuery string
		if sm.dbProvider.IsSQLite() {
			auditQuery = `
				INSERT INTO state_machine_transitions (id, entity_id, entity_type, from_state, to_state, agent_id, reason)
				VALUES (?, ?, ?, 'PENDING', ?, ?, ?)
			`
		} else {
			auditQuery = `
				INSERT INTO state_machine_transitions (id, entity_id, entity_type, from_state, to_state, agent_id, reason)
				VALUES ($1, $2, $3, 'PENDING', $4, $5, $6)
			`
		}

		_, err := tx.Exec(ctx, auditQuery, transitionID, entityID, entityType, toState, agentID, reason)
		if err != nil {
			return fmt.Errorf("failed to record transition audit log: %w", err)
		}

		if sm.broadcast != nil {
			go func(eID string) {
				payload := map[string]interface{}{
					"entity_id":   eID,
					"entity_type": entityType,
					"from_state":  "PENDING", // assuming from PENDING for PollTasks optimization
					"to_state":    toState,
					"agent_id":    agentID,
					"reason":      reason,
				}
				sm.broadcast(eID, payload)
			}(entityID)
		}
	}

	return nil
}

// Transition performs a state transition for the given entity in an isolated transaction
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

	if err := tx.Commit(ctx); err != nil {
		return fmt.Errorf("failed to commit transaction: %w", err)
	}

	return nil
}

// TransitionWithTx performs a state transition using an existing transaction
func (sm *StateMachine) TransitionWithTx(ctx context.Context, tx db.Tx, entityID, entityType, toState, agentID, reason string) error {
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
		var updateQuery string
		var execArgs []interface{}

		if sm.dbProvider.IsSQLite() {
			if agentID != "" {
				updateQuery = `UPDATE shared_tasks SET status = ?, agent_id = ?, updated_at = CURRENT_TIMESTAMP WHERE id = ?`
			} else {
				updateQuery = `UPDATE shared_tasks SET status = ?, updated_at = CURRENT_TIMESTAMP WHERE id = ?`
			}
		} else {
			if agentID != "" {
				updateQuery = `UPDATE shared_tasks SET status = $1, agent_id = $2, updated_at = CURRENT_TIMESTAMP WHERE id = $3`
			} else {
				updateQuery = `UPDATE shared_tasks SET status = $1, updated_at = CURRENT_TIMESTAMP WHERE id = $2`
			}
		}

		if agentID != "" {
			execArgs = []interface{}{toState, agentID, entityID}
		} else {
			execArgs = []interface{}{toState, entityID}
		}

		_, err := tx.Exec(ctx, updateQuery, execArgs...)
		if err != nil {
			return fmt.Errorf("failed to update entity state: %w", err)
		}
	}

	// Record audit log
	transitionID := generateID()
	var auditQuery string
	if sm.dbProvider.IsSQLite() {
		auditQuery = `
			INSERT INTO state_machine_transitions (id, entity_id, entity_type, from_state, to_state, agent_id, reason)
			VALUES (?, ?, ?, ?, ?, ?, ?)
		`
	} else {
		auditQuery = `
			INSERT INTO state_machine_transitions (id, entity_id, entity_type, from_state, to_state, agent_id, reason)
			VALUES ($1, $2, $3, $4, $5, $6, $7)
		`
	}

	_, err := tx.Exec(ctx, auditQuery, transitionID, entityID, entityType, currentState, toState, agentID, reason)
	if err != nil {
		return fmt.Errorf("failed to record transition audit log: %w", err)
	}

	// Broadcast transition asynchronously (doesn't wait for commit, but for pub/sub it's fine)
	if sm.broadcast != nil {
		go func() {
			payload := map[string]interface{}{
				"entity_id":   entityID,
				"entity_type": entityType,
				"from_state":  currentState,
				"to_state":    toState,
				"agent_id":    agentID,
				"reason":      reason,
			}
			sm.broadcast(entityID, payload)
		}()
	}

	return nil
}

// generateID generates a pseudo-uuid for SQLite compatibility.
func generateID() string {
	b := make([]byte, 16)
	_, _ = rand.Read(b)
	return hex.EncodeToString(b[0:4]) + "-" + hex.EncodeToString(b[4:6]) + "-" + hex.EncodeToString(b[6:8]) + "-" + hex.EncodeToString(b[8:10]) + "-" + hex.EncodeToString(b[10:])
}
