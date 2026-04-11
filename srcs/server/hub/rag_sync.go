package hub

import (
	"context"
	"database/sql"
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
	Vector     []byte // byte slice for postgres bytea / sqlite blob compatibility
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
	meter            = otel.Meter("github.com/onehumancorp/mono/srcs/server/hub")
	RecordsSynced, _ = meter.Int64Counter("rag_records_synced_total", metric.WithDescription("Total number of RAG records synced"))
	SyncErrors, _    = meter.Int64Counter("rag_sync_errors_total", metric.WithDescription("Total number of RAG sync errors"))
)

type HubRAGSyncService struct {
	dbProvider db.Provider
}

func NewHubRAGSyncService(provider db.Provider) *HubRAGSyncService {
	return &HubRAGSyncService{dbProvider: provider}
}

func (s *HubRAGSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	rows, err := s.dbProvider.Query(ctx, `
		SELECT memory_id, context, vector_embedding, sync_status, last_sync_at
		FROM swarm_memory_embeddings
		WHERE sync_status = 'pending'
		LIMIT $1
	`, limit)
	if err != nil {
		SyncErrors.Add(ctx, 1)
		return nil, err
	}
	defer rows.Close()

	var records []RAGSyncRecord
	for rows.Next() {
		var rec RAGSyncRecord
		var lastSyncAt sql.NullTime
		var syncStatus string
		var vector []byte
		if err := rows.Scan(&rec.ID, &rec.Context, &vector, &syncStatus, &lastSyncAt); err != nil {
			SyncErrors.Add(ctx, 1)
			return nil, err
		}
		rec.SyncStatus = SyncStatus(syncStatus)
		rec.Vector = vector
		if lastSyncAt.Valid {
			rec.LastSyncAt = lastSyncAt.Time
		}
		records = append(records, rec)
	}
	return records, rows.Err()
}

func (s *HubRAGSyncService) MarkSynced(ctx context.Context, ids []string) error {
	if len(ids) == 0 {
		return nil
	}

	// Create transaction
	tx, err := s.dbProvider.Begin(ctx)
	if err != nil {
		SyncErrors.Add(ctx, 1)
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
			SyncErrors.Add(ctx, 1)
			return err
		}
	}

	err = tx.Commit(ctx)
	if err != nil {
		SyncErrors.Add(ctx, 1)
		return err
	}

	RecordsSynced.Add(ctx, int64(len(ids)))
	return nil
}

func (s *HubRAGSyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	if len(records) == 0 {
		return nil
	}

	tx, err := s.dbProvider.Begin(ctx)
	if err != nil {
		SyncErrors.Add(ctx, 1)
		return err
	}
	defer tx.Rollback(ctx)

	for _, rec := range records {
		if !s.dbProvider.IsSQLite() {
			_, err = tx.Exec(ctx, `
				INSERT INTO swarm_memory_embeddings (memory_id, context, vector_embedding, sync_status, last_sync_at)
				VALUES ($1, $2, $3, 'synced', CURRENT_TIMESTAMP)
				ON CONFLICT (memory_id) DO UPDATE SET
					context = EXCLUDED.context,
					vector_embedding = EXCLUDED.vector_embedding,
					sync_status = 'synced',
					last_sync_at = CURRENT_TIMESTAMP
			`, rec.ID, rec.Context, rec.Vector)
		} else {
			// Attempt UPDATE first for SQLite
			rowsAffected, err := tx.Exec(ctx, `
				UPDATE swarm_memory_embeddings
				SET context = $2, vector_embedding = $3, sync_status = 'synced', last_sync_at = CURRENT_TIMESTAMP
				WHERE memory_id = $1
			`, rec.ID, rec.Context, rec.Vector)
			if err != nil {
				SyncErrors.Add(ctx, 1)
				return err
			}
			// In our db.Provider, Exec returns (int64, error) where the int64 is rows affected
			if rowsAffected == 0 {
				_, err = tx.Exec(ctx, `
					INSERT INTO swarm_memory_embeddings (memory_id, context, vector_embedding, sync_status, last_sync_at)
					VALUES ($1, $2, $3, 'synced', CURRENT_TIMESTAMP)
				`, rec.ID, rec.Context, rec.Vector)
			}
		}

		if err != nil {
			SyncErrors.Add(ctx, 1)
			return err
		}
	}

	err = tx.Commit(ctx)
	if err != nil {
		SyncErrors.Add(ctx, 1)
		return err
	}

	RecordsSynced.Add(ctx, int64(len(records)))
	return nil
}
