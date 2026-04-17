package mesh

import (
	"context"
	"sync"
)

// LocalMesh is the Standalone implementation using Go channels and sync.Mutex.
type LocalMesh struct {
	mu          sync.RWMutex
	subscribers map[string][]chan string
	locks       map[string]*sync.Mutex
	locksMu     sync.Mutex
}

func NewLocalMesh() *LocalMesh {
	return &LocalMesh{
		subscribers: make(map[string][]chan string),
		locks:       make(map[string]*sync.Mutex),
	}
}

func (l *LocalMesh) Publish(ctx context.Context, channel, message string) error {
	l.mu.RLock()
	subs := l.subscribers[channel]
	l.mu.RUnlock()

	for _, ch := range subs {
		select {
		case <-ctx.Done():
			return ctx.Err()
		case ch <- message:
		default:
			// If a subscriber is slow or blocked, we just drop the message
			// or wait for a short duration. Given this is a local mesh without
			// complex queueing, dropping is safer than leaking goroutines.
		}
	}
	return nil
}

func (l *LocalMesh) Subscribe(ctx context.Context, channel string) (<-chan string, error) {
	ch := make(chan string, 100)
	l.mu.Lock()
	l.subscribers[channel] = append(l.subscribers[channel], ch)
	l.mu.Unlock()

	// Handle unsubscription when context is cancelled
	go func() {
		<-ctx.Done()
		l.mu.Lock()
		defer l.mu.Unlock()
		subs := l.subscribers[channel]
		for i, c := range subs {
			if c == ch {
				l.subscribers[channel] = append(subs[:i], subs[i+1:]...)
				close(ch)
				break
			}
		}
	}()

	return ch, nil
}

func (l *LocalMesh) AcquireLock(ctx context.Context, key string) (func(), error) {
	l.locksMu.Lock()
	mu, ok := l.locks[key]
	if !ok {
		mu = &sync.Mutex{}
		l.locks[key] = mu
	}
	l.locksMu.Unlock()

	// Polling to respect context cancellation without leaking locks.
	// Since standard sync.Mutex doesn't support context natively, we poll.
	// In production, `golang.org/x/sync/semaphore` could be used.

	for {
		select {
		case <-ctx.Done():
			return nil, ctx.Err()
		default:
			if mu.TryLock() {
				return func() { mu.Unlock() }, nil
			}
		}
	}
}
