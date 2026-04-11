package hub

import (
	"context"
	"fmt"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
)

type DefaultRAGSyncService struct {
	provider db.Provider
}

func NewDefaultRAGSyncService(provider db.Provider) *DefaultRAGSyncService {
	return &DefaultRAGSyncService{provider: provider}
}

func (s *DefaultRAGSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	var query string
	if s.provider.IsSQLite() {
		query = "SELECT memory_id, context, vector_embedding, sync_status, last_sync_at FROM swarm_memory_embeddings WHERE sync_status = ? LIMIT ?"
	} else {
		query = "SELECT memory_id, context, vector_embedding, sync_status, last_sync_at FROM swarm_memory_embeddings WHERE sync_status = $1 LIMIT $2"
	}

	rows, err := s.provider.Query(ctx, query, SyncStatusPending, limit)
	if err != nil {
		return nil, fmt.Errorf("failed to query pending syncs: %w", err)
	}
	defer rows.Close()

	var records []RAGSyncRecord
	for rows.Next() {
		var rec RAGSyncRecord
		var lastSyncAt *time.Time
		if err := rows.Scan(&rec.ID, &rec.Context, &rec.Vector, &rec.SyncStatus, &lastSyncAt); err != nil {
			return nil, fmt.Errorf("failed to scan pending sync record: %w", err)
		}
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

	tx, err := s.provider.Begin(ctx)
	if err != nil {
		return fmt.Errorf("failed to begin tx: %w", err)
	}
	defer tx.Rollback(ctx)

	var successCount int64 = 0

	for _, id := range ids {
		var query string
		if s.provider.IsSQLite() {
			query = "UPDATE swarm_memory_embeddings SET sync_status = ?, last_sync_at = ? WHERE memory_id = ?"
		} else {
			query = "UPDATE swarm_memory_embeddings SET sync_status = $1, last_sync_at = $2 WHERE memory_id = $3"
		}

		_, err := tx.Exec(ctx, query, SyncStatusSynced, time.Now(), id)
		if err != nil {
			ragSyncErrors.Add(ctx, 1)
			return fmt.Errorf("failed to update sync status for id %s: %w", id, err)
		}
		successCount++
	}

	if err := tx.Commit(ctx); err != nil {
		return fmt.Errorf("failed to commit tx: %w", err)
	}

	if successCount > 0 {
		ragRecordsSynced.Add(ctx, successCount)
	}
	return nil
}

func (s *DefaultRAGSyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	if len(records) == 0 {
		return nil
	}

	tx, err := s.provider.Begin(ctx)
	if err != nil {
		return fmt.Errorf("failed to begin tx: %w", err)
	}
	defer tx.Rollback(ctx)

	var successCount int64 = 0

	for _, rec := range records {
		var query string
		if s.provider.IsSQLite() {
			query = `
				INSERT INTO swarm_memory_embeddings (memory_id, context, vector_embedding, sync_status, last_sync_at)
				VALUES (?, ?, ?, ?, ?)
				ON CONFLICT(memory_id) DO UPDATE SET
					context = excluded.context,
					vector_embedding = excluded.vector_embedding,
					sync_status = excluded.sync_status,
					last_sync_at = excluded.last_sync_at
			`
			_, err = tx.Exec(ctx, query, rec.ID, rec.Context, rec.Vector, SyncStatusSynced, time.Now())
		} else {
			query = `
				INSERT INTO swarm_memory_embeddings (memory_id, context, vector_embedding, sync_status, last_sync_at)
				VALUES ($1, $2, $3, $4, $5)
				ON CONFLICT(memory_id) DO UPDATE SET
					context = EXCLUDED.context,
					vector_embedding = EXCLUDED.vector_embedding,
					sync_status = EXCLUDED.sync_status,
					last_sync_at = EXCLUDED.last_sync_at
			`
			_, err = tx.Exec(ctx, query, rec.ID, rec.Context, rec.Vector, SyncStatusSynced, time.Now())
		}

		if err != nil {
			ragSyncErrors.Add(ctx, 1)
			return fmt.Errorf("failed to upsert incoming record %s: %w", rec.ID, err)
		}
		successCount++
	}

	if err := tx.Commit(ctx); err != nil {
		return fmt.Errorf("failed to commit tx: %w", err)
	}

	if successCount > 0 {
		ragRecordsSynced.Add(ctx, successCount)
	}
	return nil
}
