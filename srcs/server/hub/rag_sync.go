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
	Vector     []float32 // Convert to string internally for SQLite compat if needed
	SyncStatus SyncStatus
	LastSyncAt time.Time
}

type RAGSyncService interface {
	FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error)
	MarkSynced(ctx context.Context, ids []string) error
	ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error
}

type DatabaseRAGSyncService struct {
	provider db.Provider
}

func NewDatabaseRAGSyncService(provider db.Provider) RAGSyncService {
	return &DatabaseRAGSyncService{provider: provider}
}

func (s *DatabaseRAGSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	if s.provider == nil {
		return nil, fmt.Errorf("database provider is nil")
	}

	query := `
        SELECT id, content, sync_status, last_sync_at
        FROM autodream_memories
        WHERE sync_status = 'pending'
        LIMIT $1
    `
	rows, err := s.provider.Query(ctx, query, limit)
	if err != nil {
		if telemetry.RagSyncErrorsTotal != nil {
			telemetry.RagSyncErrorsTotal.Add(ctx, 1)
		}
		return nil, fmt.Errorf("failed to fetch pending syncs: %w", err)
	}
	defer rows.Close()

	var records []RAGSyncRecord
	for rows.Next() {
		var rec RAGSyncRecord
		var lastSyncAt *time.Time
		var status *string
		if err := rows.Scan(&rec.ID, &rec.Context, &status, &lastSyncAt); err != nil {
			continue
		}
		if status != nil {
			rec.SyncStatus = SyncStatus(*status)
		} else {
			rec.SyncStatus = SyncStatusPending
		}
		if lastSyncAt != nil {
			rec.LastSyncAt = *lastSyncAt
		}
		records = append(records, rec)
	}
	return records, nil
}

func (s *DatabaseRAGSyncService) MarkSynced(ctx context.Context, ids []string) error {
	if s.provider == nil {
		return fmt.Errorf("database provider is nil")
	}

	if len(ids) == 0 {
		return nil
	}

	for _, id := range ids {
		_, err := s.provider.Exec(ctx, "UPDATE autodream_memories SET sync_status = 'synced', last_sync_at = CURRENT_TIMESTAMP WHERE id = $1", id)
		if err != nil {
			if telemetry.RagSyncErrorsTotal != nil {
				telemetry.RagSyncErrorsTotal.Add(ctx, 1)
			}
			return fmt.Errorf("failed to mark synced for id %s: %w", id, err)
		}
	}

	if telemetry.RagRecordsSyncedTotal != nil {
		telemetry.RagRecordsSyncedTotal.Add(ctx, int64(len(ids)))
	}

	return nil
}

func (s *DatabaseRAGSyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	if s.provider == nil {
		return fmt.Errorf("database provider is nil")
	}

	for _, rec := range records {
		if s.provider.IsSQLite() {
			_, err := s.provider.Exec(ctx, `
                INSERT INTO autodream_memories (id, content, sync_status, last_sync_at)
                VALUES ($1, $2, 'synced', CURRENT_TIMESTAMP)
                ON CONFLICT(id) DO UPDATE SET
                    content = excluded.content,
                    sync_status = 'synced',
                    last_sync_at = CURRENT_TIMESTAMP
            `, rec.ID, rec.Context)

			if err != nil {
				if telemetry.RagSyncErrorsTotal != nil {
					telemetry.RagSyncErrorsTotal.Add(ctx, 1)
				}
				return fmt.Errorf("failed to upsert record %s in sqlite: %w", rec.ID, err)
			}
		} else {
			_, err := s.provider.Exec(ctx, `
                INSERT INTO autodream_memories (id, content, sync_status, last_sync_at)
                VALUES ($1, $2, 'synced', CURRENT_TIMESTAMP)
                ON CONFLICT(id) DO UPDATE SET
                    content = EXCLUDED.content,
                    sync_status = 'synced',
                    last_sync_at = CURRENT_TIMESTAMP
            `, rec.ID, rec.Context)

			if err != nil {
				if telemetry.RagSyncErrorsTotal != nil {
					telemetry.RagSyncErrorsTotal.Add(ctx, 1)
				}
				return fmt.Errorf("failed to upsert record %s in pg: %w", rec.ID, err)
			}
		}
	}

	return nil
}
