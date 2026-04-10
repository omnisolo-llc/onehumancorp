package hub

import (
	"context"
	"database/sql"
	"time"
)

type ragSyncService struct {
	db *sql.DB
}

func NewRAGSyncService(db *sql.DB) RAGSyncService {
	return &ragSyncService{db: db}
}

func (s *ragSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	query := `
		SELECT id, content, sync_status, last_sync_at
		FROM autodream_memories
		WHERE sync_status = $1
		ORDER BY created_at ASC
		LIMIT $2
	`
	rows, err := s.db.QueryContext(ctx, query, SyncStatusPending, limit)
	if err != nil {
		return nil, err
	}
	defer rows.Close()

	var records []RAGSyncRecord
	for rows.Next() {
		var r RAGSyncRecord
		var lastSync sql.NullTime
		if err := rows.Scan(&r.ID, &r.Context, &r.SyncStatus, &lastSync); err != nil {
			return nil, err
		}
		if lastSync.Valid {
			r.LastSyncAt = lastSync.Time
		}
		records = append(records, r)
	}
	return records, rows.Err()
}

func (s *ragSyncService) MarkSynced(ctx context.Context, ids []string) error {
	if len(ids) == 0 {
		return nil
	}

	tx, err := s.db.BeginTx(ctx, nil)
	if err != nil {
		return err
	}
	defer tx.Rollback()

	query := `
		UPDATE autodream_memories
		SET sync_status = $1, last_sync_at = $2
		WHERE id = $3
	`
	stmt, err := tx.PrepareContext(ctx, query)
	if err != nil {
		return err
	}
	defer stmt.Close()

	now := time.Now()
	for _, id := range ids {
		if _, err := stmt.ExecContext(ctx, SyncStatusSynced, now, id); err != nil {
			return err
		}
	}

	return tx.Commit()
}

func (s *ragSyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	if len(records) == 0 {
		return nil
	}

	tx, err := s.db.BeginTx(ctx, nil)
	if err != nil {
		return err
	}
	defer tx.Rollback()

	query := `
		INSERT INTO autodream_memories (id, content, sync_status, last_sync_at)
		VALUES ($1, $2, $3, $4)
		ON CONFLICT (id) DO UPDATE SET
			content = EXCLUDED.content,
			sync_status = EXCLUDED.sync_status,
			last_sync_at = EXCLUDED.last_sync_at
	`
	stmt, err := tx.PrepareContext(ctx, query)
	if err != nil {
		return err
	}
	defer stmt.Close()

	for _, r := range records {
		var lastSync interface{}
		if !r.LastSyncAt.IsZero() {
			lastSync = r.LastSyncAt
		}
		if _, err := stmt.ExecContext(ctx, r.ID, r.Context, r.SyncStatus, lastSync); err != nil {
			return err
		}
	}

	return tx.Commit()
}
