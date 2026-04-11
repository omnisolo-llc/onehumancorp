package hub

import (
    "context"
    "time"
    "fmt"

    "go.opentelemetry.io/otel"
    "go.opentelemetry.io/otel/metric"
    "github.com/onehumancorp/mono/srcs/server/db"
    "github.com/google/uuid"
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

type SyncService struct {
    db db.Provider
}

func NewSyncService(provider db.Provider) *SyncService {
    return &SyncService{db: provider}
}

func (s *SyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
    query := `
        SELECT id, content, sync_status, last_sync_at
        FROM autodream_memories
        WHERE sync_status = 'pending'
        LIMIT $1
    `
    rows, err := s.db.Query(ctx, query, limit)
    if err != nil {
        return nil, err
    }
    defer rows.Close()

    var records []RAGSyncRecord
    for rows.Next() {
        var rec RAGSyncRecord
        var lastSyncAt *time.Time
        if err := rows.Scan(&rec.ID, &rec.Context, &rec.SyncStatus, &lastSyncAt); err != nil {
            return nil, err
        }
        if lastSyncAt != nil {
            rec.LastSyncAt = *lastSyncAt
        }
        records = append(records, rec)
    }
    return records, nil
}

func (s *SyncService) MarkSynced(ctx context.Context, ids []string) error {
    if len(ids) == 0 {
        return nil
    }

    query := `
        UPDATE autodream_memories
        SET sync_status = 'synced', last_sync_at = CURRENT_TIMESTAMP
        WHERE id = $1
    `

    for _, id := range ids {
        if _, err := s.db.Exec(ctx, query, id); err != nil {
            SyncErrorsTotal.Add(ctx, 1)
            return err
        }
        RecordsSyncedTotal.Add(ctx, 1)
    }
    return nil
}

func vectorToString(vec []float32) string {
    // Basic representation for SQLite string conversion
    // Postgres pgvector uses the format [1.0, 2.0, 3.0]
    out := "["
    for i, v := range vec {
        out += fmt.Sprintf("%f", v)
        if i < len(vec)-1 {
            out += ","
        }
    }
    out += "]"
    return out
}

func (s *SyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
    if len(records) == 0 {
        return nil
    }

    query := `
        INSERT INTO autodream_memories (id, content, embedding, sync_status, last_sync_at)
        VALUES ($1, $2, $3, $4, CURRENT_TIMESTAMP)
        ON CONFLICT (id) DO UPDATE SET
            content = EXCLUDED.content,
            embedding = EXCLUDED.embedding,
            sync_status = EXCLUDED.sync_status,
            last_sync_at = EXCLUDED.last_sync_at
    `

    for _, rec := range records {
        id := rec.ID
        if id == "" {
            id = uuid.New().String()
        }

        vecStr := vectorToString(rec.Vector)

        if _, err := s.db.Exec(ctx, query, id, rec.Context, vecStr, SyncStatusSynced); err != nil {
             SyncErrorsTotal.Add(ctx, 1)
             return err
        }
        RecordsSyncedTotal.Add(ctx, 1)
    }
    return nil
}

var (
    meter = otel.Meter("github.com/onehumancorp/mono/srcs/server/hub")
    RecordsSyncedTotal metric.Int64Counter
    SyncErrorsTotal    metric.Int64Counter
)

func init() {
    var err error
    RecordsSyncedTotal, err = meter.Int64Counter(
        "rag_records_synced_total",
        metric.WithDescription("Total number of RAG records successfully synced"),
    )
    if err != nil {
        panic(err)
    }

    SyncErrorsTotal, err = meter.Int64Counter(
        "rag_sync_errors_total",
        metric.WithDescription("Total number of errors encountered during RAG sync"),
    )
    if err != nil {
        panic(err)
    }
}
