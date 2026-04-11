package hub

import (
    "context"
    "time"

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
    ID         string
    Context    string
    Vector     []byte
    SyncStatus SyncStatus
    LastSyncAt time.Time
}

type RAGSyncService interface {
    FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error)
    MarkSynced(ctx context.Context, ids []string) error
    ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error
}

type ragSyncService struct {
    provider db.Provider
}

func NewRAGSyncService(provider db.Provider) RAGSyncService {
    return &ragSyncService{provider: provider}
}

func (s *ragSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
    var query string
    if s.provider.IsSQLite() {
        query = "SELECT memory_id, context, vector_embedding, COALESCE(sync_status, 'pending'), last_sync_at FROM swarm_memory_embeddings WHERE sync_status = 'pending' OR sync_status IS NULL LIMIT $1"
    } else {
        query = "SELECT memory_id, context, vector_embedding, COALESCE(sync_status, 'pending'), last_sync_at FROM swarm_memory_embeddings WHERE sync_status = 'pending' OR sync_status IS NULL FOR UPDATE SKIP LOCKED LIMIT $1"
    }

    rows, err := s.provider.Query(ctx, query, limit)
    if err != nil {
        return nil, err
    }
    defer rows.Close()

    var records []RAGSyncRecord
    for rows.Next() {
        var r RAGSyncRecord
        var ls *time.Time
        if err := rows.Scan(&r.ID, &r.Context, &r.Vector, &r.SyncStatus, &ls); err != nil {
            return nil, err
        }
        if ls != nil {
            r.LastSyncAt = *ls
        }
        records = append(records, r)
    }

    if err := rows.Err(); err != nil {
        return nil, err
    }

    return records, nil
}

func (s *ragSyncService) MarkSynced(ctx context.Context, ids []string) error {
    if len(ids) == 0 {
        return nil
    }

    // Naive bulk update
    for _, id := range ids {
        _, err := s.provider.Exec(ctx, "UPDATE swarm_memory_embeddings SET sync_status = 'synced', last_sync_at = CURRENT_TIMESTAMP WHERE memory_id = $1", id)
        if err != nil {
            telemetry.RecordRagSyncError(ctx)
            return err
        }
    }
    telemetry.RecordRagRecordsSynced(ctx, int64(len(ids)))
    return nil
}

func (s *ragSyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
    if len(records) == 0 {
        return nil
    }

    for _, r := range records {
        query := `
            INSERT INTO swarm_memory_embeddings (memory_id, context, vector_embedding, sync_status, last_sync_at)
            VALUES ($1, $2, $3, 'synced', CURRENT_TIMESTAMP)
            ON CONFLICT (memory_id) DO UPDATE SET
                context = EXCLUDED.context,
                vector_embedding = EXCLUDED.vector_embedding,
                sync_status = 'synced',
                last_sync_at = CURRENT_TIMESTAMP
        `
        _, err := s.provider.Exec(ctx, query, r.ID, r.Context, r.Vector)
        if err != nil {
            telemetry.RecordRagSyncError(ctx)
            return err
        }
    }
    telemetry.RecordRagRecordsSynced(ctx, int64(len(records)))
    return nil
}
