package hub

import (
	"context"
	"fmt"
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

var (
	meter                = otel.Meter("github.com/onehumancorp/mono/srcs/server/hub")
	RecordsSyncedCounter metric.Int64Counter
	SyncErrorsCounter    metric.Int64Counter
)

func init() {
	var err error
	RecordsSyncedCounter, err = meter.Int64Counter("rag_records_synced_total", metric.WithDescription("Total number of RAG records synced"))
	if err != nil {
		fmt.Printf("Failed to initialize RecordsSyncedCounter: %v\n", err)
	}
	SyncErrorsCounter, err = meter.Int64Counter("rag_sync_errors_total", metric.WithDescription("Total number of errors during RAG sync"))
	if err != nil {
		fmt.Printf("Failed to initialize SyncErrorsCounter: %v\n", err)
	}
}

type ragSyncServiceImpl struct {
	db db.Provider
}

func NewRAGSyncService(provider db.Provider) RAGSyncService {
	return &ragSyncServiceImpl{db: provider}
}

func (s *ragSyncServiceImpl) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	rows, err := s.db.Query(ctx, "SELECT memory_id, context, sync_status, last_sync_at FROM swarm_memory_embeddings WHERE sync_status = 'pending' LIMIT $1", limit)
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
	if err := rows.Err(); err != nil {
		return nil, err
	}
	return records, nil
}

func (s *ragSyncServiceImpl) MarkSynced(ctx context.Context, ids []string) error {
	if len(ids) == 0 {
		return nil
	}

	tx, err := s.db.Begin(ctx)
	if err != nil {
		return err
	}
	defer tx.Rollback(ctx)

	for _, id := range ids {
		_, err := tx.Exec(ctx, "UPDATE swarm_memory_embeddings SET sync_status = 'synced', last_sync_at = CURRENT_TIMESTAMP WHERE memory_id = $1", id)
		if err != nil {
			if SyncErrorsCounter != nil {
				SyncErrorsCounter.Add(ctx, 1)
			}
			return err
		}
	}

	if err := tx.Commit(ctx); err != nil {
		return err
	}

	if RecordsSyncedCounter != nil {
		RecordsSyncedCounter.Add(ctx, int64(len(ids)))
	}
	return nil
}

func (s *ragSyncServiceImpl) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	if len(records) == 0 {
		return nil
	}

	tx, err := s.db.Begin(ctx)
	if err != nil {
		return err
	}
	defer tx.Rollback(ctx)

	for _, r := range records {
		_, err := tx.Exec(ctx, "INSERT INTO swarm_memory_embeddings (memory_id, context, sync_status, last_sync_at) VALUES ($1, $2, 'synced', CURRENT_TIMESTAMP) ON CONFLICT (memory_id) DO UPDATE SET context = EXCLUDED.context, sync_status = 'synced', last_sync_at = CURRENT_TIMESTAMP", r.ID, r.Context)
		if err != nil {
			if SyncErrorsCounter != nil {
				SyncErrorsCounter.Add(ctx, 1)
			}
			return err
		}
	}

	if err := tx.Commit(ctx); err != nil {
		return err
	}

	if RecordsSyncedCounter != nil {
		RecordsSyncedCounter.Add(ctx, int64(len(records)))
	}
	return nil
}
