package hub

import (
    "context"
    "time"
    "fmt"
    "strings"
    "database/sql"

    "github.com/onehumancorp/mono/srcs/server/db"
    "go.opentelemetry.io/otel"
)

var (
    meter            = otel.Meter("github.com/onehumancorp/mono/srcs/server/hub")
    ragSyncedTotal, _ = meter.Int64Counter("rag_records_synced_total")
    ragSyncErrors, _  = meter.Int64Counter("rag_sync_errors_total")
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
    FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error)
    MarkSynced(ctx context.Context, ids []string) error
    ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error
}

type DefaultRAGSyncService struct {
    provider db.Provider
}

func NewRAGSyncService(provider db.Provider) RAGSyncService {
    return &DefaultRAGSyncService{provider: provider}
}

func (s *DefaultRAGSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
    query := `SELECT id, content, sync_status, last_sync_at FROM autodream_memories WHERE sync_status = 'pending' LIMIT $1`
    rows, err := s.provider.Query(ctx, query, limit)
    if err != nil {
        return nil, fmt.Errorf("failed to fetch pending syncs: %w", err)
    }
    defer rows.Close()

    var records []RAGSyncRecord
    for rows.Next() {
        var r RAGSyncRecord
        var lastSyncAt sql.NullTime
        if err := rows.Scan(&r.ID, &r.Context, &r.SyncStatus, &lastSyncAt); err != nil {
            return nil, fmt.Errorf("failed to scan row: %w", err)
        }
        if lastSyncAt.Valid {
            r.LastSyncAt = lastSyncAt.Time
        }
        records = append(records, r)
    }
    return records, nil
}

func (s *DefaultRAGSyncService) MarkSynced(ctx context.Context, ids []string) error {
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

    _, err := s.provider.Exec(ctx, query, args...)
    if err != nil {
        ragSyncErrors.Add(ctx, 1)
        return fmt.Errorf("failed to mark synced: %w", err)
    }
    ragSyncedTotal.Add(ctx, int64(len(ids)))
    return nil
}

// vectorToString converts float32 array to a string representation for SQLite compatibility.
func vectorToString(vector []float32) string {
    if len(vector) == 0 {
        return ""
    }
    str := make([]string, len(vector))
    for i, v := range vector {
        str[i] = fmt.Sprintf("%f", v)
    }
    return "[" + strings.Join(str, ",") + "]"
}

func (s *DefaultRAGSyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
    for _, r := range records {
        query := `INSERT INTO autodream_memories (id, content, embedding, sync_status, last_sync_at)
                  VALUES ($1, $2, $3, $4, CURRENT_TIMESTAMP)
                  ON CONFLICT(id) DO UPDATE SET content = $2, embedding = $3, sync_status = $4, last_sync_at = CURRENT_TIMESTAMP`

        var embeddingArg interface{}
        if len(r.Vector) > 0 {
             embeddingArg = vectorToString(r.Vector)
        } else {
             embeddingArg = nil
        }
        _, err := s.provider.Exec(ctx, query, r.ID, r.Context, embeddingArg, "synced")
        if err != nil {
            ragSyncErrors.Add(ctx, 1)
            return fmt.Errorf("failed to process incoming sync for id %s: %w", r.ID, err)
        }
    }
    return nil
}
