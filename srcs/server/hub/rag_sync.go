package hub

import (
    "context"
    "fmt"
    "time"

    "github.com/onehumancorp/mono/srcs/server/db"
    "go.opentelemetry.io/otel"
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
    Vector       []byte
    SyncStatus   SyncStatus
    LastSyncAt   time.Time
}

type RAGSyncService interface {
    FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error)
    MarkSynced(ctx context.Context, ids []string) error
    ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error
}

type ragSyncServiceImpl struct {
    provider db.Provider
}

var (
    meter            = otel.Meter("github.com/onehumancorp/mono/srcs/server/hub")
    syncedCounter, _ = meter.Int64Counter("rag_records_synced_total")
    errorsCounter, _ = meter.Int64Counter("rag_sync_errors_total")
)

func NewRAGSyncService(provider db.Provider) RAGSyncService {
    return &ragSyncServiceImpl{provider: provider}
}

func (s *ragSyncServiceImpl) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
    tx, err := s.provider.Begin(ctx)
    if err != nil {
        return nil, fmt.Errorf("begin tx: %w", err)
    }
    defer tx.Rollback(ctx)

    var query string
    if s.provider.IsSQLite() {
        query = "SELECT memory_id, context, vector_embedding, sync_status, last_sync_at FROM swarm_memory_embeddings WHERE sync_status = 'pending' LIMIT $1"
    } else {
        query = "SELECT memory_id, context, vector_embedding, sync_status, last_sync_at FROM swarm_memory_embeddings WHERE sync_status = 'pending' LIMIT $1 FOR UPDATE SKIP LOCKED"
    }

    rows, err := tx.Query(ctx, query, limit)
    if err != nil {
        return nil, fmt.Errorf("query: %w", err)
    }
    defer rows.Close()

    var records []RAGSyncRecord
    for rows.Next() {
        var rec RAGSyncRecord
        var lastSyncAt *time.Time
        var status *string
        if err := rows.Scan(&rec.ID, &rec.Context, &rec.Vector, &status, &lastSyncAt); err != nil {
            return nil, fmt.Errorf("scan: %w", err)
        }
        if status != nil {
            rec.SyncStatus = SyncStatus(*status)
        } else {
            rec.SyncStatus = SyncStatusPending
        }
        if lastSyncAt != nil {
            rec.LastSyncAt = *lastSyncAt
        }
        records = append(records, rec)
    }

    if !s.provider.IsSQLite() && len(records) > 0 {
        var ids []string
        for _, r := range records {
            ids = append(ids, r.ID)
        }
        for _, id := range ids {
            _, err := tx.Exec(ctx, "UPDATE swarm_memory_embeddings SET sync_status = 'in_progress' WHERE memory_id = $1", id)
            if err != nil {
                return nil, fmt.Errorf("update in_progress: %w", err)
            }
        }
    }

    if err := tx.Commit(ctx); err != nil {
        return nil, fmt.Errorf("commit: %w", err)
    }

    return records, nil
}

func (s *ragSyncServiceImpl) MarkSynced(ctx context.Context, ids []string) error {
    if len(ids) == 0 {
        return nil
    }

    tx, err := s.provider.Begin(ctx)
    if err != nil {
        return fmt.Errorf("begin tx: %w", err)
    }
    defer tx.Rollback(ctx)

    for _, id := range ids {
        _, err := tx.Exec(ctx, "UPDATE swarm_memory_embeddings SET sync_status = 'synced', last_sync_at = CURRENT_TIMESTAMP WHERE memory_id = $1", id)
        if err != nil {
            errorsCounter.Add(ctx, 1)
            return fmt.Errorf("update synced: %w", err)
        }
    }

    if err := tx.Commit(ctx); err != nil {
        errorsCounter.Add(ctx, 1)
        return fmt.Errorf("commit: %w", err)
    }

    syncedCounter.Add(ctx, int64(len(ids)))
    return nil
}

func (s *ragSyncServiceImpl) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
    if len(records) == 0 {
        return nil
    }

    tx, err := s.provider.Begin(ctx)
    if err != nil {
        return fmt.Errorf("begin tx: %w", err)
    }
    defer tx.Rollback(ctx)

    for _, rec := range records {
        _, err := tx.Exec(ctx, `
            INSERT INTO swarm_memory_embeddings (memory_id, context, vector_embedding, sync_status, last_sync_at)
            VALUES ($1, $2, $3, 'synced', CURRENT_TIMESTAMP)
            ON CONFLICT (memory_id) DO UPDATE SET
                context = EXCLUDED.context,
                vector_embedding = EXCLUDED.vector_embedding,
                sync_status = 'synced',
                last_sync_at = CURRENT_TIMESTAMP
        `, rec.ID, rec.Context, rec.Vector)
        if err != nil {
            errorsCounter.Add(ctx, 1)
            return fmt.Errorf("upsert: %w", err)
        }
    }

    if err := tx.Commit(ctx); err != nil {
        errorsCounter.Add(ctx, 1)
        return fmt.Errorf("commit: %w", err)
    }

    syncedCounter.Add(ctx, int64(len(records)))
    return nil
}
