package hub

import (
    "context"
    "database/sql"
    "encoding/json"
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
    Vector       []float32
    SyncStatus   SyncStatus
    LastSyncAt   time.Time
}

var (
    RagRecordsSyncedTotal metric.Int64Counter
    RagSyncErrorsTotal    metric.Int64Counter
)

func InitRAGSyncMetrics(meter metric.Meter) error {
    var err error
    RagRecordsSyncedTotal, err = meter.Int64Counter(
        "rag_records_synced_total",
        metric.WithDescription("Total number of RAG records synced"),
    )
    if err != nil {
        return err
    }
    RagSyncErrorsTotal, err = meter.Int64Counter(
        "rag_sync_errors_total",
        metric.WithDescription("Total number of RAG sync errors"),
    )
    return err
}

type RAGSyncService interface {
    FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error)
    MarkSynced(ctx context.Context, ids []string) error
    ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error
}

type ragSyncServiceImpl struct {
    provider db.Provider
}

func NewRAGSyncService(provider db.Provider) RAGSyncService {
    return &ragSyncServiceImpl{provider: provider}
}

func (s *ragSyncServiceImpl) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
    query := `
        SELECT memory_id, context, vector_embedding, sync_status, last_sync_at
        FROM swarm_memory_embeddings
        WHERE sync_status = 'pending'
        LIMIT $1
    `
    rows, err := s.provider.Query(ctx, query, limit)
    if err != nil {
        return nil, err
    }
    defer rows.Close()

    var records []RAGSyncRecord
    for rows.Next() {
        var rec RAGSyncRecord
        var syncStatus string
        var lastSyncAt sql.NullString
        var vectorBytes []byte
        if err := rows.Scan(&rec.ID, &rec.Context, &vectorBytes, &syncStatus, &lastSyncAt); err != nil {
            return nil, err
        }
        rec.SyncStatus = SyncStatus(syncStatus)
        if lastSyncAt.Valid && lastSyncAt.String != "" {
            t, err := time.Parse(time.RFC3339, lastSyncAt.String)
            if err != nil {
                t, err = time.Parse("2006-01-02 15:04:05", lastSyncAt.String)
                if err != nil {
                    t, err = time.Parse("2006-01-02 15:04:05Z07:00", lastSyncAt.String)
                }
            }
            if err == nil {
                rec.LastSyncAt = t
            }
        }
        if len(vectorBytes) > 0 {
            _ = json.Unmarshal(vectorBytes, &rec.Vector)
        }
        records = append(records, rec)
    }
    return records, nil
}

func (s *ragSyncServiceImpl) MarkSynced(ctx context.Context, ids []string) error {
    if len(ids) == 0 {
        return nil
    }
    tx, err := s.provider.Begin(ctx)
    if err != nil {
        return err
    }
    defer tx.Rollback(ctx)
    query := `
        UPDATE swarm_memory_embeddings
        SET sync_status = 'synced', last_sync_at = CURRENT_TIMESTAMP
        WHERE memory_id = $1
    `
    for _, id := range ids {
        if _, err := tx.Exec(ctx, query, id); err != nil {
            if RagSyncErrorsTotal != nil {
                RagSyncErrorsTotal.Add(ctx, 1)
            }
            return err
        }
    }
    if err := tx.Commit(ctx); err != nil {
        if RagSyncErrorsTotal != nil {
            RagSyncErrorsTotal.Add(ctx, 1)
        }
        return err
    }
    if RagRecordsSyncedTotal != nil {
        RagRecordsSyncedTotal.Add(ctx, int64(len(ids)))
    }
    return nil
}

func (s *ragSyncServiceImpl) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
    if len(records) == 0 {
        return nil
    }
    tx, err := s.provider.Begin(ctx)
    if err != nil {
        return err
    }
    defer tx.Rollback(ctx)
    query := `
        INSERT INTO swarm_memory_embeddings (memory_id, context, vector_embedding, sync_status, last_sync_at)
        VALUES ($1, $2, $3, 'synced', CURRENT_TIMESTAMP)
        ON CONFLICT (memory_id) DO UPDATE SET
            context = EXCLUDED.context,
            vector_embedding = EXCLUDED.vector_embedding,
            sync_status = EXCLUDED.sync_status,
            last_sync_at = EXCLUDED.last_sync_at
    `
    for _, rec := range records {
        var vectorBytes []byte
        if rec.Vector != nil {
            vectorBytes, _ = json.Marshal(rec.Vector)
        }
        if _, err := tx.Exec(ctx, query, rec.ID, rec.Context, string(vectorBytes)); err != nil {
            if RagSyncErrorsTotal != nil {
                RagSyncErrorsTotal.Add(ctx, 1)
            }
            return err
        }
    }
    if err := tx.Commit(ctx); err != nil {
        if RagSyncErrorsTotal != nil {
            RagSyncErrorsTotal.Add(ctx, 1)
        }
        return err
    }
    if RagRecordsSyncedTotal != nil {
        RagRecordsSyncedTotal.Add(ctx, int64(len(records)))
    }
    return nil
}
