package hub

import (
    "context"
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
    Vector       []float32
    SyncStatus   SyncStatus
    LastSyncAt   time.Time
}

type RAGSyncService interface {
    FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error)
    MarkSynced(ctx context.Context, ids []string) error
    ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error
}

var meter = otel.Meter("github.com/onehumancorp/mono/srcs/server/hub")
var recordsSyncedTotal, _ = meter.Int64Counter("rag_records_synced_total", metric.WithDescription("Total RAG records synced"))
var syncErrorsTotal, _ = meter.Int64Counter("rag_sync_errors_total", metric.WithDescription("Total errors during RAG sync"))

type DefaultRAGSyncService struct {
    DB *db.DB
}

func NewRAGSyncService(database *db.DB) *DefaultRAGSyncService {
    return &DefaultRAGSyncService{DB: database}
}

func (s *DefaultRAGSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
    query := `SELECT id, content, sync_status, last_sync_at FROM consolidated_memory WHERE sync_status = 'pending' LIMIT $1`
    rows, err := s.DB.Query(ctx, query, limit)
    if err != nil {
        return nil, err
    }
    defer rows.Close()

    var records []RAGSyncRecord
    for rows.Next() {
        var rec RAGSyncRecord
        var lastSyncAt *time.Time
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

func (s *DefaultRAGSyncService) MarkSynced(ctx context.Context, ids []string) error {
    if len(ids) == 0 {
        return nil
    }
    tx, err := s.DB.Provider.Begin(ctx)
    if err != nil {
        return err
    }

    now := time.Now()
    for _, id := range ids {
        query := `UPDATE consolidated_memory SET sync_status = 'synced', last_sync_at = $1 WHERE id = $2`
        _, err := tx.Exec(ctx, query, now, id)
        if err != nil {
            syncErrorsTotal.Add(ctx, 1)
            tx.Rollback(ctx)
            return err
        }
    }

    if err := tx.Commit(ctx); err != nil {
        tx.Rollback(ctx)
        return err
    }

    recordsSyncedTotal.Add(ctx, int64(len(ids)))
    return nil
}

func (s *DefaultRAGSyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
    if len(records) == 0 {
        return nil
    }
    tx, err := s.DB.Provider.Begin(ctx)
    if err != nil {
        return err
    }

    for _, rec := range records {
        // Upsert syntax
        query := `INSERT INTO consolidated_memory (id, organization_id, content, source_type, sync_status, last_sync_at)
                  VALUES ($1, 'default', $2, 'cloud_sync', 'synced', $3)
                  ON CONFLICT (id) DO UPDATE SET content = EXCLUDED.content, sync_status = EXCLUDED.sync_status, last_sync_at = EXCLUDED.last_sync_at`

        _, err := tx.Exec(ctx, query, rec.ID, rec.Context, rec.LastSyncAt)
        if err != nil {
            syncErrorsTotal.Add(ctx, 1)
            tx.Rollback(ctx)
            return err
        }
    }

    if err := tx.Commit(ctx); err != nil {
        tx.Rollback(ctx)
        return err
    }

    recordsSyncedTotal.Add(ctx, int64(len(records)))
    return nil
}
