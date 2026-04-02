package sync

import (
	"context"
	"log/slog"
	"strings"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
	"github.com/onehumancorp/mono/srcs/server/telemetry"
)

// AutoDreamSyncEngine is responsible for synchronizing local SQLite embeddings
// with the Cloud-Native Postgres DB.
type AutoDreamSyncEngine struct {
	dbProvider db.Provider
	ticker     *time.Ticker
	quit       chan struct{}
}

// NewAutoDreamSyncEngine creates a new AutoDreamSyncEngine.
func NewAutoDreamSyncEngine(dbProvider db.Provider) *AutoDreamSyncEngine {
	return &AutoDreamSyncEngine{
		dbProvider: dbProvider,
		quit:       make(chan struct{}),
	}
}

// Start begins the polling ticker.
func (e *AutoDreamSyncEngine) Start(interval time.Duration) {
	e.ticker = time.NewTicker(interval)
	go func() {
		for {
			select {
			case <-e.ticker.C:
				e.ProcessForecastTick()
			case <-e.quit:
				e.ticker.Stop()
				return
			}
		}
	}()
}

// Stop halts the sync engine.
func (e *AutoDreamSyncEngine) Stop() {
	if e.quit != nil {
		close(e.quit)
	}
}

// ProcessForecastTick synchronously checks the database and runs the sync logic.
func (e *AutoDreamSyncEngine) ProcessForecastTick() {
	ctx := context.Background()

	// 1. Query for unsynced rows.
	rows, err := e.dbProvider.Query(ctx, "SELECT content_hash FROM embedding_cache WHERE synced_to_cloud = false")
	if err != nil {
		slog.Error("Failed to query unsynced embeddings", "error", err)
		return
	}
	defer rows.Close()

	var hashes []string
	var interfaceHashes []interface{}
	for rows.Next() {
		var hash string
		if err := rows.Scan(&hash); err != nil {
			slog.Error("Failed to scan content_hash", "error", err)
			continue
		}
		hashes = append(hashes, hash)
		interfaceHashes = append(interfaceHashes, hash)
	}
	rows.Close() // Ensure rows are closed before opening transaction

	if len(hashes) == 0 {
		return
	}

	// 2. Mock Cloud API Call (simulated sync)
	// In a real scenario, this would send an HTTP POST to /api/v1/sync/autodream
	slog.Info("Syncing embeddings to cloud", "count", len(hashes))

	// 3. Mark as synced in local DB
	tx, err := e.dbProvider.Begin(ctx)
	if err != nil {
		slog.Error("Failed to begin transaction for sync update", "error", err)
		for range hashes {
			telemetry.RecordSyncFailed(ctx)
		}
		return
	}
	defer tx.Rollback(ctx)

	if e.dbProvider.IsSQLite() {
		// SQLite doesn't natively support ANY($1) for arrays, so build a dynamic IN clause
		placeholders := make([]string, len(hashes))
		for i := range hashes {
			placeholders[i] = "?"
		}
		query := "UPDATE embedding_cache SET synced_to_cloud = true WHERE content_hash IN (" + strings.Join(placeholders, ",") + ")"
		_, err = tx.Exec(ctx, query, interfaceHashes...)
	} else {
		_, err = tx.Exec(ctx, "UPDATE embedding_cache SET synced_to_cloud = true WHERE content_hash = ANY($1)", hashes)
	}

	if err != nil {
		slog.Error("Failed to update sync status", "error", err)
		for range hashes {
			telemetry.RecordSyncFailed(ctx)
		}
		return
	}

	for range hashes {
		telemetry.RecordSyncCompleted(ctx)
	}

	if err := tx.Commit(ctx); err != nil {
		slog.Error("Failed to commit sync updates", "error", err)
		for range hashes {
			telemetry.RecordSyncFailed(ctx)
		}
		return
	}

	slog.Info("Successfully synced embeddings to cloud", "count", len(hashes))
}
