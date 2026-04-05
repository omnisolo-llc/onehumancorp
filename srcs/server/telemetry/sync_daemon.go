package telemetry

import (
	"context"
	"log/slog"
	"time"
)

type SyncDB interface {
	SyncBufferedMetrics(ctx context.Context, remoteEndpoint string) (int, error)
}

func StartSyncDaemon(ctx context.Context, sipdb SyncDB, endpoint string) {
	go func() {
		ticker := time.NewTicker(5 * time.Minute)
		defer ticker.Stop()
		for {
			select {
			case <-ctx.Done():
				return
			case <-ticker.C:
				for {
					syncedCount, err := sipdb.SyncBufferedMetrics(ctx, endpoint)
					if err != nil {
						slog.Warn("Failed to sync standalone metrics", "error", err)
						break
					}
					if syncedCount > 0 {
						slog.Debug("Successfully synced standalone metrics to cloud", "count", syncedCount)
					}
					if syncedCount < 500 {
						break // No more batches
					}
				}
			}
		}
	}()
}
