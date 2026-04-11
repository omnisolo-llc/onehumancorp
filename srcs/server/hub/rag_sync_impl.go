package hub

import (
	"context"
	"fmt"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
)

type defaultRAGSyncService struct {
	provider db.Provider
}

func NewRAGSyncService(p db.Provider) RAGSyncService {
	return &defaultRAGSyncService{provider: p}
}

func (s *defaultRAGSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	if !s.provider.IsSQLite() {
		return nil, fmt.Errorf("FetchPendingSyncs is only supported in SQLite Standalone mode")
	}

	query := `SELECT memory_id, context, vector_embedding, sync_status, last_sync_at FROM swarm_memory_embeddings WHERE sync_status = 'pending' LIMIT $1`
	rows, err := s.provider.Query(ctx, query, limit)
	if err != nil {
		return nil, fmt.Errorf("failed to fetch pending syncs: %w", err)
	}
	defer rows.Close()

	var records []RAGSyncRecord
	for rows.Next() {
		var rec RAGSyncRecord
		var lastSyncAt *time.Time
		var vectorBytes []byte
		if err := rows.Scan(&rec.ID, &rec.Context, &vectorBytes, &rec.SyncStatus, &lastSyncAt); err != nil {
			return nil, fmt.Errorf("failed to scan record: %w", err)
		}
		if lastSyncAt != nil {
			rec.LastSyncAt = *lastSyncAt
		}
		records = append(records, rec)
	}
	return records, rows.Err()
}

func (s *defaultRAGSyncService) MarkSynced(ctx context.Context, ids []string) error {
	if !s.provider.IsSQLite() {
		return fmt.Errorf("MarkSynced is only supported in SQLite Standalone mode")
	}

	if len(ids) == 0 {
		return nil
	}

	// Simple loop to avoid complex IN clauses for SQLite via generic provider
	now := time.Now()
	for _, id := range ids {
		query := `UPDATE swarm_memory_embeddings SET sync_status = 'synced', last_sync_at = $1 WHERE memory_id = $2`
		_, err := s.provider.Exec(ctx, query, now, id)
		if err != nil {
			RecordSyncError(ctx)
			return fmt.Errorf("failed to update sync status for id %s: %w", id, err)
		}
	}

	RecordSyncSuccess(ctx, len(ids))
	return nil
}

func (s *defaultRAGSyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	if s.provider.IsSQLite() {
		return fmt.Errorf("ProcessIncomingSync is only supported in Cloud PostgreSQL mode")
	}

	if len(records) == 0 {
		return nil
	}

	for _, rec := range records {
		query := `
			INSERT INTO swarm_memory_embeddings (memory_id, context, sync_status, last_sync_at)
			VALUES ($1, $2, 'synced', $3)
			ON CONFLICT (memory_id) DO UPDATE SET context = EXCLUDED.context, sync_status = 'synced', last_sync_at = EXCLUDED.last_sync_at
		`
		_, err := s.provider.Exec(ctx, query, rec.ID, rec.Context, rec.LastSyncAt)
		if err != nil {
			RecordSyncError(ctx)
			return fmt.Errorf("failed to upsert incoming sync for id %s: %w", rec.ID, err)
		}
		RecordSyncSuccess(ctx, 1)
	}

	return nil
}
