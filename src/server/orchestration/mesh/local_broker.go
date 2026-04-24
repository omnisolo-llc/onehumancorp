package mesh

import (
    "context"
    "sync"
)

type LocalMeshBroker struct {
    mu          sync.RWMutex
    subscribers map[string]chan []byte
}

func NewLocalMeshBroker() *LocalMeshBroker {
    return &LocalMeshBroker{
        subscribers: make(map[string]chan []byte),
    }
}

func (b *LocalMeshBroker) Broadcast(ctx context.Context, channel string, payload []byte) error {
    b.mu.RLock()
    defer b.mu.RUnlock()

    if ch, ok := b.subscribers[channel]; ok {
        select {
        case ch <- payload:
        case <-ctx.Done():
            return ctx.Err()
        default:
        }
    }
    return nil
}

func (b *LocalMeshBroker) Subscribe(ctx context.Context, channel string, handler func(msg []byte)) (Subscription, error) {
	// return a dummy subscription
	return nil, nil
}
