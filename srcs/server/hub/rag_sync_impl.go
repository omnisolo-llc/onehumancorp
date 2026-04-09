package hub

import (
    "context"
    "database/sql"

    "github.com/onehumancorp/mono/srcs/server/db"
    "github.com/onehumancorp/mono/srcs/server/auth"
)

// DefaultRAGSyncService is a placeholder implementation connecting to the DB
type DefaultRAGSyncService struct {
    db *db.DB
}

func NewDefaultRAGSyncService(dbConn *db.DB) *DefaultRAGSyncService {
    return &DefaultRAGSyncService{db: dbConn}
}

func (s *DefaultRAGSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
    claims := auth.ClaimsFromContext(ctx)
    if claims == nil {
        return nil, nil // Or return unauthorized error
    }

    query := `
        SELECT id, content, sync_status, last_sync_at
        FROM autodream_memories
        WHERE sync_status = 'pending' AND organization_id = $1
        LIMIT $2
    `
    rows, err := s.db.Query(ctx, query, claims.OrganizationID, limit)
    if err != nil {
        return nil, err
    }
    defer rows.Close()

    var records []RAGSyncRecord
    for rows.Next() {
        var r RAGSyncRecord
        var syncStatus string
        var lastSyncAt sql.NullTime
        if err := rows.Scan(&r.ID, &r.Context, &syncStatus, &lastSyncAt); err != nil {
            return nil, err
        }
        r.SyncStatus = SyncStatus(syncStatus)
        if lastSyncAt.Valid {
            r.LastSyncAt = lastSyncAt.Time
        }
        records = append(records, r)
    }

    return records, nil
}

func (s *DefaultRAGSyncService) MarkSynced(ctx context.Context, ids []string) error {
    if len(ids) == 0 {
        return nil
    }

    claims := auth.ClaimsFromContext(ctx)
    if claims == nil {
        return nil
    }

    // Since DB wrapper doesn't have BeginTx, we will just execute them contextually
    // Or we should update the mock if needed, but lets just use Exec
    for _, id := range ids {
        _, err := s.db.Exec(ctx, `
            UPDATE autodream_memories
            SET sync_status = 'synced', last_sync_at = CURRENT_TIMESTAMP
            WHERE id = $1 AND organization_id = $2
        `, id, claims.OrganizationID)
        if err != nil {
            return err
        }
    }

    ragRecordsSyncedTotal.Add(ctx, int64(len(ids)))
    return nil
}

func (s *DefaultRAGSyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
    claims := auth.ClaimsFromContext(ctx)
    if claims == nil {
        return nil
    }

    for _, r := range records {
        _, err := s.db.Exec(ctx, `
            INSERT INTO autodream_memories (id, content, sync_status, last_sync_at, organization_id, source_type)
            VALUES ($1, $2, 'synced', CURRENT_TIMESTAMP, $3, 'cloud_sync')
            ON CONFLICT (id) DO UPDATE SET
                content = EXCLUDED.content,
                sync_status = 'synced',
                last_sync_at = CURRENT_TIMESTAMP
        `, r.ID, r.Context, claims.OrganizationID)
        if err != nil {
            return err
        }
    }

    return nil
}
