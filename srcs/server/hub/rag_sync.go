package hub

import (
	"context"
	"time"
	"github.com/jackc/pgx/v5/pgxpool"
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
	ID         string
	Context    string
	Vector     []float32
	SyncStatus SyncStatus
	LastSyncAt time.Time
}

type RAGSyncService interface {
	FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error)
	MarkSynced(ctx context.Context, ids []string) error
	ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error
}

type RAGSyncServiceImpl struct {
	db *pgxpool.Pool
}

func NewRAGSyncService(db *pgxpool.Pool) *RAGSyncServiceImpl {
	return &RAGSyncServiceImpl{db: db}
}

func (s *RAGSyncServiceImpl) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	rows, err := s.db.Query(ctx, "SELECT id, context, sync_status, last_sync_at FROM autodream_memories WHERE sync_status = $1 LIMIT $2", SyncStatusPending, limit)
	if err != nil {
		return nil, err
	}
	defer rows.Close()

	var records []RAGSyncRecord
	for rows.Next() {
		var record RAGSyncRecord
		var lastSyncAt *time.Time
		if err := rows.Scan(&record.ID, &record.Context, &record.SyncStatus, &lastSyncAt); err != nil {
			return nil, err
		}
		if lastSyncAt != nil {
			record.LastSyncAt = *lastSyncAt
		}
		records = append(records, record)
	}
	return records, rows.Err()
}

func (s *RAGSyncServiceImpl) MarkSynced(ctx context.Context, ids []string) error {
	_, err := s.db.Exec(ctx, "UPDATE autodream_memories SET sync_status = $1, last_sync_at = $2 WHERE id = ANY($3)", SyncStatusSynced, time.Now(), ids)
	if err == nil {
		RecordsSynced.Add(ctx, int64(len(ids)))
	} else {
		SyncErrors.Add(ctx, 1)
	}
	return err
}

func (s *RAGSyncServiceImpl) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	// Simple upsert logic
	for _, record := range records {
		_, err := s.db.Exec(ctx, "INSERT INTO autodream_memories (id, context, sync_status, last_sync_at) VALUES ($1, $2, $3, $4) ON CONFLICT (id) DO UPDATE SET context = EXCLUDED.context, sync_status = EXCLUDED.sync_status, last_sync_at = EXCLUDED.last_sync_at", record.ID, record.Context, record.SyncStatus, record.LastSyncAt)
		if err != nil {
			SyncErrors.Add(ctx, 1)
			return err
		}
	}
	return nil
}

var (
	meter            = otel.Meter("github.com/onehumancorp/mono/srcs/server/hub")
	RecordsSynced, _ = meter.Int64Counter("rag_records_synced_total", metric.WithDescription("Total number of RAG records synced"))
	SyncErrors, _    = meter.Int64Counter("rag_sync_errors_total", metric.WithDescription("Total number of RAG sync errors"))
)
