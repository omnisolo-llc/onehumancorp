package hub

import (
	"context"
	"fmt"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
)

type ragSyncServiceImpl struct {
	provider db.Provider
}

func NewRAGSyncService(provider db.Provider) RAGSyncService {
	return &ragSyncServiceImpl{
		provider: provider,
	}
}

func (s *ragSyncServiceImpl) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	conn := s.provider

	query := `
		SELECT id, content, sync_status, last_sync_at
		FROM autodream_memories
		WHERE sync_status = 'pending'
		LIMIT $1
	`
	rows, err := conn.Query(ctx, query, limit)
	if err != nil {
		return nil, fmt.Errorf("failed to query pending syncs: %w", err)
	}
	defer rows.Close()

	var records []RAGSyncRecord
	for rows.Next() {
		var record RAGSyncRecord
		var lastSyncAt *time.Time
		var vectorStr *string // Not fetching vector currently to simplify mapping

		if err := rows.Scan(&record.ID, &record.Context, &record.SyncStatus, &lastSyncAt); err != nil {
			return nil, fmt.Errorf("failed to scan record: %w", err)
		}
		if lastSyncAt != nil {
			record.LastSyncAt = *lastSyncAt
		}

		// To keep it simple, we don't map the vector, since we just want to prove the concept.
		_ = vectorStr

		records = append(records, record)
	}

	if err := rows.Err(); err != nil {
		return nil, fmt.Errorf("rows error: %w", err)
	}

	return records, nil
}

func (s *ragSyncServiceImpl) MarkSynced(ctx context.Context, ids []string) error {
	if len(ids) == 0 {
		return nil
	}

	conn := s.provider

    now := time.Now()
	// Using a simple loop for SQLite compatibility instead of a complex ANY($1) array parameter
    // which might not work well across pgx and SQLite drivers seamlessly without a query builder.
    for _, id := range ids {
        query := `
            UPDATE autodream_memories
            SET sync_status = 'synced', last_sync_at = $1
            WHERE id = $2
        `
        _, err := conn.Exec(ctx, query, now, id)
        if err != nil {
            return fmt.Errorf("failed to update record %s: %w", id, err)
        }
        recordsSyncedCounter.Add(ctx, 1)
    }

	return nil
}

func (s *ragSyncServiceImpl) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	if len(records) == 0 {
		return nil
	}

	conn := s.provider

    now := time.Now()

    for _, record := range records {
        // Upsert logic for autodream_memories.
        // For PostgreSQL we can use ON CONFLICT (id) DO UPDATE.
        // SQLite also supports ON CONFLICT with UPSERT syntax since 3.24.0.
        query := `
            INSERT INTO autodream_memories (id, content, sync_status, last_sync_at, organization_id, source_type)
            VALUES ($1, $2, 'synced', $3, 'default', 'sync')
            ON CONFLICT(id) DO UPDATE SET
                content = excluded.content,
                sync_status = 'synced',
                last_sync_at = excluded.last_sync_at
        `
        _, err := conn.Exec(ctx, query, record.ID, record.Context, now)
        if err != nil {
            return fmt.Errorf("failed to upsert record %s: %w", record.ID, err)
        }
    }

	return nil
}
