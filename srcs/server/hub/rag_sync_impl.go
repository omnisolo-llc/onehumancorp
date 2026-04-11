package hub

import (
	"context"
	"encoding/json"
	"fmt"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
	"github.com/onehumancorp/mono/srcs/server/telemetry"
)

type ragSyncServiceImpl struct {
	db db.Provider
}

// NewRAGSyncService creates a new RAGSyncService backed by the given db.Provider.
func NewRAGSyncService(db db.Provider) RAGSyncService {
	return &ragSyncServiceImpl{
		db: db,
	}
}

func (s *ragSyncServiceImpl) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	query := `
		SELECT memory_id, context, vector_embedding, sync_status, last_sync_at
		FROM swarm_memory_embeddings
		WHERE sync_status = 'pending'
		ORDER BY created_at ASC
		LIMIT $1
	`
	rows, err := s.db.Query(ctx, query, limit)
	if err != nil {
		return nil, fmt.Errorf("failed to fetch pending syncs: %w", err)
	}
	defer rows.Close()

	var records []RAGSyncRecord
	for rows.Next() {
		var rec RAGSyncRecord
		var vectorBytes []byte
		var lastSyncAt *time.Time
		if err := rows.Scan(&rec.ID, &rec.Context, &vectorBytes, &rec.SyncStatus, &lastSyncAt); err != nil {
			return nil, fmt.Errorf("failed to scan record: %w", err)
		}

		if len(vectorBytes) > 0 {
			var floats []float32
			if err := json.Unmarshal(vectorBytes, &floats); err == nil {
				rec.Vector = floats
			} else {
				// In Postgres this might be raw bytea, but let's assume it's JSON for SQLite or simple conversion
				// Just ignore or handle differently based on actual bytea format if needed.
			}
		}

		if lastSyncAt != nil {
			rec.LastSyncAt = *lastSyncAt
		}
		records = append(records, rec)
	}
	if err := rows.Err(); err != nil {
		return nil, fmt.Errorf("rows iteration error: %w", err)
	}

	return records, nil
}

func (s *ragSyncServiceImpl) MarkSynced(ctx context.Context, ids []string) error {
	if len(ids) == 0 {
		return nil
	}

	// This is a naive implementation; ideally use a transaction and IN clause
	tx, err := s.db.Begin(ctx)
	if err != nil {
		return fmt.Errorf("failed to begin transaction: %w", err)
	}
	defer tx.Rollback(ctx)

	now := time.Now()
	for _, id := range ids {
		query := `
			UPDATE swarm_memory_embeddings
			SET sync_status = 'synced', last_sync_at = $1
			WHERE memory_id = $2
		`
		_, err := tx.Exec(ctx, query, now, id)
		if err != nil {
			return fmt.Errorf("failed to mark synced for id %s: %w", id, err)
		}
	}

	if err := tx.Commit(ctx); err != nil {
		return fmt.Errorf("failed to commit transaction: %w", err)
	}

	telemetry.RecordRAGSyncSuccess(ctx, int64(len(ids)))

	return nil
}

func (s *ragSyncServiceImpl) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	if len(records) == 0 {
		return nil
	}

	tx, err := s.db.Begin(ctx)
	if err != nil {
		return fmt.Errorf("failed to begin transaction: %w", err)
	}
	defer tx.Rollback(ctx)

	for _, rec := range records {
		var vectorBytes []byte
		if len(rec.Vector) > 0 {
			vectorBytes, _ = json.Marshal(rec.Vector)
		}

		// Simple upsert logic
		query := `
			INSERT INTO swarm_memory_embeddings (memory_id, context, vector_embedding, sync_status, last_sync_at)
			VALUES ($1, $2, $3, 'synced', $4)
			ON CONFLICT(memory_id) DO UPDATE SET
				context = excluded.context,
				vector_embedding = excluded.vector_embedding,
				sync_status = excluded.sync_status,
				last_sync_at = excluded.last_sync_at
		`
		_, err := tx.Exec(ctx, query, rec.ID, rec.Context, vectorBytes, time.Now())
		if err != nil {
			telemetry.RecordRAGSyncError(ctx, 1)
			return fmt.Errorf("failed to process incoming sync for id %s: %w", rec.ID, err)
		}
	}

	if err := tx.Commit(ctx); err != nil {
		telemetry.RecordRAGSyncError(ctx, int64(len(records)))
		return fmt.Errorf("failed to commit transaction: %w", err)
	}

	telemetry.RecordRAGSyncSuccess(ctx, int64(len(records)))

	return nil
}
