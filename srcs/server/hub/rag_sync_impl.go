package hub

import (
    "context"
    "fmt"
    "time"

    "github.com/onehumancorp/mono/srcs/server/db"
    "github.com/onehumancorp/mono/srcs/server/telemetry"
)

type RAGSyncServiceImpl struct {
    provider db.Provider
}

func NewRAGSyncService(provider db.Provider) *RAGSyncServiceImpl {
    return &RAGSyncServiceImpl{provider: provider}
}

func (s *RAGSyncServiceImpl) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
    var records []RAGSyncRecord

    // We must use a transaction to safely fetch and update rows atomically in both SQLite and Postgres.
    tx, err := s.provider.Begin(ctx)
    if err != nil {
        if telemetry.RAGSyncErrorsTotal != nil {
            telemetry.RAGSyncErrorsTotal.Add(ctx, 1)
        }
        return nil, fmt.Errorf("begin transaction: %w", err)
    }
    defer tx.Rollback(ctx)

    query := "SELECT memory_id, context, vector_embedding, sync_status, last_sync_at FROM swarm_memory_embeddings WHERE sync_status = 'pending' LIMIT $1"
    if !s.provider.IsSQLite() {
        query += " FOR UPDATE SKIP LOCKED"
    }

    rows, err := tx.Query(ctx, query, limit)
    if err != nil {
        if telemetry.RAGSyncErrorsTotal != nil {
            telemetry.RAGSyncErrorsTotal.Add(ctx, 1)
        }
        return nil, err
    }

    for rows.Next() {
        var record RAGSyncRecord
        var lastSyncAt *time.Time
        if err := rows.Scan(&record.ID, &record.Context, &record.Vector, &record.SyncStatus, &lastSyncAt); err != nil {
            rows.Close()
            return nil, err
        }
        if lastSyncAt != nil {
            record.LastSyncAt = *lastSyncAt
        }
        records = append(records, record)
    }
    rows.Close()

    // We must update the state to in_progress for both SQLite and Postgres to ensure mode parity
    // and correctly lock these rows from other workers.
    if len(records) > 0 {
        for _, r := range records {
            _, err := tx.Exec(ctx, "UPDATE swarm_memory_embeddings SET sync_status = 'in_progress' WHERE memory_id = $1", r.ID)
            if err != nil {
                return nil, fmt.Errorf("update sync_status: %w", err)
            }
        }
    }

    if err := tx.Commit(ctx); err != nil {
        return nil, fmt.Errorf("commit transaction: %w", err)
    }

    return records, nil
}

func (s *RAGSyncServiceImpl) MarkSynced(ctx context.Context, ids []string) error {
    for _, id := range ids {
        _, err := s.provider.Exec(ctx, "UPDATE swarm_memory_embeddings SET sync_status = 'synced', last_sync_at = $1 WHERE memory_id = $2", time.Now(), id)
        if err != nil {
            if telemetry.RAGSyncErrorsTotal != nil {
                telemetry.RAGSyncErrorsTotal.Add(ctx, 1)
            }
            return err
        }
    }
    if telemetry.RAGRecordsSyncedTotal != nil {
        telemetry.RAGRecordsSyncedTotal.Add(ctx, int64(len(ids)))
    }
    return nil
}

func (s *RAGSyncServiceImpl) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
    for _, record := range records {
        var err error

        if s.provider.IsSQLite() {
            // SQLite UPSERT syntax
            _, err = s.provider.Exec(ctx, `
                INSERT INTO swarm_memory_embeddings (memory_id, context, vector_embedding, sync_status, last_sync_at)
                VALUES ($1, $2, $3, $4, $5)
                ON CONFLICT (memory_id) DO UPDATE SET
                    context = excluded.context,
                    vector_embedding = excluded.vector_embedding,
                    sync_status = excluded.sync_status,
                    last_sync_at = excluded.last_sync_at
            `, record.ID, record.Context, record.Vector, record.SyncStatus, record.LastSyncAt)
        } else {
            // PostgreSQL UPSERT syntax
            _, err = s.provider.Exec(ctx, `
                INSERT INTO swarm_memory_embeddings (memory_id, context, vector_embedding, sync_status, last_sync_at)
                VALUES ($1, $2, $3, $4, $5)
                ON CONFLICT (memory_id) DO UPDATE SET
                    context = EXCLUDED.context,
                    vector_embedding = EXCLUDED.vector_embedding,
                    sync_status = EXCLUDED.sync_status,
                    last_sync_at = EXCLUDED.last_sync_at
            `, record.ID, record.Context, record.Vector, record.SyncStatus, record.LastSyncAt)
        }

        if err != nil {
            if telemetry.RAGSyncErrorsTotal != nil {
                telemetry.RAGSyncErrorsTotal.Add(ctx, 1)
            }
            return fmt.Errorf("process incoming sync for %s: %w", record.ID, err)
        }
    }
    return nil
}
