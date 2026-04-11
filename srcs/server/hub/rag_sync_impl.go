package hub

import (
    "context"
    "time"
    "github.com/onehumancorp/mono/srcs/server/db"
    "github.com/onehumancorp/mono/srcs/server/telemetry"
)

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

    var query string
    if s.provider.IsSQLite() {
        query = "SELECT memory_id, context, vector_embedding, sync_status, last_sync_at FROM swarm_memory_embeddings WHERE sync_status = 'pending' LIMIT $1"
    } else {
        query = "SELECT memory_id, context, vector_embedding, sync_status, last_sync_at FROM swarm_memory_embeddings WHERE sync_status = 'pending' LIMIT $1 FOR UPDATE SKIP LOCKED"
    }

    rows, err := tx.Query(ctx, query, limit)
    if err != nil {
        return nil, err
    }
    defer rows.Close()

    var records []RAGSyncRecord
    for rows.Next() {
        var rec RAGSyncRecord
        var lastSync *time.Time
        if err := rows.Scan(&rec.ID, &rec.Context, &rec.Vector, &rec.SyncStatus, &lastSync); err != nil {
            return nil, err
        }
        if lastSync != nil {
            rec.LastSyncAt = *lastSync
        }
        records = append(records, rec)
    }

    if len(records) > 0 {
        var ids []string
                for _, rec := range records {
            ids = append(ids, rec.ID)
                    }
        // For simplicity, do serial updates if array operations are complex
        for _, id := range ids {
            _, err := tx.Exec(ctx, "UPDATE swarm_memory_embeddings SET sync_status = 'in_progress' WHERE memory_id = $1", id)
            if err != nil {
                return nil, err
            }
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
    for _, id := range ids {
        _, err := tx.Exec(ctx, "UPDATE swarm_memory_embeddings SET sync_status = 'synced', last_sync_at = $1 WHERE memory_id = $2", now, id)
        if err != nil {
            if telemetry.RagSyncErrorsTotal != nil {
                telemetry.RagSyncErrorsTotal.Add(ctx, 1)
            }
            return err
        }
    }

    if telemetry.RagRecordsSyncedTotal != nil {
        telemetry.RagRecordsSyncedTotal.Add(ctx, int64(len(ids)))
    }

    return tx.Commit(ctx)
}

func (s *ragSyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
    tx, err := s.provider.Begin(ctx)
    if err != nil {
        return err
    }
    defer tx.Rollback(ctx)

    now := time.Now()
    for _, rec := range records {
        // Explicit existence check followed by INSERT/UPDATE
        row := tx.QueryRow(ctx, "SELECT memory_id FROM swarm_memory_embeddings WHERE memory_id = $1", rec.ID)
        var existingID string
        err := row.Scan(&existingID)

        if err != nil { // Not found or error (assume not found for simplicity)
            _, err = tx.Exec(ctx, "INSERT INTO swarm_memory_embeddings (memory_id, context, vector_embedding, sync_status, last_sync_at) VALUES ($1, $2, $3, 'synced', $4)", rec.ID, rec.Context, rec.Vector, now)
        } else {
            _, err = tx.Exec(ctx, "UPDATE swarm_memory_embeddings SET context = $1, vector_embedding = $2, sync_status = 'synced', last_sync_at = $3 WHERE memory_id = $4", rec.Context, rec.Vector, now, rec.ID)
        }

        if err != nil {
            if telemetry.RagSyncErrorsTotal != nil {
                telemetry.RagSyncErrorsTotal.Add(ctx, 1)
            }
            return err
        }
    }

    if telemetry.RagRecordsSyncedTotal != nil {
        telemetry.RagRecordsSyncedTotal.Add(ctx, int64(len(records)))
    }

    return tx.Commit(ctx)
}
