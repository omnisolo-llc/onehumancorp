package db

import (
	"context"
	"log/slog"
	"time"
)

// OptimizeStorage executes database maintenance routines.
// For SQLite, it performs an aggressive pruning of old ephemeral records and runs VACUUM
// to reclaim storage space, ensuring cost efficiency for host machines.
func OptimizeStorage(ctx context.Context, provider Provider) error {
	if !provider.IsSQLite() {
		return nil
	}

	start := time.Now()
	slog.Info("Running proactive storage optimization for Standalone SQLite DB")

	// 1. Aggressive pruning of ephemeral records older than 7 days
	pruneQueries := []string{
		"DELETE FROM telemetry_buffer WHERE created_at < datetime('now', '-7 day')",
		"DELETE FROM llm_completion_cache WHERE created_at < datetime('now', '-7 day')",
	}

	for _, query := range pruneQueries {
		if _, err := provider.Exec(ctx, query); err != nil {
			slog.Warn("Failed to aggressively prune SQLite data", "query", query, "error", err)
			// Continue even if one pruning fails, to attempt vacuum
		}
	}

	// 2. Reclaim disk space via VACUUM
	if _, err := provider.Exec(ctx, "VACUUM"); err != nil {
		slog.Error("Failed to VACUUM SQLite DB", "error", err)
		return err
	}

	slog.Info("Successfully completed storage optimization", "duration", time.Since(start).String())
	return nil
}

// StartStorageOptimizerDaemon runs a background goroutine to periodically clean up and vacuum SQLite DB.
func StartStorageOptimizerDaemon(ctx context.Context, provider Provider, interval time.Duration) {
	if !provider.IsSQLite() {
		return
	}

	go func() {
		ticker := time.NewTicker(interval)
		defer ticker.Stop()
		for {
			select {
			case <-ctx.Done():
				return
			case <-ticker.C:
				if err := OptimizeStorage(ctx, provider); err != nil {
					slog.Warn("Storage optimizer daemon encountered error", "error", err)
				}
			}
		}
	}()
}
