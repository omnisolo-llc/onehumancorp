package hub

import (
    "context"
    "time"
    "log"
    "strings"
    "fmt"
    "encoding/json"
    "database/sql"

    "github.com/onehumancorp/mono/srcs/server/db"
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

type RAGSyncServiceImpl struct {
    dbProvider db.Provider
}

func NewRAGSyncService(dbProvider db.Provider) RAGSyncService {
    return &RAGSyncServiceImpl{dbProvider: dbProvider}
}

func (s *RAGSyncServiceImpl) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
    query := `SELECT id, content, embedding, sync_status, last_sync_at FROM autodream_memories WHERE sync_status = 'pending' LIMIT $1`
    rows, err := s.dbProvider.Query(ctx, query, limit)
    if err != nil {
        return nil, err
    }
    defer rows.Close()

    var records []RAGSyncRecord
    for rows.Next() {
        var rec RAGSyncRecord
        var vecStr sql.NullString
        var lastSyncAt sql.NullTime
        if err := rows.Scan(&rec.ID, &rec.Context, &vecStr, &rec.SyncStatus, &lastSyncAt); err != nil {
            continue // Skip errors for now, in a real app log them
        }
        if vecStr.Valid {
            var vec []float32
            json.Unmarshal([]byte(vecStr.String), &vec)
            rec.Vector = vec
        }
        if lastSyncAt.Valid {
            rec.LastSyncAt = lastSyncAt.Time
        }
        records = append(records, rec)
    }
    return records, nil
}

func (s *RAGSyncServiceImpl) MarkSynced(ctx context.Context, ids []string) error {
    if len(ids) == 0 {
        return nil
    }

    placeholders := make([]string, len(ids))
    args := make([]any, len(ids))
    for i, id := range ids {
        placeholders[i] = fmt.Sprintf("$%d", i+1)
        args[i] = id
    }

    query := fmt.Sprintf(`UPDATE autodream_memories SET sync_status = 'synced', last_sync_at = CURRENT_TIMESTAMP WHERE id IN (%s)`, strings.Join(placeholders, ","))

    _, err := s.dbProvider.Exec(ctx, query, args...)
    if err == nil {
        ragRecordsSyncedTotal.Add(ctx, int64(len(ids)))
    } else {
        ragSyncErrorsTotal.Add(ctx, 1)
    }
    return err
}

func (s *RAGSyncServiceImpl) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
    for _, rec := range records {
        vecBytes, _ := json.Marshal(rec.Vector)
        vecStr := string(vecBytes)

        query := `
            INSERT INTO autodream_memories (id, content, embedding, sync_status, last_sync_at)
            VALUES ($1, $2, $3, 'synced', CURRENT_TIMESTAMP)
            ON CONFLICT (id) DO UPDATE SET
            content = EXCLUDED.content,
            embedding = EXCLUDED.embedding,
            sync_status = 'synced',
            last_sync_at = CURRENT_TIMESTAMP`

        _, err := s.dbProvider.Exec(ctx, query, rec.ID, rec.Context, vecStr)
        if err != nil {
            ragSyncErrorsTotal.Add(ctx, 1)
            return err // Or continue and aggregate errors depending on strategy. Let's return on first fail.
        }
    }
    return nil
}

var (
    ragRecordsSyncedTotal metric.Int64Counter
    ragSyncErrorsTotal    metric.Int64Counter
)

func init() {
    meter := otel.Meter("github.com/onehumancorp/mono/srcs/server/hub")

    var err error
    ragRecordsSyncedTotal, err = meter.Int64Counter(
        "rag_records_synced_total",
        metric.WithDescription("Total number of RAG records successfully synced"),
    )
    if err != nil {
        log.Printf("failed to initialize metric rag_records_synced_total: %v", err)
    }

    ragSyncErrorsTotal, err = meter.Int64Counter(
        "rag_sync_errors_total",
        metric.WithDescription("Total number of RAG sync errors encountered"),
    )
    if err != nil {
        log.Printf("failed to initialize metric rag_sync_errors_total: %v", err)
    }
}
