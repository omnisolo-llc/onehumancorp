package hub

import (
    "context"
    "time"

    "github.com/onehumancorp/mono/srcs/server/db"
)

type ragSyncService struct {
    provider db.Provider
}

func NewRAGSyncService(provider db.Provider) RAGSyncService {
    return &ragSyncService{provider: provider}
}

func (s *ragSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
    query := `SELECT memory_id, context, sync_status FROM swarm_memory_embeddings WHERE sync_status = 'pending' LIMIT $1`
    rows, err := s.provider.Query(ctx, query, limit)
    if err != nil {
        return nil, err
    }
    defer rows.Close()

    var records []RAGSyncRecord
    for rows.Next() {
        var r RAGSyncRecord
        if err := rows.Scan(&r.ID, &r.Context, &r.SyncStatus); err != nil {
            return nil, err
        }
        records = append(records, r)
    }
    return records, rows.Err()
}

func (s *ragSyncService) MarkSynced(ctx context.Context, ids []string) error {
    if len(ids) == 0 {
        return nil
    }
    // simplified batch update using individual statements since simple ids might not support IN with slices natively in the wrapper.
    for _, id := range ids {
        _, err := s.provider.Exec(ctx, "UPDATE swarm_memory_embeddings SET sync_status = 'synced', last_sync_at = $1 WHERE memory_id = $2", time.Now(), id)
        if err != nil {
            return err
        }
    }
    return nil
}

func (s *ragSyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
    for _, r := range records {
        // simplified upsert logic. Upsert context using basic statements.
        // in reality this would be an INSERT ON CONFLICT DO UPDATE
        // assuming Postgres/SQLite
        var count int
        row := s.provider.QueryRow(ctx, "SELECT COUNT(*) FROM swarm_memory_embeddings WHERE memory_id = $1", r.ID)
        if err := row.Scan(&count); err != nil {
            return err
        }

        if count > 0 {
            _, err := s.provider.Exec(ctx, "UPDATE swarm_memory_embeddings SET context = $1, sync_status = $2, last_sync_at = $3 WHERE memory_id = $4", r.Context, r.SyncStatus, r.LastSyncAt, r.ID)
            if err != nil {
                return err
            }
        } else {
            _, err := s.provider.Exec(ctx, "INSERT INTO swarm_memory_embeddings (memory_id, context, sync_status, last_sync_at) VALUES ($1, $2, $3, $4)", r.ID, r.Context, r.SyncStatus, r.LastSyncAt)
            if err != nil {
                return err
            }
        }
    }
    return nil
}
