package mesh

import (
	"context"
	"time"
)

// Subscription represents an active subscription to a mesh topic.
type Subscription interface {
	// Close terminates the subscription.
	Close() error
}

// AgentPresence represents the current status of an agent.
type AgentPresence struct {
	AgentID string    `json:"agent_id"`
	Status  string    `json:"status"`
	Updated time.Time `json:"updated"`
}

// TeammateMesh provides real-time communication, distributed locking, and presence.
type TeammateMesh interface {
	// PubSub
	Publish(ctx context.Context, topic string, payload []byte) error
	Subscribe(ctx context.Context, topic string, handler func(msg []byte)) (Subscription, error)

	// Distributed Lock
	AcquireLock(ctx context.Context, key string, ttl time.Duration) (bool, error)
	ReleaseLock(ctx context.Context, key string) error

	// Presence
	RegisterPresence(ctx context.Context, agentID string, status string) error
	GetActiveAgents(ctx context.Context) ([]AgentPresence, error)
}
