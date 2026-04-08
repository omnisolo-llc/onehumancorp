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

// NewRAGSyncService creates a new RAGSyncService backed by the database provider
func NewRAGSyncService(dbProvider db.Provider) RAGSyncService {
	return &ragSyncServiceImpl{
		dbProvider: dbProvider,
	}
}

func (s *ragSyncServiceImpl) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	query := `
		SELECT id, content, sync_status, last_sync_at
		FROM autodream_memories
		WHERE sync_status = $1
		ORDER BY created_at ASC
		LIMIT $2
	`
	rows, err := s.dbProvider.Query(ctx, query, string(SyncStatusPending), limit)
	if err != nil {
		return nil, fmt.Errorf("failed to query pending syncs: %w", err)
	}
	defer rows.Close()

	var records []RAGSyncRecord
	for rows.Next() {
		var r RAGSyncRecord
		var lastSyncAt *time.Time
		var status string
		if err := rows.Scan(&r.ID, &r.Context, &status, &lastSyncAt); err != nil {
			return nil, fmt.Errorf("failed to scan record: %w", err)
		}
		r.SyncStatus = SyncStatus(status)
		if lastSyncAt != nil {
			r.LastSyncAt = *lastSyncAt
		}
		records = append(records, r)
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

	// Assuming a simpler approach since we cannot easily use IN with variable parameters in some database providers
	tx, err := s.dbProvider.Begin(ctx)
	if err != nil {
		return fmt.Errorf("failed to begin transaction: %w", err)
	}
	defer tx.Rollback(ctx)

	query := `UPDATE autodream_memories SET sync_status = $1, last_sync_at = CURRENT_TIMESTAMP WHERE id = $2`
	for _, id := range ids {
		if _, err := tx.Exec(ctx, query, string(SyncStatusSynced), id); err != nil {
			return fmt.Errorf("failed to update record %s: %w", id, err)
		}
	}

	if err := tx.Commit(ctx); err != nil {
		return fmt.Errorf("failed to commit transaction: %w", err)
	}

	// Increment telemetry metric
	RagRecordsSyncedTotal.Add(ctx, int64(len(ids)))

	return nil
}

func (s *ragSyncServiceImpl) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	if len(records) == 0 {
		return nil
	}

	tx, err := s.dbProvider.Begin(ctx)
	if err != nil {
		return fmt.Errorf("failed to begin transaction: %w", err)
	}
	defer tx.Rollback(ctx)

	// In SQLite we use ON CONFLICT, in Postgres we use ON CONFLICT.
	// We'll do an upsert for simplicity. We don't have vector in SQLite but we can insert the content.
	query := `
		INSERT INTO autodream_memories (id, content, sync_status, last_sync_at)
		VALUES ($1, $2, $3, CURRENT_TIMESTAMP)
		ON CONFLICT (id) DO UPDATE SET
			content = EXCLUDED.content,
			sync_status = EXCLUDED.sync_status,
			last_sync_at = CURRENT_TIMESTAMP
	`
	for _, r := range records {
		if _, err := tx.Exec(ctx, query, r.ID, r.Context, string(r.SyncStatus)); err != nil {
			RagSyncErrorsTotal.Add(ctx, 1)
			return fmt.Errorf("failed to upsert record %s: %w", r.ID, err)
		}
	}

	if err := tx.Commit(ctx); err != nil {
		return fmt.Errorf("failed to commit transaction: %w", err)
	}

	return nil
}
