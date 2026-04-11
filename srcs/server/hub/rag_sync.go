package hub

import (
	"context"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
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
	FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error)
	MarkSynced(ctx context.Context, ids []string) error
	ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error
}

type ragSyncService struct {
	db                  db.Provider
	recordsSyncedCounter metric.Int64Counter
	syncErrorsCounter    metric.Int64Counter
}

func NewRAGSyncService(provider db.Provider, meter metric.Meter) (RAGSyncService, error) {
	syncedCounter, err := meter.Int64Counter("rag_records_synced_total")
	if err != nil {
		return nil, err
	}
	errorsCounter, err := meter.Int64Counter("rag_sync_errors_total")
	if err != nil {
		return nil, err
	}

	return &ragSyncService{
		db:                  provider,
		recordsSyncedCounter: syncedCounter,
		syncErrorsCounter:    errorsCounter,
	}, nil
}

func (s *ragSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	query := `SELECT memory_id, context, vector_embedding, sync_status, last_sync_at
              FROM swarm_memory_embeddings
              WHERE sync_status = $1
              LIMIT $2`
	rows, err := s.db.Query(ctx, query, string(SyncStatusPending), limit)
	if err != nil {
		s.syncErrorsCounter.Add(ctx, 1)
		return nil, err
	}
	defer rows.Close()

	var records []RAGSyncRecord
	for rows.Next() {
		var rec RAGSyncRecord
		var statusStr string
		var lastSyncAt *time.Time
		if err := rows.Scan(&rec.ID, &rec.Context, &rec.Vector, &statusStr, &lastSyncAt); err != nil {
			s.syncErrorsCounter.Add(ctx, 1)
			return nil, err
		}
		rec.SyncStatus = SyncStatus(statusStr)
		if lastSyncAt != nil {
			rec.LastSyncAt = *lastSyncAt
		}
		records = append(records, rec)
	}
	if err := rows.Err(); err != nil {
		s.syncErrorsCounter.Add(ctx, 1)
		return nil, err
	}
	return records, nil
}

func (s *ragSyncService) MarkSynced(ctx context.Context, ids []string) error {
	if len(ids) == 0 {
		return nil
	}

	tx, err := s.db.Begin(ctx)
	if err != nil {
		s.syncErrorsCounter.Add(ctx, 1)
		return err
	}
	defer tx.Rollback(ctx)

	for _, id := range ids {
		query := `UPDATE swarm_memory_embeddings
                  SET sync_status = $1, last_sync_at = $2
                  WHERE memory_id = $3`
		_, err := tx.Exec(ctx, query, string(SyncStatusSynced), time.Now(), id)
		if err != nil {
			s.syncErrorsCounter.Add(ctx, 1)
			return err
		}
	}

	if err := tx.Commit(ctx); err != nil {
		s.syncErrorsCounter.Add(ctx, 1)
		return err
	}

	s.recordsSyncedCounter.Add(ctx, int64(len(ids)))
	return nil
}

func (s *ragSyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	if len(records) == 0 {
		return nil
	}

	tx, err := s.db.Begin(ctx)
	if err != nil {
		s.syncErrorsCounter.Add(ctx, 1)
		return err
	}
	defer tx.Rollback(ctx)

	isSQLite := s.db.IsSQLite()
	now := time.Now()

	for _, rec := range records {
		if isSQLite {
			// Check if exists
			var count int
			err := tx.QueryRow(ctx, "SELECT COUNT(*) FROM swarm_memory_embeddings WHERE memory_id = $1", rec.ID).Scan(&count)
			if err != nil {
				s.syncErrorsCounter.Add(ctx, 1)
				return err
			}

			if count > 0 {
				query := `UPDATE swarm_memory_embeddings
                          SET context = $1, vector_embedding = $2, sync_status = $3, last_sync_at = $4
                          WHERE memory_id = $5`
				_, err = tx.Exec(ctx, query, rec.Context, rec.Vector, string(SyncStatusSynced), now, rec.ID)
			} else {
				query := `INSERT INTO swarm_memory_embeddings (memory_id, context, vector_embedding, sync_status, last_sync_at)
                          VALUES ($1, $2, $3, $4, $5)`
				_, err = tx.Exec(ctx, query, rec.ID, rec.Context, rec.Vector, string(SyncStatusSynced), now)
			}
			if err != nil {
				s.syncErrorsCounter.Add(ctx, 1)
				return err
			}
		} else {
			// Postgres UPSERT
			query := `INSERT INTO swarm_memory_embeddings (memory_id, context, vector_embedding, sync_status, last_sync_at)
                      VALUES ($1, $2, $3, $4, $5)
                      ON CONFLICT (memory_id)
                      DO UPDATE SET context = EXCLUDED.context, vector_embedding = EXCLUDED.vector_embedding, sync_status = EXCLUDED.sync_status, last_sync_at = EXCLUDED.last_sync_at`
			_, err = tx.Exec(ctx, query, rec.ID, rec.Context, rec.Vector, string(SyncStatusSynced), now)
			if err != nil {
				s.syncErrorsCounter.Add(ctx, 1)
				return err
			}
		}
	}

	if err := tx.Commit(ctx); err != nil {
		s.syncErrorsCounter.Add(ctx, 1)
		return err
	}

	s.recordsSyncedCounter.Add(ctx, int64(len(records)))
	return nil
}
