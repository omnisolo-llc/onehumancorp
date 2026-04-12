package mesh

import (
	"context"
	"time"
)

type Subscription interface {
	Unsubscribe() error
}

type AgentPresence struct {
	AgentID string
	Status  string
}

type TeammateMesh interface {
	// PubSub
	Publish(ctx context.Context, topic string, payload []byte) error
	Subscribe(ctx context.Context, topic string, handler func(msg []byte)) (Subscription, error)

	// Distributed Lock
	AcquireLock(ctx context.Context, key string, ttl time.Duration, token string) (bool, error)
	ReleaseLock(ctx context.Context, key string, token string) error

	// Presence
	RegisterPresence(ctx context.Context, agentID string, status string) error
	GetActiveAgents(ctx context.Context) ([]AgentPresence, error)

	// State Handoff
	HandoffState(ctx context.Context, targetAgentID string, state []byte) error
	SubscribeHandoffs(ctx context.Context, agentID string, handler func(state []byte)) (Subscription, error)
}
