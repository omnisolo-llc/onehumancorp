package mesh

import (
	"context"
	"time"
)

// Subscription represents an active subscription to a mesh topic.
type Subscription interface {
	// Unsubscribe removes the subscription.
	Unsubscribe(ctx context.Context) error
}

// AgentPresence represents the status of an agent in the swarm.
type AgentPresence struct {
	AgentID string `json:"agent_id"`
	Status  string `json:"status"` // e.g., IDLE, WORKING, FAULT
}

// TeammateMesh defines the real-time coordination layer.
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
