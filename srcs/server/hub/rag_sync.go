package hub

import (
    "context"
    "time"

    "github.com/onehumancorp/mono/srcs/server/db"
    "go.opentelemetry.io/otel"
    "go.opentelemetry.io/otel/metric"
)

var (
    meter = otel.Meter("github.com/onehumancorp/mono/srcs/server/hub")
    RecordsSyncedTotal metric.Int64Counter
    SyncErrorsTotal    metric.Int64Counter
)

func init() {
    var err error
    RecordsSyncedTotal, err = meter.Int64Counter("rag_records_synced_total", metric.WithDescription("Total number of RAG records synced"))
    if err != nil {
        panic(err)
    }

    SyncErrorsTotal, err = meter.Int64Counter("rag_sync_errors_total", metric.WithDescription("Total number of RAG sync errors"))
    if err != nil {
        panic(err)
    }
}

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

type RAGSyncServiceImpl struct {
    db db.Provider
}

func NewRAGSyncService(database db.Provider) RAGSyncService {
    return &RAGSyncServiceImpl{db: database}
}

func (s *RAGSyncServiceImpl) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
    tx, err := s.db.Begin(ctx)
    if err != nil {
        return nil, err
    }
    defer tx.Rollback(ctx)

    var query string
    if s.db.IsSQLite() {
        query = "SELECT memory_id, context, vector_embedding, sync_status, last_sync_at FROM swarm_memory_embeddings WHERE sync_status = 'pending' LIMIT ?"
    } else {
        query = "SELECT memory_id, context, vector_embedding, sync_status, last_sync_at FROM swarm_memory_embeddings WHERE sync_status = 'pending' LIMIT $1 FOR UPDATE SKIP LOCKED"
    }

    var rows db.Rows
    if s.db.IsSQLite() {
        rows, err = tx.Query(ctx, query, limit)
    } else {
        rows, err = tx.Query(ctx, query, limit)
    }

    if err != nil {
        return nil, err
    }
    defer rows.Close()

    var records []RAGSyncRecord
    var ids []string
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
        ids = append(ids, r.ID)
    }

    if err := rows.Err(); err != nil {
        return nil, err
    }

    // Update status to in_progress
    if len(ids) > 0 {
        updateQuery := ""
        if s.db.IsSQLite() {
            updateQuery = "UPDATE swarm_memory_embeddings SET sync_status = 'in_progress' WHERE memory_id = ?"
        } else {
            updateQuery = "UPDATE swarm_memory_embeddings SET sync_status = 'in_progress' WHERE memory_id = $1"
        }

        for _, id := range ids {
             _, err := tx.Exec(ctx, updateQuery, id)
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

func (s *RAGSyncServiceImpl) MarkSynced(ctx context.Context, ids []string) error {
    if len(ids) == 0 {
        return nil
    }

    updateQuery := ""
    if s.db.IsSQLite() {
        updateQuery = "UPDATE swarm_memory_embeddings SET sync_status = 'synced', last_sync_at = CURRENT_TIMESTAMP WHERE memory_id = ?"
    } else {
        updateQuery = "UPDATE swarm_memory_embeddings SET sync_status = 'synced', last_sync_at = CURRENT_TIMESTAMP WHERE memory_id = $1"
    }

    for _, id := range ids {
        _, err := s.db.Exec(ctx, updateQuery, id)
        if err != nil {
            SyncErrorsTotal.Add(ctx, 1)
            return err
        }
    }
    RecordsSyncedTotal.Add(ctx, int64(len(ids)))
    return nil
}

func (s *RAGSyncServiceImpl) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
    if len(records) == 0 {
        return nil
    }

    query := ""
    if s.db.IsSQLite() {
        query = "INSERT INTO swarm_memory_embeddings (memory_id, context, vector_embedding, sync_status, last_sync_at) VALUES (?, ?, ?, 'synced', CURRENT_TIMESTAMP) ON CONFLICT(memory_id) DO UPDATE SET context=excluded.context, vector_embedding=excluded.vector_embedding, sync_status='synced', last_sync_at=CURRENT_TIMESTAMP"
    } else {
        query = "INSERT INTO swarm_memory_embeddings (memory_id, context, vector_embedding, sync_status, last_sync_at) VALUES ($1, $2, $3, 'synced', CURRENT_TIMESTAMP) ON CONFLICT(memory_id) DO UPDATE SET context=excluded.context, vector_embedding=excluded.vector_embedding, sync_status='synced', last_sync_at=CURRENT_TIMESTAMP"
    }

    for _, r := range records {
        var err error
        if s.db.IsSQLite() {
             _, err = s.db.Exec(ctx, query, r.ID, r.Context, r.Vector)
        } else {
             _, err = s.db.Exec(ctx, query, r.ID, r.Context, r.Vector)
        }
        if err != nil {
            SyncErrorsTotal.Add(ctx, 1)
            return err
        }
    }
    RecordsSyncedTotal.Add(ctx, int64(len(records)))
    return nil
}
