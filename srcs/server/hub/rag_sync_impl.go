package hub

import (
	"context"
	"fmt"
	"strings"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
)

type ragSyncService struct {
	provider db.Provider
}

// NewRAGSyncService creates a new RAGSyncService instance.
func NewRAGSyncService(provider db.Provider) RAGSyncService {
	return &ragSyncService{
		provider: provider,
	}
}

// FetchPendingSyncs retrieves records from the local DB that need syncing
func (s *ragSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	query := "SELECT id, content, sync_status, last_sync_at FROM autodream_memories WHERE sync_status = 'pending' LIMIT $1"
	if s.provider.IsSQLite() {
		query = "SELECT id, content, sync_status, last_sync_at FROM autodream_memories WHERE sync_status = 'pending' LIMIT ?"
	}

	rows, err := s.provider.Query(ctx, query, limit)
	if err != nil {
		return nil, fmt.Errorf("failed to query pending syncs: %w", err)
	}
	defer rows.Close()

	var records []RAGSyncRecord
	for rows.Next() {
		var r RAGSyncRecord
		var lastSyncAt *time.Time
		if err := rows.Scan(&r.ID, &r.Context, &r.SyncStatus, &lastSyncAt); err != nil {
			return nil, fmt.Errorf("failed to scan record: %w", err)
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

// MarkSynced updates the local DB after a successful sync to the cloud
func (s *ragSyncService) MarkSynced(ctx context.Context, ids []string) error {
	if len(ids) == 0 {
		return nil
	}

	placeholders := make([]string, len(ids))
	args := make([]interface{}, len(ids))
	for i, id := range ids {
		if s.provider.IsSQLite() {
			placeholders[i] = "?"
		} else {
			placeholders[i] = fmt.Sprintf("$%d", i+1)
		}
		args[i] = id
	}

	query := fmt.Sprintf("UPDATE autodream_memories SET sync_status = 'synced', last_sync_at = CURRENT_TIMESTAMP WHERE id IN (%s)", strings.Join(placeholders, ","))

	_, err := s.provider.Exec(ctx, query, args...)
	if err != nil {
		ragSyncErrorsTotal.Add(ctx, 1)
		return fmt.Errorf("failed to mark synced: %w", err)
	}

	ragRecordsSyncedTotal.Add(ctx, int64(len(ids)))
	return nil
}

// ProcessIncomingSync handles data pushed from a standalone client into the cloud DB
func (s *ragSyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	if len(records) == 0 {
		return nil
	}

	tx, err := s.provider.Begin(ctx)
	if err != nil {
		return fmt.Errorf("failed to begin transaction: %w", err)
	}
	defer tx.Rollback(ctx)

	for _, rec := range records {
		var query string
		var args []interface{}

		if s.provider.IsSQLite() {
			query = `INSERT INTO autodream_memories (id, content, sync_status, last_sync_at)
			         VALUES (?, ?, 'synced', CURRENT_TIMESTAMP)
			         ON CONFLICT(id) DO UPDATE SET content = excluded.content, sync_status = 'synced', last_sync_at = CURRENT_TIMESTAMP`
			args = []interface{}{rec.ID, rec.Context}
		} else {
			query = `INSERT INTO autodream_memories (id, content, sync_status, last_sync_at)
			         VALUES ($1, $2, 'synced', CURRENT_TIMESTAMP)
			         ON CONFLICT(id) DO UPDATE SET content = EXCLUDED.content, sync_status = 'synced', last_sync_at = CURRENT_TIMESTAMP`
			args = []interface{}{rec.ID, rec.Context}
		}

		_, err := tx.Exec(ctx, query, args...)
		if err != nil {
			ragSyncErrorsTotal.Add(ctx, 1)
			return fmt.Errorf("failed to upsert record %s: %w", rec.ID, err)
		}
	}

	if err := tx.Commit(ctx); err != nil {
		return fmt.Errorf("failed to commit transaction: %w", err)
	}

	ragRecordsSyncedTotal.Add(ctx, int64(len(records)))
	return nil
}
