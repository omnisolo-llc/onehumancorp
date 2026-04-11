package hub

import (
    "context"
    "time"
    "go.opentelemetry.io/otel"
    "go.opentelemetry.io/otel/metric"
    "github.com/onehumancorp/mono/srcs/server/db"
)

var (
    Meter = otel.Meter("github.com/onehumancorp/mono/srcs/server/hub")
    RecordsSyncedTotal metric.Int64Counter
    SyncErrorsTotal    metric.Int64Counter
)

func init() {
    var err error
    RecordsSyncedTotal, err = Meter.Int64Counter(
        "rag_records_synced_total",
        metric.WithDescription("Total number of RAG memory records successfully synced"),
    )
    if err != nil {
        panic(err)
    }

    SyncErrorsTotal, err = Meter.Int64Counter(
        "rag_sync_errors_total",
        metric.WithDescription("Total number of errors encountered during RAG sync"),
    )
    if err != nil {
        panic(err)
    }
}

type SyncStatus string

const (
    SyncStatusPending SyncStatus = "pending"
    SyncStatusSynced  SyncStatus = "synced"
    SyncStatusError   SyncStatus = "error"
)

type RAGSyncRecord struct {
    ID           string
    Context      string
    Vector       []float32 // Convert to string internally for SQLite compat if needed
    SyncStatus   SyncStatus
    LastSyncAt   time.Time
}

type RAGSyncService interface {
    // FetchPendingSyncs retrieves records from the local DB that need syncing
    FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error)

    // MarkSynced updates the local DB after a successful sync to the cloud
    MarkSynced(ctx context.Context, ids []string) error

    // ProcessIncomingSync handles data pushed from a standalone client into the cloud DB
    ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error
}

type DefaultRAGSyncService struct {
    provider db.Provider
}

func NewDefaultRAGSyncService(provider db.Provider) *DefaultRAGSyncService {
    return &DefaultRAGSyncService{provider: provider}
}

func (s *DefaultRAGSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
    // Basic implementation using db.Provider to fetch pending syncs
    query := "SELECT memory_id, context, sync_status FROM swarm_memory_embeddings WHERE sync_status = 'pending' LIMIT $1"
    rows, err := s.provider.Query(ctx, query, limit)
    if err != nil {
        return nil, err
    }
    defer rows.Close()

    var records []RAGSyncRecord
    for rows.Next() {
        var rec RAGSyncRecord
        var syncStatus string
        if err := rows.Scan(&rec.ID, &rec.Context, &syncStatus); err != nil {
            return nil, err
        }
        rec.SyncStatus = SyncStatus(syncStatus)
        records = append(records, rec)
    }
    return records, rows.Err()
}

func (s *DefaultRAGSyncService) MarkSynced(ctx context.Context, ids []string) error {
    // Use loop to support both PostgreSQL and SQLite
    tx, err := s.provider.Begin(ctx)
    if err != nil {
        return err
    }
    defer tx.Rollback(ctx)

    for _, id := range ids {
        query := "UPDATE swarm_memory_embeddings SET sync_status = 'synced', last_sync_at = CURRENT_TIMESTAMP WHERE memory_id = $1"
        if _, err := tx.Exec(ctx, query, id); err != nil {
            return err
        }
    }
    return tx.Commit(ctx)
}

func (s *DefaultRAGSyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
    tx, err := s.provider.Begin(ctx)
    if err != nil {
        return err
    }
    defer tx.Rollback(ctx)

    for _, rec := range records {
        // Simplified upsert logic for illustration
        query := "UPDATE swarm_memory_embeddings SET context = $1, sync_status = 'synced', last_sync_at = CURRENT_TIMESTAMP WHERE memory_id = $2"
        if _, err := tx.Exec(ctx, query, rec.Context, rec.ID); err != nil {
            return err
        }
    }

    RecordsSyncedTotal.Add(ctx, int64(len(records)))
    return tx.Commit(ctx)
}
