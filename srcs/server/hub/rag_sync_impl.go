package hub

import (
	"context"
	"database/sql"
	"encoding/json"
	"fmt"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
	"go.opentelemetry.io/otel"
	"go.opentelemetry.io/otel/metric"
)

var (
	meter            = otel.Meter("github.com/onehumancorp/mono/srcs/server/hub")
	recordsSynced, _ = meter.Int64Counter(
		"rag_records_synced_total",
		metric.WithDescription("Total number of RAG records successfully synced"),
	)
	syncErrors, _ = meter.Int64Counter(
		"rag_sync_errors_total",
		metric.WithDescription("Total number of RAG sync errors"),
	)
)

type ragSyncService struct {
	db *db.DB
}

func NewRAGSyncService(database *db.DB) RAGSyncService {
	return &ragSyncService{
		db: database,
	}
}

func (s *ragSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	query := `
		SELECT id, content, embedding, sync_status, last_sync_at
		FROM autodream_memories
		WHERE sync_status = 'pending'
		ORDER BY created_at ASC
		LIMIT $1
	`
	// sqlite needs limit ? sometimes, let's use standard $1 which the db wrapper supports

	rows, err := s.db.Query(ctx, query, limit)
	if err != nil {
		syncErrors.Add(ctx, 1)
		return nil, fmt.Errorf("FetchPendingSyncs query failed: %w", err)
	}
	defer rows.Close()

	var records []RAGSyncRecord
	for rows.Next() {
		var rec RAGSyncRecord
		var lastSyncAt sql.NullTime
		var vectorStr sql.NullString

		err := rows.Scan(&rec.ID, &rec.Context, &vectorStr, &rec.SyncStatus, &lastSyncAt)
		if err != nil {
			syncErrors.Add(ctx, 1)
			return nil, fmt.Errorf("FetchPendingSyncs scan failed: %w", err)
		}
		if lastSyncAt.Valid {
			rec.LastSyncAt = lastSyncAt.Time
		}

		if vectorStr.Valid {
			// Vector could be a JSON string like '[0.1, 0.2, ...]'
			// Decode it
			var vec []float32
			if err := json.Unmarshal([]byte(vectorStr.String), &vec); err != nil {
				// We don't fail the whole sync for one bad vector parse, just log/skip or assign nil
				rec.Vector = nil
			} else {
				rec.Vector = vec
			}
		}

		records = append(records, rec)
	}

	if err := rows.Err(); err != nil {
		syncErrors.Add(ctx, 1)
		return nil, fmt.Errorf("FetchPendingSyncs rows error: %w", err)
	}

	return records, nil
}

func (s *ragSyncService) MarkSynced(ctx context.Context, ids []string) error {
	if len(ids) == 0 {
		return nil
	}

	tx, err := s.db.Begin(ctx)
	if err != nil {
		syncErrors.Add(ctx, 1)
		return fmt.Errorf("MarkSynced begin tx failed: %w", err)
	}
	defer tx.Rollback(ctx)

	query := `
		UPDATE autodream_memories
		SET sync_status = 'synced', last_sync_at = $1
		WHERE id = $2
	`
	now := time.Now().UTC()

	for _, id := range ids {
		_, err := tx.Exec(ctx, query, now, id)
		if err != nil {
			syncErrors.Add(ctx, 1)
			return fmt.Errorf("MarkSynced update failed for id %s: %w", id, err)
		}
	}

	if err := tx.Commit(ctx); err != nil {
		syncErrors.Add(ctx, 1)
		return fmt.Errorf("MarkSynced commit tx failed: %w", err)
	}

	recordsSynced.Add(ctx, int64(len(ids)))
	return nil
}

func (s *ragSyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	if len(records) == 0 {
		return nil
	}

	tx, err := s.db.Begin(ctx)
	if err != nil {
		syncErrors.Add(ctx, 1)
		return fmt.Errorf("ProcessIncomingSync begin tx failed: %w", err)
	}
	defer tx.Rollback(ctx)

	// ON CONFLICT DO UPDATE SET syntax works for both sqlite and postgres
	query := `
		INSERT INTO autodream_memories (id, content, embedding, sync_status, last_sync_at)
		VALUES ($1, $2, $3, $4, $5)
		ON CONFLICT (id) DO UPDATE SET
			content = EXCLUDED.content,
			embedding = EXCLUDED.embedding,
			sync_status = EXCLUDED.sync_status,
			last_sync_at = EXCLUDED.last_sync_at
	`

	for _, rec := range records {
		var vectorStr sql.NullString
		if rec.Vector != nil {
			bytes, err := json.Marshal(rec.Vector)
			if err == nil {
				vectorStr = sql.NullString{String: string(bytes), Valid: true}
			}
		}

		status := rec.SyncStatus
		if status == "" {
			status = SyncStatusSynced
		}

		var lastSync sql.NullTime
		if !rec.LastSyncAt.IsZero() {
			lastSync = sql.NullTime{Time: rec.LastSyncAt, Valid: true}
		} else {
			lastSync = sql.NullTime{Time: time.Now().UTC(), Valid: true}
		}

		// sqlite ON CONFLICT might need special handling but database/sql driver usually handles standard upserts
		_, err := tx.Exec(ctx, query, rec.ID, rec.Context, vectorStr, status, lastSync)
		if err != nil {
			syncErrors.Add(ctx, 1)
			return fmt.Errorf("ProcessIncomingSync upsert failed for id %s: %w", rec.ID, err)
		}
	}

	if err := tx.Commit(ctx); err != nil {
		syncErrors.Add(ctx, 1)
		return fmt.Errorf("ProcessIncomingSync commit tx failed: %w", err)
	}

	return nil
}
