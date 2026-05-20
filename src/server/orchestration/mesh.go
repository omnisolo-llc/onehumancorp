package orchestration

import (
	"context"
	"encoding/json"
	"fmt"
	"sync"
)

// MeshHub defines the interface for the Teammate Mesh API
type MeshHub interface {
	Publish(ctx context.Context, channel string, data interface{}) error
	Subscribe(ctx context.Context, channel string, handler func(data []byte)) error
}

// LocalTeammateMesh implements MeshHub using Go channels for standalone operation
type LocalTeammateMesh struct {
	mu          sync.RWMutex
	subscribers map[string][]func(data []byte)
}

// NewLocalTeammateMesh creates a new LocalTeammateMesh instance
func NewLocalTeammateMesh() *LocalTeammateMesh {
	return &LocalTeammateMesh{
		subscribers: make(map[string][]func(data []byte)),
	}
}

// Publish broadcasts data to all subscribers of a channel
func (m *LocalTeammateMesh) Publish(ctx context.Context, channel string, data interface{}) error {
	bytes, err := json.Marshal(data)
	if err != nil {
		return fmt.Errorf("failed to marshal data: %w", err)
	}

	m.mu.RLock()
	handlers, ok := m.subscribers[channel]
	m.mu.RUnlock()

	if !ok {
		return nil // No subscribers
	}

	for _, handler := range handlers {
		// Run handlers asynchronously to avoid blocking
		go handler(bytes)
	}

	return nil
}

// Subscribe registers a handler for a channel
func (m *LocalTeammateMesh) Subscribe(ctx context.Context, channel string, handler func(data []byte)) error {
	m.mu.Lock()
	defer m.mu.Unlock()

	m.subscribers[channel] = append(m.subscribers[channel], handler)
	return nil
}

// CentrifugeMesh implements MeshHub using rueidis and Centrifugo (stub)
type CentrifugeMesh struct {
	// redisClient rueidis.Client
	// centrifuge  *centrifuge.Node
}

// NewCentrifugeMesh creates a new CentrifugeMesh instance (stub)
func NewCentrifugeMesh() *CentrifugeMesh {
	return &CentrifugeMesh{}
}

// Publish publishes data to a channel via Redis/Centrifugo (stub)
func (m *CentrifugeMesh) Publish(ctx context.Context, channel string, data interface{}) error {
	// TODO: Implement using rueidis/Centrifugo
	return fmt.Errorf("not implemented")
}

// Subscribe subscribes to a channel via Redis/Centrifugo (stub)
func (m *CentrifugeMesh) Subscribe(ctx context.Context, channel string, handler func(data []byte)) error {
	// TODO: Implement using rueidis/Centrifugo
	return fmt.Errorf("not implemented")
}
