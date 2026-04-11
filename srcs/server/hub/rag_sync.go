package hub

import (
    "context"
    "time"
    "fmt"
    "github.com/onehumancorp/mono/srcs/server/db"
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

type ragSyncServiceImpl struct {
    provider db.Provider
}

func NewRAGSyncService(provider db.Provider) RAGSyncService {
    return &ragSyncServiceImpl{provider: provider}
}

func (s *ragSyncServiceImpl) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
    query := "SELECT memory_id, context, vector_embedding, sync_status, last_sync_at FROM swarm_memory_embeddings WHERE sync_status = "
    if s.provider.IsSQLite() {
        query += "? LIMIT ?"
    } else {
        query += "$1 LIMIT $2"
    }

    rows, err := s.provider.Query(ctx, query, string(SyncStatusPending), limit)
    if err != nil {
        return nil, err
    }
    defer rows.Close()

    var records []RAGSyncRecord
    for rows.Next() {
        var r RAGSyncRecord
        var lastSyncAt *time.Time
        var status string
        if err := rows.Scan(&r.ID, &r.Context, &r.Vector, &status, &lastSyncAt); err != nil {
            return nil, err
        }
        r.SyncStatus = SyncStatus(status)
        if lastSyncAt != nil {
            r.LastSyncAt = *lastSyncAt
        }
        records = append(records, r)
    }
    if err := rows.Err(); err != nil {
        return nil, err
    }
    return records, nil
}

func (s *ragSyncServiceImpl) MarkSynced(ctx context.Context, ids []string) error {
    if len(ids) == 0 {
        return nil
    }

    query := "UPDATE swarm_memory_embeddings SET sync_status = "
    var args []interface{}
    args = append(args, string(SyncStatusSynced), time.Now())

    if s.provider.IsSQLite() {
        query += "?, last_sync_at = ? WHERE memory_id IN ("
        for i, id := range ids {
            if i > 0 {
                query += ", "
            }
            query += "?"
            args = append(args, id)
        }
        query += ")"
    } else {
        query += "$1, last_sync_at = $2 WHERE memory_id IN ("
        for i, id := range ids {
            if i > 0 {
                query += ", "
            }
            query += fmt.Sprintf("$%d", i+3)
            args = append(args, id)
        }
        query += ")"
    }

    _, err := s.provider.Exec(ctx, query, args...)
    return err
}

func (s *ragSyncServiceImpl) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
    if len(records) == 0 {
        return nil
    }
    for _, r := range records {
        var query string
        var args []interface{}
        if s.provider.IsSQLite() {
            query = "INSERT INTO swarm_memory_embeddings (memory_id, context, vector_embedding, sync_status, last_sync_at) VALUES (?, ?, ?, ?, ?) ON CONFLICT(memory_id) DO UPDATE SET context = excluded.context, vector_embedding = excluded.vector_embedding, sync_status = excluded.sync_status, last_sync_at = excluded.last_sync_at"
            args = []interface{}{r.ID, r.Context, r.Vector, string(r.SyncStatus), r.LastSyncAt}
        } else {
            query = "INSERT INTO swarm_memory_embeddings (memory_id, context, vector_embedding, sync_status, last_sync_at) VALUES ($1, $2, $3, $4, $5) ON CONFLICT(memory_id) DO UPDATE SET context = EXCLUDED.context, vector_embedding = EXCLUDED.vector_embedding, sync_status = EXCLUDED.sync_status, last_sync_at = EXCLUDED.last_sync_at"
            args = []interface{}{r.ID, r.Context, r.Vector, string(r.SyncStatus), r.LastSyncAt}
        }
        if _, err := s.provider.Exec(ctx, query, args...); err != nil {
            return err
        }
    }
    return nil
}
