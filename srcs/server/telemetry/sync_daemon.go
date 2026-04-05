package telemetry

import (
	"context"
	"log/slog"
	"time"
)

type SyncDaemon struct {
	ticker        *time.Ticker
	quit          chan struct{}
	syncFunc      func(ctx context.Context, endpoint string) (int, error)
	cloudEndpoint string
}

func NewSyncDaemon(pollInterval time.Duration, cloudEndpoint string, syncFunc func(ctx context.Context, endpoint string) (int, error)) *SyncDaemon {
	return &SyncDaemon{
		ticker:        time.NewTicker(pollInterval),
		quit:          make(chan struct{}),
		syncFunc:      syncFunc,
		cloudEndpoint: cloudEndpoint,
	}
}

func (d *SyncDaemon) Start(ctx context.Context) {
	go func() {
		for {
			select {
			case <-ctx.Done():
				d.ticker.Stop()
				return
			case <-d.quit:
				d.ticker.Stop()
				return
			case <-d.ticker.C:
				d.ProcessSync(ctx)
			}
		}
	}()
}

func (d *SyncDaemon) Stop() {
	close(d.quit)
}

func (d *SyncDaemon) ProcessSync(ctx context.Context) {
	for {
		syncedCount, err := d.syncFunc(ctx, d.cloudEndpoint)
		if err != nil {
			slog.Warn("Failed to sync standalone metrics", "error", err)
			break
		}
		if syncedCount > 0 {
			slog.Debug("Successfully synced standalone metrics to cloud", "count", syncedCount)
		}
		// Based on sipdb.SyncBufferedMetrics limit (500)
		if syncedCount < 500 {
			break
		}
	}
}
