package hub

import (
	"context"
	"time"
	"fmt"

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
	// FetchPendingSyncs retrieves records from the local DB that need syncing
	FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error)

	// MarkSynced updates the local DB after a successful sync to the cloud
	MarkSynced(ctx context.Context, ids []string) error

	// ProcessIncomingSync handles data pushed from a standalone client into the cloud DB
	ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error
}

type dbRAGSyncService struct {
	provider db.Provider
}

func NewRAGSyncService(provider db.Provider) RAGSyncService {
	return &dbRAGSyncService{provider: provider}
}

func (s *dbRAGSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	rows, err := s.provider.Query(ctx, "SELECT memory_id, context, vector_embedding, sync_status, last_sync_timestamp FROM swarm_memory_embeddings WHERE sync_status = $1 LIMIT $2", SyncStatusPending, limit)
	if err != nil {
		return nil, fmt.Errorf("failed to fetch pending syncs: %w", err)
	}
	defer rows.Close()

	var records []RAGSyncRecord
	for rows.Next() {
		var r RAGSyncRecord
		var lastSync *time.Time
		var status *string
		var vector []byte
		if err := rows.Scan(&r.ID, &r.Context, &vector, &status, &lastSync); err != nil {
			return nil, fmt.Errorf("failed to scan record: %w", err)
		}
		if status != nil {
			r.SyncStatus = SyncStatus(*status)
		}
		if lastSync != nil {
			r.LastSyncAt = *lastSync
		}
		if vector != nil {
			r.Vector = vector
		}
		records = append(records, r)
	}
	return records, nil
}

func (s *dbRAGSyncService) MarkSynced(ctx context.Context, ids []string) error {
	for _, id := range ids {
		_, err := s.provider.Exec(ctx, "UPDATE swarm_memory_embeddings SET sync_status = $1, last_sync_timestamp = $2 WHERE memory_id = $3", SyncStatusSynced, time.Now(), id)
		if err != nil {
			RagSyncErrorsTotal.Add(ctx, 1)
			return fmt.Errorf("failed to mark synced: %w", err)
		}
	}

	RagRecordsSyncedTotal.Add(ctx, int64(len(ids)))
	return nil
}

func (s *dbRAGSyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	// Begin transaction
	tx, err := s.provider.Begin(ctx)
	if err != nil {
		return fmt.Errorf("failed to begin tx: %w", err)
	}
	defer tx.Rollback(ctx)

	for _, r := range records {
		_, err := tx.Exec(ctx, `
			INSERT INTO swarm_memory_embeddings (memory_id, context, vector_embedding, sync_status, last_sync_timestamp)
			VALUES ($1, $2, $3, $4, $5)
			ON CONFLICT (memory_id) DO UPDATE
			SET context = EXCLUDED.context, vector_embedding = EXCLUDED.vector_embedding, sync_status = EXCLUDED.sync_status, last_sync_timestamp = EXCLUDED.last_sync_timestamp
		`, r.ID, r.Context, r.Vector, r.SyncStatus, r.LastSyncAt)
		if err != nil {
			RagSyncErrorsTotal.Add(ctx, 1)
			return fmt.Errorf("failed to process incoming sync: %w", err)
		}
	}
	if err := tx.Commit(ctx); err != nil {
		return fmt.Errorf("failed to commit tx: %w", err)
	}
	return nil
}


var (
	meter                   = otel.Meter("github.com/onehumancorp/mono/srcs/server/hub")
	RagRecordsSyncedTotal metric.Int64Counter
	RagSyncErrorsTotal    metric.Int64Counter
)

func init() {
	var err error
	RagRecordsSyncedTotal, err = meter.Int64Counter(
		"rag_records_synced_total",
		metric.WithDescription("Total number of RAG records synced to the cloud"),
	)
	if err != nil {
		panic(err)
	}

	RagSyncErrorsTotal, err = meter.Int64Counter(
		"rag_sync_errors_total",
		metric.WithDescription("Total number of errors encountered during RAG sync"),
	)
	if err != nil {
		panic(err)
	}
}
