package hub

import (
    "context"
    "time"
    "database/sql"
    "strconv"
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

type sqlRAGSyncService struct {
    provider db.Provider
}

func NewSQLRAGSyncService(provider db.Provider) RAGSyncService {
    return &sqlRAGSyncService{provider: provider}
}

func parseVectorStr(s string) []float32 {
    s = strings.TrimPrefix(s, "[")
    s = strings.TrimSuffix(s, "]")
    if s == "" {
        return nil
    }
    parts := strings.Split(s, ",")
    var res []float32
    for _, p := range parts {
        p = strings.TrimSpace(p)
        if f, err := strconv.ParseFloat(p, 32); err == nil {
            res = append(res, float32(f))
        }
    }
    return res
}

func formatVectorStr(v []float32) string {
    if len(v) == 0 {
        return "[]"
    }
    var parts []string
    for _, f := range v {
        parts = append(parts, strconv.FormatFloat(float64(f), 'f', -1, 32))
    }
    return "[" + strings.Join(parts, ",") + "]"
}

func (s *sqlRAGSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
    q := `
        SELECT id, content, embedding, sync_status, last_sync_timestamp
        FROM consolidated_memory
        WHERE sync_status = 'pending'
        LIMIT $1
    `
    rows, err := s.provider.Query(ctx, q, limit)
    if err != nil {
        SyncErrorsTotal.Add(ctx, 1)
        return nil, err
    }
    defer rows.Close()

    var records []RAGSyncRecord
    for rows.Next() {
        var rec RAGSyncRecord
        var lastSync sql.NullTime
        var syncStatus sql.NullString
        var id, content, embeddingStr sql.NullString

        if err := rows.Scan(&id, &content, &embeddingStr, &syncStatus, &lastSync); err != nil {
            SyncErrorsTotal.Add(ctx, 1)
            continue
        }

        if id.Valid {
            rec.ID = id.String
        }
        if content.Valid {
            rec.Context = content.String
        }
        if embeddingStr.Valid {
            rec.Vector = parseVectorStr(embeddingStr.String)
        }
        if syncStatus.Valid {
            rec.SyncStatus = SyncStatus(syncStatus.String)
        }
        if lastSync.Valid {
            rec.LastSyncAt = lastSync.Time
        }

        records = append(records, rec)
    }

    return records, nil
}

func (s *sqlRAGSyncService) MarkSynced(ctx context.Context, ids []string) error {
    if len(ids) == 0 {
        return nil
    }

    // In a real implementation this would use IN or multiple execs
    for _, id := range ids {
        q := `UPDATE consolidated_memory SET sync_status = 'synced', last_sync_timestamp = CURRENT_TIMESTAMP WHERE id = $1`
        _, err := s.provider.Exec(ctx, q, id)
        if err != nil {
            SyncErrorsTotal.Add(ctx, 1)
            return err
        }
    }

    RecordsSyncedTotal.Add(ctx, int64(len(ids)))
    return nil
}

func (s *sqlRAGSyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
    for _, r := range records {
        vStr := formatVectorStr(r.Vector)
        q := `
            INSERT INTO consolidated_memory (id, organization_id, content, embedding, sync_status, last_sync_timestamp, source_type)
            VALUES ($1, 'default', $2, $3, 'synced', CURRENT_TIMESTAMP, 'sync')
            ON CONFLICT(id) DO UPDATE SET content = $2, embedding = $3, sync_status = 'synced', last_sync_timestamp = CURRENT_TIMESTAMP
        `
        _, err := s.provider.Exec(ctx, q, r.ID, r.Context, vStr)
        if err != nil {
            SyncErrorsTotal.Add(ctx, 1)
            return err
        }
    }
    return nil
}

var (
    meter               = otel.Meter("github.com/onehumancorp/mono/srcs/server/hub")
    RecordsSyncedTotal  metric.Int64Counter
    SyncErrorsTotal     metric.Int64Counter
)

func init() {
    var err error
    RecordsSyncedTotal, err = meter.Int64Counter("rag_records_synced_total", metric.WithDescription("Total number of RAG records synced successfully"))
    if err != nil {
        panic(err)
    }
    SyncErrorsTotal, err = meter.Int64Counter("rag_sync_errors_total", metric.WithDescription("Total number of RAG sync errors"))
    if err != nil {
        panic(err)
    }
}
