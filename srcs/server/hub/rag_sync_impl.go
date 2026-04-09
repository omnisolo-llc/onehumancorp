package hub

import (
	"context"
	"database/sql"
	"fmt"
	"time"
)

// SQLRAGSyncService is a production implementation of RAGSyncService backed by a *sql.DB
type SQLRAGSyncService struct {
	db       *sql.DB
	isSQLite bool // Track dialect to use appropriate binding placeholders
}

// NewSQLRAGSyncService creates a new SQLRAGSyncService
func NewSQLRAGSyncService(db *sql.DB, isSQLite bool) *SQLRAGSyncService {
	return &SQLRAGSyncService{
		db:       db,
		isSQLite: isSQLite,
	}
}

// FetchPendingSyncs retrieves records from the local DB that need syncing
func (s *SQLRAGSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	query := "SELECT id, content, sync_status, last_sync_at FROM autodream_memories WHERE sync_status = 'pending'"
	if limit > 0 {
		query += fmt.Sprintf(" LIMIT %d", limit)
	}

	rows, err := s.db.QueryContext(ctx, query)
	if err != nil {
		return nil, fmt.Errorf("failed to fetch pending syncs: %w", err)
	}
	defer rows.Close()

	var records []RAGSyncRecord
	for rows.Next() {
		var record RAGSyncRecord
		var lastSyncAt sql.NullTime
		if err := rows.Scan(&record.ID, &record.Context, &record.SyncStatus, &lastSyncAt); err != nil {
			return nil, fmt.Errorf("failed to scan record: %w", err)
		}
		if lastSyncAt.Valid {
			record.LastSyncAt = lastSyncAt.Time
		}
		records = append(records, record)
	}

	if err := rows.Err(); err != nil {
		return nil, fmt.Errorf("error iterating over rows: %w", err)
	}

	return records, nil
}

// MarkSynced updates the local DB after a successful sync to the cloud
func (s *SQLRAGSyncService) MarkSynced(ctx context.Context, ids []string) error {
	if len(ids) == 0 {
		return nil
	}

	now := time.Now()

	tx, err := s.db.BeginTx(ctx, nil)
	if err != nil {
		return fmt.Errorf("failed to begin transaction: %w", err)
	}
	defer tx.Rollback()

	query := "UPDATE autodream_memories SET sync_status = 'synced', last_sync_at = $1 WHERE id = $2"
	if s.isSQLite {
		query = "UPDATE autodream_memories SET sync_status = 'synced', last_sync_at = ? WHERE id = ?"
	}

	stmt, err := tx.PrepareContext(ctx, query)
	if err != nil {
		return fmt.Errorf("failed to prepare statement: %w", err)
	}
	defer stmt.Close()

	for _, id := range ids {
		_, err := stmt.ExecContext(ctx, now, id)
		if err != nil {
			ragSyncErrorsTotal.Add(ctx, 1)
			return fmt.Errorf("failed to update record %s: %w", id, err)
		}
		ragRecordsSyncedTotal.Add(ctx, 1)
	}

	if err := tx.Commit(); err != nil {
		return fmt.Errorf("failed to commit transaction: %w", err)
	}

	return nil
}

// ProcessIncomingSync handles data pushed from a standalone client into the cloud DB
func (s *SQLRAGSyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	if len(records) == 0 {
		return nil
	}

	tx, err := s.db.BeginTx(ctx, nil)
	if err != nil {
		return fmt.Errorf("failed to begin transaction: %w", err)
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
	if s.isSQLite {
		query = `
			INSERT INTO autodream_memories (id, content, sync_status, last_sync_at)
			VALUES (?, ?, ?, ?)
			ON CONFLICT (id) DO UPDATE SET
				content = EXCLUDED.content,
				sync_status = EXCLUDED.sync_status,
				last_sync_at = EXCLUDED.last_sync_at
		`
	}

	stmt, err := tx.PrepareContext(ctx, query)
	if err != nil {
		return fmt.Errorf("failed to prepare statement: %w", err)
	}
	defer stmt.Close()

	for _, r := range records {
		var lastSyncAt interface{}
		if !r.LastSyncAt.IsZero() {
			lastSyncAt = r.LastSyncAt
		}
		_, err := stmt.ExecContext(ctx, r.ID, r.Context, r.SyncStatus, lastSyncAt)
		if err != nil {
			ragSyncErrorsTotal.Add(ctx, 1)
			return fmt.Errorf("failed to process record %s: %w", r.ID, err)
		}
	}

	if err := tx.Commit(); err != nil {
		return fmt.Errorf("failed to commit transaction: %w", err)
	}

	return nil
}
