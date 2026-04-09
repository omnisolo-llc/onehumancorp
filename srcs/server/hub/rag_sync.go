package hub

import (
    "context"
    "database/sql"
    "fmt"
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
    MemoryID         string
    Context          string
    VectorEmbedding  []byte
    SourcePlugin     sql.NullString
    SyncStatus       SyncStatus
    LastSyncAt       sql.NullTime
}

var (
    meter = otel.Meter("github.com/onehumancorp/mono/srcs/server/hub")
    ragRecordsSyncedTotal, _ = meter.Int64Counter(
        "rag_records_synced_total",
        metric.WithDescription("Total RAG records synced"),
    )
    ragSyncErrorsTotal, _ = meter.Int64Counter(
        "rag_sync_errors_total",
        metric.WithDescription("Total RAG sync errors"),
    )
)

type RAGSyncService interface {
    FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error)
    MarkSynced(ctx context.Context, ids []string) error
    ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error
}

type ragSyncServiceImpl struct {
    db db.Provider
}

func NewRAGSyncService(db db.Provider) RAGSyncService {
    return &ragSyncServiceImpl{db: db}
}

func (s *ragSyncServiceImpl) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
    query := `
        SELECT memory_id, context, vector_embedding, source_plugin, sync_status, last_sync_at
        FROM swarm_memory_embeddings
        WHERE sync_status = 'pending'
        LIMIT $1
    `

    rows, err := s.db.Query(ctx, query, limit)
    if err != nil {
        return nil, fmt.Errorf("failed to fetch pending syncs: %w", err)
    }
    defer rows.Close()

    var records []RAGSyncRecord
    for rows.Next() {
        var record RAGSyncRecord
        var syncStatus sql.NullString

        err := rows.Scan(
            &record.MemoryID,
            &record.Context,
            &record.VectorEmbedding,
            &record.SourcePlugin,
            &syncStatus,
            &record.LastSyncAt,
        )
        if err != nil {
            return nil, fmt.Errorf("failed to scan pending sync: %w", err)
        }

        if syncStatus.Valid {
            record.SyncStatus = SyncStatus(syncStatus.String)
        } else {
            record.SyncStatus = SyncStatusPending
        }

        records = append(records, record)
    }

    if err := rows.Err(); err != nil {
        return nil, fmt.Errorf("error iterating pending syncs: %w", err)
    }

    return records, nil
}

func (s *ragSyncServiceImpl) MarkSynced(ctx context.Context, ids []string) error {
    if len(ids) == 0 {
        return nil
    }

    tx, err := s.db.Begin(ctx)
    if err != nil {
        return fmt.Errorf("failed to begin transaction: %w", err)
    }
    defer tx.Rollback(ctx)

    now := time.Now()
    for _, id := range ids {
        query := `
            UPDATE swarm_memory_embeddings
            SET sync_status = 'synced', last_sync_at = $1
            WHERE memory_id = $2
        `
        _, err := tx.Exec(ctx, query, now, id)
        if err != nil {
            ragSyncErrorsTotal.Add(ctx, 1)
            return fmt.Errorf("failed to mark synced for id %s: %w", id, err)
        }
    }

    if err := tx.Commit(ctx); err != nil {
        return fmt.Errorf("failed to commit transaction: %w", err)
    }

    ragRecordsSyncedTotal.Add(ctx, int64(len(ids)))
    return nil
}

func (s *ragSyncServiceImpl) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
    if len(records) == 0 {
        return nil
    }

    tx, err := s.db.Begin(ctx)
    if err != nil {
        return fmt.Errorf("failed to begin transaction: %w", err)
    }
    defer tx.Rollback(ctx)

    for _, record := range records {
        query := `
            INSERT INTO swarm_memory_embeddings (memory_id, context, vector_embedding, source_plugin, sync_status, last_sync_at)
            VALUES ($1, $2, $3, $4, 'synced', $5)
            ON CONFLICT (memory_id) DO UPDATE SET
                context = EXCLUDED.context,
                vector_embedding = EXCLUDED.vector_embedding,
                source_plugin = EXCLUDED.source_plugin,
                sync_status = 'synced',
                last_sync_at = EXCLUDED.last_sync_at
        `

        _, err := tx.Exec(ctx, query,
            record.MemoryID,
            record.Context,
            record.VectorEmbedding,
            record.SourcePlugin,
            record.LastSyncAt,
        )
        if err != nil {
            ragSyncErrorsTotal.Add(ctx, 1)
            return fmt.Errorf("failed to process incoming sync for memory_id %s: %w", record.MemoryID, err)
        }
    }

    if err := tx.Commit(ctx); err != nil {
        return fmt.Errorf("failed to commit transaction: %w", err)
    }

    return nil
}
