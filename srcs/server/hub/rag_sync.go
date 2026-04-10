package hub

import (
    "context"
    "database/sql"
    "encoding/json"
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
    Vector       []float32
    SyncStatus   SyncStatus
    LastSyncAt   time.Time
}

type RAGSyncService interface {
    FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error)
    MarkSynced(ctx context.Context, ids []string) error
    ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error
}

var (
    meter = otel.Meter("github.com/onehumancorp/mono/srcs/server/hub")
    RecordsSyncedTotal metric.Int64Counter
    SyncErrorsTotal    metric.Int64Counter
)

// InitRAGSyncMetrics initializes metrics. Must return error on failure.
func InitRAGSyncMetrics() error {
    var err error
    RecordsSyncedTotal, err = meter.Int64Counter(
        "rag_records_synced_total",
        metric.WithDescription("Total number of RAG records synced to the cloud"),
    )
    if err != nil {
        return err
    }
    SyncErrorsTotal, err = meter.Int64Counter(
        "rag_sync_errors_total",
        metric.WithDescription("Total number of errors encountered during RAG sync"),
    )
    if err != nil {
        return err
    }
    return nil
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
        var lastSync sql.NullTime
        var vectorBytes []byte
        if err := rows.Scan(&rec.ID, &rec.Context, &vectorBytes, &rec.SyncStatus, &lastSync); err != nil {
            return nil, err
        }
        if lastSync.Valid {
            rec.LastSyncAt = lastSync.Time
        }

        // SQLite compat: deserialize if present
        if len(vectorBytes) > 0 {
            var vec []float32
            if err := json.Unmarshal(vectorBytes, &vec); err == nil {
                rec.Vector = vec
            }
        }

        records = append(records, rec)
    }
    return records, rows.Err()
}

func (s *ragSyncServiceImpl) MarkSynced(ctx context.Context, ids []string) error {
    if len(ids) == 0 {
        return nil
    }
    for _, id := range ids {
        query := `
            UPDATE swarm_memory_embeddings
            SET sync_status = 'synced', last_sync_at = CURRENT_TIMESTAMP
            WHERE memory_id = $1
        `
        if _, err := s.provider.Exec(ctx, query, id); err != nil {
            return err
        }
    }
    if RecordsSyncedTotal != nil {
        RecordsSyncedTotal.Add(ctx, int64(len(ids)))
    }
    return nil
}

func (s *ragSyncServiceImpl) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
    if len(records) == 0 {
        return nil
    }
    for _, rec := range records {
        vectorBytes, _ := json.Marshal(rec.Vector)

        query := `
            INSERT INTO swarm_memory_embeddings (memory_id, context, vector_embedding, sync_status, last_sync_at)
            VALUES ($1, $2, $3, 'synced', CURRENT_TIMESTAMP)
            ON CONFLICT (memory_id) DO UPDATE SET
                context = EXCLUDED.context,
                vector_embedding = EXCLUDED.vector_embedding,
                sync_status = 'synced',
                last_sync_at = CURRENT_TIMESTAMP
        `
        if _, err := s.provider.Exec(ctx, query, rec.ID, rec.Context, vectorBytes); err != nil {
            if SyncErrorsTotal != nil {
                SyncErrorsTotal.Add(ctx, 1)
            }
            return fmt.Errorf("failed to process incoming sync for %s: %w", rec.ID, err)
        }
    }
    return nil
}
