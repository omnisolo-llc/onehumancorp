package hub

import (
    "context"
    "time"
    "fmt"
    "github.com/onehumancorp/mono/srcs/server/db"
    "github.com/onehumancorp/mono/srcs/server/telemetry"
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
    Vector       []byte
    SyncStatus   SyncStatus
    LastSyncAt   time.Time
}

type RAGSyncService interface {
    FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error)
    MarkSynced(ctx context.Context, ids []string) error
    ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error
}

type dbRAGSyncService struct {
    db *db.DB
}

func NewRAGSyncService(dbWrapper *db.DB) RAGSyncService {
    return &dbRAGSyncService{db: dbWrapper}
}

func (s *dbRAGSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
    query := fmt.Sprintf("SELECT memory_id, context, vector_embedding, sync_status, last_sync_at FROM swarm_memory_embeddings WHERE sync_status = 'pending' LIMIT %d", limit)

    rows, err := s.db.Query(ctx, query)
    if err != nil {
        return nil, fmt.Errorf("query swarm_memory_embeddings: %w", err)
    }
    defer rows.Close()

    var records []RAGSyncRecord
    for rows.Next() {
        var r RAGSyncRecord
        var lastSyncAt *time.Time
        if err := rows.Scan(&r.ID, &r.Context, &r.Vector, &r.SyncStatus, &lastSyncAt); err != nil {
            return nil, fmt.Errorf("scan swarm_memory_embeddings: %w", err)
        }
        if lastSyncAt != nil {
            r.LastSyncAt = *lastSyncAt
        }
        records = append(records, r)
    }
    return records, nil
}

func (s *dbRAGSyncService) MarkSynced(ctx context.Context, ids []string) error {
    if len(ids) == 0 {
        return nil
    }
    tx, err := s.db.Begin(ctx)
    if err != nil {
        return fmt.Errorf("begin transaction: %w", err)
    }
    defer tx.Rollback(ctx)

    for _, id := range ids {
        _, err := tx.Exec(ctx, "UPDATE swarm_memory_embeddings SET sync_status = 'synced', last_sync_at = CURRENT_TIMESTAMP WHERE memory_id = $1", id)
        if err != nil {
            telemetry.RecordRagSyncError(ctx)
            return fmt.Errorf("update swarm_memory_embeddings: %w", err)
        }
    }
    if err := tx.Commit(ctx); err != nil {
        telemetry.RecordRagSyncError(ctx)
        return fmt.Errorf("commit transaction: %w", err)
    }

    telemetry.RecordRagRecordSynced(ctx, int64(len(ids)))
    return nil
}

func (s *dbRAGSyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
    tx, err := s.db.Begin(ctx)
    if err != nil {
        return fmt.Errorf("begin transaction: %w", err)
    }
    defer tx.Rollback(ctx)

    for _, r := range records {
        var err error
        if s.db.IsSQLite() {
            query := `INSERT INTO swarm_memory_embeddings (memory_id, context, vector_embedding, sync_status, last_sync_at)
                VALUES (?, ?, ?, 'synced', CURRENT_TIMESTAMP)
                ON CONFLICT(memory_id) DO UPDATE SET
                context=excluded.context, vector_embedding=excluded.vector_embedding, sync_status='synced', last_sync_at=CURRENT_TIMESTAMP`
            _, err = tx.Exec(ctx, query, r.ID, r.Context, r.Vector)
        } else {
            query := `INSERT INTO swarm_memory_embeddings (memory_id, context, vector_embedding, sync_status, last_sync_at)
                VALUES ($1, $2, $3, 'synced', CURRENT_TIMESTAMP)
                ON CONFLICT(memory_id) DO UPDATE SET
                context=EXCLUDED.context, vector_embedding=EXCLUDED.vector_embedding, sync_status='synced', last_sync_at=CURRENT_TIMESTAMP`
            _, err = tx.Exec(ctx, query, r.ID, r.Context, r.Vector)
        }

        if err != nil {
            telemetry.RecordRagSyncError(ctx)
            return fmt.Errorf("upsert swarm_memory_embeddings: %w", err)
        }
    }

    if err := tx.Commit(ctx); err != nil {
        telemetry.RecordRagSyncError(ctx)
        return fmt.Errorf("commit transaction: %w", err)
    }

    telemetry.RecordRagRecordSynced(ctx, int64(len(records)))
    return nil
}
