package analytics

import (
	"sync"
	"time"
)

type Event struct {
	Name      string
	UserID    string
	Properties map[string]interface{}
	Timestamp time.Time
}

type Tracker struct {
	mu     sync.RWMutex
	events []Event
}

func NewTracker() *Tracker {
	return &Tracker{
		events: make([]Event, 0),
	}
}

func (t *Tracker) Track(name, userID string, properties map[string]interface{}) {
	t.mu.Lock()
	defer t.mu.Unlock()
	t.events = append(t.events, Event{
		Name:       name,
		UserID:     userID,
		Properties: properties,
		Timestamp:  time.Now().UTC(),
	})
}

func (t *Tracker) GetEvents() []Event {
	t.mu.RLock()
	defer t.mu.RUnlock()

	// Return a copy to avoid data races when iterating
	res := make([]Event, len(t.events))
	copy(res, t.events)
	return res
}
