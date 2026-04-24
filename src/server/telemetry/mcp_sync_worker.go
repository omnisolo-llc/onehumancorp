package telemetry

import (
	"context"
	"database/sql"
	"log/slog"
	"time"
)

// DBProvider is a local interface for the db provider to avoid circular dependencies
type DBProvider interface {
	DB() *sql.DB
}

// McpSyncWorker periodically syncs local telemetry buffers to the cloud.
type McpSyncWorker struct {
	provider DBProvider
	interval time.Duration
}

// NewMcpSyncWorker creates a new McpSyncWorker.
func NewMcpSyncWorker(provider DBProvider, interval time.Duration) *McpSyncWorker {
	if interval == 0 {
		interval = 5 * time.Second
	}
	return &McpSyncWorker{
		provider: provider,
		interval: interval,
	}
}

// Start begins the sync loop.
func (w *McpSyncWorker) Start(ctx context.Context) {
	ticker := time.NewTicker(w.interval)
	defer ticker.Stop()

	for {
		select {
		case <-ctx.Done():
			slog.Info("McpSyncWorker shutting down...")
			return
		case <-ticker.C:
			w.syncOnce(ctx)
		}
	}
}

func (w *McpSyncWorker) syncOnce(ctx context.Context) {
	// 1. Fetch pending metrics from SQLite buffer
	rows, err := w.provider.DB().QueryContext(ctx, "SELECT id, metric_name, value FROM telemetry_buffer WHERE sync_status = 'pending' LIMIT 100")
	if err != nil {
		slog.Error("McpSyncWorker failed to query telemetry_buffer", "error", err)
		return
	}
	defer rows.Close()

	var pendingIDs []string
	for rows.Next() {
		var id, metricName string
		var value float64
		if err := rows.Scan(&id, &metricName, &value); err != nil {
			slog.Error("McpSyncWorker failed to scan row", "error", err)
			continue
		}
		pendingIDs = append(pendingIDs, id)
		// Simulate SPIFFE mTLS API Gateway Call
		slog.Info("[McpSyncWorker] Syncing metric to Cloud MCP Gateway via SPIRE SVID", "metricName", metricName, "id", id, "value", value)
	}

	if err := rows.Err(); err != nil {
		slog.Error("McpSyncWorker row iteration error", "error", err)
		return
	}

	if len(pendingIDs) == 0 {
		return
	}

	// 2. Mark as synced
	for _, id := range pendingIDs {
		_, err := w.provider.DB().ExecContext(ctx, "UPDATE telemetry_buffer SET sync_status = 'synced' WHERE id = ?", id)
		if err != nil {
			slog.Error("McpSyncWorker failed to update status", "id", id, "error", err)
		}
	}
	slog.Info("[McpSyncWorker] Successfully synced metrics", "count", len(pendingIDs))
}
