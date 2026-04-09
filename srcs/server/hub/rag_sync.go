package hub

import (
    "context"
    "time"
    "encoding/json"
    "fmt"
    "log/slog"

    "github.com/onehumancorp/mono/srcs/server/db"
    "go.opentelemetry.io/otel"
    "go.opentelemetry.io/otel/metric"
)

var (
    meter = otel.Meter("github.com/onehumancorp/mono/srcs/server/hub")
    RagRecordsSyncedTotal metric.Int64Counter
    RagSyncErrorsTotal    metric.Int64Counter
)

func init() {
    var err error
    RagRecordsSyncedTotal, err = meter.Int64Counter("rag_records_synced_total", metric.WithDescription("Total RAG records synced to cloud"))
    if err != nil {
        panic(err)
    }
    RagSyncErrorsTotal, err = meter.Int64Counter("rag_sync_errors_total", metric.WithDescription("Total RAG sync errors"))
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

type HybridRAGSyncService struct {
    dbWrapper *db.DB
}

func NewHybridRAGSyncService(dbWrapper *db.DB) *HybridRAGSyncService {
    return &HybridRAGSyncService{
        dbWrapper: dbWrapper,
    }
}

func (s *HybridRAGSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
    if s.dbWrapper == nil {
        return nil, fmt.Errorf("db is nil")
    }
    query := `
        SELECT id, content, embedding, sync_status, last_sync_at
        FROM consolidated_memory
        WHERE sync_status = 'pending' OR sync_status IS NULL
        LIMIT $1
    `
    rows, err := s.dbWrapper.Query(ctx, query, limit)
    if err != nil {
        RagSyncErrorsTotal.Add(ctx, 1)
        return nil, err
    }
    defer rows.Close()

    var records []RAGSyncRecord
    for rows.Next() {
        var rec RAGSyncRecord
        var vectorStr *string
        var lastSyncAt *time.Time
        var syncStatus *string

        err := rows.Scan(&rec.ID, &rec.Context, &vectorStr, &syncStatus, &lastSyncAt)
        if err != nil {
            slog.Error("Failed to scan row", "error", err)
            continue
        }

        if syncStatus != nil {
            rec.SyncStatus = SyncStatus(*syncStatus)
        } else {
            rec.SyncStatus = SyncStatusPending
        }
        if lastSyncAt != nil {
            rec.LastSyncAt = *lastSyncAt
        }

        if vectorStr != nil {
            var floats []float32
            if err := json.Unmarshal([]byte(*vectorStr), &floats); err == nil {
                rec.Vector = floats
            }
        }

        records = append(records, rec)
    }

    return records, nil
}

func (s *HybridRAGSyncService) MarkSynced(ctx context.Context, ids []string) error {
    if s.dbWrapper == nil {
        return fmt.Errorf("db is nil")
    }
    if len(ids) == 0 {
        return nil
    }

    query := `
        UPDATE consolidated_memory
        SET sync_status = 'synced', last_sync_at = CURRENT_TIMESTAMP
        WHERE id = $1
    `

    for _, id := range ids {
        _, err := s.dbWrapper.Exec(ctx, query, id)
        if err != nil {
            slog.Error("Failed to mark record synced", "id", id, "error", err)
            RagSyncErrorsTotal.Add(ctx, 1)
            return err
        }
    }

    RagRecordsSyncedTotal.Add(ctx, int64(len(ids)))
    return nil
}

func (s *HybridRAGSyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
    if s.dbWrapper == nil {
        return fmt.Errorf("db is nil")
    }
    if len(records) == 0 {
        return nil
    }

    query := `
        INSERT INTO consolidated_memory (id, organization_id, content, embedding, source_type, sync_status, last_sync_at)
        VALUES ($1, 'system', $2, $3, 'hybrid_sync', 'synced', CURRENT_TIMESTAMP)
        ON CONFLICT(id) DO UPDATE SET
            content = EXCLUDED.content,
            embedding = EXCLUDED.embedding,
            sync_status = 'synced',
            last_sync_at = CURRENT_TIMESTAMP
    `

    for _, rec := range records {
        var vectorStr *string
        if rec.Vector != nil {
            b, _ := json.Marshal(rec.Vector)
            s := string(b)
            vectorStr = &s
        }

        _, err := s.dbWrapper.Exec(ctx, query, rec.ID, rec.Context, vectorStr)
        if err != nil {
            slog.Error("Failed to process incoming sync", "id", rec.ID, "error", err)
            RagSyncErrorsTotal.Add(ctx, 1)
            return err
        }
    }
    return nil
}
