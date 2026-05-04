package orchestration

import (
	"context"
	"time"
)

// AgentPresence represents the status of a specific agent.
type AgentPresence struct {
	AgentID string
	Status  string
}

// Subscription represents an active subscription to a mesh channel
type Subscription interface {
	Unsubscribe(ctx context.Context) error
}

// TeammateMesh provides a low-latency, real-time communication layer
// for agent coordination and event broadcasting.
type TeammateMesh interface {
	// Publish broadcasts a message payload to a specific topic
	Publish(ctx context.Context, topic string, payload []byte) error

	// Subscribe listens for messages on a topic and invokes the handler
	Subscribe(ctx context.Context, topic string, handler func(msg []byte)) (Subscription, error)

	// AcquireLock attempts to acquire a distributed lock for a specific key
	AcquireLock(ctx context.Context, key string, ttl time.Duration) (bool, error)

	// ReleaseLock releases a previously acquired lock
	ReleaseLock(ctx context.Context, key string) error

	// RegisterPresence broadcasts the agent's current status
	RegisterPresence(ctx context.Context, agentID string, status string) error

	// GetActiveAgents retrieves a list of all currently active agents
	GetActiveAgents(ctx context.Context) ([]AgentPresence, error)

	// Acknowledge confirms the processing of a specific message ID
	Acknowledge(ctx context.Context, messageID string) error
}
