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
	ID         string
	Context    string
	Vector     []byte
	SyncStatus SyncStatus
	LastSyncAt time.Time
}

type RAGSyncService interface {
	FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error)
	MarkSynced(ctx context.Context, ids []string) error
	ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error
}

type DefaultRAGSyncService struct {
	dbProvider db.Provider
}

func NewRAGSyncService(provider db.Provider) RAGSyncService {
	return &DefaultRAGSyncService{
		dbProvider: provider,
	}
}

func (s *DefaultRAGSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	query := "SELECT memory_id, context, vector_embedding, sync_status, last_sync_at FROM swarm_memory_embeddings WHERE sync_status = 'pending' LIMIT $1"

	if !s.dbProvider.IsSQLite() {
		query += " FOR UPDATE SKIP LOCKED"
	}

	rows, err := s.dbProvider.Query(ctx, query, limit)
	if err != nil {
		return nil, err
	}
	defer rows.Close()

	var records []RAGSyncRecord
	for rows.Next() {
		var record RAGSyncRecord
		var status string
		var lastSyncAt *time.Time
		if err := rows.Scan(&record.ID, &record.Context, &record.Vector, &status, &lastSyncAt); err != nil {
			return nil, err
		}
		record.SyncStatus = SyncStatus(status)
		if lastSyncAt != nil {
			record.LastSyncAt = *lastSyncAt
		}
		records = append(records, record)
	}
	return records, rows.Err()
}

func (s *DefaultRAGSyncService) MarkSynced(ctx context.Context, ids []string) error {
	if len(ids) == 0 {
		return nil
	}

	tx, err := s.dbProvider.Begin(ctx)
	if err != nil {
		return err
	}

	for _, id := range ids {
		_, err := tx.Exec(ctx, "UPDATE swarm_memory_embeddings SET sync_status = 'synced', last_sync_at = CURRENT_TIMESTAMP WHERE memory_id = $1", id)
		if err != nil {
			_ = tx.Rollback(ctx)
			SyncErrorsTotal.Add(ctx, 1)
			return err
		}
	}

	err = tx.Commit(ctx)
	if err == nil {
		RecordsSyncedTotal.Add(ctx, int64(len(ids)))
	}
	return err
}

func (s *DefaultRAGSyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	if len(records) == 0 {
		return nil
	}

	tx, err := s.dbProvider.Begin(ctx)
	if err != nil {
		return err
	}

	for _, record := range records {
		_, err := tx.Exec(ctx, "INSERT INTO swarm_memory_embeddings (memory_id, context, vector_embedding, sync_status, last_sync_at) VALUES ($1, $2, $3, 'synced', CURRENT_TIMESTAMP) ON CONFLICT (memory_id) DO UPDATE SET context = EXCLUDED.context, vector_embedding = EXCLUDED.vector_embedding, sync_status = 'synced', last_sync_at = CURRENT_TIMESTAMP", record.ID, record.Context, record.Vector)
		if err != nil {
			_ = tx.Rollback(ctx)
			SyncErrorsTotal.Add(ctx, 1)
			return err
		}
	}

	err = tx.Commit(ctx)
	if err == nil {
		RecordsSyncedTotal.Add(ctx, int64(len(records)))
	}
	return err
}

var (
	meter                 = otel.Meter("github.com/onehumancorp/mono/srcs/server/hub")
	RecordsSyncedTotal, _ = meter.Int64Counter("rag_records_synced_total", metric.WithDescription("Total number of RAG records synced"))
	SyncErrorsTotal, _    = meter.Int64Counter("rag_sync_errors_total", metric.WithDescription("Total number of RAG sync errors"))
)
