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

func (sm *StateMachine) Transition(ctx context.Context, entityID, entityType, toState, agentID, reason string) error {
	tx, err := sm.dbProvider.Begin(ctx)
	if err != nil {
		return fmt.Errorf("failed to begin transaction: %w", err)
	}
	defer tx.Rollback(ctx)

	var oldState string
	oldState, err = sm.TransitionWithTxReturnState(ctx, tx, entityID, entityType, toState, agentID, reason)
	if err != nil {
		return err
	}

	if err := tx.Commit(ctx); err != nil {
		return fmt.Errorf("failed to commit transaction: %w", err)
	}

	// Broadcast transition
	if sm.broadcast != nil {
		payload := map[string]interface{}{
			"entity_id":   entityID,
			"entity_type": entityType,
			"from_state":  oldState,
			"to_state":    toState,
			"agent_id":    agentID,
			"reason":      reason,
		}
		sm.broadcast(entityID, payload)
	}

	return nil
}

// TransitionWithTx performs a state transition using an existing database transaction.
func (sm *StateMachine) TransitionWithTx(ctx context.Context, tx db.Tx, entityID, entityType, toState, agentID, reason string) error {
	_, err := sm.TransitionWithTxReturnState(ctx, tx, entityID, entityType, toState, agentID, reason)
	return err
}

func (sm *StateMachine) TransitionWithTxReturnState(ctx context.Context, tx db.Tx, entityID, entityType, toState, agentID, reason string) (string, error) {
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
				return "", fmt.Errorf("entity not found: %s", entityID)
			}
			return "", fmt.Errorf("failed to read current state: %w", err)
		}
	} else {
		return "", fmt.Errorf("unsupported entity type: %s", entityType)
	}

	if !IsValidTransition(currentState, toState) {
		return "", fmt.Errorf("invalid transition from %s to %s", currentState, toState)
	}

	if currentState == toState {
		return currentState, nil
	}

	if entityType == "SHARED_TASK" {
		updateQuery := `UPDATE shared_tasks SET status = $1, updated_at = CURRENT_TIMESTAMP WHERE id = $2`
		_, err := tx.Exec(ctx, updateQuery, toState, entityID)
		if err != nil {
			return "", fmt.Errorf("failed to update entity state: %w", err)
		}
	}

	transitionID := generateID()
	auditQuery := `
		INSERT INTO state_machine_transitions (id, entity_id, entity_type, from_state, to_state, agent_id, reason)
		VALUES ($1, $2, $3, $4, $5, $6, $7)
	`
	_, err := tx.Exec(ctx, auditQuery, transitionID, entityID, entityType, currentState, toState, agentID, reason)
	if err != nil {
		return "", fmt.Errorf("failed to record transition audit log: %w", err)
	}

	return currentState, nil
}

// BulkTransitionWithTx performs a state transition for multiple entities using an existing database transaction.
func (sm *StateMachine) BulkTransitionWithTx(ctx context.Context, tx db.Tx, entityIDs []string, entityType, toState, agentID, reason string) error {
	if len(entityIDs) == 0 {
		return nil
	}

	if entityType != "SHARED_TASK" {
		return fmt.Errorf("unsupported entity type: %s", entityType)
	}

	// 1. Fetch current states
	placeholders := make([]string, len(entityIDs))
	args := []interface{}{}
	for i, id := range entityIDs {
		placeholders[i] = fmt.Sprintf("$%d", i+1)
		args = append(args, id)
	}

	var query string
	if sm.dbProvider.IsSQLite() {
		query = fmt.Sprintf("SELECT id, status FROM shared_tasks WHERE id IN (%s)", strings.Join(placeholders, ", "))
	} else {
		query = fmt.Sprintf("SELECT id, status FROM shared_tasks WHERE id IN (%s) FOR UPDATE", strings.Join(placeholders, ", "))
	}

	rows, err := tx.Query(ctx, query, args...)
	if err != nil {
		return fmt.Errorf("failed to read current states: %w", err)
	}
	defer rows.Close()

	currentStates := make(map[string]string)
	for rows.Next() {
		var id, status string
		if err := rows.Scan(&id, &status); err != nil {
			return fmt.Errorf("failed to scan state: %w", err)
		}
		currentStates[id] = status
	}
	if err := rows.Err(); err != nil {
		return fmt.Errorf("row iteration error: %w", err)
	}

	// 2. Validate transitions and build lists
	var validIDs []string
	var validCurrentStates []string

	for _, id := range entityIDs {
		currentState, exists := currentStates[id]
		if !exists {
			return fmt.Errorf("entity not found: %s", id)
		}
		if !IsValidTransition(currentState, toState) {
			return fmt.Errorf("invalid transition from %s to %s for entity %s", currentState, toState, id)
		}
		if currentState != toState {
			validIDs = append(validIDs, id)
			validCurrentStates = append(validCurrentStates, currentState)
		}
	}

	if len(validIDs) == 0 {
		return nil
	}

	// 3. Update states
	validPlaceholders := make([]string, len(validIDs))
	updateArgs := []interface{}{toState}
	for i, id := range validIDs {
		validPlaceholders[i] = fmt.Sprintf("$%d", i+2)
		updateArgs = append(updateArgs, id)
	}

	updateQuery := fmt.Sprintf(`UPDATE shared_tasks SET status = $1, updated_at = CURRENT_TIMESTAMP WHERE id IN (%s)`, strings.Join(validPlaceholders, ", "))
	_, err = tx.Exec(ctx, updateQuery, updateArgs...)
	if err != nil {
		return fmt.Errorf("failed to update entity states: %w", err)
	}

	// 4. Record audit logs
	// SQLite limits the number of variables in a query, but for a typical batch it's fine.
	// We'll construct a bulk insert query.
	var valueStrings []string
	var insertArgs []interface{}
	for i, id := range validIDs {
		transitionID := generateID()
		offset := i * 7
		valueStrings = append(valueStrings, fmt.Sprintf("($%d, $%d, $%d, $%d, $%d, $%d, $%d)", offset+1, offset+2, offset+3, offset+4, offset+5, offset+6, offset+7))
		insertArgs = append(insertArgs, transitionID, id, entityType, validCurrentStates[i], toState, agentID, reason)
	}

	auditQuery := fmt.Sprintf(`
		INSERT INTO state_machine_transitions (id, entity_id, entity_type, from_state, to_state, agent_id, reason)
		VALUES %s
	`, strings.Join(valueStrings, ","))

	_, err = tx.Exec(ctx, auditQuery, insertArgs...)
	if err != nil {
		return fmt.Errorf("failed to record transition audit logs: %w", err)
	}

	return nil
}

// generateID generates a pseudo-uuid for SQLite compatibility.
func generateID() string {
	b := make([]byte, 16)
	_, _ = rand.Read(b)
	return hex.EncodeToString(b[0:4]) + "-" + hex.EncodeToString(b[4:6]) + "-" + hex.EncodeToString(b[6:8]) + "-" + hex.EncodeToString(b[8:10]) + "-" + hex.EncodeToString(b[10:])
}
