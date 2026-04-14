package mesh

import (
	"context"
	"sync"
)

// LocalMeshBroker implements MeshBroker using in-memory channels for Standalone environments.
type LocalMeshBroker struct {
	mu          sync.RWMutex
	subscribers map[string][]chan<- []byte
}

// NewLocalMeshBroker creates a new LocalMeshBroker.
func NewLocalMeshBroker() *LocalMeshBroker {
	return &LocalMeshBroker{
		subscribers: make(map[string][]chan<- []byte),
	}
}

// Broadcast publishes the payload to all local subscribers of the specified channel.
func (b *LocalMeshBroker) Broadcast(ctx context.Context, channel string, payload []byte) error {
	b.mu.RLock()
	defer b.mu.RUnlock()

	subs, exists := b.subscribers[channel]
	if !exists {
		return nil
	}

	for _, sub := range subs {
		select {
		case sub <- payload:
		default:
			// Non-blocking send
		}
	}

	return nil
}

// Subscribe adds a channel to the subscribers list for testing or local usage.
func (b *LocalMeshBroker) Subscribe(channel string, ch chan<- []byte) {
	b.mu.Lock()
	defer b.mu.Unlock()
	b.subscribers[channel] = append(b.subscribers[channel], ch)
}
