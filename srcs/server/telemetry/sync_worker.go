package telemetry

import (
	"context"

	"log/slog"
	"time"
)

// SyncWorker is a background worker that periodically syncs buffered telemetry metrics
// with the OHC-SIP Cloud DB when in Standalone mode.
type SyncWorker struct {
	syncFunc      func(ctx context.Context, remoteEndpoint string) (int, error)
	cloudEndpoint string
	interval      time.Duration
}

// NewSyncWorker creates a new SyncWorker.
func NewSyncWorker(syncFunc func(ctx context.Context, remoteEndpoint string) (int, error), cloudEndpoint string, interval time.Duration) *SyncWorker {
	if interval == 0 {
		interval = 5 * time.Minute
	}
	return &SyncWorker{
		syncFunc:      syncFunc,
		cloudEndpoint: cloudEndpoint,
		interval:      interval,
	}
}

// Start runs the sync loop until the context is canceled.
func (w *SyncWorker) Start(ctx context.Context) {
	ticker := time.NewTicker(w.interval)
	defer ticker.Stop()

	// Initial backoff setting
	backoff := w.interval

	for {
		select {
		case <-ctx.Done():
			slog.InfoContext(ctx, "telemetry sync worker stopping")
			return
		case <-ticker.C:
			count, err := w.syncFunc(ctx, w.cloudEndpoint)
			if err != nil {
				slog.WarnContext(ctx, "failed to sync local telemetry buffer", "error", err)
				// Exponential backoff
				backoff *= 2
				if backoff > 1*time.Hour {
					backoff = 1 * time.Hour
				}
				ticker.Reset(backoff)
			} else {
				if count > 0 {
					slog.InfoContext(ctx, "successfully synced local telemetry buffer", "count", count)
				}
				// Reset backoff on success
				if backoff != w.interval {
					backoff = w.interval
					ticker.Reset(backoff)
				}
			}
		}
	}
}
