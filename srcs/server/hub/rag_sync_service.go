package hub

import (
	"context"
	"database/sql"
	"fmt"

	"github.com/onehumancorp/mono/srcs/server/db"
	"github.com/onehumancorp/mono/srcs/server/telemetry"
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

func (s *ragSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	query := `SELECT id, content, sync_status, last_sync_at FROM autodream_memories WHERE sync_status = 'pending' LIMIT $1`
	if s.provider.IsSQLite() {
		query = `SELECT id, content, sync_status, last_sync_at FROM autodream_memories WHERE sync_status = 'pending' LIMIT ?`
	}

	rows, err := s.provider.Query(ctx, query, limit)
	if err != nil {
		return nil, fmt.Errorf("failed to fetch pending syncs: %w", err)
	}
	defer rows.Close()

	var records []RAGSyncRecord
	for rows.Next() {
		var r RAGSyncRecord
		var lastSyncAt sql.NullTime
		if err := rows.Scan(&r.ID, &r.Context, &r.SyncStatus, &lastSyncAt); err != nil {
			return nil, fmt.Errorf("failed to scan sync record: %w", err)
		}
		if lastSyncAt.Valid {
			r.LastSyncAt = lastSyncAt.Time
		}
		records = append(records, r)
	}

	if err := rows.Err(); err != nil {
		return nil, fmt.Errorf("error iterating pending syncs: %w", err)
	}

	return records, nil
}

func (s *ragSyncService) MarkSynced(ctx context.Context, ids []string) error {
	if len(ids) == 0 {
		return nil
	}

	// This is a naive implementation for marking synced. In production, it might be better
	// to use batch updates or pgtype.TextArray depending on the database.
	// For cross-compatibility with SQLite, executing in a loop or building a dynamic query works.
	tx, err := s.provider.Begin(ctx)
	if err != nil {
		return fmt.Errorf("failed to begin transaction: %w", err)
	}
	defer tx.Rollback(ctx)

	query := `UPDATE autodream_memories SET sync_status = 'synced', last_sync_at = CURRENT_TIMESTAMP WHERE id = $1`
	if s.provider.IsSQLite() {
		query = `UPDATE autodream_memories SET sync_status = 'synced', last_sync_at = CURRENT_TIMESTAMP WHERE id = ?`
	}

	for _, id := range ids {
		_, err := tx.Exec(ctx, query, id)
		if err != nil {
			if telemetry.RAGSyncErrorsTotal != nil {
				telemetry.RAGSyncErrorsTotal.Add(ctx, 1)
			}
			return fmt.Errorf("failed to update record %s: %w", id, err)
		}
	}

	if telemetry.RAGRecordsSyncedTotal != nil {
		telemetry.RAGRecordsSyncedTotal.Add(ctx, int64(len(ids)))
	}

	return tx.Commit(ctx)
}

func (s *ragSyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	if len(records) == 0 {
		return nil
	}

	tx, err := s.provider.Begin(ctx)
	if err != nil {
		return fmt.Errorf("failed to begin transaction: %w", err)
	}
	defer tx.Rollback(ctx)

	// Since SQLite doesn't natively support vectors in the same way, we're ignoring them in this basic upsert
	// In a complete implementation, pgvector logic would go here.
	upsertQuery := `
		INSERT INTO autodream_memories (id, content, sync_status, last_sync_at)
		VALUES ($1, $2, 'synced', CURRENT_TIMESTAMP)
		ON CONFLICT (id) DO UPDATE SET
			content = EXCLUDED.content,
			sync_status = 'synced',
			last_sync_at = CURRENT_TIMESTAMP
	`
	if s.provider.IsSQLite() {
		upsertQuery = `
			INSERT INTO autodream_memories (id, content, sync_status, last_sync_at)
			VALUES (?, ?, 'synced', CURRENT_TIMESTAMP)
			ON CONFLICT (id) DO UPDATE SET
				content = excluded.content,
				sync_status = 'synced',
				last_sync_at = CURRENT_TIMESTAMP
		`
	}

	for _, r := range records {
		_, err := tx.Exec(ctx, upsertQuery, r.ID, r.Context)
		if err != nil {
			if telemetry.RAGSyncErrorsTotal != nil {
				telemetry.RAGSyncErrorsTotal.Add(ctx, 1)
			}
			return fmt.Errorf("failed to upsert record %s: %w", r.ID, err)
		}
	}

	return tx.Commit(ctx)
}
