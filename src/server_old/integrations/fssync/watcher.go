package fssync

import (
	"context"
)

// FileEvent represents a file system change event
type FileEvent struct {
	Path      string
	Operation string // "WRITE", "DELETE", etc.
}

// Watcher interface listens to file system changes in a given directory
type Watcher interface {
	Watch(ctx context.Context) (<-chan FileEvent, error)
	Close() error
}

// MockWatcher is a mock implementation of Watcher for testing
type MockWatcher struct {
	events chan FileEvent
}

// NewMockWatcher creates a new MockWatcher
func NewMockWatcher() *MockWatcher {
	return &MockWatcher{
		events: make(chan FileEvent, 100),
	}
}

// Watch returns a channel to receive file events
func (m *MockWatcher) Watch(ctx context.Context) (<-chan FileEvent, error) {
	return m.events, nil
}

// Close closes the watcher
func (m *MockWatcher) Close() error {
	close(m.events)
	return nil
}

// SimulateEvent simulates a file event for testing
func (m *MockWatcher) SimulateEvent(event FileEvent) {
	m.events <- event
}
