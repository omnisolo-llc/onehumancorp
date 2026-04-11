package hub

import (
	"context"
	"fmt"
	"github.com/onehumancorp/mono/srcs/server/db"
	"time"
)

type defaultRAGSyncService struct {
	localDB db.Provider
	cloudDB db.Provider
}

// NewRAGSyncService creates a new RAG sync service.
func NewRAGSyncService(localDB db.Provider, cloudDB db.Provider) RAGSyncService {
	return &defaultRAGSyncService{
		localDB: localDB,
		cloudDB: cloudDB,
	}
}

func (s *defaultRAGSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	rows, err := s.localDB.Query(ctx, `
		SELECT memory_id, context, sync_status, last_sync_at
		FROM swarm_memory_embeddings
		WHERE sync_status = 'pending'
		LIMIT $1`, limit)
	if err != nil {
		return nil, fmt.Errorf("failed to fetch pending syncs: %w", err)
	}
	defer rows.Close()

	var records []RAGSyncRecord
	for rows.Next() {
		var r RAGSyncRecord
		var lastSyncAt *time.Time
		err := rows.Scan(&r.ID, &r.Context, &r.SyncStatus, &lastSyncAt)
		if err != nil {
			return nil, fmt.Errorf("failed to scan sync record: %w", err)
		}
		if lastSyncAt != nil {
			r.LastSyncAt = *lastSyncAt
		}
		records = append(records, r)
	}
	if err := rows.Err(); err != nil {
		return nil, fmt.Errorf("error iterating sync records: %w", err)
	}
	return records, nil
}

func (s *defaultRAGSyncService) MarkSynced(ctx context.Context, ids []string) error {
	if len(ids) == 0 {
		return nil
	}

	tx, err := s.localDB.Begin(ctx)
	if err != nil {
		return fmt.Errorf("failed to begin transaction: %w", err)
	}
	defer func() {
		_ = tx.Rollback(ctx)
	}()

	now := time.Now()
	for _, id := range ids {
		_, err := tx.Exec(ctx, `
			UPDATE swarm_memory_embeddings
			SET sync_status = 'synced', last_sync_at = $1
			WHERE memory_id = $2`, now, id)
		if err != nil {
			return fmt.Errorf("failed to mark record as synced (%s): %w", id, err)
		}
	}

	if err := tx.Commit(ctx); err != nil {
		return fmt.Errorf("failed to commit transaction: %w", err)
	}
	return nil
}

func (s *defaultRAGSyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	if len(records) == 0 {
		return nil
	}

	tx, err := s.cloudDB.Begin(ctx)
	if err != nil {
		return fmt.Errorf("failed to begin transaction: %w", err)
	}
	defer func() {
		_ = tx.Rollback(ctx)
	}()

	now := time.Now()
	for _, r := range records {
		var conflictCount int
		err := tx.QueryRow(ctx, "SELECT COUNT(*) FROM swarm_memory_embeddings WHERE memory_id = $1", r.ID).Scan(&conflictCount)
		if err != nil {
			return fmt.Errorf("failed to check for conflict (%s): %w", r.ID, err)
		}

		if conflictCount == 0 {
			_, err = tx.Exec(ctx, `
				INSERT INTO swarm_memory_embeddings (memory_id, context, sync_status, last_sync_at, created_at)
				VALUES ($1, $2, 'synced', $3, $4)`, r.ID, r.Context, now, now)
		} else {
			_, err = tx.Exec(ctx, `
				UPDATE swarm_memory_embeddings
				SET context = $1, sync_status = 'synced', last_sync_at = $2
				WHERE memory_id = $3`, r.Context, now, r.ID)
		}
		if err != nil {
			return fmt.Errorf("failed to process incoming sync (%s): %w", r.ID, err)
		}
	}

	if err := tx.Commit(ctx); err != nil {
		return fmt.Errorf("failed to commit transaction: %w", err)
	}
	return nil
}
