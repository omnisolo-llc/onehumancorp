package hub

import (
	"context"
	"fmt"
	"time"
	"encoding/json"

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
	if !s.provider.IsSQLite() {
		return nil, fmt.Errorf("FetchPendingSyncs should only be called on standalone SQLite")
	}

	query := `
		SELECT key, value, sync_status, last_sync_at
		FROM swarm_memory
		WHERE sync_status = 'pending'
		LIMIT $1
	`
	rows, err := s.provider.Query(ctx, query, limit)
	if err != nil {
		return nil, fmt.Errorf("failed to fetch pending syncs: %w", err)
	}
	defer rows.Close()

	var records []RAGSyncRecord
	for rows.Next() {
		var r RAGSyncRecord
		var lastSync *time.Time
		if err := rows.Scan(&r.ID, &r.Context, &r.SyncStatus, &lastSync); err != nil {
			return nil, fmt.Errorf("failed to scan record: %w", err)
		}
		if lastSync != nil {
			r.LastSyncAt = *lastSync
		}
		records = append(records, r)
	}

	return records, nil
}

func (s *ragSyncServiceImpl) MarkSynced(ctx context.Context, ids []string) error {
	if len(ids) == 0 {
		return nil
	}

	query := `
		UPDATE swarm_memory
		SET sync_status = 'synced', last_sync_at = CURRENT_TIMESTAMP
		WHERE key = $1
	`

	// Executing sequentially to ensure compatibility with SQLite without complex batch statements
	var lastErr error
	for _, id := range ids {
		_, err := s.provider.Exec(ctx, query, id)
		if err != nil {
			lastErr = err
			telemetry.RecordRAGSyncError(ctx)
		} else {
			telemetry.RecordRAGRecordSynced(ctx)
		}
	}

	if lastErr != nil {
		return fmt.Errorf("failed to mark some records as synced: %w", lastErr)
	}
	return nil
}

func (s *ragSyncServiceImpl) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	if s.provider.IsSQLite() {
		return fmt.Errorf("ProcessIncomingSync should only be called on cloud PostgreSQL")
	}

	if len(records) == 0 {
		return nil
	}

	// Last-Write-Wins (LWW) UPSERT approach
	query := `
		INSERT INTO swarm_memory (key, value, sync_status, last_sync_at)
		VALUES ($1, $2, 'synced', CURRENT_TIMESTAMP)
		ON CONFLICT (key) DO UPDATE
		SET value = EXCLUDED.value, sync_status = 'synced', last_sync_at = CURRENT_TIMESTAMP
	`

	var lastErr error
	for _, rec := range records {
		vectorBytes, _ := json.Marshal(rec.Vector)
		// For simplicity we just encode the whole struct into value for context
		valueBytes, _ := json.Marshal(map[string]interface{}{
			"context": rec.Context,
			"vector": vectorBytes,
		})

		_, err := s.provider.Exec(ctx, query, rec.ID, string(valueBytes))
		if err != nil {
			lastErr = err
			telemetry.RecordRAGSyncError(ctx)
		} else {
			telemetry.RecordRAGRecordSynced(ctx)
		}
	}

	if lastErr != nil {
		return fmt.Errorf("failed to process incoming sync: %w", lastErr)
	}
	return nil
}
