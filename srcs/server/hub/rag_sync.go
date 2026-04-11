package hub

import (
    "context"
    "time"
    "strings"
    "fmt"
    "log"

    "github.com/onehumancorp/mono/srcs/server/db"
    "go.opentelemetry.io/otel"
)

var (
    meter                  = otel.Meter("github.com/onehumancorp/mono/srcs/server/hub")
    RecordsSyncedTotal, _  = meter.Int64Counter("rag_records_synced_total")
    SyncErrorsTotal, _     = meter.Int64Counter("rag_sync_errors_total")
)

type SyncStatus string

const (
    SyncStatusPending SyncStatus = "pending"
    SyncStatusSynced  SyncStatus = "synced"
    SyncStatusError   SyncStatus = "error"
)

type RAGSyncRecord struct {
    MemoryID     string
    Context      string
    Vector       []byte
    SourcePlugin *string
    CreatedAt    time.Time
    SyncStatus   SyncStatus
    LastSyncAt   *time.Time
}

type RAGSyncService interface {
    FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error)
    MarkSynced(ctx context.Context, ids []string) error
    ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error
}

type dbRAGSyncService struct {
    provider db.Provider
}

func NewRAGSyncService(provider db.Provider) RAGSyncService {
    return &dbRAGSyncService{provider: provider}
}

func (s *dbRAGSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
    query := "SELECT memory_id, context, vector_embedding, source_plugin, created_at, sync_status, last_sync_at FROM swarm_memory_embeddings WHERE sync_status = 'pending' LIMIT "

    if s.provider.IsSQLite() {
        query += "?"
    } else {
        query += "$1"
    }

    rows, err := s.provider.Query(ctx, query, limit)
    if err != nil {
        SyncErrorsTotal.Add(ctx, 1)
        return nil, err
    }
    defer rows.Close()

    var records []RAGSyncRecord
    for rows.Next() {
        var r RAGSyncRecord
        err := rows.Scan(&r.MemoryID, &r.Context, &r.Vector, &r.SourcePlugin, &r.CreatedAt, &r.SyncStatus, &r.LastSyncAt)
        if err != nil {
            SyncErrorsTotal.Add(ctx, 1)
            return nil, err
        }
        records = append(records, r)
    }
    return records, rows.Err()
}

func (s *dbRAGSyncService) MarkSynced(ctx context.Context, ids []string) error {
    if len(ids) == 0 {
        return nil
    }

    placeholders := make([]string, len(ids))
    args := make([]any, len(ids) + 2)
    args[0] = SyncStatusSynced
    args[1] = time.Now()

    for i, id := range ids {
        if s.provider.IsSQLite() {
            placeholders[i] = "?"
        } else {
            placeholders[i] = fmt.Sprintf("$%d", i+3)
        }
        args[i+2] = id
    }

    query := fmt.Sprintf("UPDATE swarm_memory_embeddings SET sync_status = %s, last_sync_at = %s WHERE memory_id IN (%s)",
        func() string { if s.provider.IsSQLite() { return "?" } else { return "$1" } }(),
        func() string { if s.provider.IsSQLite() { return "?" } else { return "$2" } }(),
        strings.Join(placeholders, ", "))

    _, err := s.provider.Exec(ctx, query, args...)
    if err != nil {
        SyncErrorsTotal.Add(ctx, 1)
        return err
    }
    RecordsSyncedTotal.Add(ctx, int64(len(ids)))
    return nil
}

func (s *dbRAGSyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
    if len(records) == 0 {
        return nil
    }

    for _, r := range records {
        var query string
        var args []any

        if s.provider.IsSQLite() {
           query = `INSERT INTO swarm_memory_embeddings (memory_id, context, vector_embedding, source_plugin, created_at, sync_status, last_sync_at)
                    VALUES (?, ?, ?, ?, ?, ?, ?)
                    ON CONFLICT(memory_id) DO UPDATE SET
                    context = excluded.context,
                    vector_embedding = excluded.vector_embedding,
                    source_plugin = excluded.source_plugin,
                    sync_status = excluded.sync_status,
                    last_sync_at = excluded.last_sync_at`
           args = []any{r.MemoryID, r.Context, r.Vector, r.SourcePlugin, r.CreatedAt, r.SyncStatus, r.LastSyncAt}
        } else {
           query = `INSERT INTO swarm_memory_embeddings (memory_id, context, vector_embedding, source_plugin, created_at, sync_status, last_sync_at)
                    VALUES ($1, $2, $3, $4, $5, $6, $7)
                    ON CONFLICT(memory_id) DO UPDATE SET
                    context = EXCLUDED.context,
                    vector_embedding = EXCLUDED.vector_embedding,
                    source_plugin = EXCLUDED.source_plugin,
                    sync_status = EXCLUDED.sync_status,
                    last_sync_at = EXCLUDED.last_sync_at`
           args = []any{r.MemoryID, r.Context, r.Vector, r.SourcePlugin, r.CreatedAt, r.SyncStatus, r.LastSyncAt}
        }

        _, err := s.provider.Exec(ctx, query, args...)
        if err != nil {
            SyncErrorsTotal.Add(ctx, 1)
            log.Printf("ProcessIncomingSync error: %v", err)
            return err
        }
    }

    return nil
}
