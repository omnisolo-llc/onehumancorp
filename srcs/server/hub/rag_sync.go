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
	ID           string
	Context      string
	Vector       []byte
	SyncStatus   SyncStatus
	LastSyncAt   time.Time
}

type RAGSyncService interface {
	FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error)
	MarkSynced(ctx context.Context, ids []string) error
	ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error
}

type HybridRAGSyncService struct {
	db db.Provider
}

func NewHybridRAGSyncService(database db.Provider) *HybridRAGSyncService {
	return &HybridRAGSyncService{db: database}
}

func (s *HybridRAGSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	query := "SELECT memory_id, context, vector_embedding, sync_status, last_sync_at FROM swarm_memory_embeddings WHERE sync_status = $1 LIMIT $2"

	rows, err := s.db.Query(ctx, query, SyncStatusPending, limit)
	if err != nil {
		SyncErrorsTotal.Add(ctx, 1)
		return nil, fmt.Errorf("failed to fetch pending syncs: %w", err)
	}
	defer rows.Close()

	var records []RAGSyncRecord
	for rows.Next() {
		var r RAGSyncRecord
		var lastSyncAt *time.Time
		var vector []byte
		err := rows.Scan(&r.ID, &r.Context, &vector, &r.SyncStatus, &lastSyncAt)
		if err != nil {
			SyncErrorsTotal.Add(ctx, 1)
			return nil, fmt.Errorf("failed to scan record: %w", err)
		}
		r.Vector = vector
		if lastSyncAt != nil {
			r.LastSyncAt = *lastSyncAt
		}
		records = append(records, r)
	}

	if err := rows.Err(); err != nil {
		SyncErrorsTotal.Add(ctx, 1)
		return nil, fmt.Errorf("rows iteration error: %w", err)
	}

	return records, nil
}

func (s *HybridRAGSyncService) MarkSynced(ctx context.Context, ids []string) error {
	if len(ids) == 0 {
		return nil
	}

	now := time.Now()
	for _, id := range ids {
		query := "UPDATE swarm_memory_embeddings SET sync_status = $1, last_sync_at = $2 WHERE memory_id = $3"
		_, err := s.db.Exec(ctx, query, SyncStatusSynced, now, id)
		if err != nil {
			SyncErrorsTotal.Add(ctx, 1)
			return fmt.Errorf("failed to mark record synced (ID: %s): %w", id, err)
		}
		RecordsSyncedTotal.Add(ctx, 1)
	}
	return nil
}

func (s *HybridRAGSyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	if len(records) == 0 {
		return nil
	}

	now := time.Now()
	for _, r := range records {
		query := `
			INSERT INTO swarm_memory_embeddings (memory_id, context, vector_embedding, sync_status, last_sync_at)
			VALUES ($1, $2, $3, $4, $5)
			ON CONFLICT (memory_id) DO UPDATE
			SET context = EXCLUDED.context, vector_embedding = EXCLUDED.vector_embedding, sync_status = EXCLUDED.sync_status, last_sync_at = EXCLUDED.last_sync_at
		`
		_, err := s.db.Exec(ctx, query, r.ID, r.Context, r.Vector, SyncStatusSynced, now)
		if err != nil {
			SyncErrorsTotal.Add(ctx, 1)
			return fmt.Errorf("failed to process incoming sync (ID: %s): %w", r.ID, err)
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
	RecordsSyncedTotal, err = meter.Int64Counter("rag_records_synced_total", metric.WithDescription("Total RAG records successfully synced to cloud"))
	if err != nil {
		panic(err)
	}
	SyncErrorsTotal, err = meter.Int64Counter("rag_sync_errors_total", metric.WithDescription("Total RAG sync errors encountered"))
	if err != nil {
		panic(err)
	}
}
