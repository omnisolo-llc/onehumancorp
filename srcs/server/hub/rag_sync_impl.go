package hub

import (
	"context"
	"database/sql"
	"fmt"
	"strings"
	"time"
)

type DBProvider interface {
	QueryContext(ctx context.Context, query string, args ...any) (*sql.Rows, error)
	ExecContext(ctx context.Context, query string, args ...any) (sql.Result, error)
}

type DefaultRAGSyncService struct {
	db DBProvider
}

func NewDefaultRAGSyncService(db DBProvider) *DefaultRAGSyncService {
	return &DefaultRAGSyncService{db: db}
}

func (s *DefaultRAGSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	query := `
		SELECT id, content, sync_status, last_sync_at
		FROM autodream_memories
		WHERE sync_status = $1 OR sync_status IS NULL
		ORDER BY created_at ASC
		LIMIT $2
	`
	rows, err := s.db.QueryContext(ctx, query, SyncStatusPending, limit)
	if err != nil {
		return nil, fmt.Errorf("failed to fetch pending syncs: %w", err)
	}
	defer rows.Close()

	var records []RAGSyncRecord
	for rows.Next() {
		var r RAGSyncRecord
		var lastSync sql.NullTime
		if err := rows.Scan(&r.ID, &r.Context, &r.SyncStatus, &lastSync); err != nil {
			return nil, fmt.Errorf("failed to scan record: %w", err)
		}
		if lastSync.Valid {
			r.LastSyncAt = lastSync.Time
		}
		records = append(records, r)
	}
	if err := rows.Err(); err != nil {
		return nil, fmt.Errorf("row iteration error: %w", err)
	}
	return records, nil
}

func (s *DefaultRAGSyncService) MarkSynced(ctx context.Context, ids []string) error {
	if len(ids) == 0 {
		return nil
	}

	// For standard SQLite/Postgres compatibility we use parameterized query with ANY or IN
	// IN clause generation
	placeholders := make([]string, len(ids))
	args := make([]interface{}, len(ids)+2)
	args[0] = SyncStatusSynced
	args[1] = time.Now()

	for i, id := range ids {
		placeholders[i] = fmt.Sprintf("$%d", i+3)
		args[i+2] = id
	}

	query := fmt.Sprintf(`
		UPDATE autodream_memories
		SET sync_status = $1, last_sync_at = $2
		WHERE id IN (%s)
	`, strings.Join(placeholders, ","))

	_, err := s.db.ExecContext(ctx, query, args...)
	if err != nil {
		RagSyncErrorsTotal.Add(ctx, 1)
		return fmt.Errorf("failed to mark synced: %w", err)
	}
	RagRecordsSyncedTotal.Add(ctx, int64(len(ids)))
	return nil
}

func (s *DefaultRAGSyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	if len(records) == 0 {
		return nil
	}

	// Upsert logic compatible with Postgres
	for _, r := range records {
		query := `
			INSERT INTO autodream_memories (id, content, sync_status, last_sync_at)
			VALUES ($1, $2, $3, $4)
			ON CONFLICT (id) DO UPDATE SET
				content = EXCLUDED.content,
				sync_status = EXCLUDED.sync_status,
				last_sync_at = EXCLUDED.last_sync_at
		`
		_, err := s.db.ExecContext(ctx, query, r.ID, r.Context, r.SyncStatus, r.LastSyncAt)
		if err != nil {
			RagSyncErrorsTotal.Add(ctx, 1)
			return fmt.Errorf("failed to process incoming sync for %s: %w", r.ID, err)
		}
	}
	return nil
}
