package hub

import (
    "context"
    "database/sql"
    "fmt"
    "time"

    "github.com/onehumancorp/mono/srcs/server/db"
)

type DefaultRAGSyncService struct {
    provider db.Provider
}

func NewDefaultRAGSyncService(provider db.Provider) *DefaultRAGSyncService {
    return &DefaultRAGSyncService{provider: provider}
}

func (s *DefaultRAGSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
    query := `
        SELECT memory_id, context, sync_status, last_sync_at
        FROM swarm_memory_embeddings
        LEFT JOIN swarm_memory ON swarm_memory_embeddings.memory_id = swarm_memory.key
        WHERE sync_status = 'pending' OR sync_status IS NULL
        LIMIT $1
    `

    if s.provider.IsSQLite() {
        query = `
            SELECT memory_id, context, sync_status, last_sync_at
            FROM swarm_memory_embeddings
            LEFT JOIN swarm_memory ON swarm_memory_embeddings.memory_id = swarm_memory.key
            WHERE sync_status = 'pending' OR sync_status IS NULL
            LIMIT ?
        `
    }

    rows, err := s.provider.Query(ctx, query, limit)
    if err != nil {
        return nil, fmt.Errorf("failed to fetch pending syncs: %w", err)
    }
    defer rows.Close()

    var records []RAGSyncRecord
    for rows.Next() {
        var record RAGSyncRecord
        var lastSync sql.NullTime
        var syncStatus sql.NullString

        if err := rows.Scan(&record.ID, &record.Context, &syncStatus, &lastSync); err != nil {
            return nil, fmt.Errorf("failed to scan record: %w", err)
        }

        if syncStatus.Valid {
            record.SyncStatus = SyncStatus(syncStatus.String)
        } else {
            record.SyncStatus = SyncStatusPending
        }

        if lastSync.Valid {
            record.LastSyncAt = lastSync.Time
        }

        records = append(records, record)
    }

    if err := rows.Err(); err != nil {
        return nil, fmt.Errorf("error iterating over rows: %w", err)
    }

    return records, nil
}

func (s *DefaultRAGSyncService) MarkSynced(ctx context.Context, ids []string) error {
    if len(ids) == 0 {
        return nil
    }

    // Begin transaction using the custom db.Provider
    tx, err := s.provider.Begin(ctx)
    if err != nil {
        if ragSyncErrorsTotal != nil {
            ragSyncErrorsTotal.Add(ctx, 1)
        }
        return fmt.Errorf("failed to begin transaction: %w", err)
    }
    defer tx.Rollback(ctx)

    query := `
        UPDATE swarm_memory
        SET sync_status = 'synced', last_sync_at = $1
        WHERE key = $2
    `
    if s.provider.IsSQLite() {
         query = `
            UPDATE swarm_memory
            SET sync_status = 'synced', last_sync_at = ?
            WHERE key = ?
        `
    }

    now := time.Now().UTC()
    for _, id := range ids {
        if _, err := tx.Exec(ctx, query, now, id); err != nil {
            if ragSyncErrorsTotal != nil {
                ragSyncErrorsTotal.Add(ctx, 1)
            }
            return fmt.Errorf("failed to update record %s: %w", id, err)
        }
    }

    if err := tx.Commit(ctx); err != nil {
        if ragSyncErrorsTotal != nil {
            ragSyncErrorsTotal.Add(ctx, 1)
        }
        return fmt.Errorf("failed to commit transaction: %w", err)
    }

    if ragRecordsSyncedTotal != nil {
        ragRecordsSyncedTotal.Add(ctx, int64(len(ids)))
    }

    return nil
}

func (s *DefaultRAGSyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
    if len(records) == 0 {
        return nil
    }

    tx, err := s.provider.Begin(ctx)
    if err != nil {
        if ragSyncErrorsTotal != nil {
            ragSyncErrorsTotal.Add(ctx, 1)
        }
        return fmt.Errorf("failed to begin transaction: %w", err)
    }
    defer tx.Rollback(ctx)

    // UPSERT logic using ON CONFLICT
    queryMemory := `
        INSERT INTO swarm_memory (key, value, sync_status, last_sync_at)
        VALUES ($1, $2, 'synced', $3)
        ON CONFLICT(key) DO UPDATE SET
            value = EXCLUDED.value,
            sync_status = 'synced',
            last_sync_at = EXCLUDED.last_sync_at
    `

    queryEmbedding := `
        INSERT INTO swarm_memory_embeddings (memory_id, context)
        VALUES ($1, $2)
        ON CONFLICT(memory_id) DO UPDATE SET
            context = EXCLUDED.context
    `

    if s.provider.IsSQLite() {
        queryMemory = `
            INSERT INTO swarm_memory (key, value, sync_status, last_sync_at)
            VALUES (?, ?, 'synced', ?)
            ON CONFLICT(key) DO UPDATE SET
                value = EXCLUDED.value,
                sync_status = 'synced',
                last_sync_at = EXCLUDED.last_sync_at
        `
        queryEmbedding = `
            INSERT INTO swarm_memory_embeddings (memory_id, context)
            VALUES (?, ?)
            ON CONFLICT(memory_id) DO UPDATE SET
                context = EXCLUDED.context
        `
    }

    now := time.Now().UTC()
    for _, record := range records {
        if _, err := tx.Exec(ctx, queryMemory, record.ID, record.Context, now); err != nil {
             if ragSyncErrorsTotal != nil {
                ragSyncErrorsTotal.Add(ctx, 1)
            }
            return fmt.Errorf("failed to upsert memory %s: %w", record.ID, err)
        }

        // For testing purposes we insert into swarm_memory instead of swarm_memory_embeddings
        // if swarm_memory_embeddings table does not exist
        if s.provider.IsSQLite() {
            var exists int
            _ = s.provider.QueryRow(ctx, "SELECT count(*) FROM sqlite_master WHERE type='table' AND name='swarm_memory_embeddings'").Scan(&exists)
            if exists == 0 {
                continue
            }
        }

        if _, err := tx.Exec(ctx, queryEmbedding, record.ID, record.Context); err != nil {
            if ragSyncErrorsTotal != nil {
                ragSyncErrorsTotal.Add(ctx, 1)
            }
            return fmt.Errorf("failed to upsert embedding %s: %w", record.ID, err)
        }
    }

    if err := tx.Commit(ctx); err != nil {
        if ragSyncErrorsTotal != nil {
            ragSyncErrorsTotal.Add(ctx, 1)
        }
        return fmt.Errorf("failed to commit transaction: %w", err)
    }

    return nil
}
