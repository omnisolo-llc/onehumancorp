package hub

import (
	"context"
	"fmt"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
	"github.com/onehumancorp/mono/srcs/server/telemetry"
)

type SyncStatus string

const (
	SyncStatusPending SyncStatus = "pending"
	SyncStatusSynced  SyncStatus = "synced"
	SyncStatusError   SyncStatus = "error"
)

type RAGSyncRecord struct {
	ID         string
	Context    string
	Vector     []float32 // Currently not fully mapped in Go struct to avoid pgvector/sqlite diffs
	SyncStatus SyncStatus
	LastSyncAt time.Time
}

type RAGSyncService interface {
	// FetchPendingSyncs retrieves records from the local DB that need syncing
	FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error)

	// MarkSynced updates the local DB after a successful sync to the cloud
	MarkSynced(ctx context.Context, ids []string) error

	// ProcessIncomingSync handles data pushed from a standalone client into the cloud DB
	ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error
}

type DefaultRAGSyncService struct {
	provider db.Provider
}

func NewDefaultRAGSyncService(provider db.Provider) *DefaultRAGSyncService {
	return &DefaultRAGSyncService{
		provider: provider,
	}
}

func (s *DefaultRAGSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	query := `
		SELECT id, content, sync_status, last_sync_at
		FROM autodream_memories
		WHERE sync_status = 'pending' OR sync_status IS NULL
		LIMIT $1
	`
	rows, err := s.provider.Query(ctx, query, limit)
	if err != nil {
		return nil, fmt.Errorf("failed to fetch pending syncs: %w", err)
	}
	defer rows.Close()

	var records []RAGSyncRecord
	for rows.Next() {
		var rec RAGSyncRecord
		var lastSyncAt *time.Time
		var syncStatus *string
		if err := rows.Scan(&rec.ID, &rec.Context, &syncStatus, &lastSyncAt); err != nil {
			return nil, fmt.Errorf("failed to scan record: %w", err)
		}

		if syncStatus != nil {
			rec.SyncStatus = SyncStatus(*syncStatus)
		} else {
			rec.SyncStatus = SyncStatusPending
		}

		if lastSyncAt != nil {
			rec.LastSyncAt = *lastSyncAt
		}

		records = append(records, rec)
	}

	if err := rows.Err(); err != nil {
		return nil, fmt.Errorf("rows error: %w", err)
	}

	return records, nil
}

func (s *DefaultRAGSyncService) MarkSynced(ctx context.Context, ids []string) error {
	if len(ids) == 0 {
		return nil
	}

	// For standard SQLite/Postgres compatibility we iterate.
	// In a real high-throughput scenario, we would use an IN clause or unnest.
	tx, err := s.provider.Begin(ctx)
	if err != nil {
		return err
	}
	defer tx.Rollback(ctx)

	query := `
		UPDATE autodream_memories
		SET sync_status = 'synced', last_sync_at = CURRENT_TIMESTAMP
		WHERE id = $1
	`

	for _, id := range ids {
		if _, err := tx.Exec(ctx, query, id); err != nil {
			return fmt.Errorf("failed to update record %s: %w", id, err)
		}
	}

	if err := tx.Commit(ctx); err != nil {
		return fmt.Errorf("failed to commit transaction: %w", err)
	}

	if telemetry.RagRecordsSyncedTotal != nil {
		telemetry.RagRecordsSyncedTotal.Add(ctx, int64(len(ids)))
	}

	return nil
}

func (s *DefaultRAGSyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	if len(records) == 0 {
		return nil
	}

	tx, err := s.provider.Begin(ctx)
	if err != nil {
		return err
	}
	defer tx.Rollback(ctx)

	// We use ON CONFLICT DO UPDATE for both Postgres and SQLite.
	// Postgres uses ON CONFLICT (id), SQLite also supports ON CONFLICT (id).
	query := `
		INSERT INTO autodream_memories (id, content, sync_status, last_sync_at)
		VALUES ($1, $2, $3, $4)
		ON CONFLICT(id) DO UPDATE SET
			content = EXCLUDED.content,
			sync_status = 'synced',
			last_sync_at = CURRENT_TIMESTAMP
	`

	for _, rec := range records {
		_, err := tx.Exec(ctx, query, rec.ID, rec.Context, "synced", time.Now())
		if err != nil {
			return fmt.Errorf("failed to upsert incoming record %s: %w", rec.ID, err)
		}
	}

	if err := tx.Commit(ctx); err != nil {
		return fmt.Errorf("failed to commit incoming sync: %w", err)
	}

	if telemetry.RagRecordsSyncedTotal != nil {
		telemetry.RagRecordsSyncedTotal.Add(ctx, int64(len(records)))
	}

	return nil
}
