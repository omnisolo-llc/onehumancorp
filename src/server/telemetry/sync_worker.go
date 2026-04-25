package telemetry

import (
	"context"
	"log/slog"
	"time"
)

// SyncFunc is a type alias for the function signature that syncs buffered metrics
// to a remote endpoint. This is implemented by orchestration.SIPDB.SyncBufferedMetrics.
type SyncFunc func(ctx context.Context, remoteEndpoint string, batchSize int) (int, error)

// StartSyncDaemon starts a background worker that periodically calls syncFunc
// to push locally buffered metrics to the cloud API endpoint.
// It uses the specified interval and respects context cancellation.
func StartSyncDaemon(ctx context.Context, syncFunc SyncFunc, depthFunc func(context.Context) (int, error), endpoint string, interval time.Duration) {
	go func() {
		ticker := time.NewTicker(interval)
		defer ticker.Stop()

		baseBatchSize := int64(500)
		currentBatchSize := baseBatchSize
		errorCount := 0
		maxBackoff := 5 * time.Minute

		for {
			select {
			case <-ctx.Done():
				return
			case <-ticker.C:
				if depthFunc != nil {
					depth, err := depthFunc(ctx)
					if err == nil {
						RecordTelemetryBufferDepth(ctx, int64(depth))
					}
				}
				RecordTelemetryBatchSize(ctx, currentBatchSize)
				for {
					syncedCount, err := syncFunc(ctx, endpoint, int(currentBatchSize))
					if err != nil {
						errorCount++
						slog.Warn("Failed to sync standalone metrics", "error", err, "errorCount", errorCount)

						// Extract error reason
						reason := "unknown"
						errStr := err.Error()
						if len(errStr) > 4 && errStr[0:4] == "429:" {
							reason = "429"
						} else if len(errStr) > 4 && errStr[0:4] == "500:" {
							reason = "500"
						} else if len(errStr) > 8 && errStr[0:8] == "timeout:" {
							reason = "timeout"
						}
						RecordSyncErrorReason(ctx, reason)

						backoffDuration := time.Duration(1<<errorCount) * time.Second
						if backoffDuration > maxBackoff {
							backoffDuration = maxBackoff
						}

						// AIMD Multiplicative Decrease
						currentBatchSize = currentBatchSize / 2
						if currentBatchSize < 10 {
							currentBatchSize = 10
						}

						RecordTelemetrySyncBackoff(ctx, backoffDuration.Seconds())

						timer := time.NewTimer(backoffDuration)
						select {
						case <-ctx.Done():
							timer.Stop()
							return
						case <-timer.C:
						}
						break
					}

					if errorCount > 0 {
						errorCount = 0
					}

					// AIMD Additive Increase
					if currentBatchSize < 2000 {
						currentBatchSize += 100
						if currentBatchSize > 2000 {
							currentBatchSize = 2000
						}
					}

					if syncedCount > 0 {
						slog.Debug("Successfully synced standalone metrics to cloud", "count", syncedCount)
						RecordSyncDaemonBatchSize(ctx, int64(syncedCount), "Standalone")
					}
					if int64(syncedCount) < currentBatchSize {
						break
					}
				}
			}
		}
	}()
}
