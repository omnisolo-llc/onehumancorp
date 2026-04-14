package workers

import (
    "bytes"
    "context"
    "crypto/tls"
    "encoding/json"
    "log/slog"
    "net/http"
    "time"

    "github.com/onehumancorp/mono/srcs/server/db"
    "github.com/onehumancorp/mono/srcs/server/telemetry"
)

type HybridSyncWorker struct {
    pool     db.Provider
    client   *http.Client
    cloudURL string
}

func NewHybridSyncWorker(pool db.Provider, cloudURL string, tlsConfig *tls.Config) *HybridSyncWorker {
    transport := &http.Transport{
        TLSClientConfig: tlsConfig,
    }
    return &HybridSyncWorker{
        pool:     pool,
        client:   &http.Client{
            Timeout: 10 * time.Second,
            Transport: transport,
        },
        cloudURL: cloudURL,
    }
}

func (w *HybridSyncWorker) Start(ctx context.Context) {
    ticker := time.NewTicker(30 * time.Second)
    defer ticker.Stop()
    for {
        select {
        case <-ctx.Done():
            return
        case <-ticker.C:
            w.poll(ctx)
        }
    }
}

func (w *HybridSyncWorker) poll(ctx context.Context) {
    tx, err := w.pool.Begin(ctx)
    if err != nil {
        return
    }
    defer tx.Rollback(ctx)

    query := `SELECT memory_id, context FROM swarm_memory_embeddings WHERE sync_enabled = true AND sync_status = 'pending'`
    if !w.pool.IsSQLite() {
        query += ` FOR UPDATE SKIP LOCKED`
    }

    rows, err := tx.Query(ctx, query)
    if err != nil {
        return
    }

    var items []map[string]interface{}
    for rows.Next() {
        var id, contextData string
        if err := rows.Scan(&id, &contextData); err == nil {
            items = append(items, map[string]interface{}{
                "memory_id": id,
                "context":   contextData,
            })
        }
    }
    rows.Close()

    if len(items) == 0 {
        return
    }

    payloadBytes, err := json.Marshal(items)
    if err != nil {
        slog.Error("failed to marshal sync payload", "error", err)
        return
    }

    req, err := http.NewRequestWithContext(ctx, "POST", w.cloudURL+"/api/sync/vectors", bytes.NewReader(payloadBytes))
    if err != nil {
        slog.Error("failed to create sync request", "error", err)
        return
    }
    req.Header.Set("Content-Type", "application/json")

    resp, err := w.client.Do(req)
    if err != nil {
        slog.Error("failed to execute sync request", "error", err)
        return
    }
    defer resp.Body.Close()

    if resp.StatusCode != http.StatusOK {
        slog.Error("cloud sync request failed", "status", resp.StatusCode)
        return
    }

    synced := 0
    for _, item := range items {
        _, err := tx.Exec(ctx, `UPDATE swarm_memory_embeddings SET sync_status = 'synced', last_sync_at = CURRENT_TIMESTAMP WHERE memory_id = $1`, item["memory_id"])
        if err == nil {
            synced++
        }
    }
    _ = tx.Commit(ctx)

    if synced > 0 && telemetry.VectorsSyncedCount != nil {
        telemetry.VectorsSyncedCount.Add(ctx, int64(synced))
    }
}
