package mesh

import (
	"context"
	"time"
)

type Subscription interface {
	Channel() <-chan []byte
	Close() error
}

type AgentPresence struct {
	AgentID string
	Status  string
}

type TeammateMesh interface {
	Publish(ctx context.Context, topic string, payload []byte) error
	Subscribe(ctx context.Context, topic string, handler func(msg []byte)) (Subscription, error)
	AcquireLock(ctx context.Context, key string, ttl time.Duration) (bool, error)
	ReleaseLock(ctx context.Context, key string) error
	RegisterPresence(ctx context.Context, agentID string, status string) error
	GetActiveAgents(ctx context.Context) ([]AgentPresence, error)
}
