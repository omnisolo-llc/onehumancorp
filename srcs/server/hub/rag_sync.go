package hub

import (
    "context"
    "time"

    "go.opentelemetry.io/otel"
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
    Vector       []float32
    SyncStatus   SyncStatus
    LastSyncAt   time.Time
}

type RAGSyncService interface {
    FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error)
    MarkSynced(ctx context.Context, ids []string) error
    ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error
}

type ragSyncServiceImpl struct {
    dbProvider db.Provider
}

func NewRAGSyncService(dbProvider db.Provider) RAGSyncService {
    return &ragSyncServiceImpl{dbProvider: dbProvider}
}

func (s *ragSyncServiceImpl) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
    query := `
        SELECT memory_id, context, sync_status, last_sync_at
        FROM swarm_memory_embeddings
        WHERE sync_status = 'pending'
        LIMIT $1
    `
    rows, err := s.dbProvider.Query(ctx, query, limit)
    if err != nil {
        return nil, err
    }
    defer rows.Close()

    var records []RAGSyncRecord
    for rows.Next() {
        var rec RAGSyncRecord
        var lastSyncAt *time.Time
        // In this implementation we omit vector reading for simplicity.
        if err := rows.Scan(&rec.ID, &rec.Context, &rec.SyncStatus, &lastSyncAt); err != nil {
            return nil, err
        }
        if lastSyncAt != nil {
            rec.LastSyncAt = *lastSyncAt
        }
        records = append(records, rec)
    }

    return records, nil
}

func (s *ragSyncServiceImpl) MarkSynced(ctx context.Context, ids []string) error {
    // SQLite doesn't natively support ANY($1), so we loop using a transaction for cross-dialect compat.
    tx, err := s.dbProvider.Begin(ctx)
    if err != nil {
        return err
    }
    defer tx.Rollback(ctx)

    for _, id := range ids {
        _, err = tx.Exec(ctx, `
            UPDATE swarm_memory_embeddings
            SET sync_status = 'synced', last_sync_at = CURRENT_TIMESTAMP
            WHERE memory_id = $1
        `, id)
        if err != nil {
            SyncErrorsTotal.Add(ctx, 1)
            return err
        }
        RecordsSyncedTotal.Add(ctx, 1)
    }
    return tx.Commit(ctx)
}

func (s *ragSyncServiceImpl) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
    tx, err := s.dbProvider.Begin(ctx)
    if err != nil {
        return err
    }
    defer tx.Rollback(ctx)

    for _, rec := range records {
        _, err = tx.Exec(ctx, `
            INSERT INTO swarm_memory_embeddings (memory_id, context, sync_status, last_sync_at)
            VALUES ($1, $2, 'synced', CURRENT_TIMESTAMP)
            ON CONFLICT (memory_id) DO UPDATE SET
                context = EXCLUDED.context,
                sync_status = 'synced',
                last_sync_at = CURRENT_TIMESTAMP
        `, rec.ID, rec.Context)
        if err != nil {
            SyncErrorsTotal.Add(ctx, 1)
            return err
        }
    }
    return tx.Commit(ctx)
}

var (
    meter = otel.Meter("github.com/onehumancorp/mono/srcs/server/hub")
    RecordsSyncedTotal metric.Int64Counter
    SyncErrorsTotal    metric.Int64Counter
)

func init() {
    var err error
    RecordsSyncedTotal, err = meter.Int64Counter("rag_records_synced_total")
    if err != nil {
        panic(err)
    }
    SyncErrorsTotal, err = meter.Int64Counter("rag_sync_errors_total")
    if err != nil {
        panic(err)
    }
}
