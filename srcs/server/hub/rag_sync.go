package hub

import (
    "context"
    "time"
    "fmt"
    "database/sql"
    "encoding/json"

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
    `
    if !s.provider.IsSQLite() {
        query += " FOR UPDATE SKIP LOCKED"
    }
    query += " LIMIT $1"

    rows, err := s.provider.Query(ctx, query, limit)
    if err != nil {
        SyncErrorsTotal.Add(ctx, 1)
        return nil, fmt.Errorf("failed to fetch pending syncs: %w", err)
    }
    defer rows.Close()

    var records []RAGSyncRecord
    for rows.Next() {
        var rec RAGSyncRecord
        var syncStatus string
        var lastSyncAt sql.NullTime
        var vectorData []byte

        if err := rows.Scan(&rec.ID, &rec.Context, &vectorData, &syncStatus, &lastSyncAt); err != nil {
            SyncErrorsTotal.Add(ctx, 1)
            return nil, fmt.Errorf("failed to scan row: %w", err)
        }
        rec.SyncStatus = SyncStatus(syncStatus)
        if lastSyncAt.Valid {
            rec.LastSyncAt = lastSyncAt.Time
        }

        if len(vectorData) > 0 {
            if err := json.Unmarshal(vectorData, &rec.Vector); err != nil {
                // If it fails to parse json, we might have binary format depending on plugin
                // For this implementation, we will log error and skip setting vector
                SyncErrorsTotal.Add(ctx, 1)
                return nil, fmt.Errorf("failed to unmarshal vector: %w", err)
            }
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
        return fmt.Errorf("failed to begin transaction: %w", err)
    }
    defer tx.Rollback(ctx)

    for _, id := range ids {
        query := `
            UPDATE swarm_memory_embeddings
            SET sync_status = 'synced', last_sync_at = CURRENT_TIMESTAMP
            WHERE memory_id = $1
        `
        _, err := tx.Exec(ctx, query, id)
        if err != nil {
            SyncErrorsTotal.Add(ctx, 1)
            return fmt.Errorf("failed to mark synced for id %s: %w", id, err)
        }
    }

    if err := tx.Commit(ctx); err != nil {
        return fmt.Errorf("failed to commit transaction: %w", err)
    }

    SyncRecordsTotal.Add(ctx, int64(len(ids)))
    return nil
}

func (s *ragSyncServiceImpl) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
    if len(records) == 0 {
        return nil
    }

    tx, err := s.provider.Begin(ctx)
    if err != nil {
        return fmt.Errorf("failed to begin transaction: %w", err)
    }
    defer tx.Rollback(ctx)

    for _, rec := range records {
        var vectorData string
        if len(rec.Vector) > 0 {
            bytes, err := json.Marshal(rec.Vector)
            if err != nil {
                SyncErrorsTotal.Add(ctx, 1)
                return fmt.Errorf("failed to marshal vector: %w", err)
            }
            vectorData = string(bytes)
        }

        query := `
            INSERT INTO swarm_memory_embeddings (memory_id, context, vector_embedding, sync_status, last_sync_at)
            VALUES ($1, $2, $3, 'synced', CURRENT_TIMESTAMP)
            ON CONFLICT (memory_id) DO UPDATE SET
                context = EXCLUDED.context,
                vector_embedding = EXCLUDED.vector_embedding,
                sync_status = 'synced',
                last_sync_at = CURRENT_TIMESTAMP
        `
        _, err := tx.Exec(ctx, query, rec.ID, rec.Context, vectorData)
        if err != nil {
            SyncErrorsTotal.Add(ctx, 1)
            return fmt.Errorf("failed to process incoming sync for id %s: %w", rec.ID, err)
        }
    }

    if err := tx.Commit(ctx); err != nil {
        return fmt.Errorf("failed to commit transaction: %w", err)
    }

    SyncRecordsTotal.Add(ctx, int64(len(records)))
    return nil
}

var (
    SyncRecordsTotal metric.Int64Counter
    SyncErrorsTotal  metric.Int64Counter
)

func init() {
    meter := otel.Meter("github.com/onehumancorp/mono/srcs/server/hub")
    var err error
    SyncRecordsTotal, err = meter.Int64Counter("rag_records_synced_total")
    if err != nil {
        panic(err)
    }
    SyncErrorsTotal, err = meter.Int64Counter("rag_sync_errors_total")
    if err != nil {
        panic(err)
    }
}
