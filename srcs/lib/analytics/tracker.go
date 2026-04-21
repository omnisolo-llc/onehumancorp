package analytics

import (
	"context"
	"sync"
)

type Event struct {
	Name       string
	Properties map[string]interface{}
}

type Tracker interface {
	Track(ctx context.Context, event Event) error
}

type InMemoryTracker struct {
	mu     sync.Mutex
	events []Event
}

func NewInMemoryTracker() *InMemoryTracker {
	return &InMemoryTracker{
		events: make([]Event, 0),
	}
}

func (t *InMemoryTracker) Track(ctx context.Context, event Event) error {
	t.mu.Lock()
	defer t.mu.Unlock()
	t.events = append(t.events, event)
	return nil
}

func (t *InMemoryTracker) Events() []Event {
	t.mu.Lock()
	defer t.mu.Unlock()
	return append([]Event{}, t.events...)
}
