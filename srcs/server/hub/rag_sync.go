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

type RAGSyncServiceImpl struct {
    provider db.Provider
}

func NewRAGSyncService(provider db.Provider) *RAGSyncServiceImpl {
    return &RAGSyncServiceImpl{provider: provider}
}

func (s *RAGSyncServiceImpl) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
    query := `SELECT id, content, sync_status, last_sync_at FROM autodream_memories WHERE sync_status = 'pending' LIMIT $1`
    rows, err := s.provider.Query(ctx, query, limit)
    if err != nil {
        return nil, err
    }
    defer rows.Close()

    var records []RAGSyncRecord
    for rows.Next() {
        var record RAGSyncRecord
        var lastSyncAt *time.Time
        if err := rows.Scan(&record.ID, &record.Context, &record.SyncStatus, &lastSyncAt); err != nil {
            return nil, err
        }
        if lastSyncAt != nil {
            record.LastSyncAt = *lastSyncAt
        }
        records = append(records, record)
    }
    if err := rows.Err(); err != nil {
        return nil, err
    }

    return records, nil
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

    now := time.Now()
    for _, id := range ids {
        query := `UPDATE autodream_memories SET sync_status = 'synced', last_sync_at = $1 WHERE id = $2`
        if _, err := tx.Exec(ctx, query, now, id); err != nil {
            return err
        }
    }

    if err := tx.Commit(ctx); err != nil {
        return err
    }

    RagRecordsSyncedTotal.Add(ctx, int64(len(ids)))
    return nil
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
        // Upsert logic.
        // For simplicity, checking existence first (to be dialect agnostic)
        var exists bool
        checkQuery := `SELECT 1 FROM autodream_memories WHERE id = $1`
        if err := tx.QueryRow(ctx, checkQuery, r.ID).Scan(&exists); err == nil {
            // Update
            updateQuery := `UPDATE autodream_memories SET content = $1, sync_status = 'synced', last_sync_at = $2 WHERE id = $3`
            if _, err := tx.Exec(ctx, updateQuery, r.Context, time.Now(), r.ID); err != nil {
                RagSyncErrorsTotal.Add(ctx, 1)
                return err
            }
        } else {
            // Insert
            insertQuery := `INSERT INTO autodream_memories (id, content, sync_status, last_sync_at) VALUES ($1, $2, 'synced', $3)`
            if _, err := tx.Exec(ctx, insertQuery, r.ID, r.Context, time.Now()); err != nil {
                RagSyncErrorsTotal.Add(ctx, 1)
                return err
            }
        }
    }

    if err := tx.Commit(ctx); err != nil {
        return err
    }
    return nil
}

var (
    meter = otel.GetMeterProvider().Meter("github.com/onehumancorp/mono/srcs/server/hub")
    RagRecordsSyncedTotal metric.Int64Counter
    RagSyncErrorsTotal metric.Int64Counter
)

func init() {
    var err error
    RagRecordsSyncedTotal, err = meter.Int64Counter("rag_records_synced_total", metric.WithDescription("Total number of RAG records synced successfully"))
    if err != nil {
        panic(err)
    }
    RagSyncErrorsTotal, err = meter.Int64Counter("rag_sync_errors_total", metric.WithDescription("Total number of RAG sync errors"))
    if err != nil {
        panic(err)
    }
}
