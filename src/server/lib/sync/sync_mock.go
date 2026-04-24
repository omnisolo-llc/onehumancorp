package sync

import "sync"

// MockSynchronizer is a mock implementation of HybridSynchronizer for testing.
type MockSynchronizer struct {
	mu     sync.Mutex
	status SyncStatus
	err    error
}

// NewMockSynchronizer creates a new MockSynchronizer.
func NewMockSynchronizer() *MockSynchronizer {
	return &MockSynchronizer{
		status: SyncStatusIdle,
	}
}

// StartSync mocks the StartSync method.
func (m *MockSynchronizer) StartSync() error {
	m.mu.Lock()
	defer m.mu.Unlock()
	if m.err != nil {
		m.status = SyncStatusError
		return m.err
	}
	m.status = SyncStatusSyncing
	return nil
}

// StopSync mocks the StopSync method.
func (m *MockSynchronizer) StopSync() error {
	m.mu.Lock()
	defer m.mu.Unlock()
	m.status = SyncStatusIdle
	return nil
}

// GetSyncStatus mocks the GetSyncStatus method.
func (m *MockSynchronizer) GetSyncStatus() SyncStatus {
	m.mu.Lock()
	defer m.mu.Unlock()
	return m.status
}

// SetError sets an error to be returned by StartSync.
func (m *MockSynchronizer) SetError(err error) {
	m.mu.Lock()
	defer m.mu.Unlock()
	m.err = err
}

// SetStatus sets a specific status for testing.
func (m *MockSynchronizer) SetStatus(status SyncStatus) {
	m.mu.Lock()
	defer m.mu.Unlock()
	m.status = status
}
