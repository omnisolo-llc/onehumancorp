package hub

import (
    "context"
    "fmt"
    "time"

    "github.com/onehumancorp/mono/srcs/server/db"
    "go.opentelemetry.io/otel"
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
    Vector       []float32 // Convert to string internally for SQLite compat if needed
    SyncStatus   SyncStatus
    LastSyncAt   time.Time
}

type RAGSyncService interface {
    // FetchPendingSyncs retrieves records from the local DB that need syncing
    FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error)

    // MarkSynced updates the local DB after a successful sync to the cloud
    MarkSynced(ctx context.Context, ids []string) error

    // ProcessIncomingSync handles data pushed from a standalone client into the cloud DB
    ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error
}

var (
    meter = otel.Meter("github.com/onehumancorp/mono/srcs/server/hub")
    ragRecordsSyncedTotal, _ = meter.Int64Counter("rag_records_synced_total", metric.WithDescription("Total RAG records synced"))
    ragSyncErrorsTotal, _  = meter.Int64Counter("rag_sync_errors_total", metric.WithDescription("Total RAG sync errors"))
)

type ragSyncServiceImpl struct {
    dbProvider db.Provider
}

func NewRAGSyncService(dbProvider db.Provider) RAGSyncService {
    return &ragSyncServiceImpl{
        dbProvider: dbProvider,
    }
}

func (s *ragSyncServiceImpl) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
    if s.dbProvider == nil {
        return nil, fmt.Errorf("db provider is nil")
    }

    query := "SELECT memory_id, context, sync_status, last_sync_at FROM swarm_memory_embeddings WHERE sync_status = $1 LIMIT $2"

    // In PostgreSQL/SQLite, the query parameter syntax might differ slightly (e.g. $1 vs ?).
    // db.Provider abstracts this, usually using positional parameters or we assume PostgreSQL-style works.

    rows, err := s.dbProvider.Query(ctx, query, string(SyncStatusPending), limit)
    if err != nil {
        ragSyncErrorsTotal.Add(ctx, 1)
        return nil, fmt.Errorf("querying pending syncs: %w", err)
    }
    defer rows.Close()

    var records []RAGSyncRecord
    for rows.Next() {
        var rec RAGSyncRecord
        var lastSyncAt *time.Time
        var statusStr string
        if err := rows.Scan(&rec.ID, &rec.Context, &statusStr, &lastSyncAt); err != nil {
            ragSyncErrorsTotal.Add(ctx, 1)
            continue
        }
        rec.SyncStatus = SyncStatus(statusStr)
        if lastSyncAt != nil {
            rec.LastSyncAt = *lastSyncAt
        }
        records = append(records, rec)
    }

    if err := rows.Err(); err != nil {
        ragSyncErrorsTotal.Add(ctx, 1)
        return nil, fmt.Errorf("iterating pending syncs: %w", err)
    }

    return records, nil
}

func (s *ragSyncServiceImpl) MarkSynced(ctx context.Context, ids []string) error {
    if len(ids) == 0 {
        return nil
    }

    if s.dbProvider == nil {
        return fmt.Errorf("db provider is nil")
    }

    tx, err := s.dbProvider.Begin(ctx)
    if err != nil {
        ragSyncErrorsTotal.Add(ctx, 1)
        return err
    }
    defer tx.Rollback(ctx)

    for _, id := range ids {
        query := "UPDATE swarm_memory_embeddings SET sync_status = $1, last_sync_at = $2 WHERE memory_id = $3"
        _, err := tx.Exec(ctx, query, string(SyncStatusSynced), time.Now(), id)
        if err != nil {
            ragSyncErrorsTotal.Add(ctx, 1)
            return err
        }
        ragRecordsSyncedTotal.Add(ctx, 1)
    }

    if err := tx.Commit(ctx); err != nil {
        ragSyncErrorsTotal.Add(ctx, 1)
        return err
    }

    return nil
}

func (s *ragSyncServiceImpl) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
    if len(records) == 0 {
        return nil
    }

    if s.dbProvider == nil {
        return fmt.Errorf("db provider is nil")
    }

    tx, err := s.dbProvider.Begin(ctx)
    if err != nil {
        ragSyncErrorsTotal.Add(ctx, 1)
        return err
    }
    defer tx.Rollback(ctx)

    for _, rec := range records {
        query := `
            INSERT INTO swarm_memory_embeddings (memory_id, context, sync_status, last_sync_at)
            VALUES ($1, $2, $3, $4)
            ON CONFLICT (memory_id) DO UPDATE SET
            context = EXCLUDED.context,
            sync_status = EXCLUDED.sync_status,
            last_sync_at = EXCLUDED.last_sync_at`

        _, err := tx.Exec(ctx, query, rec.ID, rec.Context, string(SyncStatusSynced), time.Now())
        if err != nil {
            ragSyncErrorsTotal.Add(ctx, 1)
            return err
        }
    }

    if err := tx.Commit(ctx); err != nil {
        ragSyncErrorsTotal.Add(ctx, 1)
        return err
    }

    return nil
}
