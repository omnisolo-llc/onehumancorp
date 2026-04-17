package sync

import (
	"context"
	"log/slog"
	"sync"
	"time"
)

// SyncDelta represents a state change to be synchronized from Local to Cloud.
type SyncDelta struct {
	ID        string    `json:"id"`
	EntityID  string    `json:"entity_id"`
	Data      string    `json:"data"`
	UpdatedAt time.Time `json:"updated_at"`
}

// DeltaStore is the interface for interacting with the local SQLite storage.
type DeltaStore interface {
	// GetUnsyncedDeltas returns a list of deltas that have not yet been synchronized.
	GetUnsyncedDeltas(ctx context.Context, limit int) ([]SyncDelta, error)
	// MarkDeltasSynced marks the specified deltas as successfully synchronized.
	MarkDeltasSynced(ctx context.Context, ids []string) error
}

// CloudGateway is the interface for pushing deltas to the Cloud Postgres DB.
type CloudGateway interface {
	// PushDeltas pushes a list of deltas to the cloud.
	PushDeltas(ctx context.Context, deltas []SyncDelta) error
}

// HybridStateSynchronizer synchronizes local state deltas to the cloud.
type HybridStateSynchronizer struct {
	store    DeltaStore
	gateway  CloudGateway
	interval time.Duration
	ticker   *time.Ticker
	quit     chan struct{}
	stopOnce sync.Once
}

// NewHybridStateSynchronizer creates a new HybridStateSynchronizer.
func NewHybridStateSynchronizer(store DeltaStore, gateway CloudGateway, interval time.Duration) *HybridStateSynchronizer {
	return &HybridStateSynchronizer{
		store:    store,
		gateway:  gateway,
		interval: interval,
		quit:     make(chan struct{}),
	}
}

// Start begins the periodic synchronization loop.
func (s *HybridStateSynchronizer) Start(ctx context.Context) {
	s.ticker = time.NewTicker(s.interval)
	go func() {
		for {
			select {
			case <-s.ticker.C:
				s.sync(ctx)
			case <-s.quit:
				s.ticker.Stop()
				return
			case <-ctx.Done():
				if s.ticker != nil {
					s.ticker.Stop()
				}
				return
			}
		}
	}()
}

// Stop stops the synchronization loop.
func (s *HybridStateSynchronizer) Stop() {
	s.stopOnce.Do(func() {
		if s.quit != nil {
			close(s.quit)
		}
	})
}

// sync performs a single synchronization pass.
func (s *HybridStateSynchronizer) sync(ctx context.Context) {
	for {
		deltas, err := s.store.GetUnsyncedDeltas(ctx, 50)
		if err != nil {
			slog.Error("sync: failed to get unsynced deltas", "error", err)
			return
		}

		if len(deltas) == 0 {
			return
		}

		if err := s.gateway.PushDeltas(ctx, deltas); err != nil {
			slog.Error("sync: failed to push deltas to cloud", "error", err)
			return
		}

		ids := make([]string, len(deltas))
		for i, delta := range deltas {
			ids[i] = delta.ID
		}

		if err := s.store.MarkDeltasSynced(ctx, ids); err != nil {
			slog.Error("sync: failed to mark deltas as synced", "error", err)
			return
		}

		slog.Debug("sync: successfully synced deltas", "count", len(deltas))
	}
}
