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
	Vector     []float32 // Convert to string internally for SQLite compat if needed
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

var meter = otel.Meter("github.com/onehumancorp/mono/srcs/server/hub")

var (
	RecordsSyncedTotal metric.Int64Counter
	SyncErrorsTotal    metric.Int64Counter
)

// InitRAGSyncMetrics can be called to initialize the counters
func InitRAGSyncMetrics(m metric.Meter) {
    if m == nil {
        return
    }
	RecordsSyncedTotal, _ = m.Int64Counter("rag_records_synced_total", metric.WithDescription("Total number of RAG records synced"))
	SyncErrorsTotal, _ = m.Int64Counter("rag_sync_errors_total", metric.WithDescription("Total number of RAG sync errors"))
}

type ragSyncServiceImpl struct {
	provider db.Provider
}

// NewRAGSyncService creates a new instance of the RAG Sync Service
func NewRAGSyncService(provider db.Provider) RAGSyncService {
	return &ragSyncServiceImpl{
		provider: provider,
	}
}

// FetchPendingSyncs retrieves records from the local DB that need syncing
func (s *ragSyncServiceImpl) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	query := `
		SELECT memory_id, context, sync_status, last_sync_at
		FROM swarm_memory_embeddings
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
		var r RAGSyncRecord
		var lastSyncAt *time.Time
		err := rows.Scan(&r.ID, &r.Context, &r.SyncStatus, &lastSyncAt)
		if err != nil {
			return nil, err
		}
		if lastSyncAt != nil {
			r.LastSyncAt = *lastSyncAt
		}
		records = append(records, r)
	}

	return records, nil
}

// MarkSynced updates the local DB after a successful sync to the cloud
func (s *ragSyncServiceImpl) MarkSynced(ctx context.Context, ids []string) error {
	if len(ids) == 0 {
		return nil
	}

	// Create a transaction since we are mutating state
	tx, err := s.provider.Begin(ctx)
	if err != nil {
		return err
	}
	defer tx.Rollback(ctx)

	query := `
		UPDATE swarm_memory_embeddings
		SET sync_status = 'synced', last_sync_at = CURRENT_TIMESTAMP
		WHERE memory_id = $1
	`

	for _, id := range ids {
		_, err := tx.Exec(ctx, query, id)
		if err != nil {
			return err
		}
	}

	if err := tx.Commit(ctx); err != nil {
		return err
	}

	if RecordsSyncedTotal != nil {
		RecordsSyncedTotal.Add(ctx, int64(len(ids)))
	}

	return nil
}

// ProcessIncomingSync handles data pushed from a standalone client into the cloud DB
func (s *ragSyncServiceImpl) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	if len(records) == 0 {
		return nil
	}

	tx, err := s.provider.Begin(ctx)
	if err != nil {
		return err
	}
	defer tx.Rollback(ctx)

	// basic upsert logic, handling sqlite/postgres gracefully could be needed but
	// for now we stick to a basic insert, or do select then update/insert
	// the mission specifically states "gateway receives, validates, and upserts into the multi-tenant Postgres DB".

	upsertQuery := `
		INSERT INTO swarm_memory_embeddings (memory_id, context, sync_status, last_sync_at)
		VALUES ($1, $2, $3, CURRENT_TIMESTAMP)
		ON CONFLICT (memory_id) DO UPDATE
		SET context = EXCLUDED.context, sync_status = EXCLUDED.sync_status, last_sync_at = EXCLUDED.last_sync_at
	`

	for _, r := range records {
		_, err := tx.Exec(ctx, upsertQuery, r.ID, r.Context, "synced")
		if err != nil {
			if SyncErrorsTotal != nil {
				SyncErrorsTotal.Add(ctx, 1)
			}
			return err
		}
	}

	if err := tx.Commit(ctx); err != nil {
		return err
	}

	if RecordsSyncedTotal != nil {
		RecordsSyncedTotal.Add(ctx, int64(len(records)))
	}

	return nil
}
