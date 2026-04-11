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
    query := "SELECT memory_id, context, vector_embedding, sync_status, last_sync_at FROM swarm_memory_embeddings WHERE sync_status = 'pending' LIMIT $1"
    rows, err := s.provider.Query(ctx, query, limit)
    if err != nil {
        telemetry.RecordRagSyncError(ctx)
        return nil, err
    }
    defer rows.Close()

    var records []RAGSyncRecord
    for rows.Next() {
        var rec RAGSyncRecord
        var lastSync *time.Time
        var status *string
        if err := rows.Scan(&rec.ID, &rec.Context, &rec.Vector, &status, &lastSync); err != nil {
            telemetry.RecordRagSyncError(ctx)
            continue
        }
        if status != nil {
            rec.SyncStatus = SyncStatus(*status)
        } else {
            rec.SyncStatus = SyncStatusPending
        }
        if lastSync != nil {
            rec.LastSyncAt = *lastSync
        }
        records = append(records, rec)
    }
    return records, nil
}

func (s *ragSyncService) MarkSynced(ctx context.Context, ids []string) error {
    if len(ids) == 0 {
        return nil
    }
    now := time.Now()
    for _, id := range ids {
        _, err := s.provider.Exec(ctx, "UPDATE swarm_memory_embeddings SET sync_status = 'synced', last_sync_at = $1 WHERE memory_id = $2", now, id)
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
    now := time.Now()
    for _, rec := range records {
        var exists bool
        err := s.provider.QueryRow(ctx, "SELECT EXISTS(SELECT 1 FROM swarm_memory_embeddings WHERE memory_id = $1)", rec.ID).Scan(&exists)
        if err != nil {
            telemetry.RecordRagSyncError(ctx)
            return err
        }
        if exists {
            _, err = s.provider.Exec(ctx, "UPDATE swarm_memory_embeddings SET context = $1, vector_embedding = $2, sync_status = 'synced', last_sync_at = $3 WHERE memory_id = $4", rec.Context, rec.Vector, now, rec.ID)
        } else {
            _, err = s.provider.Exec(ctx, "INSERT INTO swarm_memory_embeddings (memory_id, context, vector_embedding, sync_status, last_sync_at) VALUES ($1, $2, $3, 'synced', $4)", rec.ID, rec.Context, rec.Vector, now)
        }
        if err != nil {
            telemetry.RecordRagSyncError(ctx)
            return err
        }
    }
    telemetry.RecordRagRecordsSynced(ctx, int64(len(records)))
    return nil
}
