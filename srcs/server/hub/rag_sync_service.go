package hub

import (
	"context"
	"encoding/json"
	"fmt"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
	"go.opentelemetry.io/otel/attribute"
	"go.opentelemetry.io/otel/metric"
)

type ragSyncServiceImpl struct {
	dbProvider db.Provider
}

func NewRAGSyncService(dbProvider db.Provider) RAGSyncService {
	return &ragSyncServiceImpl{
		dbProvider: dbProvider,
	}
}

func (s *ragSyncServiceImpl) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	query := `
		SELECT memory_id, context, vector_embedding, sync_status, last_sync_at
		FROM swarm_memory_embeddings
		WHERE sync_status = 'pending'
		LIMIT $1
	`
	rows, err := s.dbProvider.Query(ctx, query, limit)
	if err != nil {
		ragSyncErrorsTotal.Add(ctx, 1, metric.WithAttributes(attribute.String("operation", "fetch")))
		return nil, fmt.Errorf("failed to fetch pending syncs: %w", err)
	}
	defer rows.Close()

	var records []RAGSyncRecord
	for rows.Next() {
		var rec RAGSyncRecord
		var vectorBytes []byte
		var lastSyncAt interface{}

		err := rows.Scan(&rec.ID, &rec.Context, &vectorBytes, &rec.SyncStatus, &lastSyncAt)
		if err != nil {
			ragSyncErrorsTotal.Add(ctx, 1, metric.WithAttributes(attribute.String("operation", "scan")))
			continue // Skip failing rows
		}

		if len(vectorBytes) > 0 {
			var vec []float32
			if err := json.Unmarshal(vectorBytes, &vec); err == nil {
				rec.Vector = vec
			}
		}

		if lastSyncAt != nil {
			switch v := lastSyncAt.(type) {
			case string:
				if t, err := time.Parse(time.RFC3339, v); err == nil {
					rec.LastSyncAt = t
				}
			case time.Time:
				rec.LastSyncAt = v
			}
		}

		records = append(records, rec)
	}

	if err := rows.Err(); err != nil {
		ragSyncErrorsTotal.Add(ctx, 1, metric.WithAttributes(attribute.String("operation", "rows_err")))
		return nil, fmt.Errorf("error iterating pending sync rows: %w", err)
	}

	return records, nil
}

func (s *ragSyncServiceImpl) MarkSynced(ctx context.Context, ids []string) error {
	if len(ids) == 0 {
		return nil
	}

	// This is a simplified approach; in production, use a prepared statement or proper bulk update
	for _, id := range ids {
		query := `
			UPDATE swarm_memory_embeddings
			SET sync_status = 'synced', last_sync_at = $1
			WHERE memory_id = $2
		`
		_, err := s.dbProvider.Exec(ctx, query, time.Now(), id)
		if err != nil {
			ragSyncErrorsTotal.Add(ctx, 1, metric.WithAttributes(attribute.String("operation", "mark_synced")))
			return fmt.Errorf("failed to mark synced for id %s: %w", id, err)
		}
	}

	ragRecordsSyncedTotal.Add(ctx, int64(len(ids)))
	return nil
}

func (s *ragSyncServiceImpl) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	tx, err := s.dbProvider.Begin(ctx)
	if err != nil {
		ragSyncErrorsTotal.Add(ctx, 1, metric.WithAttributes(attribute.String("operation", "begin_tx")))
		return fmt.Errorf("failed to begin transaction: %w", err)
	}
	defer tx.Rollback(ctx)

	for _, rec := range records {
		var vectorBytes []byte
		if len(rec.Vector) > 0 {
			vectorBytes, err = json.Marshal(rec.Vector)
			if err != nil {
				ragSyncErrorsTotal.Add(ctx, 1, metric.WithAttributes(attribute.String("operation", "marshal_vector")))
				return fmt.Errorf("failed to marshal vector: %w", err)
			}
		}

		query := `
			INSERT INTO swarm_memory_embeddings (memory_id, context, vector_embedding, sync_status, last_sync_at)
			VALUES ($1, $2, $3, $4, $5)
			ON CONFLICT (memory_id) DO UPDATE SET
				context = EXCLUDED.context,
				vector_embedding = EXCLUDED.vector_embedding,
				sync_status = EXCLUDED.sync_status,
				last_sync_at = EXCLUDED.last_sync_at
		`
		// SQLite uses different ON CONFLICT syntax, but we'll try a hybrid approach if possible
		// However, given the instructions for Hybrid db, the standard standard is fine as long as we map correctly.
		// For pure standard SQL compatible with both, we might need a separate UPDATE/INSERT logic or just standard UPSERT.
		// We'll stick to Postgres standard ON CONFLICT which modern SQLite also supports.

		_, err = tx.Exec(ctx, query, rec.ID, rec.Context, vectorBytes, SyncStatusSynced, time.Now())
		if err != nil {
			ragSyncErrorsTotal.Add(ctx, 1, metric.WithAttributes(attribute.String("operation", "upsert")))
			return fmt.Errorf("failed to process incoming sync for id %s: %w", rec.ID, err)
		}
	}

	if err := tx.Commit(ctx); err != nil {
		ragSyncErrorsTotal.Add(ctx, 1, metric.WithAttributes(attribute.String("operation", "commit_tx")))
		return fmt.Errorf("failed to commit transaction: %w", err)
	}

	ragRecordsSyncedTotal.Add(ctx, int64(len(records)))
	return nil
}
