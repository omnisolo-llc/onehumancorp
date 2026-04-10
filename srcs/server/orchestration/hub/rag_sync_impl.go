package hub

import (
	"context"
	"fmt"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
	"go.opentelemetry.io/otel"
	"go.opentelemetry.io/otel/metric"
)

var (
	meter                = otel.Meter("github.com/onehumancorp/mono/srcs/server/orchestration/hub")
	ragRecordsSynced     metric.Int64Counter
	ragSyncErrors        metric.Int64Counter
)

func init() {
	var err error
	ragRecordsSynced, err = meter.Int64Counter("rag_records_synced_total", metric.WithDescription("Total number of RAG records synced"))
	if err != nil {
		panic(err)
	}
	ragSyncErrors, err = meter.Int64Counter("rag_sync_errors_total", metric.WithDescription("Total number of RAG sync errors"))
	if err != nil {
		panic(err)
	}
}

type DefaultRAGSyncService struct {
	dbProvider db.Provider
}

func NewRAGSyncService(dbProvider db.Provider) *DefaultRAGSyncService {
	return &DefaultRAGSyncService{
		dbProvider: dbProvider,
	}
}

func (s *DefaultRAGSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	query := `SELECT memory_id, context, vector_embedding, sync_status, last_sync_at FROM swarm_memory_embeddings WHERE sync_status = 'pending' LIMIT $1`
	if s.dbProvider.IsSQLite() {
		query = `SELECT memory_id, context, vector_embedding, sync_status, last_sync_at FROM swarm_memory_embeddings WHERE sync_status = 'pending' LIMIT ?`
	}

	rows, err := s.dbProvider.Query(ctx, query, limit)
	if err != nil {
		if ragSyncErrors != nil {
			ragSyncErrors.Add(ctx, 1)
		}
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
			if ragSyncErrors != nil {
				ragSyncErrors.Add(ctx, 1)
			}
			return nil, fmt.Errorf("failed to scan row: %w", err)
		}
		if lastSyncAt != nil {
			r.LastSyncAt = *lastSyncAt
		}
		r.Vector = vector
		records = append(records, r)
	}

	return records, nil
}

func (s *DefaultRAGSyncService) MarkSynced(ctx context.Context, ids []string) error {
	if len(ids) == 0 {
		return nil
	}

	tx, err := s.dbProvider.Begin(ctx)
	if err != nil {
		if ragSyncErrors != nil {
			ragSyncErrors.Add(ctx, 1)
		}
		return fmt.Errorf("failed to begin tx: %w", err)
	}

	query := `UPDATE swarm_memory_embeddings SET sync_status = 'synced', last_sync_at = $1 WHERE memory_id = $2`
	if s.dbProvider.IsSQLite() {
		query = `UPDATE swarm_memory_embeddings SET sync_status = 'synced', last_sync_at = ? WHERE memory_id = ?`
	}

	now := time.Now()
	for _, id := range ids {
		_, err := tx.Exec(ctx, query, now, id)
		if err != nil {
			tx.Rollback(ctx)
			if ragSyncErrors != nil {
				ragSyncErrors.Add(ctx, 1)
			}
			return fmt.Errorf("failed to mark synced for id %s: %w", id, err)
		}
	}

	if err := tx.Commit(ctx); err != nil {
		if ragSyncErrors != nil {
			ragSyncErrors.Add(ctx, 1)
		}
		return fmt.Errorf("failed to commit tx: %w", err)
	}

    if ragRecordsSynced != nil {
        ragRecordsSynced.Add(ctx, int64(len(ids)))
    }

	return nil
}

func (s *DefaultRAGSyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	if len(records) == 0 {
		return nil
	}

	tx, err := s.dbProvider.Begin(ctx)
	if err != nil {
		if ragSyncErrors != nil {
			ragSyncErrors.Add(ctx, 1)
		}
		return fmt.Errorf("failed to begin tx: %w", err)
	}

	query := `INSERT INTO swarm_memory_embeddings (memory_id, context, vector_embedding, sync_status, last_sync_at)
			  VALUES ($1, $2, $3, 'synced', $4)
			  ON CONFLICT (memory_id) DO UPDATE SET context = EXCLUDED.context, vector_embedding = EXCLUDED.vector_embedding, sync_status = 'synced', last_sync_at = EXCLUDED.last_sync_at`
	if s.dbProvider.IsSQLite() {
		query = `INSERT INTO swarm_memory_embeddings (memory_id, context, vector_embedding, sync_status, last_sync_at)
				 VALUES (?, ?, ?, 'synced', ?)
				 ON CONFLICT (memory_id) DO UPDATE SET context = EXCLUDED.context, vector_embedding = EXCLUDED.vector_embedding, sync_status = 'synced', last_sync_at = EXCLUDED.last_sync_at`
	}

	for _, r := range records {
		_, err := tx.Exec(ctx, query, r.ID, r.Context, r.Vector, r.LastSyncAt)
		if err != nil {
			tx.Rollback(ctx)
			if ragSyncErrors != nil {
				ragSyncErrors.Add(ctx, 1)
			}
			return fmt.Errorf("failed to process incoming sync for id %s: %w", r.ID, err)
		}
	}

	if err := tx.Commit(ctx); err != nil {
		if ragSyncErrors != nil {
			ragSyncErrors.Add(ctx, 1)
		}
		return fmt.Errorf("failed to commit tx: %w", err)
	}

    if ragRecordsSynced != nil {
        ragRecordsSynced.Add(ctx, int64(len(records)))
    }

	return nil
}
