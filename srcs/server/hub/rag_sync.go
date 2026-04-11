package hub

import (
    "context"
    "time"
    "fmt"

    "github.com/onehumancorp/mono/srcs/server/db"
    "go.opentelemetry.io/otel"
    "go.opentelemetry.io/otel/metric"
)

type SyncStatus string

const (
    SyncStatusPending   SyncStatus = "pending"
    SyncStatusInProgress SyncStatus = "in_progress"
    SyncStatusSynced    SyncStatus = "synced"
    SyncStatusError     SyncStatus = "error"
)

type RAGSyncRecord struct {
    ID           string
    Context      string
    Vector       []byte
    SyncStatus   SyncStatus
    LastSyncAt   time.Time
}

type RAGSyncService interface {
    FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error)
    MarkSynced(ctx context.Context, ids []string) error
    ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error
}

type DBAGSyncService struct {
    db db.Provider
}

func NewDBAGSyncService(provider db.Provider) *DBAGSyncService {
    return &DBAGSyncService{db: provider}
}

func (s *DBAGSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
    tx, err := s.db.Begin(ctx)
    if err != nil {
         SyncErrors.Add(ctx, 1)
         return nil, fmt.Errorf("failed to begin transaction: %w", err)
    }
    defer tx.Rollback(ctx)

    query := `
        SELECT memory_id, context, vector_embedding, sync_status, last_sync_at
        FROM swarm_memory_embeddings
        WHERE sync_status = 'pending'
        LIMIT `
    if s.db.IsSQLite() {
        query += "?"
    } else {
        query += "$1 FOR UPDATE SKIP LOCKED"
    }

    rows, err := tx.Query(ctx, query, limit)
    if err != nil {
        SyncErrors.Add(ctx, 1)
        return nil, fmt.Errorf("failed to fetch pending syncs: %w", err)
    }
    defer rows.Close()

    var records []RAGSyncRecord
    var idsToUpdate []string
    for rows.Next() {
        var rec RAGSyncRecord
        var lastSyncAt *time.Time
        if err := rows.Scan(&rec.ID, &rec.Context, &rec.Vector, &rec.SyncStatus, &lastSyncAt); err != nil {
            SyncErrors.Add(ctx, 1)
            return nil, fmt.Errorf("failed to scan record: %w", err)
        }
        if lastSyncAt != nil {
            rec.LastSyncAt = *lastSyncAt
        }
        records = append(records, rec)
        idsToUpdate = append(idsToUpdate, rec.ID)
    }
    if err := rows.Err(); err != nil {
        SyncErrors.Add(ctx, 1)
        return nil, fmt.Errorf("error iterating pending syncs: %w", err)
    }
    rows.Close()

    if len(idsToUpdate) > 0 {
         for _, id := range idsToUpdate {
             updateQuery := "UPDATE swarm_memory_embeddings SET sync_status = 'in_progress' WHERE memory_id = "
             if s.db.IsSQLite() {
                 updateQuery += "?"
             } else {
                 updateQuery += "$1"
             }
             if _, err := tx.Exec(ctx, updateQuery, id); err != nil {
                 SyncErrors.Add(ctx, 1)
                 return nil, fmt.Errorf("failed to update status to in_progress for %s: %w", id, err)
             }
         }
    }

    if err := tx.Commit(ctx); err != nil {
        SyncErrors.Add(ctx, 1)
        return nil, fmt.Errorf("failed to commit transaction: %w", err)
    }

    return records, nil
}

func (s *DBAGSyncService) MarkSynced(ctx context.Context, ids []string) error {
    if len(ids) == 0 {
        return nil
    }

    // Fallback to iterating for simplicity in the mock/interface constraint,
    // in a production environment we would construct a parameterized IN clause or use pgx.Batch.
    for _, id := range ids {
        query := "UPDATE swarm_memory_embeddings SET sync_status = 'synced', last_sync_at = "
        if s.db.IsSQLite() {
            query += "CURRENT_TIMESTAMP WHERE memory_id = ?"
        } else {
            query += "CURRENT_TIMESTAMP WHERE memory_id = $1"
        }
        _, err := s.db.Exec(ctx, query, id)
        if err != nil {
             SyncErrors.Add(ctx, 1)
             return fmt.Errorf("failed to mark synced for id %s: %w", id, err)
        }
        RecordsSynced.Add(ctx, 1)
    }
    return nil
}

func (s *DBAGSyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
    for _, rec := range records {
        var query string
        if s.db.IsSQLite() {
            query = `
                INSERT INTO swarm_memory_embeddings (memory_id, context, vector_embedding, sync_status, last_sync_at)
                VALUES (?, ?, ?, 'synced', CURRENT_TIMESTAMP)
                ON CONFLICT(memory_id) DO UPDATE SET
                context=excluded.context, vector_embedding=excluded.vector_embedding, sync_status='synced', last_sync_at=CURRENT_TIMESTAMP`
            _, err := s.db.Exec(ctx, query, rec.ID, rec.Context, rec.Vector)
            if err != nil {
                 SyncErrors.Add(ctx, 1)
                 return fmt.Errorf("failed to process incoming sync for id %s: %w", rec.ID, err)
            }
        } else {
             query = `
                INSERT INTO swarm_memory_embeddings (memory_id, context, vector_embedding, sync_status, last_sync_at)
                VALUES ($1, $2, $3, 'synced', CURRENT_TIMESTAMP)
                ON CONFLICT(memory_id) DO UPDATE SET
                context=EXCLUDED.context, vector_embedding=EXCLUDED.vector_embedding, sync_status='synced', last_sync_at=CURRENT_TIMESTAMP`
             _, err := s.db.Exec(ctx, query, rec.ID, rec.Context, rec.Vector)
             if err != nil {
                 SyncErrors.Add(ctx, 1)
                 return fmt.Errorf("failed to process incoming sync for id %s: %w", rec.ID, err)
             }
        }
        RecordsSynced.Add(ctx, 1)
    }
    return nil
}

var (
    meter            = otel.Meter("github.com/onehumancorp/mono/srcs/server/hub")
    RecordsSynced, _ = meter.Int64Counter("rag_records_synced_total", metric.WithDescription("Total RAG records synced"))
    SyncErrors, _    = meter.Int64Counter("rag_sync_errors_total", metric.WithDescription("Total RAG sync errors"))
)
