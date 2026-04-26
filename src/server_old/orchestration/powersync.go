package orchestration

import (
	"github.com/onehumancorp/mono/src/server/db"

	"fmt"
	"time"
	"log/slog"

	"github.com/onehumancorp/mono/src/server/integrations/sync_adapter"
	syncpkg "github.com/onehumancorp/mono/src/server/lib/sync"
)

// PowerSyncOrchestrator manages the synchronization between local SQLite and cloud PostgreSQL.
type PowerSyncOrchestrator struct {
	adapter *sync_adapter.PowerSyncAdapter
	ticker  *time.Ticker
	done    chan struct{}
}

// NewPowerSyncOrchestrator creates a new PowerSyncOrchestrator.
func NewPowerSyncOrchestrator(database *db.DB) *PowerSyncOrchestrator {
	return &PowerSyncOrchestrator{
		adapter: sync_adapter.NewPowerSyncAdapter(),
		done:    make(chan struct{}),
	}
}

// Start starts the background synchronization ticker.
func (p *PowerSyncOrchestrator) Start(interval time.Duration) {
	p.ticker = time.NewTicker(interval)

	go func() {
		for {
			select {
			case <-p.ticker.C:
				if err := p.pushSync(); err != nil {
					slog.Error("PowerSync push failed", "error", err)
				}
			case <-p.done:
				return
			}
		}
	}()
}

// Stop stops the background synchronization.
func (p *PowerSyncOrchestrator) Stop() {
	if p.ticker != nil {
		p.ticker.Stop()
	}
	close(p.done)
	p.adapter.StopSync()
}

// pushSync triggers a synchronization cycle.
func (p *PowerSyncOrchestrator) pushSync() error {
	slog.Info("Starting PowerSync push to cloud")
	if err := p.adapter.StartSync(); err != nil {
		return fmt.Errorf("failed to start sync: %w", err)
	}
	return nil
}

// GetStatus returns the current status of the sync adapter.
func (p *PowerSyncOrchestrator) GetStatus() syncpkg.SyncStatus {
	return p.adapter.GetSyncStatus()
}
