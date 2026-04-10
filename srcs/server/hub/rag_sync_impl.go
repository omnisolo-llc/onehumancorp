package hub

import (
	"context"
	"fmt"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
	"github.com/onehumancorp/mono/srcs/server/telemetry"
)

type ragSyncServiceImpl struct {
	provider db.Provider
}

// NewRAGSyncService creates a new RAGSyncService instance.
func NewRAGSyncService(provider db.Provider) RAGSyncService {
	return &ragSyncServiceImpl{
		provider: provider,
	}
}

func (s *ragSyncServiceImpl) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	// Only Standalone Mode fetches pending syncs to push to cloud
	if !s.provider.IsSQLite() {
		return nil, fmt.Errorf("FetchPendingSyncs is only supported in Standalone Mode")
	}

	query := `
		SELECT key, value, sync_status, last_sync_at
		FROM swarm_memory
		WHERE sync_status = $1
		LIMIT $2
	`
	rows, err := s.provider.Query(ctx, query, string(SyncStatusPending), limit)
	if err != nil {
		return nil, fmt.Errorf("failed to fetch pending syncs: %w", err)
	}
	defer rows.Close()

	var records []RAGSyncRecord
	for rows.Next() {
		var r RAGSyncRecord
		var lastSyncAt *time.Time
		var statusStr string
		if err := rows.Scan(&r.ID, &r.Context, &statusStr, &lastSyncAt); err != nil {
			return nil, fmt.Errorf("failed to scan row: %w", err)
		}
		r.SyncStatus = SyncStatus(statusStr)
		if lastSyncAt != nil {
			r.LastSyncAt = *lastSyncAt
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

	tx, err := s.provider.Begin(ctx)
	if err != nil {
		return fmt.Errorf("failed to begin transaction: %w", err)
	}
	defer tx.Rollback(ctx)

	for _, id := range ids {
		query := `
			UPDATE swarm_memory
			SET sync_status = $1, last_sync_at = $2
			WHERE key = $3
		`
		_, err := tx.Exec(ctx, query, string(SyncStatusSynced), time.Now(), id)
		if err != nil {
			telemetry.RecordRagSyncError(ctx)
			return fmt.Errorf("failed to update sync status for id %s: %w", id, err)
		}
		telemetry.RecordRagRecordSynced(ctx)
	}

	if err := tx.Commit(ctx); err != nil {
		return fmt.Errorf("failed to commit transaction: %w", err)
	}

	return nil
}

func (s *ragSyncServiceImpl) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	// Only Cloud Mode accepts incoming syncs
	if s.provider.IsSQLite() {
		return fmt.Errorf("ProcessIncomingSync is only supported in Cloud Mode")
	}

	tx, err := s.provider.Begin(ctx)
	if err != nil {
		return fmt.Errorf("failed to begin transaction: %w", err)
	}
	defer tx.Rollback(ctx)

	for _, r := range records {
		query := `
			INSERT INTO swarm_memory (key, value, sync_status, last_sync_at)
			VALUES ($1, $2, $3, $4)
			ON CONFLICT (key) DO UPDATE
			SET value = EXCLUDED.value,
				sync_status = EXCLUDED.sync_status,
				last_sync_at = EXCLUDED.last_sync_at
		`
		_, err := tx.Exec(ctx, query, r.ID, r.Context, string(SyncStatusSynced), time.Now())
		if err != nil {
			telemetry.RecordRagSyncError(ctx)
			return fmt.Errorf("failed to upsert record id %s: %w", r.ID, err)
		}
		telemetry.RecordRagRecordSynced(ctx)
	}

	if err := tx.Commit(ctx); err != nil {
		return fmt.Errorf("failed to commit transaction: %w", err)
	}

	return nil
}
