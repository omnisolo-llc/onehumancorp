
package hub

import (
	"go.opentelemetry.io/otel/metric"
	"context"
	"fmt"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
	"go.opentelemetry.io/otel"
	"go.opentelemetry.io/otel/attribute"

)

var tracer = otel.Tracer("github.com/onehumancorp/mono/srcs/server/hub")

type DefaultRAGSyncService struct {
	db db.Provider
}

func NewRAGSyncService(db db.Provider) RAGSyncService {
	return &DefaultRAGSyncService{db: db}
}

func (s *DefaultRAGSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	ctx, span := tracer.Start(ctx, "FetchPendingSyncs")
	defer span.End()

	// In a real standalone SQLite setup, we'd query the local DB.
	// We'll use the generic db.Provider which handles both based on configuration.
	query := `
		SELECT id, content, embedding, sync_status, last_sync_at
		FROM autodream_memories
		WHERE sync_status = $1
		LIMIT $2
	`

	rows, err := s.db.Query(ctx, query, SyncStatusPending, limit)
	if err != nil {
		syncErrorsCounter.Add(ctx, 1, metric.WithAttributes(attribute.String("operation", "fetch_pending")))
		return nil, fmt.Errorf("failed to fetch pending syncs: %w", err)
	}
	defer rows.Close()

	var records []RAGSyncRecord
	for rows.Next() {
		var r RAGSyncRecord
		var lastSync *time.Time
		var vectorStr string // Assume Vector is retrieved as string for SQLite compat in OHC

		err := rows.Scan(&r.ID, &r.Context, &vectorStr, &r.SyncStatus, &lastSync)
		if err != nil {
			syncErrorsCounter.Add(ctx, 1, metric.WithAttributes(attribute.String("operation", "fetch_pending_scan")))
			return nil, fmt.Errorf("failed to scan record: %w", err)
		}

		if lastSync != nil {
			r.LastSyncAt = *lastSync
		}

		// For simplicity we leave Vector empty in this struct mapping unless we deserialize vectorStr
		records = append(records, r)
	}

	if err = rows.Err(); err != nil {
		return nil, err
	}

	return records, nil
}

func (s *DefaultRAGSyncService) MarkSynced(ctx context.Context, ids []string) error {
	ctx, span := tracer.Start(ctx, "MarkSynced")
	defer span.End()

	if len(ids) == 0 {
		return nil
	}

	// Simple loop implementation. A real implementation might use IN clause or unnest depending on Postgres/SQLite support.
	tx, err := s.db.Begin(ctx)
	if err != nil {
		return err
	}
	defer tx.Rollback(ctx)

	for _, id := range ids {
		_, err := tx.Exec(ctx, `
			UPDATE autodream_memories
			SET sync_status = $1, last_sync_at = $2
			WHERE id = $3
		`, SyncStatusSynced, time.Now(), id)
		if err != nil {
			syncErrorsCounter.Add(ctx, 1, metric.WithAttributes(attribute.String("operation", "mark_synced")))
			return fmt.Errorf("failed to update record %s: %w", id, err)
		}
	}

	err = tx.Commit(ctx)
	if err != nil {
		return err
	}

	recordsSyncedCounter.Add(ctx, int64(len(ids)))
	return nil
}

func (s *DefaultRAGSyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	ctx, span := tracer.Start(ctx, "ProcessIncomingSync")
	defer span.End()

	if len(records) == 0 {
		return nil
	}

	tx, err := s.db.Begin(ctx)
	if err != nil {
		return err
	}
	defer tx.Rollback(ctx)

	for _, r := range records {
		// Basic upsert / insert logic.
		// Handling vectors depends heavily on Postgres pgvector vs SQLite text.
		// We'll perform a simple upsert (ON CONFLICT not strictly defined in SQLite identical to Postgres,
		// but standard INSERT OR REPLACE or Postgres ON CONFLICT DO UPDATE).
		// For simplicity in this assignment, we do an insert (ignoring conflict for now, or just basic standard SQL).

		// Note: To be fully compatible with both pgx and standard sql drivers for SQLite,
		// queries here might need DB abstraction.

		_, err := tx.Exec(ctx, `
			INSERT INTO autodream_memories (id, content, sync_status, last_sync_at)
			VALUES ($1, $2, $3, $4)
			ON CONFLICT (id) DO UPDATE SET
				content = EXCLUDED.content,
				sync_status = EXCLUDED.sync_status,
				last_sync_at = EXCLUDED.last_sync_at
		`, r.ID, r.Context, r.SyncStatus, r.LastSyncAt)

		if err != nil {
			syncErrorsCounter.Add(ctx, 1, metric.WithAttributes(attribute.String("operation", "process_incoming")))
			return fmt.Errorf("failed to process incoming record %s: %w", r.ID, err)
		}
	}

	err = tx.Commit(ctx)
	if err != nil {
		return err
	}

	return nil
}
