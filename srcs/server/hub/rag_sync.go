package hub

import (
    "context"
    "time"
    "database/sql"
    _db "github.com/onehumancorp/mono/srcs/server/db"
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

type SQLRAGSyncService struct {
    provider _db.Provider
}

func NewSQLRAGSyncService(provider _db.Provider) *SQLRAGSyncService {
    return &SQLRAGSyncService{
        provider: provider,
    }
}

func (s *SQLRAGSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
    var records []RAGSyncRecord

    tx, err := s.provider.Begin(ctx)
    if err != nil {
        return nil, err
    }
    defer tx.Rollback(ctx)

    var rows _db.Rows
    if s.provider.IsSQLite() {
        rows, err = tx.Query(ctx, "SELECT memory_id, context, vector_embedding, sync_status, last_sync_at FROM swarm_memory_embeddings WHERE sync_status = 'pending' LIMIT $1", limit)
    } else {
        rows, err = tx.Query(ctx, "SELECT memory_id, context, vector_embedding, sync_status, last_sync_at FROM swarm_memory_embeddings WHERE sync_status = 'pending' LIMIT $1 FOR UPDATE SKIP LOCKED", limit)
    }
    if err != nil {
        if telemetry.RagSyncErrorsTotal != nil {
            telemetry.RagSyncErrorsTotal.Add(ctx, 1)
        }
        return nil, err
    }
    defer rows.Close()

    for rows.Next() {
        var r RAGSyncRecord
        var lastSyncAt sql.NullTime
        var vecBytes []byte
        if err := rows.Scan(&r.ID, &r.Context, &vecBytes, &r.SyncStatus, &lastSyncAt); err != nil {
            return nil, err
        }
        r.Vector = vecBytes
        if lastSyncAt.Valid {
            r.LastSyncAt = lastSyncAt.Time
        }
        records = append(records, r)
    }
    if err := rows.Err(); err != nil {
        return nil, err
    }

    if err := tx.Commit(ctx); err != nil {
        return nil, err
    }

    return records, nil
}

func (s *SQLRAGSyncService) MarkSynced(ctx context.Context, ids []string) error {
    if len(ids) == 0 {
        return nil
    }
    now := time.Now()

    tx, err := s.provider.Begin(ctx)
    if err != nil {
        return err
    }
    defer tx.Rollback(ctx)

    for _, id := range ids {
        _, err := tx.Exec(ctx, "UPDATE swarm_memory_embeddings SET sync_status = 'synced', last_sync_at = $1 WHERE memory_id = $2", now, id)
        if err != nil {
            if telemetry.RagSyncErrorsTotal != nil {
                telemetry.RagSyncErrorsTotal.Add(ctx, 1)
            }
            return err
        }
    }

    if err := tx.Commit(ctx); err != nil {
        if telemetry.RagSyncErrorsTotal != nil {
            telemetry.RagSyncErrorsTotal.Add(ctx, 1)
        }
        return err
    }

    if telemetry.RagRecordsSyncedTotal != nil {
        telemetry.RagRecordsSyncedTotal.Add(ctx, int64(len(ids)))
    }
    return nil
}

func (s *SQLRAGSyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
    now := time.Now()

    tx, err := s.provider.Begin(ctx)
    if err != nil {
        return err
    }
    defer tx.Rollback(ctx)

    for _, r := range records {
        if s.provider.IsSQLite() {
            _, err = tx.Exec(ctx, "INSERT INTO swarm_memory_embeddings (memory_id, context, vector_embedding, sync_status, last_sync_at) VALUES ($1, $2, $3, 'synced', $4) ON CONFLICT(memory_id) DO UPDATE SET context=excluded.context, vector_embedding=excluded.vector_embedding, sync_status='synced', last_sync_at=excluded.last_sync_at", r.ID, r.Context, r.Vector, now)
        } else {
            _, err = tx.Exec(ctx, "INSERT INTO swarm_memory_embeddings (memory_id, context, vector_embedding, sync_status, last_sync_at) VALUES ($1, $2, $3, 'synced', $4) ON CONFLICT(memory_id) DO UPDATE SET context=EXCLUDED.context, vector_embedding=EXCLUDED.vector_embedding, sync_status='synced', last_sync_at=EXCLUDED.last_sync_at", r.ID, r.Context, r.Vector, now)
        }
        if err != nil {
            if telemetry.RagSyncErrorsTotal != nil {
                telemetry.RagSyncErrorsTotal.Add(ctx, 1)
            }
            return err
        }
    }

    if err := tx.Commit(ctx); err != nil {
        if telemetry.RagSyncErrorsTotal != nil {
            telemetry.RagSyncErrorsTotal.Add(ctx, 1)
        }
        return err
    }

    if telemetry.RagRecordsSyncedTotal != nil {
        telemetry.RagRecordsSyncedTotal.Add(ctx, int64(len(records)))
    }
    return nil
}
