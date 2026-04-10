package hub

import (
	"context"
	"database/sql"
	"fmt"
	"strings"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
)

type ragSyncServiceImpl struct {
	db db.Provider
}

func NewRAGSyncService(provider db.Provider) RAGSyncService {
	return &ragSyncServiceImpl{db: provider}
}

func (s *ragSyncServiceImpl) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	query := `SELECT id, content, sync_status, last_sync_at FROM autodream_memories WHERE sync_status = 'pending' LIMIT $1`
	if s.db.IsSQLite() {
		query = strings.ReplaceAll(query, "$1", "?")
	}

	rows, err := s.db.Query(ctx, query, limit)
	if err != nil {
		return nil, fmt.Errorf("failed to query pending syncs: %w", err)
	}
	defer rows.Close()

	var records []RAGSyncRecord
	for rows.Next() {
		var rec RAGSyncRecord
		var lastSync sql.NullTime
		var status sql.NullString

		if err := rows.Scan(&rec.ID, &rec.Context, &status, &lastSync); err != nil {
			return nil, fmt.Errorf("failed to scan row: %w", err)
		}

		if status.Valid {
			rec.SyncStatus = SyncStatus(status.String)
		}
		if lastSync.Valid {
			rec.LastSyncAt = lastSync.Time
		}

		records = append(records, rec)
	}
	if err := rows.Err(); err != nil {
		return nil, fmt.Errorf("rows error: %w", err)
	}

	return records, nil
}

func (s *ragSyncServiceImpl) MarkSynced(ctx context.Context, ids []string) error {
	if len(ids) == 0 {
		return nil
	}

	// For simplicity, update one by one or create a query with placeholders
	// Since ids is a slice, we can execute a simple IN query.
	placeholders := make([]string, len(ids))
	args := make([]any, len(ids)+2)
	args[0] = string(SyncStatusSynced)
	args[1] = time.Now()

	for i, id := range ids {
		if s.db.IsSQLite() {
			placeholders[i] = "?"
		} else {
			placeholders[i] = fmt.Sprintf("$%d", i+3)
		}
		args[i+2] = id
	}

	query := fmt.Sprintf(`UPDATE autodream_memories SET sync_status = $1, last_sync_at = $2 WHERE id IN (%s)`, strings.Join(placeholders, ","))
	if s.db.IsSQLite() {
		query = strings.Replace(query, "$1", "?", 1)
		query = strings.Replace(query, "$2", "?", 1)
	}

	_, err := s.db.Exec(ctx, query, args...)
	if err != nil {
		SyncErrorsTotal.Add(ctx, 1)
		return fmt.Errorf("failed to mark synced: %w", err)
	}

	RecordsSyncedTotal.Add(ctx, int64(len(ids)))
	return nil
}

func (s *ragSyncServiceImpl) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	if len(records) == 0 {
		return nil
	}

	tx, err := s.db.Begin(ctx)
	if err != nil {
		return fmt.Errorf("failed to begin tx: %w", err)
	}
	defer tx.Rollback(ctx)

	for _, rec := range records {
		// Last write wins - basic upsert based on ID
		// Actually, let's do a simple check and update, or conflict resolution.
		query := `
			INSERT INTO autodream_memories (id, content, sync_status, last_sync_at)
			VALUES ($1, $2, $3, $4)
			ON CONFLICT (id) DO UPDATE SET
				content = EXCLUDED.content,
				sync_status = EXCLUDED.sync_status,
				last_sync_at = EXCLUDED.last_sync_at
		`
		if s.db.IsSQLite() {
			query = `
				INSERT INTO autodream_memories (id, content, sync_status, last_sync_at)
				VALUES (?, ?, ?, ?)
				ON CONFLICT (id) DO UPDATE SET
					content = excluded.content,
					sync_status = excluded.sync_status,
					last_sync_at = excluded.last_sync_at
			`
		}

		_, err := tx.Exec(ctx, query, rec.ID, rec.Context, string(rec.SyncStatus), rec.LastSyncAt)
		if err != nil {
			SyncErrorsTotal.Add(ctx, 1)
			return fmt.Errorf("failed to process incoming sync for record %s: %w", rec.ID, err)
		}
	}

	if err := tx.Commit(ctx); err != nil {
		return fmt.Errorf("failed to commit tx: %w", err)
	}

	RecordsSyncedTotal.Add(ctx, int64(len(records)))
	return nil
}
