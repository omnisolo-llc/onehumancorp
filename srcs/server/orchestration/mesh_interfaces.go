package orchestration

import (
	"context"
	"time"
)

// Subscription represents an active subscription to a mesh channel.
type Subscription interface {
	Unsubscribe() error
}

// AgentPresence represents the current status of an agent.
type AgentPresence struct {
	AgentID string
	Status  string
}

// TeammateMesh provides a low-latency, real-time communication layer for agent coordination.
type TeammateMesh interface {
	// Publish broadcasts a message to the specified topic.
	Publish(ctx context.Context, topic string, payload []byte) error

	// Subscribe listens for messages on the specified topic.
	Subscribe(ctx context.Context, topic string, handler func(msg []byte)) (Subscription, error)

	// AcquireLock attempts to acquire a distributed lock.
	AcquireLock(ctx context.Context, key string, ttl time.Duration) (bool, error)

	// ReleaseLock releases a previously acquired distributed lock.
	ReleaseLock(ctx context.Context, key string) error

	// RegisterPresence registers the agent's current status (e.g., IDLE, WORKING).
	RegisterPresence(ctx context.Context, agentID string, status string) error

	// GetActiveAgents retrieves a list of all currently active agents.
	GetActiveAgents(ctx context.Context) ([]AgentPresence, error)
}
