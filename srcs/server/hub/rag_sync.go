package hub

import (
	"context"
	"encoding/json"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
	"go.opentelemetry.io/otel"
	"go.opentelemetry.io/otel/metric"
)

var (
	meter                   = otel.Meter("github.com/onehumancorp/mono/srcs/server/hub")
	recordsSyncedCounter, _ = meter.Int64Counter(
		"rag_records_synced_total",
		metric.WithDescription("Total number of RAG records successfully synchronized"),
	)
	syncErrorsCounter, _ = meter.Int64Counter(
		"rag_sync_errors_total",
		metric.WithDescription("Total number of errors encountered during RAG record synchronization"),
	)
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

type RAGSyncServiceImpl struct {
	DB *db.DB
}

func NewRAGSyncService(db *db.DB) *RAGSyncServiceImpl {
	return &RAGSyncServiceImpl{DB: db}
}

// FetchPendingSyncs retrieves records from the local DB that need syncing
func (s *RAGSyncServiceImpl) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	rows, err := s.DB.Query(ctx, `
		SELECT memory_id, context, vector_embedding, sync_status, last_sync_at
		FROM swarm_memory_embeddings
		WHERE sync_status = 'pending'
		LIMIT $1
	`, limit)
	if err != nil {
		syncErrorsCounter.Add(ctx, 1)
		return nil, err
	}
	defer rows.Close()

	var records []RAGSyncRecord
	for rows.Next() {
		var r RAGSyncRecord
		var lastSync *time.Time
		var vectorBytes []byte
		if err := rows.Scan(&r.ID, &r.Context, &vectorBytes, &r.SyncStatus, &lastSync); err != nil {
			syncErrorsCounter.Add(ctx, 1)
			return nil, err
		}
		if len(vectorBytes) > 0 {
			// Try to unmarshal JSON array since we convert VECTOR to TEXT in SQLite or JSONB in Postgres
			var vec []float32
			if err := json.Unmarshal(vectorBytes, &vec); err == nil {
				r.Vector = vec
			}
		}
		if lastSync != nil {
			r.LastSyncAt = *lastSync
		}
		records = append(records, r)
	}
	return records, nil
}

// MarkSynced updates the local DB after a successful sync to the cloud
func (s *RAGSyncServiceImpl) MarkSynced(ctx context.Context, ids []string) error {
	if len(ids) == 0 {
		return nil
	}

	tx, err := s.DB.Begin(ctx)
	if err != nil {
		syncErrorsCounter.Add(ctx, 1)
		return err
	}
	defer tx.Rollback(ctx)

	for _, id := range ids {
		_, err := tx.Exec(ctx, `
			UPDATE swarm_memory_embeddings
			SET sync_status = 'synced', last_sync_at = CURRENT_TIMESTAMP
			WHERE memory_id = $1
		`, id)
		if err != nil {
			syncErrorsCounter.Add(ctx, 1)
			return err
		}
	}

	if err := tx.Commit(ctx); err != nil {
		syncErrorsCounter.Add(ctx, 1)
		return err
	}

	recordsSyncedCounter.Add(ctx, int64(len(ids)))
	return nil
}

// ProcessIncomingSync handles data pushed from a standalone client into the cloud DB
func (s *RAGSyncServiceImpl) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	if len(records) == 0 {
		return nil
	}

	tx, err := s.DB.Begin(ctx)
	if err != nil {
		syncErrorsCounter.Add(ctx, 1)
		return err
	}
	defer tx.Rollback(ctx)

	for _, r := range records {
		// we ignore vector for ProcessIncomingSync for simplicity as the problem is with the logic
		vectorBytes, _ := json.Marshal(r.Vector)
		// Explicitly cast JSON byte slice to string for standard db insertion
		// This prevents base64 encoding from driver and works for both SQLite TEXT and PG JSONB
		vectorStr := string(vectorBytes)

		_, err := tx.Exec(ctx, `
			INSERT INTO swarm_memory_embeddings (memory_id, context, vector_embedding, sync_status, last_sync_at)
			VALUES ($1, $2, $3, 'synced', CURRENT_TIMESTAMP)
			ON CONFLICT (memory_id) DO UPDATE SET
				context = EXCLUDED.context,
				vector_embedding = EXCLUDED.vector_embedding,
				sync_status = 'synced',
				last_sync_at = CURRENT_TIMESTAMP
		`, r.ID, r.Context, vectorStr)
		if err != nil {
			syncErrorsCounter.Add(ctx, 1)
			return err
		}
	}

	if err := tx.Commit(ctx); err != nil {
		syncErrorsCounter.Add(ctx, 1)
		return err
	}

	recordsSyncedCounter.Add(ctx, int64(len(records)))
	return nil
}

type RAGSyncService interface {
	// FetchPendingSyncs retrieves records from the local DB that need syncing
	FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error)

	// MarkSynced updates the local DB after a successful sync to the cloud
	MarkSynced(ctx context.Context, ids []string) error

	// ProcessIncomingSync handles data pushed from a standalone client into the cloud DB
	ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error
}
