package mesh

import (
	"context"
	"sync"
)

// MemoryPubSub implements MeshPubSub using in-memory channels
type MemoryPubSub struct {
	mu          sync.RWMutex
	subscribers map[string][]chan TeammateMeshEvent
}

// NewMemoryPubSub creates a new MemoryPubSub
func NewMemoryPubSub() *MemoryPubSub {
	return &MemoryPubSub{
		subscribers: make(map[string][]chan TeammateMeshEvent),
	}
}

func (m *MemoryPubSub) Publish(ctx context.Context, topic string, message TeammateMeshEvent) error {
	m.mu.RLock()
	defer m.mu.RUnlock()

	subs := m.subscribers[topic]
	for _, ch := range subs {
		select {
		case ch <- message:
		default:
			// Drop message if channel is full or block?
		}
	}
	return nil
}

func (m *MemoryPubSub) Subscribe(ctx context.Context, topic string) (<-chan TeammateMeshEvent, error) {
	ch := make(chan TeammateMeshEvent, 100)

	m.mu.Lock()
	m.subscribers[topic] = append(m.subscribers[topic], ch)
	m.mu.Unlock()

	go func() {
		<-ctx.Done()
		m.mu.Lock()
		defer m.mu.Unlock()
		subs := m.subscribers[topic]
		for i, sub := range subs {
			if sub == ch {
				m.subscribers[topic] = append(subs[:i], subs[i+1:]...)
				close(ch)
				break
			}
		}
	}()

	return ch, nil
}

func (m *MemoryPubSub) Close() error {
	return nil
}
