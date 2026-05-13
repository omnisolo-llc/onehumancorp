package orchestration

import (
	"context"
	"fmt"
	"sync"

	pb "github.com/onehumancorp/ohc/srcs/proto"
)

// MemoryMeshTransport implements MeshTransport using local Go channels for Standalone mode.
type MemoryMeshTransport struct {
	mu          sync.RWMutex
	subscribers map[string][]func(*pb.MeshEvent)
}

// NewMemoryMeshTransport creates a new MemoryMeshTransport.
func NewMemoryMeshTransport() *MemoryMeshTransport {
	return &MemoryMeshTransport{
		subscribers: make(map[string][]func(*pb.MeshEvent)),
	}
}

// Publish broadcasts an event to all subscribers of a channel.
func (t *MemoryMeshTransport) Publish(ctx context.Context, channel string, event *pb.MeshEvent) error {
	select {
	case <-ctx.Done():
		return ctx.Err()
	default:
	}

	t.mu.RLock()
	handlers, ok := t.subscribers[channel]
	t.mu.RUnlock()

	if !ok {
		return nil // No subscribers, nothing to do
	}

	for _, handler := range handlers {
		// Execute handler asynchronously so one slow subscriber doesn't block the publisher
		// In a production system, we'd want worker pools or buffered channels to handle backpressure
		go func(h func(*pb.MeshEvent), e *pb.MeshEvent) {
			h(e)
		}(handler, event)
	}

	return nil
}

// Subscribe registers a handler for a specific channel.
func (t *MemoryMeshTransport) Subscribe(ctx context.Context, channel string, handler func(*pb.MeshEvent)) error {
	t.mu.Lock()
	defer t.mu.Unlock()

	if t.subscribers == nil {
		return fmt.Errorf("transport is closed")
	}

	t.subscribers[channel] = append(t.subscribers[channel], handler)
	return nil
}

// Close closes the transport.
func (t *MemoryMeshTransport) Close() error {
	t.mu.Lock()
	defer t.mu.Unlock()
	t.subscribers = nil
	return nil
}
