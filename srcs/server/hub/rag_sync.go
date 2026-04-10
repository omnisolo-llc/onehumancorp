package hub

import (
    "context"
    "time"
    "fmt"
    "log/slog"
    "strings"
    "encoding/json"

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

var (
    ragRecordsSynced metric.Int64Counter
    ragSyncErrors    metric.Int64Counter
)

func init() {
    meter := otel.Meter("github.com/onehumancorp/mono/srcs/server/hub")
    var err error
    ragRecordsSynced, err = meter.Int64Counter("rag_records_synced_total", metric.WithDescription("Total RAG records synced"))
    if err != nil {
        slog.Error("Failed to initialize rag_records_synced_total counter", "error", err)
    }
    ragSyncErrors, err = meter.Int64Counter("rag_sync_errors_total", metric.WithDescription("Total RAG sync errors"))
    if err != nil {
        slog.Error("Failed to initialize rag_sync_errors_total counter", "error", err)
    }
}

type RAGSyncManager struct {
    provider db.Provider
}

func NewRAGSyncManager(provider db.Provider) *RAGSyncManager {
    return &RAGSyncManager{provider: provider}
}

func (m *RAGSyncManager) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
    query := `SELECT id, content, sync_status, last_sync_at, embedding FROM autodream_memories WHERE sync_status = 'pending' LIMIT $1`
    rows, err := m.provider.Query(ctx, query, limit)
    if err != nil {
        if ragSyncErrors != nil {
            ragSyncErrors.Add(ctx, 1)
        }
        return nil, fmt.Errorf("failed to fetch pending syncs: %w", err)
    }
    defer rows.Close()

    var records []RAGSyncRecord
    for rows.Next() {
        var rec RAGSyncRecord
        var embeddingStr *string
        var lastSyncAt *time.Time
        var syncStatus *string

        err := rows.Scan(&rec.ID, &rec.Context, &syncStatus, &lastSyncAt, &embeddingStr)
        if err != nil {
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

        if embeddingStr != nil && *embeddingStr != "" {
            str := strings.TrimPrefix(strings.TrimSuffix(*embeddingStr, "]"), "[")
            if str != "" {
               var vec []float32
               err = json.Unmarshal([]byte(*embeddingStr), &vec)
               if err == nil {
                   rec.Vector = vec
               }
            }
        }
        records = append(records, rec)
    }
    return records, nil
}

func (m *RAGSyncManager) MarkSynced(ctx context.Context, ids []string) error {
    if len(ids) == 0 {
        return nil
    }

    placeholders := make([]string, len(ids))
    args := make([]any, len(ids)+1)
    args[0] = time.Now()

    for i, id := range ids {
        placeholders[i] = fmt.Sprintf("$%d", i+2)
        args[i+1] = id
    }

    query := fmt.Sprintf(`UPDATE autodream_memories SET sync_status = 'synced', last_sync_at = $1 WHERE id IN (%s)`, strings.Join(placeholders, ","))

    _, err := m.provider.Exec(ctx, query, args...)
    if err != nil {
        if ragSyncErrors != nil {
            ragSyncErrors.Add(ctx, 1)
        }
        return fmt.Errorf("failed to mark records as synced: %w", err)
    }

    if ragRecordsSynced != nil {
        ragRecordsSynced.Add(ctx, int64(len(ids)))
    }

    return nil
}

func (m *RAGSyncManager) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
    for _, rec := range records {
        vectorBytes, _ := json.Marshal(rec.Vector)
        vectorStr := string(vectorBytes)

        query := `
            INSERT INTO autodream_memories (id, content, sync_status, last_sync_at, embedding)
            VALUES ($1, $2, $3, $4, $5)
            ON CONFLICT (id) DO UPDATE SET
                content = excluded.content,
                sync_status = excluded.sync_status,
                last_sync_at = excluded.last_sync_at,
                embedding = excluded.embedding
        `
        _, err := m.provider.Exec(ctx, query, rec.ID, rec.Context, "synced", time.Now(), vectorStr)
        if err != nil {
            if ragSyncErrors != nil {
                ragSyncErrors.Add(ctx, 1)
            }
            return fmt.Errorf("failed to upsert incoming record %s: %w", rec.ID, err)
        }

        if ragRecordsSynced != nil {
            ragRecordsSynced.Add(ctx, 1)
        }
    }
    return nil
}
