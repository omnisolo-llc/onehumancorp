package hub

import (
    "context"
    "log/slog"
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

type ragSyncService struct {
    dbProvider db.Provider
}

func NewRAGSyncService(provider db.Provider) RAGSyncService {
    return &ragSyncService{dbProvider: provider}
}

var (
    ragRecordsSyncedTotal metric.Int64Counter
    ragSyncErrorsTotal    metric.Int64Counter
)

func init() {
    meter := otel.Meter("github.com/onehumancorp/mono/srcs/server/hub")
    var err error
    ragRecordsSyncedTotal, err = meter.Int64Counter("rag_records_synced_total", metric.WithDescription("Total RAG records successfully synced"))
    if err != nil {
        slog.Error("Failed to initialize rag_records_synced_total metric", "error", err)
    }
    ragSyncErrorsTotal, err = meter.Int64Counter("rag_sync_errors_total", metric.WithDescription("Total RAG sync errors"))
    if err != nil {
        slog.Error("Failed to initialize rag_sync_errors_total metric", "error", err)
    }
}

func (s *ragSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
    query := `SELECT id, content, sync_status, last_sync_at FROM autodream_memories WHERE sync_status = 'pending' LIMIT $1`
    rows, err := s.dbProvider.Query(ctx, query, limit)
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
        records = append(records, r)
    }
    return records, rows.Err()
}

func (s *ragSyncService) MarkSynced(ctx context.Context, ids []string) error {
    if len(ids) == 0 {
        return nil
    }

    tx, err := s.dbProvider.Begin(ctx)
    if err != nil {
        return err
    }
    defer tx.Rollback(ctx)

    now := time.Now()
    query := `UPDATE autodream_memories SET sync_status = 'synced', last_sync_at = $1 WHERE id = $2`
    for _, id := range ids {
        _, err := tx.Exec(ctx, query, now, id)
        if err != nil {
            return err
        }
    }
    if err := tx.Commit(ctx); err != nil {
        return err
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

    tx, err := s.dbProvider.Begin(ctx)
    if err != nil {
        if ragSyncErrorsTotal != nil {
            ragSyncErrorsTotal.Add(ctx, 1)
        }
        return err
    }
    defer tx.Rollback(ctx)

    // For this test we will just insert or update them in autodream_memories
    query := `INSERT INTO autodream_memories (id, content, sync_status, last_sync_at) VALUES ($1, $2, $3, $4)
              ON CONFLICT(id) DO UPDATE SET content=EXCLUDED.content, sync_status=EXCLUDED.sync_status, last_sync_at=EXCLUDED.last_sync_at`

    for _, r := range records {
        var lastSyncAt *time.Time
        if !r.LastSyncAt.IsZero() {
            lastSyncAt = &r.LastSyncAt
        }
        _, err := tx.Exec(ctx, query, r.ID, r.Context, r.SyncStatus, lastSyncAt)
        if err != nil {
            if ragSyncErrorsTotal != nil {
                ragSyncErrorsTotal.Add(ctx, 1)
            }
            return err
        }
    }
    if err := tx.Commit(ctx); err != nil {
        if ragSyncErrorsTotal != nil {
            ragSyncErrorsTotal.Add(ctx, 1)
        }
        return err
    }
    return nil
}
