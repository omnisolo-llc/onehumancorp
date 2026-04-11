package hub

import (
	"context"
	"fmt"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
)

type ragSyncServiceImpl struct {
	dbProvider db.Provider
}

func NewRAGSyncService(provider db.Provider) RAGSyncService {
	return &ragSyncServiceImpl{
		dbProvider: provider,
	}
}

func (s *ragSyncServiceImpl) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	query := `
		SELECT id, content, sync_status, last_sync_at
		FROM autodream_memories
		WHERE sync_status = 'pending'
		ORDER BY created_at ASC
		LIMIT $1
	`
	rows, err := s.dbProvider.Query(ctx, query, limit)
	if err != nil {
		ragSyncErrorsTotal.Add(ctx, 1)
		return nil, fmt.Errorf("failed to fetch pending syncs: %w", err)
	}
	defer rows.Close()

	var records []RAGSyncRecord
	for rows.Next() {
		var rec RAGSyncRecord
		var syncStatus string
		var lastSyncAt *time.Time
		if err := rows.Scan(&rec.ID, &rec.Context, &syncStatus, &lastSyncAt); err != nil {
			ragSyncErrorsTotal.Add(ctx, 1)
			return nil, fmt.Errorf("failed to scan pending sync: %w", err)
		}
		rec.SyncStatus = SyncStatus(syncStatus)
		if lastSyncAt != nil {
			rec.LastSyncAt = *lastSyncAt
		}
		records = append(records, rec)
	}

	if err := rows.Err(); err != nil {
		ragSyncErrorsTotal.Add(ctx, 1)
		return nil, fmt.Errorf("error iterating pending syncs: %w", err)
	}

	return records, nil
}

func (s *ragSyncServiceImpl) MarkSynced(ctx context.Context, ids []string) error {
	if len(ids) == 0 {
		return nil
	}

	tx, err := s.dbProvider.Begin(ctx)
	if err != nil {
		ragSyncErrorsTotal.Add(ctx, 1)
		return fmt.Errorf("failed to begin tx for marking synced: %w", err)
	}
	defer tx.Rollback(ctx)

	now := time.Now()
	for _, id := range ids {
		query := `UPDATE autodream_memories SET sync_status = 'synced', last_sync_at = $1 WHERE id = $2`
		_, err := tx.Exec(ctx, query, now, id)
		if err != nil {
			ragSyncErrorsTotal.Add(ctx, 1)
			return fmt.Errorf("failed to update sync status for id %s: %w", id, err)
		}
	}

	if err := tx.Commit(ctx); err != nil {
		ragSyncErrorsTotal.Add(ctx, 1)
		return fmt.Errorf("failed to commit tx for marking synced: %w", err)
	}

	ragRecordsSyncedTotal.Add(ctx, int64(len(ids)))
	return nil
}

func (s *ragSyncServiceImpl) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	if len(records) == 0 {
		return nil
	}

	tx, err := s.dbProvider.Begin(ctx)
	if err != nil {
		ragSyncErrorsTotal.Add(ctx, 1)
		return fmt.Errorf("failed to begin tx for incoming sync: %w", err)
	}
	defer tx.Rollback(ctx)

	for _, rec := range records {
		// Conflict Resolution Engine: Last-Write-Wins based on LastSyncAt
		// Upsert logic for cloud Postgres DB
		query := `
			INSERT INTO autodream_memories (id, content, sync_status, last_sync_at)
			VALUES ($1, $2, 'synced', $3)
			ON CONFLICT (id) DO UPDATE
			SET content = EXCLUDED.content,
				sync_status = 'synced',
				last_sync_at = EXCLUDED.last_sync_at
			WHERE autodream_memories.last_sync_at IS NULL OR autodream_memories.last_sync_at < EXCLUDED.last_sync_at
		`
		_, err := tx.Exec(ctx, query, rec.ID, rec.Context, rec.LastSyncAt)
		if err != nil {
			ragSyncErrorsTotal.Add(ctx, 1)
			return fmt.Errorf("failed to upsert incoming record %s: %w", rec.ID, err)
		}
	}

	if err := tx.Commit(ctx); err != nil {
		ragSyncErrorsTotal.Add(ctx, 1)
		return fmt.Errorf("failed to commit tx for incoming sync: %w", err)
	}

	return nil
}
