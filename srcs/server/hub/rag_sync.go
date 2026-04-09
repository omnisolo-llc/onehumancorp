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
	Vector     []float32
	SyncStatus SyncStatus
	LastSyncAt time.Time
}

type RAGSyncService interface {
	FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error)
	MarkSynced(ctx context.Context, ids []string) error
	ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error
}

var (
	meter                    = otel.Meter("hub_rag_sync")
	RagRecordsSyncedTotal, _ = meter.Int64Counter(
		"rag_records_synced_total",
		metric.WithDescription("Total number of RAG records synced to cloud"),
	)
	RagSyncErrorsTotal, _ = meter.Int64Counter(
		"rag_sync_errors_total",
		metric.WithDescription("Total number of errors encountered during RAG sync"),
	)
)

type ragSyncService struct {
	db db.Provider
}

func NewRAGSyncService(db db.Provider) RAGSyncService {
	return &ragSyncService{
		db: db,
	}
}

func (s *ragSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	rows, err := s.db.Query(ctx, "SELECT memory_id, context, sync_status, last_sync_at FROM swarm_memory_embeddings WHERE sync_status = $1 LIMIT $2", string(SyncStatusPending), limit)
	if err != nil {
		RagSyncErrorsTotal.Add(ctx, 1)
		return nil, fmt.Errorf("failed to fetch pending syncs: %w", err)
	}
	defer rows.Close()

	var records []RAGSyncRecord
	for rows.Next() {
		var rec RAGSyncRecord
		var lastSyncAt any
		if err := rows.Scan(&rec.ID, &rec.Context, &rec.SyncStatus, &lastSyncAt); err != nil {
			RagSyncErrorsTotal.Add(ctx, 1)
			return nil, fmt.Errorf("failed to scan sync record: %w", err)
		}

		switch v := lastSyncAt.(type) {
		case string:
			t, err := time.Parse("2006-01-02 15:04:05.999999999Z07:00", v)
			if err == nil {
				rec.LastSyncAt = t
			}
		case time.Time:
			rec.LastSyncAt = v
		}

		records = append(records, rec)
	}

	if err := rows.Err(); err != nil {
		RagSyncErrorsTotal.Add(ctx, 1)
		return nil, fmt.Errorf("rows iteration error: %w", err)
	}

	return records, nil
}

func (s *ragSyncService) MarkSynced(ctx context.Context, ids []string) error {
	if len(ids) == 0 {
		return nil
	}

	tx, err := s.db.Begin(ctx)
	if err != nil {
		RagSyncErrorsTotal.Add(ctx, 1)
		return fmt.Errorf("failed to begin tx: %w", err)
	}
	defer tx.Rollback(ctx)

	for _, id := range ids {
		_, err := tx.Exec(ctx, "UPDATE swarm_memory_embeddings SET sync_status = $1, last_sync_at = CURRENT_TIMESTAMP WHERE memory_id = $2", string(SyncStatusSynced), id)
		if err != nil {
			RagSyncErrorsTotal.Add(ctx, 1)
			return fmt.Errorf("failed to mark synced for id %s: %w", id, err)
		}
	}

	if err := tx.Commit(ctx); err != nil {
		RagSyncErrorsTotal.Add(ctx, 1)
		return fmt.Errorf("failed to commit tx: %w", err)
	}

	RagRecordsSyncedTotal.Add(ctx, int64(len(ids)))
	return nil
}

func (s *ragSyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	if len(records) == 0 {
		return nil
	}

	tx, err := s.db.Begin(ctx)
	if err != nil {
		RagSyncErrorsTotal.Add(ctx, 1)
		return fmt.Errorf("failed to begin tx: %w", err)
	}
	defer tx.Rollback(ctx)

	for _, rec := range records {
		_, err := tx.Exec(ctx, "INSERT INTO swarm_memory_embeddings (memory_id, context, sync_status, last_sync_at) VALUES ($1, $2, $3, CURRENT_TIMESTAMP) ON CONFLICT (memory_id) DO UPDATE SET context = EXCLUDED.context, sync_status = EXCLUDED.sync_status, last_sync_at = EXCLUDED.last_sync_at", rec.ID, rec.Context, string(SyncStatusSynced))
		if err != nil {
			RagSyncErrorsTotal.Add(ctx, 1)
			return fmt.Errorf("failed to process incoming sync for id %s: %w", rec.ID, err)
		}
	}

	if err := tx.Commit(ctx); err != nil {
		RagSyncErrorsTotal.Add(ctx, 1)
		return fmt.Errorf("failed to commit tx: %w", err)
	}

	RagRecordsSyncedTotal.Add(ctx, int64(len(records)))
	return nil
}
