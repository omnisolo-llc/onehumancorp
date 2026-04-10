package hub

import (
    "context"
    "database/sql"
    "fmt"
    "time"

    "github.com/onehumancorp/mono/srcs/server/db"
    "go.opentelemetry.io/otel/metric"
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

var (
    ragRecordsSyncedTotal metric.Int64Counter
    ragSyncErrorsTotal    metric.Int64Counter
)

func InitRAGSyncMetrics(meter metric.Meter) error {
    if meter == nil {
        return nil
    }
    var err error
    ragRecordsSyncedTotal, err = meter.Int64Counter("rag_records_synced_total", metric.WithDescription("Total RAG records synced"))
    if err != nil {
        return err
    }
    ragSyncErrorsTotal, err = meter.Int64Counter("rag_sync_errors_total", metric.WithDescription("Total RAG sync errors"))
    if err != nil {
        return err
    }
    return nil
}

type ragSyncService struct {
    provider db.Provider
}

func NewRAGSyncService(provider db.Provider) RAGSyncService {
    return &ragSyncService{provider: provider}
}

func (s *ragSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
    query := `SELECT memory_id, context, vector_embedding, sync_status, last_sync_at FROM swarm_memory_embeddings WHERE sync_status = 'pending' LIMIT $1`

    rows, err := s.provider.Query(ctx, query, limit)
    if err != nil {
        return nil, fmt.Errorf("failed to fetch pending syncs: %w", err)
    }
    defer rows.Close()

    var records []RAGSyncRecord
    for rows.Next() {
        var r RAGSyncRecord
        var lastSyncAt sql.NullTime
        if err := rows.Scan(&r.ID, &r.Context, &r.Vector, &r.SyncStatus, &lastSyncAt); err != nil {
            return nil, fmt.Errorf("failed to scan row: %w", err)
        }
        if lastSyncAt.Valid {
            r.LastSyncAt = lastSyncAt.Time
        }
        records = append(records, r)
    }
    return records, rows.Err()
}

func (s *ragSyncService) MarkSynced(ctx context.Context, ids []string) error {
    if len(ids) == 0 {
        return nil
    }

    tx, err := s.provider.Begin(ctx)
    if err != nil {
        return fmt.Errorf("failed to begin tx: %w", err)
    }
    defer tx.Rollback(ctx)

    for _, id := range ids {
        query := `UPDATE swarm_memory_embeddings SET sync_status = 'synced', last_sync_at = CURRENT_TIMESTAMP WHERE memory_id = $1`
        if _, err := tx.Exec(ctx, query, id); err != nil {
            return fmt.Errorf("failed to update sync status for id %s: %w", id, err)
        }
    }

    if err := tx.Commit(ctx); err != nil {
        return fmt.Errorf("failed to commit tx: %w", err)
    }

    if ragRecordsSyncedTotal != nil {
        ragRecordsSyncedTotal.Add(ctx, int64(len(ids)))
    }

    return nil
}

func (s *ragSyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
    if len(records) == 0 {
        return nil
    }

    tx, err := s.provider.Begin(ctx)
    if err != nil {
        return fmt.Errorf("failed to begin tx: %w", err)
    }
    defer tx.Rollback(ctx)

    for _, r := range records {
        query := `
            INSERT INTO swarm_memory_embeddings (memory_id, context, vector_embedding, sync_status, last_sync_at)
            VALUES ($1, $2, $3, $4, $5)
            ON CONFLICT (memory_id) DO UPDATE SET
                context = EXCLUDED.context,
                vector_embedding = EXCLUDED.vector_embedding,
                sync_status = EXCLUDED.sync_status,
                last_sync_at = EXCLUDED.last_sync_at
        `
        if _, err := tx.Exec(ctx, query, r.ID, r.Context, r.Vector, r.SyncStatus, r.LastSyncAt); err != nil {
            if ragSyncErrorsTotal != nil {
                ragSyncErrorsTotal.Add(ctx, 1)
            }
            return fmt.Errorf("failed to upsert record %s: %w", r.ID, err)
        }
    }

    if err := tx.Commit(ctx); err != nil {
        return fmt.Errorf("failed to commit tx: %w", err)
    }
    return nil
}
