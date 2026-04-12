package mesh

import (
	"context"
	"time"
)

// Subscription represents a handle to a PubSub subscription
type Subscription interface {
	Unsubscribe(ctx context.Context) error
}

// AgentPresence represents the status of an agent in the swarm
type AgentPresence struct {
	AgentID   string    `json:"agent_id"`
	Status    string    `json:"status"`
	UpdatedAt time.Time `json:"updated_at"`
}

// TeammateMesh defines the real-time API for agent coordination.
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
