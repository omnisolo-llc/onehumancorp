package sync_adapter

import (
	"errors"
	"sync"
	"time"

	syncpkg "github.com/onehumancorp/mono/src/server/lib/sync"
)

// PowerSyncAdapter implements the HybridSynchronizer interface using a PowerSync-like approach.
type PowerSyncAdapter struct {
	mu     sync.Mutex
	status syncpkg.SyncStatus
	err    error
	done   chan struct{}
}

// NewPowerSyncAdapter creates a new PowerSyncAdapter.
func NewPowerSyncAdapter() *PowerSyncAdapter {
	return &PowerSyncAdapter{
		status: syncpkg.SyncStatusIdle,
		done:   make(chan struct{}),
	}
}

// StartSync initiates the synchronization process.
func (p *PowerSyncAdapter) StartSync() error {
	p.mu.Lock()
	defer p.mu.Unlock()

	if p.err != nil {
		p.status = syncpkg.SyncStatusError
		return p.err
	}

	if p.status == syncpkg.SyncStatusSyncing {
		return errors.New("sync already in progress")
	}

	p.status = syncpkg.SyncStatusSyncing

	// Simulate starting a sync process in the background
	p.done = make(chan struct{})
	go p.syncLoop()

	return nil
}

// StopSync halts the synchronization process.
func (p *PowerSyncAdapter) StopSync() error {
	p.mu.Lock()
	defer p.mu.Unlock()

	if p.status != syncpkg.SyncStatusSyncing {
		return nil
	}

	close(p.done)
	p.status = syncpkg.SyncStatusIdle
	return nil
}

// GetSyncStatus returns the current status of the synchronization.
func (p *PowerSyncAdapter) GetSyncStatus() syncpkg.SyncStatus {
	p.mu.Lock()
	defer p.mu.Unlock()
	return p.status
}

// SetError is used for testing to simulate errors.
func (p *PowerSyncAdapter) SetError(err error) {
	p.mu.Lock()
	defer p.mu.Unlock()
	p.err = err
}

// syncLoop simulates a background synchronization process.
func (p *PowerSyncAdapter) syncLoop() {
	// Simulate doing some work
	select {
	case <-time.After(100 * time.Millisecond):
		p.mu.Lock()
		if p.status == syncpkg.SyncStatusSyncing {
			p.status = syncpkg.SyncStatusUpToDate
		}
		p.mu.Unlock()
	case <-p.done:
		// Stopped
	}
}
