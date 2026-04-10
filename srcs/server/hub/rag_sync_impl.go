package hub

import (
	"context"
	"database/sql"
	"fmt"
	"github.com/onehumancorp/mono/srcs/server/db"
	"time"
)

type ragSyncServiceImpl struct {
	dbProvider db.Provider
}

// NewRAGSyncService creates a new instance of the RAG sync service.
func NewRAGSyncService(dbProvider db.Provider) RAGSyncService {
	return &ragSyncServiceImpl{
		dbProvider: dbProvider,
	}
}

func (s *ragSyncServiceImpl) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	var records []RAGSyncRecord

	// Query the swarm_memory table where sync_status is pending.
	query := "SELECT key, value, sync_status, last_sync_at FROM swarm_memory WHERE sync_status = $1 LIMIT $2"

	rows, err := s.dbProvider.Query(ctx, query, string(SyncStatusPending), limit)
	if err != nil {
		if err == sql.ErrNoRows {
			return records, nil
		}
		return nil, fmt.Errorf("failed to fetch pending syncs: %w", err)
	}
	defer rows.Close()

	for rows.Next() {
		var r RAGSyncRecord
		var lastSync sql.NullTime
		var status string
		if err := rows.Scan(&r.ID, &r.Context, &status, &lastSync); err != nil {
			return nil, fmt.Errorf("failed to scan pending sync record: %w", err)
		}
		r.SyncStatus = SyncStatus(status)
		if lastSync.Valid {
			r.LastSyncAt = lastSync.Time
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

	tx, err := s.dbProvider.Begin(ctx)
	if err != nil {
		return fmt.Errorf("failed to begin transaction: %w", err)
	}
	defer tx.Rollback(ctx)

	query := "UPDATE swarm_memory SET sync_status = $1, last_sync_at = $2 WHERE key = $3"
	now := time.Now()

	for _, id := range ids {
		_, err := tx.Exec(ctx, query, string(SyncStatusSynced), now, id)
		if err != nil {
			if RAGSyncErrorsTotal != nil { RAGSyncErrorsTotal.Add(ctx, 1) }
			return fmt.Errorf("failed to mark record synced (ID: %s): %w", id, err)
		}
		if RAGRecordsSyncedTotal != nil { RAGRecordsSyncedTotal.Add(ctx, 1) }
	}

	if err := tx.Commit(ctx); err != nil {
		return fmt.Errorf("failed to commit transaction: %w", err)
	}

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

	// Since we are hybrid, we might be upserting to Postgres. Use ON CONFLICT.
	query := `
		INSERT INTO swarm_memory (key, value, sync_status, last_sync_at, updated_at)
		VALUES ($1, $2, $3, $4, $5)
		ON CONFLICT (key) DO UPDATE SET
			value = EXCLUDED.value,
			sync_status = EXCLUDED.sync_status,
			last_sync_at = EXCLUDED.last_sync_at,
			updated_at = EXCLUDED.updated_at
	`

	now := time.Now()
	for _, r := range records {
		_, err := tx.Exec(ctx, query, r.ID, r.Context, string(SyncStatusSynced), now, now)
		if err != nil {
			if RAGSyncErrorsTotal != nil { RAGSyncErrorsTotal.Add(ctx, 1) }
			return fmt.Errorf("failed to process incoming sync (ID: %s): %w", r.ID, err)
		}
		if RAGRecordsSyncedTotal != nil { RAGRecordsSyncedTotal.Add(ctx, 1) }
	}

	if err := tx.Commit(ctx); err != nil {
		return fmt.Errorf("failed to commit transaction: %w", err)
	}

	return nil
}
