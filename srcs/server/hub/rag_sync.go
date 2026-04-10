package hub

import (
    "context"
    "time"
    "log/slog"
    "fmt"

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
    FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error)
    MarkSynced(ctx context.Context, ids []string) error
    ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error
}

// DefaultRAGSyncService is a concrete implementation of RAGSyncService
type DefaultRAGSyncService struct {
    provider db.Provider
}

func NewRAGSyncService(provider db.Provider) *DefaultRAGSyncService {
    return &DefaultRAGSyncService{
        provider: provider,
    }
}

func (s *DefaultRAGSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
    rows, err := s.provider.Query(ctx, "SELECT id, content, sync_status, last_sync_at FROM autodream_memories WHERE sync_status = $1 LIMIT $2", string(SyncStatusPending), limit)
    if err != nil {
        return nil, fmt.Errorf("failed to fetch pending syncs: %w", err)
    }
    defer rows.Close()

    var records []RAGSyncRecord
    for rows.Next() {
        var record RAGSyncRecord
        var status string
        var lastSyncAt *time.Time
        // Since we omitted vector parsing for simplicity in this baseline implementation
        if err := rows.Scan(&record.ID, &record.Context, &status, &lastSyncAt); err != nil {
            slog.Error("failed to scan row", "error", err)
            continue
        }
        record.SyncStatus = SyncStatus(status)
        if lastSyncAt != nil {
            record.LastSyncAt = *lastSyncAt
        }
        records = append(records, record)
    }

    if err := rows.Err(); err != nil {
        return nil, fmt.Errorf("rows iteration error: %w", err)
    }

    return records, nil
}

func (s *DefaultRAGSyncService) MarkSynced(ctx context.Context, ids []string) error {
    if len(ids) == 0 {
        return nil
    }

    tx, err := s.provider.Begin(ctx)
    if err != nil {
        return fmt.Errorf("failed to begin transaction: %w", err)
    }
    defer tx.Rollback(ctx)

    now := time.Now()
    for _, id := range ids {
        _, err := tx.Exec(ctx, "UPDATE autodream_memories SET sync_status = $1, last_sync_at = $2 WHERE id = $3", string(SyncStatusSynced), now, id)
        if err != nil {
            return fmt.Errorf("failed to update record %s: %w", id, err)
        }
    }

    if err := tx.Commit(ctx); err != nil {
        return fmt.Errorf("failed to commit transaction: %w", err)
    }

    ragRecordsSyncedTotal.Add(ctx, int64(len(ids)))
    return nil
}

func (s *DefaultRAGSyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
    if len(records) == 0 {
        return nil
    }

    tx, err := s.provider.Begin(ctx)
    if err != nil {
        return fmt.Errorf("failed to begin transaction: %w", err)
    }
    defer tx.Rollback(ctx)

    now := time.Now()
    for _, record := range records {
        // Insert ... ON CONFLICT DO UPDATE is standard for Postgres and Modern SQLite
        _, err := tx.Exec(ctx, `
            INSERT INTO autodream_memories (id, content, sync_status, last_sync_at)
            VALUES ($1, $2, $3, $4)
            ON CONFLICT (id) DO UPDATE SET
                content = EXCLUDED.content,
                sync_status = EXCLUDED.sync_status,
                last_sync_at = EXCLUDED.last_sync_at`,
            record.ID, record.Context, string(SyncStatusSynced), now)
        if err != nil {
            ragSyncErrorsTotal.Add(ctx, 1)
            return fmt.Errorf("failed to upsert record %s: %w", record.ID, err)
        }
    }

    if err := tx.Commit(ctx); err != nil {
        return fmt.Errorf("failed to commit transaction: %w", err)
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
    ragRecordsSyncedTotal, err = meter.Int64Counter("rag_records_synced_total", metric.WithDescription("Total number of RAG records synced"))
    if err != nil {
        slog.Error("Failed to initialize rag_records_synced_total metric", "error", err)
    }

    ragSyncErrorsTotal, err = meter.Int64Counter("rag_sync_errors_total", metric.WithDescription("Total number of RAG sync errors"))
    if err != nil {
        slog.Error("Failed to initialize rag_sync_errors_total metric", "error", err)
    }
}
