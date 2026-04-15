package mesh

import (
	"context"
	"sync"
)

type LocalMeshBroker struct {
	mu       sync.RWMutex
	channels map[string]chan []byte
}

func NewLocalMeshBroker() *LocalMeshBroker {
	return &LocalMeshBroker{
		channels: make(map[string]chan []byte),
	}
}

func (b *LocalMeshBroker) Broadcast(ctx context.Context, channel string, payload []byte) error {
	b.mu.RLock()
	defer b.mu.RUnlock()
	if ch, ok := b.channels[channel]; ok {
		select {
		case ch <- payload:
		default:
		}
	}
	return nil
}
