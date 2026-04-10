package hub

import (
	"context"
	"fmt"
	"strings"
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
	Vector     []byte // Note: vector embeddings as bytes to handle raw DB storage
	SyncStatus SyncStatus
	LastSyncAt time.Time
}

type RAGSyncService interface {
	FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error)
	MarkSynced(ctx context.Context, ids []string) error
	ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error
}

var (
	meter                = otel.Meter("github.com/onehumancorp/mono/srcs/server/hub")
	ragRecordsSyncedTotal metric.Int64Counter
	ragSyncErrorsTotal    metric.Int64Counter
)

func init() {
	var err error
	ragRecordsSyncedTotal, err = meter.Int64Counter("rag_records_synced_total", metric.WithDescription("Total number of RAG records successfully synced to cloud"))
	if err != nil {
		panic(err)
	}
	ragSyncErrorsTotal, err = meter.Int64Counter("rag_sync_errors_total", metric.WithDescription("Total number of errors encountered during RAG sync"))
	if err != nil {
		panic(err)
	}
}

// Ensure implementation matches interface
var _ RAGSyncService = (*DBRAGSyncService)(nil)

type DBRAGSyncService struct {
	db db.Provider
}

func NewDBRAGSyncService(provider db.Provider) *DBRAGSyncService {
	return &DBRAGSyncService{db: provider}
}

func (s *DBRAGSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	rows, err := s.db.Query(ctx, "SELECT memory_id, context, vector_embedding, sync_status, last_sync_at FROM swarm_memory_embeddings WHERE sync_status = $1 LIMIT $2", SyncStatusPending, limit)
	if err != nil {
		return nil, fmt.Errorf("failed to fetch pending syncs: %w", err)
	}
	defer rows.Close()

	var records []RAGSyncRecord
	for rows.Next() {
		var rec RAGSyncRecord
		var lastSyncAt *time.Time
		var vector []byte
		if err := rows.Scan(&rec.ID, &rec.Context, &vector, &rec.SyncStatus, &lastSyncAt); err != nil {
			return nil, fmt.Errorf("failed to scan sync record: %w", err)
		}
		rec.Vector = vector
		if lastSyncAt != nil {
			rec.LastSyncAt = *lastSyncAt
		}
		records = append(records, rec)
	}
	return records, nil
}

func (s *DBRAGSyncService) MarkSynced(ctx context.Context, ids []string) error {
	if len(ids) == 0 {
		return nil
	}

	// Batch update for performance
	placeholders := make([]string, len(ids))
	args := make([]interface{}, len(ids)+2)
	args[0] = SyncStatusSynced
	args[1] = time.Now()
	for i, id := range ids {
		placeholders[i] = fmt.Sprintf("$%d", i+3)
		args[i+2] = id
	}

	query := fmt.Sprintf("UPDATE swarm_memory_embeddings SET sync_status = $1, last_sync_at = $2 WHERE memory_id IN (%s)", strings.Join(placeholders, ","))
	_, err := s.db.Exec(ctx, query, args...)
	if err != nil {
		ragSyncErrorsTotal.Add(ctx, int64(len(ids)))
		return fmt.Errorf("failed to mark records as synced: %w", err)
	}
	ragRecordsSyncedTotal.Add(ctx, int64(len(ids)))

	return nil
}

func (s *DBRAGSyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	tx, err := s.db.Begin(ctx)
	if err != nil {
		return fmt.Errorf("failed to begin transaction: %w", err)
	}
	defer tx.Rollback(ctx)

	for _, rec := range records {
		_, err := tx.Exec(ctx, `
			INSERT INTO swarm_memory_embeddings (memory_id, context, vector_embedding, sync_status, last_sync_at)
			VALUES ($1, $2, $3, $4, $5)
			ON CONFLICT (memory_id) DO UPDATE SET
				context = EXCLUDED.context,
				vector_embedding = EXCLUDED.vector_embedding,
				sync_status = EXCLUDED.sync_status,
				last_sync_at = EXCLUDED.last_sync_at
		`, rec.ID, rec.Context, rec.Vector, SyncStatusSynced, time.Now())
		if err != nil {
			ragSyncErrorsTotal.Add(ctx, 1)
			return fmt.Errorf("failed to upsert record %s: %w", rec.ID, err)
		}
		ragRecordsSyncedTotal.Add(ctx, 1)
	}

	if err := tx.Commit(ctx); err != nil {
		return fmt.Errorf("failed to commit transaction: %w", err)
	}
	return nil
}
