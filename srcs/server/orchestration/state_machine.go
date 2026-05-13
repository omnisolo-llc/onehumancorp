package orchestration

import (
	"context"
	"database/sql"
	"encoding/json"
	"errors"
	"fmt"
	"strings"
	"sync"
	"time"

	"github.com/google/uuid"
)

type TaskEvent string

const (
	EventSubTaskCompleted      TaskEvent = "SubTaskCompleted"
	EventSubTaskFailed         TaskEvent = "SubTaskFailed"
	EventDecompositionComplete TaskEvent = "DecompositionComplete"
)

// ValidTransitions defines the allowed state transitions
var ValidTransitions = map[string][]string{
	"PENDING":     {"EXECUTING", "DECOMPOSING", "FAILED"},
	"DECOMPOSING": {"EXECUTING", "FAILED"},
	"EXECUTING":   {"VERIFYING", "DONE", "FAILED"},
	"VERIFYING":   {"DONE", "FAILED"},
	"DONE":        {},
	"FAILED":      {},
}

type TaskStateMachine struct {
	db   *sql.DB
	mesh MeshTransport
	mu   sync.Mutex // For SQLite concurrent updates
}

func NewTaskStateMachine(db *sql.DB, mesh MeshTransport) *TaskStateMachine {
	return &TaskStateMachine{db: db, mesh: mesh}
}

// Transition performs a state transition and logs it to the audit table.
func (sm *TaskStateMachine) Transition(ctx context.Context, entityID, entityType, fromState, toState, agentID, reason string) error {
	// Validate transition
	valid := false
	for _, allowed := range ValidTransitions[fromState] {
		if allowed == toState {
			valid = true
			break
		}
	}
	if !valid {
		return fmt.Errorf("invalid state transition: %s -> %s", fromState, toState)
	}

	// Handle sqlite syntax difference for tests vs postgres
	var isSqlite bool
	err := sm.db.QueryRow("SELECT sqlite_version()").Scan(new(string))
	isSqlite = err == nil

	if isSqlite {
		sm.mu.Lock()
		defer sm.mu.Unlock()
	}

	tx, err := sm.db.BeginTx(ctx, nil)
	if err != nil {
		return err
	}
	defer tx.Rollback()

	// Record transition in audit log
	transID := uuid.New().String()
	_, err = tx.ExecContext(ctx, `
		INSERT INTO state_machine_transitions (id, entity_id, entity_type, from_state, to_state, agent_id, reason, created_at)
		VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
	`, transID, entityID, entityType, fromState, toState, agentID, reason, time.Now())

	if err != nil {
		return err
	}

	if err := tx.Commit(); err != nil {
		return err
	}

	// Broadcast the transition
	return sm.BroadcastTransition(ctx, entityID, entityType, fromState, toState, agentID, reason)
}

// BroadcastTransition publishes the state transition event to the Teammate Mesh
func (sm *TaskStateMachine) BroadcastTransition(ctx context.Context, entityID, entityType, fromState, toState, agentID, reason string) error {
	if sm.mesh == nil {
		return nil // Mesh not configured
	}

	eventData := map[string]string{
		"entity_id":   entityID,
		"entity_type": entityType,
		"from_state":  fromState,
		"to_state":    toState,
		"agent_id":    agentID,
		"reason":      reason,
	}

	bytesData, err := json.Marshal(eventData)
	if err != nil {
		return err
	}

	rawData := json.RawMessage(bytesData)

	msg := MeshMessage{
		AgentID:   agentID,
		EventType: "StateTransition",
		Data:      &rawData,
		Channel:   "mesh:tasks",
	}

	msgBytes, err := json.Marshal(msg)
	if err != nil {
		return err
	}

	return sm.mesh.Publish(ctx, "mesh:tasks", msgBytes)
}

func (sm *TaskStateMachine) ProcessEvent(ctx context.Context, taskID string, event TaskEvent) error {
	// Handle sqlite syntax difference for tests vs postgres
	var isSqlite bool
	err := sm.db.QueryRow("SELECT sqlite_version()").Scan(new(string))
	isSqlite = err == nil

	if isSqlite {
		sm.mu.Lock()
		defer sm.mu.Unlock()
	}

	tx, err := sm.db.BeginTx(ctx, nil)
	if err != nil {
		return err
	}
	defer tx.Rollback()

	// 1. Lock the parent task row
	var status string
	var workflowState sql.NullString
	query := "SELECT status, workflow_state FROM ohc_tasks WHERE id = $1"
	if !isSqlite {
		query += " FOR UPDATE"
	}
	err = tx.QueryRowContext(ctx, query, taskID).Scan(&status, &workflowState)
	if err != nil {
		if errors.Is(err, sql.ErrNoRows) {
			return fmt.Errorf("task not found: %s", taskID)
		}
		return err
	}

	if status == "DONE" || status == "FAILED" {
		return nil // Already in terminal state
	}

	switch event {
	case EventSubTaskCompleted:
		// Check if all subtasks are complete
		var incompleteCount int
		err = tx.QueryRowContext(ctx, "SELECT COUNT(*) FROM ohc_tasks WHERE parent_task_id = $1 AND status != 'DONE'", taskID).Scan(&incompleteCount)
		if err != nil {
			return err
		}

		if incompleteCount == 0 {
			_, err = tx.ExecContext(ctx, "UPDATE ohc_tasks SET status = 'VERIFYING' WHERE id = $1", taskID)
			if err != nil {
				return err
			}
		}

	case EventSubTaskFailed:
		_, err = tx.ExecContext(ctx, "UPDATE ohc_tasks SET status = 'FAILED' WHERE id = $1", taskID)
		if err != nil {
			return err
		}

	case EventDecompositionComplete:
		if status == "DECOMPOSING" {
			_, err = tx.ExecContext(ctx, "UPDATE ohc_tasks SET status = 'EXECUTING' WHERE id = $1", taskID)
			if err != nil {
				return err
			}
		}
	}

	_ = strings.ToLower(status)

	return tx.Commit()
}
