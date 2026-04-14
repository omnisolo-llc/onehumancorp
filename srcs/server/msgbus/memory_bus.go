package msgbus

import (
	"context"
	"sync"
)

// MemoryBus is an in-process pub/sub bus backed by Go channels.
// It is the default backend and has zero external dependencies.
type MemoryBus struct {
	mu   sync.RWMutex
	subs map[string][]subEntry
}

type subEntry struct {
	id      uint64
	handler Handler
}

var memorySubCounter uint64

func newMemoryBus() *MemoryBus {
	return &MemoryBus{subs: make(map[string][]subEntry)}
}

// Publish delivers msg to all current subscribers of msg.Topic.
func (b *MemoryBus) Publish(_ context.Context, msg Message) error {
	b.mu.RLock()
	entries := b.subs[msg.Topic]
	if len(entries) == 0 {
		b.mu.RUnlock()
		return nil
	}
	// Copy so we can release the lock before invoking handlers.
	snapshot := make([]subEntry, len(entries))
	copy(snapshot, entries)
	b.mu.RUnlock()

	for _, e := range snapshot {
		e.handler(msg)
	}
	return nil
}

// Subscribe registers handler for topic.  Returns a cancel function that
// removes the subscription.
func (b *MemoryBus) Subscribe(topic string, handler Handler) (func(), error) {
	b.mu.Lock()
	memorySubCounter++
	id := memorySubCounter
	b.subs[topic] = append(b.subs[topic], subEntry{id: id, handler: handler})
	b.mu.Unlock()

	cancel := func() {
		b.mu.Lock()
		defer b.mu.Unlock()
		list := b.subs[topic]
		filtered := list[:0]
		for _, e := range list {
			if e.id != id {
				filtered = append(filtered, e)
			}
		}
		if len(filtered) == 0 {
			delete(b.subs, topic)
		} else {
			b.subs[topic] = filtered
		}
	}
	return cancel, nil
}

// Close is a no-op for the memory bus.
func (b *MemoryBus) Close() error { return nil }
