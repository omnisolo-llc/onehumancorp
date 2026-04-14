package telemetry

import (
    "context"
    "log/slog"
    "time"

    "github.com/onehumancorp/mono/lib/crypto"
    "github.com/onehumancorp/mono/srcs/server/db"
)

type McpSyncWorker struct {
    provider db.Provider
    endpoint string
}

func NewMcpSyncWorker(provider db.Provider, endpoint string) *McpSyncWorker {
    return &McpSyncWorker{
        provider: provider,
        endpoint: endpoint,
    }
}

func (w *McpSyncWorker) Start(ctx context.Context) {
    ticker := time.NewTicker(30 * time.Second)
    defer ticker.Stop()
    for {
        select {
        case <-ctx.Done():
            return
        case <-ticker.C:
            w.sync(ctx)
        }
    }
}

func (w *McpSyncWorker) sync(ctx context.Context) {
    if w.provider == nil {
        return
    }

    // Use local lib/crypto wrapper which stubs SPIFFE SVID retrieval
    svid, err := crypto.GetWorkloadSVID(ctx)
    if err != nil {
        slog.Warn("Failed to initialize SPIFFE Workload API source", "error", err)
    } else {
        slog.Info("Acquired SPIFFE SVID for MCP telemetry sync", "spiffe_id", svid)
    }

    query := `SELECT id, metric_name, value, labels_json, timestamp FROM telemetry_buffer WHERE sync_status = 'pending' LIMIT 100`
    rows, err := w.provider.Query(ctx, query)
    if err != nil {
        slog.Error("Failed to query telemetry buffer", "error", err)
        return
    }
    defer rows.Close()

    var ids []int
    for rows.Next() {
        var id int
        var name string
        var value float64
        var labels string
        var ts time.Time
        if err := rows.Scan(&id, &name, &value, &labels, &ts); err != nil {
            slog.Error("Failed to scan telemetry buffer row", "error", err)
            continue
        }
        ids = append(ids, id)
    }

    if len(ids) == 0 {
        return
    }

    // Simulate MCP upload
    slog.Info("Simulating MCP upload of telemetry metrics", "count", len(ids), "endpoint", w.endpoint)

    // Update to synced
    updateQuery := `UPDATE telemetry_buffer SET sync_status = 'synced' WHERE id = $1`
    if w.provider.IsSQLite() {
        updateQuery = `UPDATE telemetry_buffer SET sync_status = 'synced' WHERE id = ?`
    }
    for _, id := range ids {
        _, err := w.provider.Exec(ctx, updateQuery, id)
        if err != nil {
            slog.Error("Failed to update telemetry sync status", "error", err, "id", id)
        }
    }
}
