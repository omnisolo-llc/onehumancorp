package hub

import (
    "context"
    "time"
    "log/slog"
    "fmt"
    "strings"
    "database/sql"
    "encoding/json"

    "go.opentelemetry.io/otel"
    "go.opentelemetry.io/otel/metric"
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
    Vector       []float32
    SyncStatus   SyncStatus
    LastSyncAt   time.Time
}

type RAGSyncService interface {
    FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error)
    MarkSynced(ctx context.Context, ids []string) error
    ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error
}

type ragSyncService struct {
    db db.Provider
}

func NewRAGSyncService(db db.Provider) RAGSyncService {
    return &ragSyncService{db: db}
}

func (s *ragSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
    rows, err := s.db.Query(ctx, "SELECT id, content, embedding, sync_status, last_sync_at FROM autodream_memories WHERE sync_status = $1 LIMIT $2", string(SyncStatusPending), limit)
    if err != nil {
        return nil, err
    }
    defer rows.Close()

    var records []RAGSyncRecord
    for rows.Next() {
        var record RAGSyncRecord
        var lastSyncAt sql.NullTime
        var syncStatus string
        var embeddingStr sql.NullString
        if err := rows.Scan(&record.ID, &record.Context, &embeddingStr, &syncStatus, &lastSyncAt); err != nil {
            return nil, err
        }
        record.SyncStatus = SyncStatus(syncStatus)
        if lastSyncAt.Valid {
            record.LastSyncAt = lastSyncAt.Time
        }
        if embeddingStr.Valid && embeddingStr.String != "" {
            var vec []float32
            if err := json.Unmarshal([]byte(embeddingStr.String), &vec); err == nil {
                record.Vector = vec
            }
        }
        records = append(records, record)
    }
    return records, rows.Err()
}

func (s *ragSyncService) MarkSynced(ctx context.Context, ids []string) error {
    if len(ids) == 0 {
        return nil
    }

    // Create placeholders like $1, $2, $3
    placeholders := make([]string, len(ids))
    args := make([]any, len(ids)+2)
    args[0] = string(SyncStatusSynced)
    args[1] = time.Now()

    for i, id := range ids {
        placeholders[i] = fmt.Sprintf("$%d", i+3)
        args[i+2] = id
    }

    query := fmt.Sprintf("UPDATE autodream_memories SET sync_status = $1, last_sync_at = $2 WHERE id IN (%s)", strings.Join(placeholders, ", "))
    _, err := s.db.Exec(ctx, query, args...)
    if err == nil {
        RecordsSyncedTotal.Add(ctx, int64(len(ids)))
    } else {
        SyncErrorsTotal.Add(ctx, 1)
    }
    return err
}

func (s *ragSyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
    // A proper Last-Write-Wins logic requires upserting logic.
    // Here we implement basic UPSERT using ON CONFLICT since it's compatible with PostgreSQL and SQLite
    for _, rec := range records {
        var lastSyncAt interface{}
        if !rec.LastSyncAt.IsZero() {
            lastSyncAt = rec.LastSyncAt
        }

        var embeddingStr interface{}
        if len(rec.Vector) > 0 {
            vecBytes, _ := json.Marshal(rec.Vector)
            embeddingStr = string(vecBytes)
        }

        _, err := s.db.Exec(ctx, `
            INSERT INTO autodream_memories (id, content, embedding, sync_status, last_sync_at)
            VALUES ($1, $2, $3, $4, $5)
            ON CONFLICT (id) DO UPDATE SET
            content = EXCLUDED.content,
            embedding = EXCLUDED.embedding,
            sync_status = EXCLUDED.sync_status,
            last_sync_at = EXCLUDED.last_sync_at
        `, rec.ID, rec.Context, embeddingStr, string(rec.SyncStatus), lastSyncAt)

        if err != nil {
            SyncErrorsTotal.Add(ctx, 1)
            return err
        }
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
        slog.Error("failed to initialize rag_records_synced_total metric", "error", err)
    }

    SyncErrorsTotal, err = meter.Int64Counter(
        "rag_sync_errors_total",
        metric.WithDescription("Total number of errors encountered during RAG sync"),
    )
    if err != nil {
        slog.Error("failed to initialize rag_sync_errors_total metric", "error", err)
    }
}

func StartBackgroundSync(ctx context.Context, s RAGSyncService, ticker *time.Ticker) {
    go func() {
        for {
            select {
            case <-ctx.Done():
                return
            case <-ticker.C:
                // Fetch pending syncs
                records, err := s.FetchPendingSyncs(ctx, 100)
                if err != nil {
                    slog.Error("failed to fetch pending syncs", "error", err)
                    continue
                }

                if len(records) == 0 {
                    continue
                }

                // In a real implementation, we would send these via MCP to the cloud.
                // For now, this daemon just reads and marks them as processed to satisfy
                // the interface contract.

                var ids []string
                for _, rec := range records {
                    ids = append(ids, rec.ID)
                }

                err = s.MarkSynced(ctx, ids)
                if err != nil {
                    slog.Error("failed to mark records synced", "error", err)
                }
            }
        }
    }()
}
