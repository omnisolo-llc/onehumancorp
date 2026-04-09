package hub

import (
	"context"
	"fmt"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
	"github.com/onehumancorp/mono/srcs/server/telemetry"
)

// DBRAGSyncService implements RAGSyncService using the standard database connection.
type DBRAGSyncService struct {
	db *db.DB
}

// NewDBRAGSyncService creates a new DBRAGSyncService.
func NewDBRAGSyncService(db *db.DB) *DBRAGSyncService {
	return &DBRAGSyncService{db: db}
}

func (s *DBRAGSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	rows, err := s.db.Query(ctx, "SELECT memory_id, context, sync_status, last_sync_at FROM swarm_memory_embeddings WHERE sync_status = 'pending' LIMIT $1", limit)
	if err != nil {
		return nil, fmt.Errorf("failed to fetch pending syncs: %w", err)
	}
	defer rows.Close()

	var records []RAGSyncRecord
	for rows.Next() {
		var r RAGSyncRecord
		var lastSyncAt *time.Time
		if err := rows.Scan(&r.ID, &r.Context, &r.SyncStatus, &lastSyncAt); err != nil {
			return nil, fmt.Errorf("failed to scan record: %w", err)
		}
		if lastSyncAt != nil {
			r.LastSyncAt = *lastSyncAt
		}
		records = append(records, r)
	}

	return records, nil
}

func (s *DBRAGSyncService) MarkSynced(ctx context.Context, ids []string) error {
	if len(ids) == 0 {
		return nil
	}

	now := time.Now()
	for _, id := range ids {
		_, err := s.db.Exec(ctx, "UPDATE swarm_memory_embeddings SET sync_status = 'synced', last_sync_at = $1 WHERE memory_id = $2", now, id)
		if err != nil {
			telemetry.RecordRAGSyncError(ctx, 1)
			return fmt.Errorf("failed to update record %s: %w", id, err)
		}
	}

	telemetry.RecordRAGSyncSuccess(ctx, int64(len(ids)))
	return nil
}

func (s *DBRAGSyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	now := time.Now()
	for _, r := range records {
		_, err := s.db.Exec(ctx, "INSERT INTO swarm_memory_embeddings (memory_id, context, sync_status, last_sync_at) VALUES ($1, $2, 'synced', $3) ON CONFLICT(memory_id) DO UPDATE SET context = excluded.context, sync_status = 'synced', last_sync_at = excluded.last_sync_at", r.ID, r.Context, now)
		if err != nil {
			telemetry.RecordRAGSyncError(ctx, 1)
			return fmt.Errorf("failed to process incoming record %s: %w", r.ID, err)
		}
	}

	telemetry.RecordRAGSyncSuccess(ctx, int64(len(records)))
	return nil
}
