package hub

import (
	"context"
	"fmt"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
)

type ragSyncServiceImpl struct {
	db db.Provider
}

// NewRAGSyncService creates a new RAGSyncService instance.
func NewRAGSyncService(db db.Provider) RAGSyncService {
	return &ragSyncServiceImpl{db: db}
}

func (s *ragSyncServiceImpl) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	if !s.db.IsSQLite() {
		return nil, fmt.Errorf("FetchPendingSyncs is only supported in Standalone Mode (SQLite)")
	}

	query := `
		SELECT memory_id, context, vector_embedding, sync_status, last_sync_at
		FROM swarm_memory_embeddings
		WHERE sync_status = 'pending' OR sync_status IS NULL
		LIMIT $1
	`
	rows, err := s.db.Query(ctx, query, limit)
	if err != nil {
		return nil, fmt.Errorf("failed to fetch pending syncs: %w", err)
	}
	defer rows.Close()

	var records []RAGSyncRecord
	for rows.Next() {
		var r RAGSyncRecord
		var lastSyncAt *time.Time
		if err := rows.Scan(&r.ID, &r.Context, &r.Vector, &r.SyncStatus, &lastSyncAt); err != nil {
			return nil, fmt.Errorf("failed to scan pending sync record: %w", err)
		}
		if lastSyncAt != nil {
			r.LastSyncAt = *lastSyncAt
		}
		records = append(records, r)
	}
	if err := rows.Err(); err != nil {
		return nil, fmt.Errorf("error iterating pending syncs rows: %w", err)
	}

	return records, nil
}

func (s *ragSyncServiceImpl) MarkSynced(ctx context.Context, ids []string) error {
	if !s.db.IsSQLite() {
		return fmt.Errorf("MarkSynced is only supported in Standalone Mode (SQLite)")
	}

	if len(ids) == 0 {
		return nil
	}

	// Because different drivers handle IN clauses differently, we do this one by one in a transaction for simplicity and safety across both.
	tx, err := s.db.Begin(ctx)
	if err != nil {
		return fmt.Errorf("failed to begin transaction: %w", err)
	}
	defer tx.Rollback(ctx)

	now := time.Now()
	for _, id := range ids {
		_, err := tx.Exec(ctx, `
			UPDATE swarm_memory_embeddings
			SET sync_status = 'synced', last_sync_at = $1
			WHERE memory_id = $2
		`, now, id)
		if err != nil {
			return fmt.Errorf("failed to update sync status for id %s: %w", id, err)
		}
	}

	if err := tx.Commit(ctx); err != nil {
		return fmt.Errorf("failed to commit sync status updates: %w", err)
	}

	RecordSyncSuccess(ctx, len(ids))
	return nil
}

func (s *ragSyncServiceImpl) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	if s.db.IsSQLite() {
		return fmt.Errorf("ProcessIncomingSync is only supported in Cloud Mode (PostgreSQL)")
	}

	if len(records) == 0 {
		return nil
	}

	tx, err := s.db.Begin(ctx)
	if err != nil {
		return fmt.Errorf("failed to begin transaction: %w", err)
	}
	defer tx.Rollback(ctx)

	for _, r := range records {
		// Basic upsert mechanism. In Postgres: ON CONFLICT (memory_id) DO UPDATE
		query := `
			INSERT INTO swarm_memory_embeddings (memory_id, context, vector_embedding, sync_status, last_sync_at)
			VALUES ($1, $2, $3, 'synced', $4)
			ON CONFLICT (memory_id) DO UPDATE SET
				context = EXCLUDED.context,
				vector_embedding = EXCLUDED.vector_embedding,
				sync_status = EXCLUDED.sync_status,
				last_sync_at = EXCLUDED.last_sync_at
		`
		_, err := tx.Exec(ctx, query, r.ID, r.Context, r.Vector, time.Now())
		if err != nil {
			RecordSyncError(ctx)
			return fmt.Errorf("failed to upsert incoming sync record %s: %w", r.ID, err)
		}
	}

	if err := tx.Commit(ctx); err != nil {
		return fmt.Errorf("failed to commit incoming syncs: %w", err)
	}

	RecordSyncSuccess(ctx, len(records))
	return nil
}
