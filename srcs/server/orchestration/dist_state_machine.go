package orchestration

import (
	"context"
	"encoding/json"
	"errors"
	"fmt"
	"time"

	"github.com/google/uuid"
	"github.com/onehumancorp/mono/srcs/server/db"
	"github.com/redis/rueidis"
)

var (
	ErrInvalidTransition = errors.New("invalid state transition")
	ErrEntityNotFound    = errors.New("entity not found")
)

type DistributedStateMachine interface {
	Transition(ctx context.Context, entityType, entityID, event, agentID string) (string, error)
	GetState(ctx context.Context, entityType, entityID string) (string, error)
}

type Manager struct {
	dbProvider    db.Provider
	redisClient   rueidis.Client
	mutexProvider MutexProvider
	meshTransport MeshTransport
	rules         map[string]map[string]string
}

func NewDistributedStateMachineManager(ctx context.Context, dbProvider db.Provider, redisClient rueidis.Client, meshTransport MeshTransport) (*Manager, error) {
	mp, err := NewMutexProvider(ctx, dbProvider, redisClient)
	if err != nil {
		return nil, err
	}

	rules := map[string]map[string]string{
		"PENDING": {
			"Start": "IN_PROGRESS",
			"Decompose": "DECOMPOSING",
			"Cancel": "FAILED",
		},
		"IN_PROGRESS": {
			"Complete": "COMPLETED",
			"Block": "BLOCKED",
			"Fail": "FAILED",
			"Done": "DONE",
		},
		"BLOCKED": {
			"Unblock": "IN_PROGRESS",
			"Cancel": "FAILED",
		},
		"DECOMPOSING": {
			"DecompositionComplete": "EXECUTING_SUBTASKS",
		},
		"EXECUTING_SUBTASKS": {
			"SubTaskCompleted": "VERIFYING",
			"SubTaskFailed": "FAILED",
		},
		"VERIFYING": {
			"VerifySuccess": "COMPLETED",
			"VerifyFailed": "FAILED",
		},
		"COMPLETED": {
			"Acknowledge": "DONE",
		},
		"DONE": {
            // No transitions
        },
        "FAILED": {
            "Retry": "PENDING",
        },
	}

	return &Manager{
		dbProvider:    dbProvider,
		redisClient:   redisClient,
		mutexProvider: mp,
		meshTransport: meshTransport,
		rules:         rules,
	}, nil
}

func (m *Manager) getTableName(entityType string) (string, error) {
	switch entityType {
	case "shared_tasks":
		return "shared_tasks", nil
	case "swarm_ultra_plans":
		return "swarm_ultra_plans", nil
	default:
		return "", fmt.Errorf("unsupported entity type: %s", entityType)
	}
}

func (m *Manager) GetState(ctx context.Context, entityType, entityID string) (string, error) {
	tableName, err := m.getTableName(entityType)
	if err != nil {
		return "", err
	}
	query := fmt.Sprintf("SELECT status FROM %s WHERE id = $1", tableName)

	var status string
	err := m.dbProvider.QueryRow(ctx, query, entityID).Scan(&status)
	if err != nil {
		return "", fmt.Errorf("%w: %v", ErrEntityNotFound, err)
	}

	return status, nil
}

func (m *Manager) Transition(ctx context.Context, entityType, entityID, event, agentID string) (string, error) {
	lockKey := fmt.Sprintf("state_machine:%s:%s", entityType, entityID)
	mutex := m.mutexProvider.NewMutex(lockKey)

	// Acquire a distributed lock for the entityID
	if err := mutex.Lock(ctx, 10*time.Second); err != nil {
		return "", fmt.Errorf("failed to acquire lock: %w", err)
	}
	defer mutex.Unlock(ctx)

	// Fetch the current state from the database.
	currentState, err := m.GetState(ctx, entityType, entityID)
	if err != nil {
		return "", err
	}

	// Validate if the event is allowed for the current state.
	allowedEvents, ok := m.rules[currentState]
	if !ok {
		return "", fmt.Errorf("%w: state %s has no transitions", ErrInvalidTransition, currentState)
	}

	newState, ok := allowedEvents[event]
	if !ok {
		return "", fmt.Errorf("%w: event %s not allowed for state %s", ErrInvalidTransition, event, currentState)
	}

	// Update the state in the relevant table
	tableName, err := m.getTableName(entityType)
	if err != nil {
		return "", err
	}

	tx, err := m.dbProvider.Begin(ctx)
	if err != nil {
		return "", err
	}
	defer tx.Rollback(ctx)

	updateQuery := fmt.Sprintf("UPDATE %s SET status = $1 WHERE id = $2", tableName)
	_, err = tx.Exec(ctx, updateQuery, newState, entityID)
	if err != nil {
		return "", fmt.Errorf("failed to update state: %w", err)
	}

	// Log the transition in state_machine_transitions.
	transitionID := uuid.New().String()
	insertQuery := `
		INSERT INTO state_machine_transitions
		(id, entity_id, entity_type, from_state, to_state, agent_id, reason)
		VALUES ($1, $2, $3, $4, $5, $6, $7)
	`
	_, err = tx.Exec(ctx, insertQuery, transitionID, entityID, entityType, currentState, newState, agentID, event)
	if err != nil {
		return "", fmt.Errorf("failed to log transition: %w", err)
	}

	if err := tx.Commit(ctx); err != nil {
		return "", fmt.Errorf("failed to commit transaction: %w", err)
	}

	// Broadcast the change via the Teammate Mesh
	if m.meshTransport != nil {
		payload := map[string]string{
			"from": currentState,
			"to":   newState,
		}
		payloadBytes, _ := json.Marshal(payload)

		meshMsg := MeshMessage{
			AgentID:   agentID,
			Action:    "STATE_TRANSITION",
			Status:    "SUCCESS",
			Content:   string(payloadBytes),
			Timestamp: time.Now().UTC(),
		}

		msgBytes, _ := json.Marshal(meshMsg)
		_ = m.meshTransport.BroadcastMeshEvent(ctx, "state_transitions", msgBytes)
	}

	return newState, nil
}
