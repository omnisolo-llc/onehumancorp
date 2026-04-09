package hub

import (
	"context"
	"time"
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

type sqlRAGSyncService struct {
    provider db.Provider
}

func NewSQLRAGSyncService(provider db.Provider) RAGSyncService {
    return &sqlRAGSyncService{provider: provider}
}

func (s *sqlRAGSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
    query := `
        SELECT id, content, sync_status, last_sync_at
        FROM autodream_memories
        WHERE sync_status = 'pending'
        LIMIT $1
    `
    rows, err := s.provider.Query(ctx, query, limit)
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

func (s *sqlRAGSyncService) MarkSynced(ctx context.Context, ids []string) error {
    if len(ids) == 0 {
        return nil
    }

    for _, id := range ids {
        _, err := s.provider.Exec(ctx, `UPDATE autodream_memories SET sync_status = 'synced', last_sync_at = CURRENT_TIMESTAMP WHERE id = $1`, id)
        if err != nil {
            syncErrors.Add(ctx, 1)
            return err
        }
    }
    recordsSynced.Add(ctx, int64(len(ids)))
    return nil
}

func (s *sqlRAGSyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
    for _, rec := range records {
        query := `
            INSERT INTO autodream_memories (id, content, sync_status, last_sync_at)
            VALUES ($1, $2, 'synced', CURRENT_TIMESTAMP)
            ON CONFLICT (id) DO UPDATE SET
                content = EXCLUDED.content,
                sync_status = 'synced',
                last_sync_at = CURRENT_TIMESTAMP
        `
        _, err := s.provider.Exec(ctx, query, rec.ID, rec.Context)
        if err != nil {
            syncErrors.Add(ctx, 1)
            return err
        }
    }
    return nil
}


var (
	meter            = otel.Meter("github.com/onehumancorp/mono/srcs/server/hub")
	recordsSynced    metric.Int64Counter
	syncErrors       metric.Int64Counter
)

func init() {
	var err error
	recordsSynced, err = meter.Int64Counter("rag_records_synced_total", metric.WithDescription("Total number of RAG records synced"))
	if err != nil {
		panic(err)
	}
	syncErrors, err = meter.Int64Counter("rag_sync_errors_total", metric.WithDescription("Total number of RAG sync errors"))
	if err != nil {
		panic(err)
	}
}
