package orchestration

import (
	"context"
	"sync"
)

// MeshHub defines the interface for the Teammate Mesh realtime communication layer.
type MeshHub interface {
	Publish(ctx context.Context, channel string, data []byte) error
	Subscribe(ctx context.Context, channel string, handler func([]byte)) error
}

// LocalTeammateMesh implements MeshHub using in-memory channels/sync for standalone mode.
type LocalTeammateMesh struct {
	mu          sync.RWMutex
	subscribers map[string][]func([]byte)
}

// NewLocalTeammateMesh creates a new LocalTeammateMesh.
func NewLocalTeammateMesh() *LocalTeammateMesh {
	return &LocalTeammateMesh{
		subscribers: make(map[string][]func([]byte)),
	}
}

// Publish broadcasts the given data to all subscribers of the specified channel.
func (l *LocalTeammateMesh) Publish(ctx context.Context, channel string, data []byte) error {
	l.mu.RLock()
	handlers, ok := l.subscribers[channel]
	l.mu.RUnlock()

	if !ok {
		return nil
	}

	for _, handler := range handlers {
		// Run handler in a separate goroutine to prevent blocking the publisher
		go handler(data)
	}

	return nil
}

// Subscribe registers a handler to receive messages on the specified channel.
func (l *LocalTeammateMesh) Subscribe(ctx context.Context, channel string, handler func([]byte)) error {
	l.mu.Lock()
	defer l.mu.Unlock()

	l.subscribers[channel] = append(l.subscribers[channel], handler)
	return nil
}

// CentrifugeMesh implements MeshHub using external Redis/Centrifugo configurations.
type CentrifugeMesh struct {
	// Stub implementation
}

// NewCentrifugeMesh creates a new CentrifugeMesh.
func NewCentrifugeMesh() *CentrifugeMesh {
	return &CentrifugeMesh{}
}

// Publish publishes the data to the external mesh.
func (c *CentrifugeMesh) Publish(ctx context.Context, channel string, data []byte) error {
	// Stub implementation
	return nil
}

// Subscribe subscribes to a channel on the external mesh.
func (c *CentrifugeMesh) Subscribe(ctx context.Context, channel string, handler func([]byte)) error {
	// Stub implementation
	return nil
}
