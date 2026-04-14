package mesh

import (
	"context"
	"sync"
)

type LocalMeshBroker struct {
	mu          sync.RWMutex
	subscribers map[string][]chan []byte
}

func NewLocalMeshBroker() *LocalMeshBroker {
	return &LocalMeshBroker{
		subscribers: make(map[string][]chan []byte),
	}
}

func (l *LocalMeshBroker) Subscribe(channel string) <-chan []byte {
	l.mu.Lock()
	defer l.mu.Unlock()

	ch := make(chan []byte, 100)
	l.subscribers[channel] = append(l.subscribers[channel], ch)
	return ch
}

func (l *LocalMeshBroker) Broadcast(ctx context.Context, channel string, payload []byte) error {
	l.mu.RLock()
	defer l.mu.RUnlock()

	subs, ok := l.subscribers[channel]
	if !ok {
		return nil
	}

	for _, sub := range subs {
		select {
		case sub <- payload:
		case <-ctx.Done():
			return ctx.Err()
		default:
			// Non-blocking drop if channel is full
		}
	}
	return nil
}
