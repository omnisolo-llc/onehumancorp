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
	subscribers map[string]map[int]func(data []byte)
	nextID      int
}

// NewLocalTeammateMesh creates a new LocalTeammateMesh.
func NewLocalTeammateMesh() *LocalTeammateMesh {
	return &LocalTeammateMesh{
		subscribers: make(map[string]map[int]func(data []byte)),
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
	subsCopy := make(map[int]func(data []byte), len(subs))
	for k, v := range subs {
		subsCopy[k] = v
	}
	m.mu.RUnlock()

	dataCopy := make([]byte, len(data))
	copy(dataCopy, data)

	for _, handler := range subsCopy {
		go handler(dataCopy)
	}

	return nil
}

// Subscribe registers a handler for the given channel. Unsubscribes when ctx is done.
func (m *LocalTeammateMesh) Subscribe(ctx context.Context, channel string, handler func(data []byte)) error {
	m.mu.Lock()
	id := m.nextID
	m.nextID++
	if m.subscribers[channel] == nil {
		m.subscribers[channel] = make(map[int]func(data []byte))
	}
	m.subscribers[channel][id] = handler
	m.mu.Unlock()

	go func() {
		<-ctx.Done()
		m.mu.Lock()
		defer m.mu.Unlock()
		delete(m.subscribers[channel], id)
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
