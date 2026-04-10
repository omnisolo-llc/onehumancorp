package hub

import (
	"context"
	"fmt"
	"time"

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
	ID         string
	Context    string
	Vector     []byte
	SyncStatus SyncStatus
	LastSyncAt time.Time
}

type RAGSyncService interface {
	// FetchPendingSyncs retrieves records from the local DB that need syncing
	FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error)

	// MarkSynced updates the local DB after a successful sync to the cloud
	MarkSynced(ctx context.Context, ids []string) error

	// ProcessIncomingSync handles data pushed from a standalone client into the cloud DB
	ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error
}

var (
	meter                   = otel.Meter("srcs/server/hub")
	RagRecordsSyncedTotal, _ = meter.Int64Counter(
		"rag_records_synced_total",
		metric.WithDescription("Total number of RAG records successfully synced"),
	)
	RagSyncErrorsTotal, _ = meter.Int64Counter(
		"rag_sync_errors_total",
		metric.WithDescription("Total number of errors encountered during RAG syncing"),
	)
)

type DB_RAGSyncService struct {
	provider db.Provider
}

func NewDB_RAGSyncService(provider db.Provider) *DB_RAGSyncService {
	return &DB_RAGSyncService{provider: provider}
}

func (s *DB_RAGSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	query := `SELECT memory_id, context, vector_embedding, sync_status, last_sync_at FROM swarm_memory_embeddings WHERE sync_status = 'pending' LIMIT $1`
	rows, err := s.provider.Query(ctx, query, limit)
	if err != nil {
		RagSyncErrorsTotal.Add(ctx, 1)
		return nil, err
	}
	defer rows.Close()

	var records []RAGSyncRecord
	for rows.Next() {
		var record RAGSyncRecord
		var lastSyncAt *time.Time
		var vectorEmbedding []byte
		var syncStatus string
		if err := rows.Scan(&record.ID, &record.Context, &vectorEmbedding, &syncStatus, &lastSyncAt); err != nil {
			RagSyncErrorsTotal.Add(ctx, 1)
			return nil, err
		}
		record.Vector = vectorEmbedding
		record.SyncStatus = SyncStatus(syncStatus)
		if lastSyncAt != nil {
			record.LastSyncAt = *lastSyncAt
		}
		records = append(records, record)
	}

	if err := rows.Err(); err != nil {
		RagSyncErrorsTotal.Add(ctx, 1)
		return nil, err
	}

	return records, nil
}

func (s *DB_RAGSyncService) MarkSynced(ctx context.Context, ids []string) error {
	if len(ids) == 0 {
		return nil
	}

	// db provider doesn't support array arguments out of the box so let's do a loop or string builder
	tx, err := s.provider.Begin(ctx)
	if err != nil {
		RagSyncErrorsTotal.Add(ctx, 1)
		return err
	}
	defer tx.Rollback(ctx)

	for _, id := range ids {
		query := `UPDATE swarm_memory_embeddings SET sync_status = 'synced', last_sync_at = CURRENT_TIMESTAMP WHERE memory_id = $1`
		_, err := tx.Exec(ctx, query, id)
		if err != nil {
			RagSyncErrorsTotal.Add(ctx, 1)
			return err
		}
	}

	if err := tx.Commit(ctx); err != nil {
		RagSyncErrorsTotal.Add(ctx, 1)
		return err
	}

	RagRecordsSyncedTotal.Add(ctx, int64(len(ids)))
	return nil
}

func (s *DB_RAGSyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	if len(records) == 0 {
		return nil
	}

	tx, err := s.provider.Begin(ctx)
	if err != nil {
		RagSyncErrorsTotal.Add(ctx, 1)
		return err
	}
	defer tx.Rollback(ctx)

	for _, record := range records {
		query := `INSERT INTO swarm_memory_embeddings (memory_id, context, vector_embedding, sync_status, last_sync_at) VALUES ($1, $2, $3, 'synced', CURRENT_TIMESTAMP) ON CONFLICT(memory_id) DO UPDATE SET context = EXCLUDED.context, vector_embedding = EXCLUDED.vector_embedding, sync_status = 'synced', last_sync_at = CURRENT_TIMESTAMP`
		_, err := tx.Exec(ctx, query, record.ID, record.Context, record.Vector)
		if err != nil {
			RagSyncErrorsTotal.Add(ctx, 1)
			return fmt.Errorf("failed to process incoming sync for record %s: %w", record.ID, err)
		}
	}

	if err := tx.Commit(ctx); err != nil {
		RagSyncErrorsTotal.Add(ctx, 1)
		return err
	}

	return nil
}
