package orchestration

import (
	"context"
	"encoding/json"
	"fmt"
	"strings"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
	"github.com/onehumancorp/mono/srcs/server/orchestration/statemachine"
	"github.com/redis/rueidis"
)

// DistributedStateMachine defines the interface for state machine
type DistributedStateMachine interface {
	Transition(ctx context.Context, entityID, entityType, toState, agentID, reason string) error
	GetState(ctx context.Context, entityID, entityType string) (string, error)
}

// Manager implements the DistributedStateMachine using MutexProvider for locking
type Manager struct {
	dbProvider    db.Provider
	redisClient   rueidis.Client
	mutexProvider MutexProvider
	mesh          MeshTransport
	inner         *statemachine.StateMachine
}

// NewDistributedStateMachine creates a new DistributedStateMachine manager
func NewDistributedStateMachine(ctx context.Context, provider db.Provider, redisClient rueidis.Client, mesh MeshTransport) (*Manager, error) {
	mp, err := NewMutexProvider(ctx, provider, redisClient)
	if err != nil {
		return nil, fmt.Errorf("failed to initialize mutex provider: %w", err)
	}

	m := &Manager{
		dbProvider:    provider,
		redisClient:   redisClient,
		mutexProvider: mp,
		mesh:          mesh,
	}

	m.inner = statemachine.NewStateMachine(provider, m.broadcastTransition)

	return m, nil
}

// broadcastTransition is called by the inner StateMachine after a successful transition commit
func (m *Manager) broadcastTransition(entityID string, payload map[string]interface{}) {
	if m.mesh == nil {
		return
	}

	// Add event metadata
	fullPayload := map[string]interface{}{
		"agent_id": payload["agent_id"],
		"action":   "STATE_TRANSITION",
		"status":   "SUCCESS",
		"payload": map[string]interface{}{
			"from":        payload["from_state"],
			"to":          payload["to_state"],
			"entity_id":   entityID,
			"entity_type": payload["entity_type"],
			"reason":      payload["reason"],
		},
	}

	data, err := json.Marshal(fullPayload)
	if err != nil {
		// Log error but don't fail the transition
		return
	}

	// Fire and forget via MeshTransport
	// Assuming topic is "tasks" for shared tasks or a general state_machine topic
	topic := "tasks"
	if entityType, ok := payload["entity_type"].(string); ok {
		if entityType == "SWARM_ULTRA_PLAN" {
			topic = "coordination"
		}
	}

	_ = m.mesh.BroadcastMeshEvent(context.Background(), topic, data)
}

// Transition performs a distributed state transition. It locks the entity using MutexProvider.
func (m *Manager) Transition(ctx context.Context, entityID, entityType, toState, agentID, reason string) error {
	// 1. Acquire distributed lock for this entity
	lockKey := fmt.Sprintf("statemachine:%s:%s", entityType, entityID)
	mu := m.mutexProvider.NewMutex(lockKey)

	// Try to acquire lock with 10 seconds TTL
	err := mu.Lock(ctx, 10*time.Second)
	if err != nil {
		return fmt.Errorf("failed to acquire distributed lock for entity %s: %w", entityID, err)
	}
	defer mu.Unlock(context.Background())

	// 2. Perform transition within a database transaction using the inner statemachine logic
	return m.inner.Transition(ctx, entityID, entityType, toState, agentID, reason)
}

// GetState retrieves the current state of an entity directly from the DB
func (m *Manager) GetState(ctx context.Context, entityID, entityType string) (string, error) {
	var currentState string
	var query string

	if entityType == "SHARED_TASK" {
		query = `SELECT status FROM shared_tasks WHERE id = $1`
		err := m.dbProvider.QueryRow(ctx, query, entityID).Scan(&currentState)
		if err != nil {
			if strings.Contains(err.Error(), "no rows in result set") {
				return "", fmt.Errorf("entity not found: %s", entityID)
			}
			return "", fmt.Errorf("failed to read current state: %w", err)
		}
		return currentState, nil
	}

	return "", fmt.Errorf("unsupported entity type: %s", entityType)
}
