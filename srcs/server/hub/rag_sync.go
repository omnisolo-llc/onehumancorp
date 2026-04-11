package hub

import (
    "context"
    "time"
    "encoding/json"

    "go.opentelemetry.io/otel/metric"
    "github.com/onehumancorp/mono/srcs/server/db"
)

type SyncStatus string

const (
    SyncStatusPending SyncStatus = "pending"
    SyncStatusSynced  SyncStatus = "synced"
    SyncStatusError   SyncStatus = "error"
)

type RAGSyncRecord struct {
    ID           string
    Context      string
    Vector       []float32 // Convert to string internally for SQLite compat if needed
    SyncStatus   SyncStatus
    LastSyncAt   time.Time
}

type RAGSyncService interface {
    FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error)
    MarkSynced(ctx context.Context, ids []string) error
    ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error
}

type DefaultRAGSyncService struct {
    provider db.Provider
    syncsTotal metric.Int64Counter
    errorsTotal metric.Int64Counter
}

func NewDefaultRAGSyncService(provider db.Provider, meter metric.Meter) (*DefaultRAGSyncService, error) {
    syncs, err := meter.Int64Counter("rag_records_synced_total")
    if err != nil {
        return nil, err
    }
    errors, err := meter.Int64Counter("rag_sync_errors_total")
    if err != nil {
        return nil, err
    }
    return &DefaultRAGSyncService{
        provider: provider,
        syncsTotal: syncs,
        errorsTotal: errors,
    }, nil
}

func (s *DefaultRAGSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
    q := "SELECT id, content, embedding, sync_status, last_sync_at FROM autodream_memories WHERE sync_status = 'pending' LIMIT $1"
    rows, err := s.provider.Query(ctx, q, limit)
    if err != nil {
        s.errorsTotal.Add(ctx, 1)
        return nil, err
    }
    defer rows.Close()

    var records []RAGSyncRecord
    for rows.Next() {
        var r RAGSyncRecord
        var lastSyncAtStr *string
        var lastSyncAtTime *time.Time
        var embeddingStr *string

        if s.provider.IsSQLite() {
            if err := rows.Scan(&r.ID, &r.Context, &embeddingStr, &r.SyncStatus, &lastSyncAtStr); err != nil {
                s.errorsTotal.Add(ctx, 1)
                return nil, err
            }
            if lastSyncAtStr != nil {
                t, err := time.Parse("2006-01-02 15:04:05.999999999-07:00", *lastSyncAtStr)
                if err == nil {
                    r.LastSyncAt = t
                } else {
                    t2, err2 := time.Parse("2006-01-02 15:04:05", *lastSyncAtStr)
                    if err2 == nil {
                        r.LastSyncAt = t2
                    }
                }
            }
        } else {
            if err := rows.Scan(&r.ID, &r.Context, &embeddingStr, &r.SyncStatus, &lastSyncAtTime); err != nil {
                s.errorsTotal.Add(ctx, 1)
                return nil, err
            }
            if lastSyncAtTime != nil {
                r.LastSyncAt = *lastSyncAtTime
            }
        }

        if embeddingStr != nil {
            json.Unmarshal([]byte(*embeddingStr), &r.Vector)
        }

        records = append(records, r)
    }
    return records, nil
}

func (s *DefaultRAGSyncService) MarkSynced(ctx context.Context, ids []string) error {
    if len(ids) == 0 {
        return nil
    }
    tx, err := s.provider.Begin(ctx)
    if err != nil {
        s.errorsTotal.Add(ctx, 1)
        return err
    }
    defer tx.Rollback(ctx)

    for _, id := range ids {
        _, err = tx.Exec(ctx, "UPDATE autodream_memories SET sync_status = 'synced', last_sync_at = CURRENT_TIMESTAMP WHERE id = $1", id)
        if err != nil {
            s.errorsTotal.Add(ctx, 1)
            return err
        }
    }

    if err := tx.Commit(ctx); err != nil {
        s.errorsTotal.Add(ctx, 1)
        return err
    }

    s.syncsTotal.Add(ctx, int64(len(ids)))
    return nil
}

func (s *DefaultRAGSyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
    if len(records) == 0 {
        return nil
    }
    tx, err := s.provider.Begin(ctx)
    if err != nil {
        s.errorsTotal.Add(ctx, 1)
        return err
    }
    defer tx.Rollback(ctx)

    for _, r := range records {
        var emb string
        if s.provider.IsSQLite() {
            embBytes, _ := json.Marshal(r.Vector)
            emb = string(embBytes)
            _, err = tx.Exec(ctx, "INSERT INTO autodream_memories (id, content, embedding, sync_status, last_sync_at) VALUES ($1, $2, $3, 'synced', CURRENT_TIMESTAMP) ON CONFLICT(id) DO UPDATE SET content = EXCLUDED.content, embedding = EXCLUDED.embedding, sync_status = 'synced', last_sync_at = CURRENT_TIMESTAMP", r.ID, r.Context, emb)
        } else {
            embBytes, _ := json.Marshal(r.Vector)
            embStr := string(embBytes)
            _, err = tx.Exec(ctx, "INSERT INTO autodream_memories (id, content, embedding, sync_status, last_sync_at) VALUES ($1, $2, $3::vector, 'synced', CURRENT_TIMESTAMP) ON CONFLICT(id) DO UPDATE SET content = EXCLUDED.content, embedding = EXCLUDED.embedding, sync_status = 'synced', last_sync_at = CURRENT_TIMESTAMP", r.ID, r.Context, embStr)
        }

        if err != nil {
            s.errorsTotal.Add(ctx, 1)
            return err
        }
    }

    if err := tx.Commit(ctx); err != nil {
        s.errorsTotal.Add(ctx, 1)
        return err
    }

    s.syncsTotal.Add(ctx, int64(len(records)))
    return nil
}
