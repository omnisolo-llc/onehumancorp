package telemetry

import (
	"context"
	"log/slog"
	"time"
)

// SyncFunc is a type alias for the function signature that syncs buffered metrics
// to a remote endpoint. This is implemented by orchestration.SIPDB.SyncBufferedMetrics.
type SyncFunc func(ctx context.Context, remoteEndpoint string) (int, error)

// StartSyncDaemon starts a background worker that periodically calls syncFunc
// to push locally buffered metrics to the cloud API endpoint.
// It uses the specified interval and respects context cancellation.
func StartSyncDaemon(ctx context.Context, syncFunc SyncFunc, endpoint string, interval time.Duration) {
	go func() {
		ticker := time.NewTicker(interval)
		defer ticker.Stop()
		for {
			select {
			case <-ctx.Done():
				return
			case <-ticker.C:
				for {
					syncedCount, err := syncFunc(ctx, endpoint)
					if err != nil {
						slog.Warn("Failed to sync standalone metrics", "error", err)
						break
					}
					if syncedCount > 0 {
						slog.Debug("Successfully synced standalone metrics to cloud", "count", syncedCount)
						// Record the batch size
						RecordSyncDaemonBatchSize(ctx, int64(syncedCount))
					}
					if syncedCount < 500 {
						break // No more batches or fetched less than the limit
					}
				}
			}
		}
	}()
}
