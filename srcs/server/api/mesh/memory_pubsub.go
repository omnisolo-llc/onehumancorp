package mesh

import (
	"context"
	"sync"
)

type MemoryPubSub struct {
	mu          sync.RWMutex
	subscribers map[string][]chan []byte
}

func NewMemoryPubSub() *MemoryPubSub {
	return &MemoryPubSub{
		subscribers: make(map[string][]chan []byte),
	}
}

func (m *MemoryPubSub) Publish(ctx context.Context, topic string, message []byte) error {
	m.mu.RLock()
	defer m.mu.RUnlock()

	subs := m.subscribers[topic]
	for _, sub := range subs {
		select {
		case <-ctx.Done():
			return ctx.Err()
		case sub <- message:
		default:
			// Non-blocking publish if channel is full or slow
		}
	}
	return nil
}

func (m *MemoryPubSub) Subscribe(ctx context.Context, topic string) (<-chan []byte, func() error, error) {
	m.mu.Lock()
	defer m.mu.Unlock()

	ch := make(chan []byte, 100) // buffer size 100
	m.subscribers[topic] = append(m.subscribers[topic], ch)

	unsubscribe := func() error {
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
		return nil
	}

	return ch, unsubscribe, nil
}
