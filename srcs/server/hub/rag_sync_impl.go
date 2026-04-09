package hub

import (
	"context"
	"fmt"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
)

type DefaultRAGSyncService struct {
	dbProvider db.Provider
}

func NewRAGSyncService(dbProvider db.Provider) *DefaultRAGSyncService {
	return &DefaultRAGSyncService{
		dbProvider: dbProvider,
	}
}

func (s *DefaultRAGSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	query := `
		SELECT memory_id, context, sync_status, last_sync_at
		FROM swarm_memory_embeddings
		WHERE sync_status = $1
		LIMIT $2
	`
	rows, err := s.dbProvider.Query(ctx, query, SyncStatusPending, limit)
	if err != nil {
		return nil, fmt.Errorf("failed to query pending syncs: %w", err)
	}
	defer rows.Close()

	var records []RAGSyncRecord
	for rows.Next() {
		var rec RAGSyncRecord
		var lastSyncAt *time.Time
		var status string
		err := rows.Scan(&rec.ID, &rec.Context, &status, &lastSyncAt)
		if err != nil {
			return nil, fmt.Errorf("failed to scan record: %w", err)
		}
		rec.SyncStatus = SyncStatus(status)
		if lastSyncAt != nil {
			rec.LastSyncAt = *lastSyncAt
		}
		records = append(records, rec)
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

	tx, err := s.dbProvider.Begin(ctx)
	if err != nil {
		return fmt.Errorf("failed to begin tx: %w", err)
	}
	defer tx.Rollback(ctx)

	for _, id := range ids {
		query := `
			UPDATE swarm_memory_embeddings
			SET sync_status = $1, last_sync_at = CURRENT_TIMESTAMP
			WHERE memory_id = $2
		`
		_, err := tx.Exec(ctx, query, SyncStatusSynced, id)
		if err != nil {
			return fmt.Errorf("failed to update record %s: %w", id, err)
		}
	}

	if err := tx.Commit(ctx); err != nil {
		return fmt.Errorf("failed to commit tx: %w", err)
	}

	RagRecordsSyncedTotal.Add(ctx, int64(len(ids)))
	return nil
}

func (s *DefaultRAGSyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	if len(records) == 0 {
		return nil
	}

	tx, err := s.dbProvider.Begin(ctx)
	if err != nil {
		return fmt.Errorf("failed to begin tx: %w", err)
	}
	defer tx.Rollback(ctx)

	for _, rec := range records {
		query := `
			INSERT INTO swarm_memory_embeddings (memory_id, context, sync_status, last_sync_at)
			VALUES ($1, $2, $3, $4)
			ON CONFLICT (memory_id) DO UPDATE SET
				context = EXCLUDED.context,
				sync_status = EXCLUDED.sync_status,
				last_sync_at = EXCLUDED.last_sync_at
		`
		_, err := tx.Exec(ctx, query, rec.ID, rec.Context, SyncStatusSynced, rec.LastSyncAt)
		if err != nil {
			RagSyncErrorsTotal.Add(ctx, 1)
			return fmt.Errorf("failed to upsert record %s: %w", rec.ID, err)
		}
	}

	if err := tx.Commit(ctx); err != nil {
		return fmt.Errorf("failed to commit tx: %w", err)
	}

	return nil
}
