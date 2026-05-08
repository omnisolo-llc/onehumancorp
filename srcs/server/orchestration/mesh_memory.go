package orchestration

import (
	"context"
	"sync"
)

type memorySubscriber struct {
	id      int
	handler func(data []byte)
}

type MemoryMeshTransport struct {
	mu          sync.RWMutex
	subscribers map[string][]memorySubscriber
	nextID      int
}

func NewMemoryMeshTransport() *MemoryMeshTransport {
	return &MemoryMeshTransport{
		subscribers: make(map[string][]memorySubscriber),
	}
}

func (m *MemoryMeshTransport) Publish(ctx context.Context, channel string, data []byte) error {
	m.mu.RLock()
	subs, ok := m.subscribers[channel]
	if !ok {
		m.mu.RUnlock()
		return nil
	}
	subsCopy := make([]memorySubscriber, len(subs))
	copy(subsCopy, subs)
	m.mu.RUnlock()

	dataCopy := make([]byte, len(data))
	copy(dataCopy, data)

	for _, sub := range subsCopy {
		go sub.handler(dataCopy)
	}

	return nil
}

func (m *MemoryMeshTransport) Subscribe(ctx context.Context, channel string, handler func(data []byte)) error {
	m.mu.Lock()
	id := m.nextID
	m.nextID++
	m.subscribers[channel] = append(m.subscribers[channel], memorySubscriber{id: id, handler: handler})
	m.mu.Unlock()

	go func() {
		<-ctx.Done()
		m.mu.Lock()
		defer m.mu.Unlock()
		subs := m.subscribers[channel]
		for i, sub := range subs {
			if sub.id == id {
				m.subscribers[channel] = append(subs[:i], subs[i+1:]...)
				break
			}
		}
	}()

	return nil
}
