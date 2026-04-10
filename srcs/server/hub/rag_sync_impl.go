package hub

import (
    "context"
    "encoding/json"
    "time"

    "github.com/onehumancorp/mono/srcs/server/db"
    "github.com/onehumancorp/mono/srcs/server/telemetry"
)

type RAGSyncServiceImpl struct {
    provider db.Provider
}

func NewRAGSyncService(provider db.Provider) RAGSyncService {
    return &RAGSyncServiceImpl{provider: provider}
}

func (s *RAGSyncServiceImpl) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
    query := "SELECT id, content, sync_status, last_sync_at FROM autodream_memories WHERE sync_status = 'pending' LIMIT $1"
    rows, err := s.provider.Query(ctx, query, limit)
    if err != nil {
        return nil, err
    }
    defer rows.Close()

    var records []RAGSyncRecord
    for rows.Next() {
        var r RAGSyncRecord
        var lastSyncAt *time.Time
        if err := rows.Scan(&r.ID, &r.Context, &r.SyncStatus, &lastSyncAt); err != nil {
            return nil, err
        }
        if lastSyncAt != nil {
            r.LastSyncAt = *lastSyncAt
        }
        // Skipping vector selection for simplicity of extraction, but it can be implemented if required by fetch logic
        records = append(records, r)
    }
    return records, rows.Err()
}

func (s *RAGSyncServiceImpl) MarkSynced(ctx context.Context, ids []string) error {
    if len(ids) == 0 {
        return nil
    }

    tx, err := s.provider.Begin(ctx)
    if err != nil {
        return err
    }
    defer tx.Rollback(ctx)

    if s.provider.IsSQLite() {
        // SQLite doesn't natively support ANY($1) array operators easily with standard go-sqlite3 driver
        // without json_each or complex binds. A loop within a single transaction is acceptable for SQLite
        query := "UPDATE autodream_memories SET sync_status = 'synced', last_sync_at = CURRENT_TIMESTAMP WHERE id = $1"
        for _, id := range ids {
            if _, err := tx.Exec(ctx, query, id); err != nil {
                return err
            }
        }
    } else {
        query := "UPDATE autodream_memories SET sync_status = 'synced', last_sync_at = CURRENT_TIMESTAMP WHERE id = ANY($1)"
        if _, err := tx.Exec(ctx, query, ids); err != nil {
            return err
        }
    }

    err = tx.Commit(ctx)
    if err == nil {
        telemetry.RecordRagRecordsSynced(ctx, int64(len(ids)))
    } else {
        telemetry.RecordRagSyncError(ctx)
    }
    return err
}

func (s *RAGSyncServiceImpl) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
    if len(records) == 0 {
        return nil
    }

    tx, err := s.provider.Begin(ctx)
    if err != nil {
        return err
    }
    defer tx.Rollback(ctx)

    for _, r := range records {
        vecBytes, err := json.Marshal(r.Vector)
        if err != nil {
            return err
        }
        vecStr := string(vecBytes)

        query := "INSERT INTO autodream_memories (id, content, embedding, sync_status, last_sync_at) VALUES ($1, $2, $3, 'synced', CURRENT_TIMESTAMP) ON CONFLICT(id) DO UPDATE SET content=EXCLUDED.content, embedding=EXCLUDED.embedding, sync_status='synced', last_sync_at=CURRENT_TIMESTAMP"
        if _, err := tx.Exec(ctx, query, r.ID, r.Context, vecStr); err != nil {
            return err
        }
    }

    err = tx.Commit(ctx)
    if err == nil {
        telemetry.RecordRagRecordsSynced(ctx, int64(len(records)))
    } else {
        telemetry.RecordRagSyncError(ctx)
    }
    return err
}
