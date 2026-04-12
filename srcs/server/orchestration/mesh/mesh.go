package mesh

import (
	"context"
	"time"
)

type Subscription interface {
	Unsubscribe() error
}

type AgentPresence struct {
	AgentID  string
	Status   string
	LastSeen time.Time
}

type TeammateMesh interface {
	// PubSub
	Publish(ctx context.Context, topic string, payload []byte) error
	Subscribe(ctx context.Context, topic string, handler func(msg []byte)) (Subscription, error)

	// Distributed Lock
	AcquireLock(ctx context.Context, key string, token string, ttl time.Duration) (bool, error)
	ReleaseLock(ctx context.Context, key string, token string) error

	// Presence
	RegisterPresence(ctx context.Context, agentID string, status string) error
	GetActiveAgents(ctx context.Context) ([]AgentPresence, error)
}
