package hub

import (
    "context"
    "database/sql"
    "encoding/json"
    "fmt"
    "time"
    "strings"

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

type ragSyncServiceImpl struct {
    provider db.Provider
}

func NewRAGSyncService(provider db.Provider) RAGSyncService {
    return &ragSyncServiceImpl{
        provider: provider,
    }
}

func (s *ragSyncServiceImpl) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
    query := `SELECT id, content, CAST(embedding AS TEXT), sync_status, last_sync_at FROM autodream_memories WHERE sync_status = $1 LIMIT $2`
    rows, err := s.provider.Query(ctx, query, string(SyncStatusPending), limit)
    if err != nil {
        ragSyncErrorsTotal.Add(ctx, 1)
        return nil, fmt.Errorf("failed to fetch pending syncs: %w", err)
    }
    defer rows.Close()

    var records []RAGSyncRecord
    for rows.Next() {
        var r RAGSyncRecord
        var syncStatus sql.NullString
        var lastSyncAt sql.NullTime
        var vectorStr sql.NullString

        if err := rows.Scan(&r.ID, &r.Context, &vectorStr, &syncStatus, &lastSyncAt); err != nil {
            ragSyncErrorsTotal.Add(ctx, 1)
            return nil, fmt.Errorf("failed to scan record: %w", err)
        }

        if syncStatus.Valid {
            r.SyncStatus = SyncStatus(syncStatus.String)
        }
        if lastSyncAt.Valid {
            r.LastSyncAt = lastSyncAt.Time
        }

        if vectorStr.Valid && vectorStr.String != "" {
            var vec []float32
            if err := json.Unmarshal([]byte(vectorStr.String), &vec); err != nil {
                // If it fails to parse (e.g. pgvector syntax instead of JSON), we log/ignore or handle it.
                // For this implementation, we just attempt basic JSON array parsing which handles SQLite representations.
            } else {
                r.Vector = vec
            }
        }

        records = append(records, r)
    }
    if err := rows.Err(); err != nil {
        ragSyncErrorsTotal.Add(ctx, 1)
        return nil, err
    }
    return records, nil
}

func (s *ragSyncServiceImpl) MarkSynced(ctx context.Context, ids []string) error {
    if len(ids) == 0 {
        return nil
    }

    placeholders := make([]string, len(ids))
    args := make([]any, len(ids)+2)
    for i, id := range ids {
        placeholders[i] = fmt.Sprintf("$%d", i+3)
        args[i+2] = id
    }

    args[0] = string(SyncStatusSynced)
    args[1] = time.Now()

    query := fmt.Sprintf(`UPDATE autodream_memories SET sync_status = $1, last_sync_at = $2 WHERE id IN (%s)`, strings.Join(placeholders, ", "))

    affected, err := s.provider.Exec(ctx, query, args...)
    if err != nil {
        ragSyncErrorsTotal.Add(ctx, 1)
        return fmt.Errorf("failed to mark synced: %w", err)
    }

    ragRecordsSyncedTotal.Add(ctx, affected)

    return nil
}

func (s *ragSyncServiceImpl) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
    if len(records) == 0 {
        return nil
    }

    for _, r := range records {
        var vectorParam any
        if len(r.Vector) > 0 {
            // Convert to JSON string for SQLite/PG compatibility
            vBytes, _ := json.Marshal(r.Vector)
            vectorParam = string(vBytes)
        } else {
            vectorParam = nil
        }

        query := `INSERT INTO autodream_memories (id, content, embedding, sync_status, last_sync_at)
                  VALUES ($1, $2, $3, $4, $5)
                  ON CONFLICT (id) DO UPDATE SET
                  content = EXCLUDED.content,
                  embedding = EXCLUDED.embedding,
                  sync_status = EXCLUDED.sync_status,
                  last_sync_at = EXCLUDED.last_sync_at`

        _, err := s.provider.Exec(ctx, query, r.ID, r.Context, vectorParam, string(SyncStatusSynced), time.Now())
        if err != nil {
            ragSyncErrorsTotal.Add(ctx, 1)
            return fmt.Errorf("failed to upsert record: %w", err)
        }
    }

    ragRecordsSyncedTotal.Add(ctx, int64(len(records)))

    return nil
}

var (
    meter = otel.Meter("github.com/onehumancorp/mono/srcs/server/hub")

    ragRecordsSyncedTotal metric.Int64Counter
    ragSyncErrorsTotal    metric.Int64Counter
)

func init() {
    var err error
    ragRecordsSyncedTotal, err = meter.Int64Counter("rag_records_synced_total", metric.WithDescription("Total number of RAG records successfully synced"))
    if err != nil {
        panic(err)
    }
    ragSyncErrorsTotal, err = meter.Int64Counter("rag_sync_errors_total", metric.WithDescription("Total number of errors encountered during RAG sync"))
    if err != nil {
        panic(err)
    }
}
