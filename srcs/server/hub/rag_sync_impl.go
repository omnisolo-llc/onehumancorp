package hub

import (
	"context"
	"fmt"
	"strings"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
)

type DefaultRAGSyncService struct {
	provider db.Provider
}

func NewDefaultRAGSyncService(provider db.Provider) *DefaultRAGSyncService {
	return &DefaultRAGSyncService{
		provider: provider,
	}
}

func (s *DefaultRAGSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	query := `SELECT id, content, sync_status, last_sync_at FROM autodream_memories WHERE sync_status = 'pending' LIMIT $1`
	rows, err := s.provider.Query(ctx, query, limit)
	if err != nil {
		return nil, fmt.Errorf("failed to query pending syncs: %w", err)
	}
	defer rows.Close()

	var records []RAGSyncRecord
	for rows.Next() {
		var r RAGSyncRecord
		var lastSyncAt *time.Time
		var syncStatus string
		if err := rows.Scan(&r.ID, &r.Context, &syncStatus, &lastSyncAt); err != nil {
			return nil, fmt.Errorf("failed to scan sync record: %w", err)
		}
		r.SyncStatus = SyncStatus(syncStatus)
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

func (s *DefaultRAGSyncService) MarkSynced(ctx context.Context, ids []string) error {
	if len(ids) == 0 {
		return nil
	}

	placeholders := make([]string, len(ids))
	args := make([]any, len(ids)+1)
	args[0] = time.Now()
	for i, id := range ids {
		placeholders[i] = fmt.Sprintf("$%d", i+2)
		args[i+1] = id
	}

	query := fmt.Sprintf(`UPDATE autodream_memories SET sync_status = 'synced', last_sync_at = $1 WHERE id IN (%s)`, strings.Join(placeholders, ","))

	_, err := s.provider.Exec(ctx, query, args...)
	if err != nil {
		ragSyncErrorsTotal.Add(ctx, int64(len(ids)))
		return fmt.Errorf("failed to mark records as synced: %w", err)
	}

	ragRecordsSyncedTotal.Add(ctx, int64(len(ids)))
	return nil
}

func (s *DefaultRAGSyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	if len(records) == 0 {
		return nil
	}

	tx, err := s.provider.Begin(ctx)
	if err != nil {
		return fmt.Errorf("failed to begin transaction: %w", err)
	}
	defer tx.Rollback(ctx)

	for _, r := range records {
		query := `
			INSERT INTO autodream_memories (id, content, sync_status, last_sync_at)
			VALUES ($1, $2, 'synced', $3)
			ON CONFLICT (id) DO UPDATE SET
				content = EXCLUDED.content,
				sync_status = EXCLUDED.sync_status,
				last_sync_at = EXCLUDED.last_sync_at
		`
		if s.provider.IsSQLite() {
			query = `
				INSERT INTO autodream_memories (id, content, sync_status, last_sync_at)
				VALUES ($1, $2, 'synced', $3)
				ON CONFLICT (id) DO UPDATE SET
					content = excluded.content,
					sync_status = excluded.sync_status,
					last_sync_at = excluded.last_sync_at
			`
		}

		_, err := tx.Exec(ctx, query, r.ID, r.Context, time.Now())
		if err != nil {
			ragSyncErrorsTotal.Add(ctx, 1)
			return fmt.Errorf("failed to upsert incoming record %s: %w", r.ID, err)
		}
		ragRecordsSyncedTotal.Add(ctx, 1)
	}

	if err := tx.Commit(ctx); err != nil {
		return fmt.Errorf("failed to commit transaction: %w", err)
	}

	return nil
}
