package hybrid_sync

import (
    "context"
    "database/sql"

    "github.com/onehumancorp/mono/srcs/server/db"
)

type ragSyncServiceImpl struct {
    provider db.Provider
}

// NewRAGSyncService creates a new RAGSyncService instance.
func NewRAGSyncService(provider db.Provider) RAGSyncService {
    return &ragSyncServiceImpl{
        provider: provider,
    }
}

func (s *ragSyncServiceImpl) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
    // Basic implementation for fetching pending syncs
    query := `
        SELECT id, content, embedding, sync_status, last_sync_at
        FROM autodream_memories
        WHERE sync_status = 'pending'
        LIMIT $1
    `

    var limitArg interface{} = limit
    if s.provider.IsSQLite() {
        query = `
        SELECT id, content, embedding, sync_status, last_sync_at
        FROM autodream_memories
        WHERE sync_status = 'pending'
        LIMIT ?
        `
    }

    rows, err := s.provider.Query(ctx, query, limitArg)
    if err != nil {
        return nil, err
    }
    defer rows.Close()

    var records []RAGSyncRecord
    for rows.Next() {
        var rec RAGSyncRecord
        var embedding []byte // Assuming bytes for vector for generic scanning
        var lastSyncAt sql.NullString
        err := rows.Scan(&rec.ID, &rec.Context, &embedding, &rec.SyncStatus, &lastSyncAt)
        if err != nil {
            return nil, err
        }
        // Handle NullString for lastSyncAt properly (memory SQLite does not always return native time)
        // Simplified mapping for the sake of the interface definition
        records = append(records, rec)
    }

    return records, nil
}

func (s *ragSyncServiceImpl) MarkSynced(ctx context.Context, ids []string) error {
    if len(ids) == 0 {
        return nil
    }

    // Simplified update logic
    for _, id := range ids {
        query := `
            UPDATE autodream_memories
            SET sync_status = 'synced', last_sync_at = CURRENT_TIMESTAMP
            WHERE id = $1
        `
        if s.provider.IsSQLite() {
            query = `
            UPDATE autodream_memories
            SET sync_status = 'synced', last_sync_at = CURRENT_TIMESTAMP
            WHERE id = ?
            `
        }

        _, err := s.provider.Exec(ctx, query, id)
        if err != nil {
            return err
        }
    }

    return nil
}

func (s *ragSyncServiceImpl) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
    // Simplified upsert logic
    for _, rec := range records {
        query := `
            INSERT INTO autodream_memories (id, content, sync_status, last_sync_at)
            VALUES ($1, $2, $3, $4)
            ON CONFLICT (id) DO UPDATE SET
                content = EXCLUDED.content,
                sync_status = EXCLUDED.sync_status,
                last_sync_at = EXCLUDED.last_sync_at
        `
        if s.provider.IsSQLite() {
             query = `
            INSERT INTO autodream_memories (id, content, sync_status, last_sync_at)
            VALUES (?, ?, ?, ?)
            ON CONFLICT (id) DO UPDATE SET
                content = EXCLUDED.content,
                sync_status = EXCLUDED.sync_status,
                last_sync_at = EXCLUDED.last_sync_at
        `
        }

        _, err := s.provider.Exec(ctx, query, rec.ID, rec.Context, rec.SyncStatus, rec.LastSyncAt)
        if err != nil {
            return err
        }
    }
    return nil
}
