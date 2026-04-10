package hub

import (
	"context"
	"database/sql"
	"fmt"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
)

// SQLRAGSyncService implements RAGSyncService using a database provider.
type SQLRAGSyncService struct {
	provider db.Provider
}

// NewSQLRAGSyncService creates a new SQLRAGSyncService.
func NewSQLRAGSyncService(provider db.Provider) *SQLRAGSyncService {
	return &SQLRAGSyncService{
		provider: provider,
	}
}

func (s *SQLRAGSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	start := time.Now()
	defer func() { RecordSyncLatency(ctx, time.Since(start)) }()

	query := `
		SELECT id, content, sync_status, last_sync_at
		FROM autodream_memories
		WHERE sync_status = $1 OR sync_status IS NULL
		ORDER BY created_at ASC
		LIMIT $2
	`

	rows, err := s.provider.Query(ctx, query, string(SyncStatusPending), limit)
	if err != nil {
		RecordSyncError(ctx)
		return nil, fmt.Errorf("failed to fetch pending syncs: %w", err)
	}
	defer rows.Close()

	var records []RAGSyncRecord
	for rows.Next() {
		var rec RAGSyncRecord
		var status sql.NullString
		var lastSync sql.NullTime

		if err := rows.Scan(&rec.ID, &rec.Context, &status, &lastSync); err != nil {
			RecordSyncError(ctx)
			return nil, fmt.Errorf("failed to scan pending sync record: %w", err)
		}

		if status.Valid {
			rec.SyncStatus = SyncStatus(status.String)
		} else {
			rec.SyncStatus = SyncStatusPending
		}

		if lastSync.Valid {
			rec.LastSyncAt = lastSync.Time
		}

		records = append(records, rec)
	}

	if err := rows.Err(); err != nil {
		RecordSyncError(ctx)
		return nil, fmt.Errorf("error iterating pending syncs: %w", err)
	}

	return records, nil
}

func (s *SQLRAGSyncService) MarkSynced(ctx context.Context, ids []string) error {
	if len(ids) == 0 {
		return nil
	}

	start := time.Now()
	defer func() { RecordSyncLatency(ctx, time.Since(start)) }()

	// Update records one by one to avoid complex dynamic SQL, or use a batch mechanism if provider supports it.
	for _, id := range ids {
		query := `
			UPDATE autodream_memories
			SET sync_status = $1, last_sync_at = $2
			WHERE id = $3
		`
		_, err := s.provider.Exec(ctx, query, string(SyncStatusSynced), time.Now(), id)
		if err != nil {
			RecordSyncError(ctx)
			return fmt.Errorf("failed to mark record %s as synced: %w", id, err)
		}
	}

	RecordSyncSuccess(ctx, len(ids))
	return nil
}

func (s *SQLRAGSyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	if len(records) == 0 {
		return nil
	}

	start := time.Now()
	defer func() { RecordSyncLatency(ctx, time.Since(start)) }()

	for _, rec := range records {
		// Basic upsert logic. We rely on the ID being present and unique.
		// If it exists, update it. Otherwise, insert it.
		// For SQLite and Postgres compatibility, we can do a simple check then insert/update.

		var exists bool
		checkQuery := `SELECT EXISTS(SELECT 1 FROM autodream_memories WHERE id = $1)`
		err := s.provider.QueryRow(ctx, checkQuery, rec.ID).Scan(&exists)
		if err != nil {
			RecordSyncError(ctx)
			return fmt.Errorf("failed to check existence for record %s: %w", rec.ID, err)
		}

		if exists {
			updateQuery := `
				UPDATE autodream_memories
				SET content = $1, sync_status = $2, last_sync_at = $3
				WHERE id = $4
			`
			_, err = s.provider.Exec(ctx, updateQuery, rec.Context, string(SyncStatusSynced), rec.LastSyncAt, rec.ID)
			if err != nil {
				RecordSyncError(ctx)
				return fmt.Errorf("failed to update incoming record %s: %w", rec.ID, err)
			}
		} else {
			insertQuery := `
				INSERT INTO autodream_memories (id, content, sync_status, last_sync_at)
				VALUES ($1, $2, $3, $4)
			`
			_, err = s.provider.Exec(ctx, insertQuery, rec.ID, rec.Context, string(SyncStatusSynced), rec.LastSyncAt)
			if err != nil {
				RecordSyncError(ctx)
				return fmt.Errorf("failed to insert incoming record %s: %w", rec.ID, err)
			}
		}
	}

	RecordSyncSuccess(ctx, len(records))
	return nil
}
