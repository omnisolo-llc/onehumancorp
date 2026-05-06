package orchestration

import (
	"context"
	"sync"
)

// MeshHub defines the interface for the highly available realtime communication layer.
type MeshHub interface {
	Publish(ctx context.Context, channel string, data []byte) error
	Subscribe(ctx context.Context, channel string, handler func(data []byte)) error
}

type subscriber struct {
	id      int
	handler func(data []byte)
}

// LocalTeammateMesh implements MeshHub for standalone operation using Go channels.
type LocalTeammateMesh struct {
	mu          sync.RWMutex
	subscribers map[string][]subscriber
	nextID      int
}

// NewLocalTeammateMesh creates a new LocalTeammateMesh.
func NewLocalTeammateMesh() *LocalTeammateMesh {
	return &LocalTeammateMesh{
		subscribers: make(map[string][]subscriber),
	}
}

// Publish sends data to all subscribers of the given channel concurrently.
func (m *LocalTeammateMesh) Publish(ctx context.Context, channel string, data []byte) error {
	m.mu.RLock()
	subs, ok := m.subscribers[channel]
	if !ok {
		m.mu.RUnlock()
		return nil
	}
	// Copy subs to avoid holding lock while dispatching
	subsCopy := make([]subscriber, len(subs))
	copy(subsCopy, subs)
	m.mu.RUnlock()

	dataCopy := make([]byte, len(data))
	copy(dataCopy, data)

	for _, sub := range subsCopy {
		go sub.handler(dataCopy)
	}

	return nil
}

// Subscribe registers a handler for the given channel. Unsubscribes when ctx is done.
func (m *LocalTeammateMesh) Subscribe(ctx context.Context, channel string, handler func(data []byte)) error {
	m.mu.Lock()
	id := m.nextID
	m.nextID++
	m.subscribers[channel] = append(m.subscribers[channel], subscriber{id: id, handler: handler})
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

// CentrifugeMesh implements MeshHub using rueidis and Centrifugo primitives.
// This is currently a stub for cloud-native setup.
type CentrifugeMesh struct {
}

// NewCentrifugeMesh creates a new CentrifugeMesh.
func NewCentrifugeMesh() *CentrifugeMesh {
	return &CentrifugeMesh{}
}

// Publish is a stub for CentrifugeMesh.
func (m *CentrifugeMesh) Publish(ctx context.Context, channel string, data []byte) error {
	return nil
}

// Subscribe is a stub for CentrifugeMesh.
func (m *CentrifugeMesh) Subscribe(ctx context.Context, channel string, handler func(data []byte)) error {
	return nil
}
