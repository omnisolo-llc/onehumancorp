package hub

import (
    "github.com/onehumancorp/mono/srcs/server/db"
    "context"
    "time"
    "go.opentelemetry.io/otel"
    "go.opentelemetry.io/otel/metric"
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
    Vector       []float32
    SyncStatus   SyncStatus
    LastSyncAt   time.Time
}

type RAGSyncService interface {
    FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error)
    MarkSynced(ctx context.Context, ids []string) error
    ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error
}

var (
    meter = otel.Meter("github.com/onehumancorp/mono/srcs/server/hub")
    RAGRecordsSyncedTotal metric.Int64Counter
    RAGSyncErrorsTotal    metric.Int64Counter
)

func init() {
    var err error
    RAGRecordsSyncedTotal, err = meter.Int64Counter("rag_records_synced_total", metric.WithDescription("Total RAG records synced"))
    if err != nil {
        panic(err)
    }
    RAGSyncErrorsTotal, err = meter.Int64Counter("rag_sync_errors_total", metric.WithDescription("Total RAG sync errors"))
    if err != nil {
        panic(err)
    }
}


type ragSyncServiceImpl struct {
    db *db.DB
}

func NewRAGSyncService(database *db.DB) RAGSyncService {
    return &ragSyncServiceImpl{db: database}
}

func (s *ragSyncServiceImpl) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
    // Querying swarm_memory table directly for pending syncs.
    // The vector field can be ignored or fetched from swarm_memory_embeddings later if needed.
    rows, err := s.db.Query(ctx, "SELECT key, value, sync_status FROM swarm_memory WHERE sync_status = $1 LIMIT $2", SyncStatusPending, limit)
    if err != nil {
        return nil, err
    }
    defer rows.Close()

    var records []RAGSyncRecord
    for rows.Next() {
        var rec RAGSyncRecord
        var statusStr string
        if err := rows.Scan(&rec.ID, &rec.Context, &statusStr); err != nil {
            return nil, err
        }
        rec.SyncStatus = SyncStatus(statusStr)
        records = append(records, rec)
    }
    return records, nil
}

func (s *ragSyncServiceImpl) MarkSynced(ctx context.Context, ids []string) error {
    if len(ids) == 0 {
        return nil
    }
    now := time.Now()
    for _, id := range ids {
        _, err := s.db.Exec(ctx, "UPDATE swarm_memory SET sync_status = $1, last_sync_at = $2 WHERE key = $3", SyncStatusSynced, now, id)
        if err != nil {
            RAGSyncErrorsTotal.Add(ctx, 1)
            return err
        }
    }
    RAGRecordsSyncedTotal.Add(ctx, int64(len(ids)))
    return nil
}

func (s *ragSyncServiceImpl) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
    now := time.Now()
    for _, r := range records {
        _, err := s.db.Exec(ctx, "INSERT INTO swarm_memory (key, value, sync_status, last_sync_at) VALUES ($1, $2, $3, $4) ON CONFLICT(key) DO UPDATE SET value = EXCLUDED.value, sync_status = EXCLUDED.sync_status, last_sync_at = EXCLUDED.last_sync_at", r.ID, r.Context, SyncStatusSynced, now)
        if err != nil {
            RAGSyncErrorsTotal.Add(ctx, 1)
            return err
        }
    }
    return nil
}
