package orchestration

import (
	"context"
	"testing"
	"time"
)

type MockMutex struct {
	Acquired bool
}

func (m *MockMutex) Lock(ctx context.Context, ttl time.Duration) error {
	if m.Acquired {
		return ErrLockAcquisitionFailed
	}
	m.Acquired = true
	return nil
}

func (m *MockMutex) Unlock(ctx context.Context) error {
	m.Acquired = false
	return nil
}

type MockMutexProvider struct {
	Mutex *MockMutex
}

func (m *MockMutexProvider) NewMutex(key string) Mutex {
	if m.Mutex == nil {
		m.Mutex = &MockMutex{}
	}
	return m.Mutex
}

func TestHandoffState(t *testing.T) {
	ctx := context.Background()
	mockProvider := &MockMutexProvider{}

	err := HandoffState(ctx, "cloud", []byte("state1"), mockProvider)
	if err != nil {
		t.Fatalf("expected nil err, got %v", err)
	}

	// Because of defer Release, mockMutex should be available again
	if mockProvider.Mutex.Acquired {
		t.Fatalf("expected mutex to be released")
	}
}
