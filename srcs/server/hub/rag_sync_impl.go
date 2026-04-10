package hub

import (
    "context"
    "database/sql"
    "encoding/json"
    "time"

    "github.com/onehumancorp/mono/srcs/server/db"
)

type RAGSyncServiceImpl struct {
    provider db.Provider
}

func NewRAGSyncService(provider db.Provider) RAGSyncService {
    return &RAGSyncServiceImpl{
        provider: provider,
    }
}

func (s *RAGSyncServiceImpl) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
    query := `
        SELECT id, content, CAST(embedding AS TEXT), sync_status, last_sync_at
        FROM autodream_memories
        WHERE sync_status = 'pending'
        LIMIT $1
    `

    rows, err := s.provider.Query(ctx, query, limit)
    if err != nil {
        ragSyncErrorsTotal.Add(ctx, 1)
        return nil, err
    }
    defer rows.Close()

    var records []RAGSyncRecord
    for rows.Next() {
        var rec RAGSyncRecord
        var lastSyncAt sql.NullTime
        var embeddingStr sql.NullString
        var syncStatus sql.NullString

        if err := rows.Scan(&rec.ID, &rec.Context, &embeddingStr, &syncStatus, &lastSyncAt); err != nil {
            ragSyncErrorsTotal.Add(ctx, 1)
            return nil, err
        }

        if syncStatus.Valid {
            rec.SyncStatus = SyncStatus(syncStatus.String)
        }
        if lastSyncAt.Valid {
            timeVal := lastSyncAt.Time
            rec.LastSyncAt = &timeVal
        }
        if embeddingStr.Valid && embeddingStr.String != "" {
            var vector []float32
            if err := json.Unmarshal([]byte(embeddingStr.String), &vector); err == nil {
                rec.Vector = vector
            }
        }

        records = append(records, rec)
    }

    if err := rows.Err(); err != nil {
        ragSyncErrorsTotal.Add(ctx, 1)
        return nil, err
    }

    return records, nil
}

func (s *RAGSyncServiceImpl) MarkSynced(ctx context.Context, ids []string) error {
    if len(ids) == 0 {
        return nil
    }

    tx, err := s.provider.Begin(ctx)
    if err != nil {
        ragSyncErrorsTotal.Add(ctx, 1)
        return err
    }
    defer tx.Rollback(ctx)

    query := `
        UPDATE autodream_memories
        SET sync_status = 'synced', last_sync_at = $1
        WHERE id = $2
    `

    now := time.Now()
    for _, id := range ids {
        _, err := tx.Exec(ctx, query, now, id)
        if err != nil {
            ragSyncErrorsTotal.Add(ctx, 1)
            return err
        }
    }

    if err := tx.Commit(ctx); err != nil {
        ragSyncErrorsTotal.Add(ctx, 1)
        return err
    }

    ragRecordsSyncedTotal.Add(ctx, int64(len(ids)))
    return nil
}

func (s *RAGSyncServiceImpl) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
    if len(records) == 0 {
        return nil
    }

    tx, err := s.provider.Begin(ctx)
    if err != nil {
        ragSyncErrorsTotal.Add(ctx, 1)
        return err
    }
    defer tx.Rollback(ctx)

    for _, rec := range records {
        query := `
            INSERT INTO autodream_memories (id, content, embedding, sync_status, last_sync_at)
            VALUES ($1, $2, $3, $4, $5)
            ON CONFLICT (id) DO UPDATE SET
                content = EXCLUDED.content,
                embedding = EXCLUDED.embedding,
                sync_status = EXCLUDED.sync_status,
                last_sync_at = EXCLUDED.last_sync_at
        `

        var embeddingStr *string
        if len(rec.Vector) > 0 {
            if b, err := json.Marshal(rec.Vector); err == nil {
                str := string(b)
                embeddingStr = &str
            }
        }

        var lastSyncAt interface{}
        if rec.LastSyncAt != nil {
            lastSyncAt = *rec.LastSyncAt
        } else {
            lastSyncAt = nil
        }

        _, err := tx.Exec(ctx, query, rec.ID, rec.Context, embeddingStr, rec.SyncStatus, lastSyncAt)
        if err != nil {
            ragSyncErrorsTotal.Add(ctx, 1)
            return err
        }
    }

    if err := tx.Commit(ctx); err != nil {
        ragSyncErrorsTotal.Add(ctx, 1)
        return err
    }

    return nil
}
