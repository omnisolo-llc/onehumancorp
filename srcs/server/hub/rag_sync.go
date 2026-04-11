package hub

import (
    "context"
    "time"

    "github.com/onehumancorp/mono/srcs/server/db"
    "github.com/onehumancorp/mono/srcs/server/telemetry"
)

type SyncStatus string

const (
    SyncStatusPending    SyncStatus = "pending"
    SyncStatusInProgress SyncStatus = "in_progress"
    SyncStatusSynced     SyncStatus = "synced"
    SyncStatusError      SyncStatus = "error"
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

type ragSyncService struct {
    provider db.Provider
}

func NewRAGSyncService(provider db.Provider) RAGSyncService {
    return &ragSyncService{provider: provider}
}

func (s *ragSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
    tx, err := s.provider.Begin(ctx)
    if err != nil {
        return nil, err
    }
    defer tx.Rollback(ctx)

    var records []RAGSyncRecord

    if s.provider.IsSQLite() {
        // Immediate serial UPDATE to mark as in_progress for SQLite
        // First fetch IDs
        rows, err := tx.Query(ctx, `SELECT memory_id, context, vector_embedding, sync_status, last_sync_at FROM swarm_memory_embeddings WHERE sync_status = 'pending' LIMIT $1`, limit)
        if err != nil {
            return nil, err
        }

        for rows.Next() {
            var r RAGSyncRecord
            var status string
            var lastSync *time.Time
            if err := rows.Scan(&r.ID, &r.Context, &r.Vector, &status, &lastSync); err != nil {
                rows.Close()
                return nil, err
            }
            r.SyncStatus = SyncStatus(status)
            if lastSync != nil {
                r.LastSyncAt = *lastSync
            }
            records = append(records, r)
        }
        rows.Close()

        // Now mark them as in_progress
        for i := range records {
            _, err := tx.Exec(ctx, `UPDATE swarm_memory_embeddings SET sync_status = 'in_progress' WHERE memory_id = $1`, records[i].ID)
            if err != nil {
                return nil, err
            }
            records[i].SyncStatus = SyncStatusInProgress
        }

    } else {
        // PostgreSQL FOR UPDATE SKIP LOCKED
        query := `UPDATE swarm_memory_embeddings SET sync_status = 'in_progress' WHERE memory_id IN (
            SELECT memory_id FROM swarm_memory_embeddings WHERE sync_status = 'pending' LIMIT $1 FOR UPDATE SKIP LOCKED
        ) RETURNING memory_id, context, vector_embedding, sync_status, last_sync_at`

        rows, err := tx.Query(ctx, query, limit)
        if err != nil {
            return nil, err
        }
        defer rows.Close()

        for rows.Next() {
            var r RAGSyncRecord
            var status string
            var lastSync *time.Time
            if err := rows.Scan(&r.ID, &r.Context, &r.Vector, &status, &lastSync); err != nil {
                return nil, err
            }
            r.SyncStatus = SyncStatus(status)
            if lastSync != nil {
                r.LastSyncAt = *lastSync
            }
            records = append(records, r)
        }
    }

    if err := tx.Commit(ctx); err != nil {
        return nil, err
    }

    return records, nil
}

func (s *ragSyncService) MarkSynced(ctx context.Context, ids []string) error {
    if len(ids) == 0 {
        return nil
    }

    tx, err := s.provider.Begin(ctx)
    if err != nil {
        return err
    }
    defer tx.Rollback(ctx)

    now := time.Now()
    query := `UPDATE swarm_memory_embeddings SET sync_status = 'synced', last_sync_at = $1 WHERE memory_id = $2`
    for _, id := range ids {
        _, err := tx.Exec(ctx, query, now, id)
        if err != nil {
            telemetry.RecordRagSyncError(ctx)
            return err
        }
        telemetry.RecordRagRecordsSynced(ctx)
    }

    return tx.Commit(ctx)
}

func (s *ragSyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
    if len(records) == 0 {
        return nil
    }

    tx, err := s.provider.Begin(ctx)
    if err != nil {
        return err
    }
    defer tx.Rollback(ctx)

    now := time.Now()
    for _, r := range records {
        if s.provider.IsSQLite() {
            var exists bool
            err := tx.QueryRow(ctx, "SELECT EXISTS(SELECT 1 FROM swarm_memory_embeddings WHERE memory_id = $1)", r.ID).Scan(&exists)
            if err != nil {
                return err
            }
            if exists {
                _, err = tx.Exec(ctx, `UPDATE swarm_memory_embeddings SET context=$1, vector_embedding=$2, sync_status='synced', last_sync_at=$3 WHERE memory_id=$4`, r.Context, r.Vector, now, r.ID)
            } else {
                _, err = tx.Exec(ctx, `INSERT INTO swarm_memory_embeddings (memory_id, context, vector_embedding, sync_status, last_sync_at) VALUES ($1, $2, $3, 'synced', $4)`, r.ID, r.Context, r.Vector, now)
            }
            if err != nil {
                return err
            }
        } else {
            query := `INSERT INTO swarm_memory_embeddings (memory_id, context, vector_embedding, sync_status, last_sync_at) VALUES ($1, $2, $3, 'synced', $4) ON CONFLICT (memory_id) DO UPDATE SET context = excluded.context, vector_embedding = excluded.vector_embedding, sync_status = 'synced', last_sync_at = excluded.last_sync_at`
            _, err := tx.Exec(ctx, query, r.ID, r.Context, r.Vector, now)
            if err != nil {
                return err
            }
        }
    }

    return tx.Commit(ctx)
}
