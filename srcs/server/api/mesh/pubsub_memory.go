package mesh

import (
	"context"
	"sync"
	"time"
)

type MemoryPubSub struct {
	subscribers map[string]map[chan []byte]struct{}
	mu          sync.RWMutex
}

func NewMemoryPubSub() *MemoryPubSub {
	return &MemoryPubSub{
		subscribers: make(map[string]map[chan []byte]struct{}),
	}
}

func (p *MemoryPubSub) Publish(ctx context.Context, topic string, message []byte) error {
	p.mu.RLock()
	defer p.mu.RUnlock()

	subs, ok := p.subscribers[topic]
	if !ok {
		return nil
	}

	for sub := range subs {
		select {
		case sub <- message:
		case <-time.After(10 * time.Millisecond): // Drop if blocked
		}
	}
	return nil
}

func (p *MemoryPubSub) Subscribe(ctx context.Context, topic string) (<-chan []byte, error) {
	out := make(chan []byte, 100)

	p.mu.Lock()
	if _, ok := p.subscribers[topic]; !ok {
		p.subscribers[topic] = make(map[chan []byte]struct{})
	}
	p.subscribers[topic][out] = struct{}{}
	p.mu.Unlock()

	go func() {
		<-ctx.Done()
		p.mu.Lock()
		if subs, ok := p.subscribers[topic]; ok {
			delete(subs, out)
			if len(subs) == 0 {
				delete(p.subscribers, topic)
			}
		}
		p.mu.Unlock()
		close(out)
	}()

	return out, nil
}
