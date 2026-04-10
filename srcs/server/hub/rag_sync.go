package hub

import (
	"context"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
	"github.com/onehumancorp/mono/srcs/server/telemetry"
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
	Vector     []byte // Mapped to BYTEA / byte array per OHC conventions for vector_embedding
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

type ragSyncServiceImpl struct {
	dbWrapper *db.DB
}

func NewRAGSyncService(dbWrapper *db.DB) RAGSyncService {
	return &ragSyncServiceImpl{
		dbWrapper: dbWrapper,
	}
}

func (s *ragSyncServiceImpl) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	tx, err := s.dbWrapper.Begin(ctx)
	if err != nil {
		return nil, err
	}
	defer tx.Rollback(ctx)

	query := `
		SELECT memory_id, context, vector_embedding, sync_status, last_sync_at
		FROM swarm_memory_embeddings
		WHERE sync_status = 'pending'
		LIMIT $1
	`
	if s.dbWrapper.IsSQLite() {
		query = `
			SELECT memory_id, context, vector_embedding, sync_status, last_sync_at
			FROM swarm_memory_embeddings
			WHERE sync_status = 'pending'
			LIMIT ?
		`
	}

	rows, err := tx.Query(ctx, query, limit)
	if err != nil {
		return nil, err
	}
	defer rows.Close()

	var records []RAGSyncRecord
	for rows.Next() {
		var record RAGSyncRecord
		var lastSyncAt *time.Time
		var vector []byte
		var syncStatus string
		if err := rows.Scan(&record.ID, &record.Context, &vector, &syncStatus, &lastSyncAt); err != nil {
			return nil, err
		}
		record.Vector = vector
		record.SyncStatus = SyncStatus(syncStatus)
		if lastSyncAt != nil {
			record.LastSyncAt = *lastSyncAt
		}
		records = append(records, record)
	}

	return records, nil
}

func (s *ragSyncServiceImpl) MarkSynced(ctx context.Context, ids []string) error {
	if len(ids) == 0 {
		return nil
	}

	tx, err := s.dbWrapper.Begin(ctx)
	if err != nil {
		if telemetry.RAGSyncErrorsTotal != nil {
			telemetry.RAGSyncErrorsTotal.Add(ctx, 1)
		}
		return err
	}
	defer tx.Rollback(ctx)

	query := `
		UPDATE swarm_memory_embeddings
		SET sync_status = 'synced', last_sync_at = CURRENT_TIMESTAMP
		WHERE memory_id = $1
	`
	if s.dbWrapper.IsSQLite() {
		query = `
			UPDATE swarm_memory_embeddings
			SET sync_status = 'synced', last_sync_at = CURRENT_TIMESTAMP
			WHERE memory_id = ?
		`
	}

	for _, id := range ids {
		_, err := tx.Exec(ctx, query, id)
		if err != nil {
			if telemetry.RAGSyncErrorsTotal != nil {
				telemetry.RAGSyncErrorsTotal.Add(ctx, 1)
			}
			return err
		}
	}

	if err := tx.Commit(ctx); err != nil {
		if telemetry.RAGSyncErrorsTotal != nil {
			telemetry.RAGSyncErrorsTotal.Add(ctx, 1)
		}
		return err
	}

	if telemetry.RAGRecordsSyncedTotal != nil {
		telemetry.RAGRecordsSyncedTotal.Add(ctx, int64(len(ids)), metric.WithAttributes())
	}

	return nil
}

func (s *ragSyncServiceImpl) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	tx, err := s.dbWrapper.Begin(ctx)
	if err != nil {
		if telemetry.RAGSyncErrorsTotal != nil {
			telemetry.RAGSyncErrorsTotal.Add(ctx, 1)
		}
		return err
	}
	defer tx.Rollback(ctx)

	query := `
		INSERT INTO swarm_memory_embeddings (memory_id, context, vector_embedding, sync_status, last_sync_at)
		VALUES ($1, $2, $3, 'synced', CURRENT_TIMESTAMP)
		ON CONFLICT (memory_id) DO UPDATE SET
		context = EXCLUDED.context,
		vector_embedding = EXCLUDED.vector_embedding,
		sync_status = 'synced',
		last_sync_at = CURRENT_TIMESTAMP
	`
	if s.dbWrapper.IsSQLite() {
		query = `
			INSERT INTO swarm_memory_embeddings (memory_id, context, vector_embedding, sync_status, last_sync_at)
			VALUES (?, ?, ?, 'synced', CURRENT_TIMESTAMP)
			ON CONFLICT (memory_id) DO UPDATE SET
			context = excluded.context,
			vector_embedding = excluded.vector_embedding,
			sync_status = 'synced',
			last_sync_at = CURRENT_TIMESTAMP
		`
	}

	for _, record := range records {
		_, err := tx.Exec(ctx, query, record.ID, record.Context, record.Vector)
		if err != nil {
			if telemetry.RAGSyncErrorsTotal != nil {
				telemetry.RAGSyncErrorsTotal.Add(ctx, 1)
			}
			return err
		}
	}

	if err := tx.Commit(ctx); err != nil {
		if telemetry.RAGSyncErrorsTotal != nil {
			telemetry.RAGSyncErrorsTotal.Add(ctx, 1)
		}
		return err
	}

	if telemetry.RAGRecordsSyncedTotal != nil {
		telemetry.RAGRecordsSyncedTotal.Add(ctx, int64(len(records)), metric.WithAttributes())
	}

	return nil
}
