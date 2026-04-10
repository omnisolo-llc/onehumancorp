package hub

import (
    "context"
    "time"
    "go.opentelemetry.io/otel/metric"
    "go.opentelemetry.io/otel"
    "github.com/onehumancorp/mono/srcs/server/db"
)

var (
    ragRecordsSyncedTotal metric.Int64Counter
    ragSyncErrorsTotal    metric.Int64Counter
)

func init() {
    meter := otel.Meter("github.com/onehumancorp/mono/srcs/server/hub")

    var err error
    ragRecordsSyncedTotal, err = meter.Int64Counter(
        "rag_records_synced_total",
        metric.WithDescription("Total number of RAG records successfully synced"),
    )
    if err != nil {
        panic(err)
    }

    ragSyncErrorsTotal, err = meter.Int64Counter(
        "rag_sync_errors_total",
        metric.WithDescription("Total number of RAG sync errors"),
    )
    if err != nil {
        panic(err)
    }
}

type SyncStatus string

const (
    SyncStatusPending SyncStatus = "pending"
    SyncStatusSynced  SyncStatus = "synced"
    SyncStatusError   SyncStatus = "error"
)

type RAGSyncRecord struct {
    ID           string
    Context      string
    Vector       []byte // Mapped to []byte for vector embeddings per memory rules
    SyncStatus   SyncStatus
    LastSyncAt   time.Time
}

type RAGSyncService interface {
    FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error)
    MarkSynced(ctx context.Context, ids []string) error
    ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error
}

type DBRAGSyncService struct {
    provider db.Provider
}

func NewDBRAGSyncService(provider db.Provider) *DBRAGSyncService {
    return &DBRAGSyncService{
        provider: provider,
    }
}

func (s *DBRAGSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
    query := `
        SELECT memory_id, context, vector_embedding, sync_status, last_sync_at
        FROM swarm_memory_embeddings
        WHERE sync_status = 'pending'
        LIMIT $1
    `
    if s.provider.IsSQLite() {
        query = `
            SELECT memory_id, context, vector_embedding, sync_status, last_sync_at
            FROM swarm_memory_embeddings
            WHERE sync_status = 'pending'
            LIMIT ?
        `
    }

    tx, err := s.provider.Begin(ctx)
    if err != nil {
        return nil, err
    }
    defer tx.Rollback(ctx)

    rows, err := tx.Query(ctx, query, limit)
    if err != nil {
        return nil, err
    }
    defer rows.Close()

    var pending []RAGSyncRecord
    for rows.Next() {
        var record RAGSyncRecord
        var syncStatus *string
        var lastSyncAt *time.Time
        err := rows.Scan(&record.ID, &record.Context, &record.Vector, &syncStatus, &lastSyncAt)
        if err != nil {
            return nil, err
        }
        if syncStatus != nil {
            record.SyncStatus = SyncStatus(*syncStatus)
        }
        if lastSyncAt != nil {
            record.LastSyncAt = *lastSyncAt
        }
        pending = append(pending, record)
    }

    if err := tx.Commit(ctx); err != nil {
        return nil, err
    }

    return pending, nil
}

func (s *DBRAGSyncService) MarkSynced(ctx context.Context, ids []string) error {
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
    if s.provider.IsSQLite() {
        query = `
            UPDATE swarm_memory_embeddings
            SET sync_status = 'synced', last_sync_at = CURRENT_TIMESTAMP
            WHERE memory_id = ?
        `
    }

    successCount := 0
    for _, id := range ids {
        _, err = tx.Exec(ctx, query, id)
        if err != nil {
            if ragSyncErrorsTotal != nil {
                ragSyncErrorsTotal.Add(ctx, 1)
            }
            return err
        }
        successCount++
    }

    if err := tx.Commit(ctx); err != nil {
        return err
    }

    if ragRecordsSyncedTotal != nil && successCount > 0 {
        ragRecordsSyncedTotal.Add(ctx, int64(successCount))
    }

    return nil
}

func (s *DBRAGSyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
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
        ON CONFLICT(memory_id) DO UPDATE SET
            context = excluded.context,
            vector_embedding = excluded.vector_embedding,
            sync_status = 'synced',
            last_sync_at = CURRENT_TIMESTAMP
    `
    if s.provider.IsSQLite() {
        query = `
            INSERT INTO swarm_memory_embeddings (memory_id, context, vector_embedding, sync_status, last_sync_at)
            VALUES (?, ?, ?, 'synced', CURRENT_TIMESTAMP)
            ON CONFLICT(memory_id) DO UPDATE SET
                context = excluded.context,
                vector_embedding = excluded.vector_embedding,
                sync_status = 'synced',
                last_sync_at = CURRENT_TIMESTAMP
        `
    }

    successCount := 0
    for _, record := range records {
        _, err = tx.Exec(ctx, query, record.ID, record.Context, record.Vector)
        if err != nil {
            if ragSyncErrorsTotal != nil {
                ragSyncErrorsTotal.Add(ctx, 1)
            }
            return err
        }
        successCount++
    }

    if err := tx.Commit(ctx); err != nil {
        return err
    }

    if ragRecordsSyncedTotal != nil && successCount > 0 {
        ragRecordsSyncedTotal.Add(ctx, int64(successCount))
    }

    return nil
}
