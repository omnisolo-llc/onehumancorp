package mesh

import (
	"sync"
)

// LocalMesh implements TeammateMesh using in-memory channels.
type LocalMesh struct {
	mu          sync.RWMutex
	subscribers map[string][]chan []byte
}

// NewLocalMesh creates a new LocalMesh.
func NewLocalMesh() *LocalMesh {
	return &LocalMesh{
		subscribers: make(map[string][]chan []byte),
	}
}

// Publish sends a message to all subscribers of the specified channel.
func (l *LocalMesh) Publish(channel string, message []byte) error {
	l.mu.RLock()
	defer l.mu.RUnlock()

	subs, ok := l.subscribers[channel]
	if !ok {
		return nil
	}

	for _, ch := range subs {
		select {
		case ch <- message:
		default:
			// Non-blocking send to avoid hanging if a subscriber is slow.
		}
	}
	return nil
}

// Subscribe returns a channel that receives messages from the specified channel.
func (l *LocalMesh) Subscribe(channel string) (<-chan []byte, error) {
	l.mu.Lock()
	defer l.mu.Unlock()

	ch := make(chan []byte, 100)
	l.subscribers[channel] = append(l.subscribers[channel], ch)

	return ch, nil
}
