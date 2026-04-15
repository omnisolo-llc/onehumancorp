package mesh

import (
	"context"
	"sync"
)

type LocalMeshBroker struct {
	mu       sync.RWMutex
	channels map[string][]chan []byte
}

func NewLocalMeshBroker() *LocalMeshBroker {
	return &LocalMeshBroker{
		channels: make(map[string][]chan []byte),
	}
}

func (b *LocalMeshBroker) Broadcast(ctx context.Context, channel string, payload []byte) error {
	b.mu.RLock()
	defer b.mu.RUnlock()

	if subs, ok := b.channels[channel]; ok {
		for _, sub := range subs {
			// non-blocking send
			select {
			case sub <- payload:
			default:
			}
		}
	}
	return nil
}

// Subscribe is an additional helper method for local tests, although not explicitly required.
func (b *LocalMeshBroker) Subscribe(channel string) <-chan []byte {
	b.mu.Lock()
	defer b.mu.Unlock()

	ch := make(chan []byte, 100)
	b.channels[channel] = append(b.channels[channel], ch)
	return ch
}
