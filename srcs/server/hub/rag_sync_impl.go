package hub

import (
	"context"
	"fmt"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
)

type RAGSyncServiceImpl struct {
	provider db.Provider
}

func NewRAGSyncService(provider db.Provider) *RAGSyncServiceImpl {
	return &RAGSyncServiceImpl{
		provider: provider,
	}
}

func (s *RAGSyncServiceImpl) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	query := `
		SELECT id, content, vector, sync_status, last_sync_at
		FROM autodream_memories
		WHERE sync_status = 'pending'
		LIMIT $1
	`
	rows, err := s.provider.Query(ctx, query, limit)
	if err != nil {
		return nil, fmt.Errorf("failed to query pending syncs: %w", err)
	}
	defer rows.Close()

	var records []RAGSyncRecord
	for rows.Next() {
		var r RAGSyncRecord
		var lastSyncAt *time.Time
		if err := rows.Scan(&r.ID, &r.Context, &r.Vector, &r.SyncStatus, &lastSyncAt); err != nil {
			return nil, fmt.Errorf("failed to scan row: %w", err)
		}
		if lastSyncAt != nil {
			r.LastSyncAt = *lastSyncAt
		}
		records = append(records, r)
	}
	if err := rows.Err(); err != nil {
		return nil, fmt.Errorf("row iteration error: %w", err)
	}

	return records, nil
}

func (s *RAGSyncServiceImpl) MarkSynced(ctx context.Context, ids []string) error {
	if len(ids) == 0 {
		return nil
	}

	tx, err := s.provider.Begin(ctx)
	if err != nil {
		return fmt.Errorf("failed to begin transaction: %w", err)
	}
	defer func() { _ = tx.Rollback(ctx) }()

	now := time.Now()
	for _, id := range ids {
		query := `
			UPDATE autodream_memories
			SET sync_status = 'synced', last_sync_at = $1
			WHERE id = $2
		`
		if _, err := tx.Exec(ctx, query, now, id); err != nil {
			SyncErrorsCounter.Add(ctx, 1)
			return fmt.Errorf("failed to mark record %s as synced: %w", id, err)
		}
		RecordsSyncedCounter.Add(ctx, 1)
	}

	if err := tx.Commit(ctx); err != nil {
		return fmt.Errorf("failed to commit transaction: %w", err)
	}

	return nil
}

func (s *RAGSyncServiceImpl) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	tx, err := s.provider.Begin(ctx)
	if err != nil {
		return fmt.Errorf("failed to begin transaction: %w", err)
	}
	defer func() { _ = tx.Rollback(ctx) }()

	for _, r := range records {
		var exists bool
		err := tx.QueryRow(ctx, "SELECT EXISTS(SELECT 1 FROM autodream_memories WHERE id = $1)", r.ID).Scan(&exists)
		if err != nil {
			return fmt.Errorf("failed to check existence of record %s: %w", r.ID, err)
		}

		if exists {
			query := `
				UPDATE autodream_memories
				SET content = $1, vector = $2, sync_status = $3, last_sync_at = $4
				WHERE id = $5
			`
			if _, err := tx.Exec(ctx, query, r.Context, r.Vector, string(r.SyncStatus), r.LastSyncAt, r.ID); err != nil {
				return fmt.Errorf("failed to update record %s: %w", r.ID, err)
			}
		} else {
			query := `
				INSERT INTO autodream_memories (id, content, vector, sync_status, last_sync_at)
				VALUES ($1, $2, $3, $4, $5)
			`
			if _, err := tx.Exec(ctx, query, r.ID, r.Context, r.Vector, string(r.SyncStatus), r.LastSyncAt); err != nil {
				return fmt.Errorf("failed to insert record %s: %w", r.ID, err)
			}
		}
	}

	if err := tx.Commit(ctx); err != nil {
		return fmt.Errorf("failed to commit transaction: %w", err)
	}

	return nil
}
