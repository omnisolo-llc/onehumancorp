package hub

import (
    "context"
    "fmt"
    "strings"
    "time"

    "github.com/onehumancorp/mono/srcs/server/db"
    "go.opentelemetry.io/otel"
    "go.opentelemetry.io/otel/metric"
)

var (
    meter            = otel.Meter("github.com/onehumancorp/mono/srcs/server/hub")
    recordsSynced, _ = meter.Int64Counter("rag_records_synced_total")
    syncErrors, _    = meter.Int64Counter("rag_sync_errors_total")
)

type SyncStatus string

const (
    SyncStatusPending SyncStatus = "pending"
    SyncStatusSynced  SyncStatus = "synced"
    SyncStatusError   SyncStatus = "error"
)

type RAGSyncRecord struct {
    ID         string
    Context    string
    Vector     []byte
    SyncStatus SyncStatus
    LastSyncAt time.Time
}

type RAGSyncService interface {
    FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error)
    MarkSynced(ctx context.Context, ids []string) error
    ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error
}

type ragSyncService struct {
    db db.Provider
}

func NewRAGSyncService(provider db.Provider) RAGSyncService {
    return &ragSyncService{
        db: provider,
    }
}

func (s *ragSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
    var query string
    if s.db.IsSQLite() {
        query = "SELECT memory_id, context, vector_embedding, sync_status, last_sync_at FROM swarm_memory_embeddings WHERE sync_status = 'pending' LIMIT $1"
    } else {
        query = "SELECT memory_id, context, vector_embedding, sync_status, last_sync_at FROM swarm_memory_embeddings WHERE sync_status = 'pending' LIMIT $1 FOR UPDATE SKIP LOCKED"
    }

    rows, err := s.db.Query(ctx, query, limit)
    if err != nil {
        syncErrors.Add(ctx, 1, metric.WithAttributes())
        return nil, fmt.Errorf("failed to query pending syncs: %w", err)
    }
    defer rows.Close()

    var records []RAGSyncRecord
    for rows.Next() {
        var rec RAGSyncRecord
        var lastSyncAt *time.Time
        if err := rows.Scan(&rec.ID, &rec.Context, &rec.Vector, &rec.SyncStatus, &lastSyncAt); err != nil {
            syncErrors.Add(ctx, 1, metric.WithAttributes())
            return nil, fmt.Errorf("failed to scan record: %w", err)
        }
        if lastSyncAt != nil {
            rec.LastSyncAt = *lastSyncAt
        }
        records = append(records, rec)
    }
    return records, rows.Err()
}

func (s *ragSyncService) MarkSynced(ctx context.Context, ids []string) error {
    if len(ids) == 0 {
        return nil
    }
    placeholders := make([]string, len(ids))
    args := make([]any, len(ids)+1)
    args[0] = time.Now()
    for i, id := range ids {
        placeholders[i] = fmt.Sprintf("$%d", i+2)
        args[i+1] = id
    }

    query := fmt.Sprintf("UPDATE swarm_memory_embeddings SET sync_status = 'synced', last_sync_at = $1 WHERE memory_id IN (%s)", strings.Join(placeholders, ","))

    _, err := s.db.Exec(ctx, query, args...)
    if err != nil {
        syncErrors.Add(ctx, 1, metric.WithAttributes())
        return err
    }
    recordsSynced.Add(ctx, int64(len(ids)), metric.WithAttributes())
    return nil
}

func (s *ragSyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
    tx, err := s.db.Begin(ctx)
    if err != nil {
        syncErrors.Add(ctx, 1, metric.WithAttributes())
        return err
    }
    defer tx.Rollback(ctx)

    for _, rec := range records {
        var err error
        if s.db.IsSQLite() {
            query := `
                INSERT INTO swarm_memory_embeddings (memory_id, context, vector_embedding, sync_status, last_sync_at)
                VALUES ($1, $2, $3, $4, $5)
                ON CONFLICT(memory_id) DO UPDATE SET
                context=excluded.context, vector_embedding=excluded.vector_embedding, sync_status=excluded.sync_status, last_sync_at=excluded.last_sync_at
            `
            _, err = tx.Exec(ctx, query, rec.ID, rec.Context, rec.Vector, string(rec.SyncStatus), rec.LastSyncAt)
        } else {
            query := `
                INSERT INTO swarm_memory_embeddings (memory_id, context, vector_embedding, sync_status, last_sync_at)
                VALUES ($1, $2, $3, $4, $5)
                ON CONFLICT (memory_id) DO UPDATE SET
                context=EXCLUDED.context, vector_embedding=EXCLUDED.vector_embedding, sync_status=EXCLUDED.sync_status, last_sync_at=EXCLUDED.last_sync_at
            `
            _, err = tx.Exec(ctx, query, rec.ID, rec.Context, rec.Vector, string(rec.SyncStatus), rec.LastSyncAt)
        }
        if err != nil {
            syncErrors.Add(ctx, 1, metric.WithAttributes())
            return err
        }
    }

    if err := tx.Commit(ctx); err != nil {
        syncErrors.Add(ctx, 1, metric.WithAttributes())
        return err
    }
    return nil
}
