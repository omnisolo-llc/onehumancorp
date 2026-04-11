package hub

import (
	"context"
	"database/sql"
	"fmt"
	"strings"
)

// RAGSyncServiceImpl implements the RAGSyncService interface.
type RAGSyncServiceImpl struct {
	db *sql.DB
}

// NewRAGSyncService creates a new RAGSyncServiceImpl.
func NewRAGSyncService(db *sql.DB) *RAGSyncServiceImpl {
	return &RAGSyncServiceImpl{db: db}
}

// FetchPendingSyncs retrieves records from the local DB that need syncing.
func (s *RAGSyncServiceImpl) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	query := `
		SELECT memory_id, context, sync_status, last_sync_at
		FROM swarm_memory_embeddings
		WHERE sync_status = 'pending'
		LIMIT $1
	`
	rows, err := s.db.QueryContext(ctx, query, limit)
	if err != nil {
		return nil, fmt.Errorf("failed to fetch pending syncs: %w", err)
	}
	defer rows.Close()

	var records []RAGSyncRecord
	for rows.Next() {
		var rec RAGSyncRecord
		var syncStatus sql.NullString
		var lastSyncAt sql.NullTime

		err := rows.Scan(&rec.ID, &rec.Context, &syncStatus, &lastSyncAt)
		if err != nil {
			return nil, fmt.Errorf("failed to scan pending sync record: %w", err)
		}

		if syncStatus.Valid {
			rec.SyncStatus = SyncStatus(syncStatus.String)
		}
		if lastSyncAt.Valid {
			rec.LastSyncAt = lastSyncAt.Time
		}

		records = append(records, rec)
	}

	if err := rows.Err(); err != nil {
		return nil, fmt.Errorf("error iterating pending sync records: %w", err)
	}

	return records, nil
}

// MarkSynced updates the local DB after a successful sync to the cloud.
func (s *RAGSyncServiceImpl) MarkSynced(ctx context.Context, ids []string) error {
	if len(ids) == 0 {
		return nil
	}

	// Create placeholders for the IN clause
	placeholders := make([]string, len(ids))
	args := make([]interface{}, len(ids))
	for i, id := range ids {
		placeholders[i] = fmt.Sprintf("$%d", i+1)
		args[i] = id
	}

	query := fmt.Sprintf(`
		UPDATE swarm_memory_embeddings
		SET sync_status = 'synced', last_sync_at = CURRENT_TIMESTAMP
		WHERE memory_id IN (%s)
	`, strings.Join(placeholders, ","))

	_, err := s.db.ExecContext(ctx, query, args...)
	if err != nil {
		return fmt.Errorf("failed to mark records as synced: %w", err)
	}

	return nil
}

// ProcessIncomingSync handles data pushed from a standalone client into the cloud DB.
func (s *RAGSyncServiceImpl) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	if len(records) == 0 {
		return nil
	}

	tx, err := s.db.BeginTx(ctx, nil)
	if err != nil {
		return fmt.Errorf("failed to begin transaction: %w", err)
	}
	defer tx.Rollback()

	// Ensure the context is synced.
	// Assume ON CONFLICT handles existing records (e.g., using PostgreSQL UPSERT syntax).
	// Since SQLite doesn't natively support this exact syntax in the same way, we use a standard approach.
	query := `
		INSERT INTO swarm_memory_embeddings (memory_id, context, sync_status, last_sync_at)
		VALUES ($1, $2, $3, CURRENT_TIMESTAMP)
		ON CONFLICT (memory_id) DO UPDATE SET
			context = EXCLUDED.context,
			sync_status = EXCLUDED.sync_status,
			last_sync_at = EXCLUDED.last_sync_at
	`

	stmt, err := tx.PrepareContext(ctx, query)
	if err != nil {
		return fmt.Errorf("failed to prepare statement: %w", err)
	}
	defer stmt.Close()

	for _, rec := range records {
		_, err := stmt.ExecContext(ctx, rec.ID, rec.Context, rec.SyncStatus)
		if err != nil {
			return fmt.Errorf("failed to upsert incoming sync record %s: %w", rec.ID, err)
		}
	}

	if err := tx.Commit(); err != nil {
		return fmt.Errorf("failed to commit transaction: %w", err)
	}

	return nil
}
