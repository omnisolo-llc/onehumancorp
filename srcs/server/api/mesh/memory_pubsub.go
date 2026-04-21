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
	subs := m.subscribers[topic]
	m.mu.RUnlock()

	for _, ch := range subs {
		select {
		case ch <- message:
		default:
			// If channel is full, we drop it to not block
		}
	}
	return nil
}

func (m *MemoryPubSub) Subscribe(ctx context.Context, topic string) (<-chan []byte, error) {
	ch := make(chan []byte, 100)

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
