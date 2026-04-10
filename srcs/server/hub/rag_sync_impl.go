package hub

import (
	"context"
	"fmt"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
)

type ragSyncService struct {
	db db.Provider
}

func NewRAGSyncService(database db.Provider) RAGSyncService {
	return &ragSyncService{
		db: database,
	}
}

func (s *ragSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	query := `
		SELECT id, content, sync_status, last_sync_at
		FROM autodream_memories
		WHERE sync_status = 'pending' OR sync_status IS NULL
		ORDER BY created_at ASC
		LIMIT $1
	`
	rows, err := s.db.Query(ctx, query, limit)
	if err != nil {
		syncErrors.Add(ctx, 1)
		return nil, fmt.Errorf("failed to fetch pending syncs: %w", err)
	}
	defer rows.Close()

	var records []RAGSyncRecord
	for rows.Next() {
		var rec RAGSyncRecord
		var status *string
		var lastSync *time.Time
		if err := rows.Scan(&rec.ID, &rec.Context, &status, &lastSync); err != nil {
			syncErrors.Add(ctx, 1)
			return nil, fmt.Errorf("failed to scan record: %w", err)
		}
		if status != nil {
			rec.SyncStatus = SyncStatus(*status)
		} else {
			rec.SyncStatus = SyncStatusPending
		}
		if lastSync != nil {
			rec.LastSyncAt = *lastSync
		}
		records = append(records, rec)
	}
	if err := rows.Err(); err != nil {
		syncErrors.Add(ctx, 1)
		return nil, fmt.Errorf("rows error: %w", err)
	}

	return records, nil
}

func (s *ragSyncService) MarkSynced(ctx context.Context, ids []string) error {
	if len(ids) == 0 {
		return nil
	}

	// This is a naive implementation that does N queries.
	// For production, this should use a transaction and an IN clause.
	now := time.Now()
	for _, id := range ids {
		query := `
			UPDATE autodream_memories
			SET sync_status = 'synced', last_sync_at = $1
			WHERE id = $2
		`
		_, err := s.db.Exec(ctx, query, now, id)
		if err != nil {
			syncErrors.Add(ctx, 1)
			return fmt.Errorf("failed to mark synced for id %s: %w", id, err)
		}
		recordsSynced.Add(ctx, 1)
	}

	return nil
}

func (s *ragSyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	if len(records) == 0 {
		return nil
	}

	for _, rec := range records {
		var count int
		err := s.db.QueryRow(ctx, "SELECT count(1) FROM autodream_memories WHERE id = $1", rec.ID).Scan(&count)
		if err != nil {
			syncErrors.Add(ctx, 1)
			return fmt.Errorf("failed to check record %s: %w", rec.ID, err)
		}

		if count > 0 {
			query := `
				UPDATE autodream_memories
				SET content = $1, sync_status = 'synced', last_sync_at = $2
				WHERE id = $3
			`
			_, err = s.db.Exec(ctx, query, rec.Context, time.Now(), rec.ID)
			if err != nil {
				syncErrors.Add(ctx, 1)
				return fmt.Errorf("failed to update record %s: %w", rec.ID, err)
			}
		} else {
			query := `
				INSERT INTO autodream_memories (id, content, sync_status, last_sync_at, organization_id, agent_id, source_type)
				VALUES ($1, $2, 'synced', $3, 'default', 'sync', 'cloud')
			`
			_, err = s.db.Exec(ctx, query, rec.ID, rec.Context, time.Now())
			if err != nil {
				syncErrors.Add(ctx, 1)
				return fmt.Errorf("failed to insert record %s: %w", rec.ID, err)
			}
		}
		recordsSynced.Add(ctx, 1)
	}

	return nil
}
