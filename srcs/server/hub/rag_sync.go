package hub

import (
    "context"
    "time"
    "fmt"
    "strings"

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

type HubRAGSyncService struct {
    Provider db.Provider
}

func NewHubRAGSyncService(provider db.Provider) *HubRAGSyncService {
    return &HubRAGSyncService{Provider: provider}
}

func (s *HubRAGSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
    var rows db.Rows
    var err error

    if s.Provider.IsSQLite() {
        // For SQLite, update status to in_progress first to prevent serial worker contention, then select them
        _, err = s.Provider.Exec(ctx, "UPDATE swarm_memory_embeddings SET sync_status = 'in_progress' WHERE memory_id IN (SELECT memory_id FROM swarm_memory_embeddings WHERE sync_status = 'pending' LIMIT $1)", limit)
        if err != nil {
            return nil, err
        }
        rows, err = s.Provider.Query(ctx, "SELECT memory_id, context, vector_embedding, sync_status, last_sync_at FROM swarm_memory_embeddings WHERE sync_status = 'in_progress' LIMIT $1", limit)
    } else {
        // For PostgreSQL, use FOR UPDATE SKIP LOCKED
        rows, err = s.Provider.Query(ctx, "UPDATE swarm_memory_embeddings SET sync_status = 'in_progress' WHERE memory_id IN (SELECT memory_id FROM swarm_memory_embeddings WHERE sync_status = 'pending' LIMIT $1 FOR UPDATE SKIP LOCKED) RETURNING memory_id, context, vector_embedding, sync_status, last_sync_at", limit)
    }

    if err != nil {
        return nil, err
    }
    defer rows.Close()

    var records []RAGSyncRecord
    for rows.Next() {
        var r RAGSyncRecord
        var lastSyncAt *time.Time
        if err := rows.Scan(&r.ID, &r.Context, &r.Vector, &r.SyncStatus, &lastSyncAt); err != nil {
            return nil, err
        }
        if lastSyncAt != nil {
            r.LastSyncAt = *lastSyncAt
        }
        records = append(records, r)
    }
    return records, nil
}

func (s *HubRAGSyncService) MarkSynced(ctx context.Context, ids []string) error {
    if len(ids) == 0 {
        return nil
    }

    placeholders := make([]string, len(ids))
    args := make([]interface{}, len(ids))
    for i, id := range ids {
        placeholders[i] = fmt.Sprintf("$%d", i+1)
        args[i] = id
    }

    query := fmt.Sprintf("UPDATE swarm_memory_embeddings SET sync_status = 'synced', last_sync_at = CURRENT_TIMESTAMP WHERE memory_id IN (%s)", strings.Join(placeholders, ", "))
    _, err := s.Provider.Exec(ctx, query, args...)
    if err != nil {
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

func (s *HubRAGSyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
    if len(records) == 0 {
        return nil
    }

    for _, r := range records {
        var exists bool
        row := s.Provider.QueryRow(ctx, "SELECT EXISTS(SELECT 1 FROM swarm_memory_embeddings WHERE memory_id = $1)", r.ID)
        err := row.Scan(&exists)
        if err != nil {
            continue
        }
        if exists {
            _, err = s.Provider.Exec(ctx, "UPDATE swarm_memory_embeddings SET context = $1, vector_embedding = $2, sync_status = 'synced', last_sync_at = CURRENT_TIMESTAMP WHERE memory_id = $3", r.Context, r.Vector, r.ID)
        } else {
            _, err = s.Provider.Exec(ctx, "INSERT INTO swarm_memory_embeddings (memory_id, context, vector_embedding, sync_status, last_sync_at) VALUES ($1, $2, $3, 'synced', CURRENT_TIMESTAMP)", r.ID, r.Context, r.Vector)
        }
    }
    return nil
}
