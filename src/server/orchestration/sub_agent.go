package orchestration

import (
	"context"
	"database/sql"
	"encoding/json"
	"fmt"
	"log"
	"time"
)

// SharedTask represents the payload structure expected from shared_tasks
type SharedTask struct {
	ID      string
	Payload json.RawMessage
}

// SubAgentSpawner handles the isolated spawning of sub-agents
type SubAgentSpawner interface {
	Spawn(ctx context.Context, task *SharedTask) error
	Monitor(ctx context.Context) error
}

// DefaultSubAgentSpawner implements SubAgentSpawner
type DefaultSubAgentSpawner struct {
	db       *sql.DB
	hub      MeshTransport
	isSQLite bool
	throttle chan struct{}
}

// MeshTransport defines the interface for teammate mesh interactions
type MeshTransport interface {
	PublishTaskBroadcast(topic string, event string, taskID string) error
}

// NewDefaultSubAgentSpawner creates a new instance of the spawner
func NewDefaultSubAgentSpawner(db *sql.DB, hub MeshTransport, isSQLite bool) *DefaultSubAgentSpawner {
	return &DefaultSubAgentSpawner{
		db:       db,
		hub:      hub,
		isSQLite: isSQLite,
		// Limit concurrency to 5 for standalone mode
		throttle: make(chan struct{}, 5),
	}
}

// Spawn initiates a new sub-agent for the delegated task
func (s *DefaultSubAgentSpawner) Spawn(ctx context.Context, task *SharedTask) error {
	if s.isSQLite {
		// Enforce local concurrency limit
		select {
		case s.throttle <- struct{}{}:
			defer func() { <-s.throttle }()
		case <-ctx.Done():
			return ctx.Err()
		}
	}

	// 1. Emit SUB_AGENT_SPAWNED
	if err := s.hub.PublishTaskBroadcast("mesh:tasks", "SUB_AGENT_SPAWNED", task.ID); err != nil {
		log.Printf("Failed to broadcast spawn event: %v", err)
	}

	// 2. Simulate sub-agent execution delay
	err := s.executeWithRetry(ctx, task)
	if err != nil {
		s.failTask(ctx, task.ID, err.Error())
		return err
	}

	// 3. Emit SUB_AGENT_COMPLETED
	if err := s.hub.PublishTaskBroadcast("mesh:tasks", "SUB_AGENT_COMPLETED", task.ID); err != nil {
		log.Printf("Failed to broadcast completion event: %v", err)
	}

	return nil
}

// executeWithRetry simulates an operation with exponential backoff
func (s *DefaultSubAgentSpawner) executeWithRetry(ctx context.Context, task *SharedTask) error {
	maxRetries := 3
	backoff := time.Millisecond * 10

	for attempt := 0; attempt < maxRetries; attempt++ {
		// Simulate work
		select {
		case <-time.After(backoff):
			// simulated success
			return nil
		case <-ctx.Done():
			return ctx.Err()
		}
	}
	return fmt.Errorf("sub-agent task failed after %d retries", maxRetries)
}

func (s *DefaultSubAgentSpawner) failTask(ctx context.Context, taskID, reason string) {
	// Simple stub for failing a task
	log.Printf("Task %s failed: %s", taskID, reason)
}

// Monitor observes long-running tasks
func (s *DefaultSubAgentSpawner) Monitor(ctx context.Context) error {
	// Stub for heartbeats logic
	return nil
}
