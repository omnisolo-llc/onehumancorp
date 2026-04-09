package hub

import (
	"context"
	"fmt"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
)

type ragSyncServiceImpl struct {
	dbProvider db.Provider
}

// NewRAGSyncService creates a new RAGSyncService instance.
func NewRAGSyncService(dbProvider db.Provider) RAGSyncService {
	return &ragSyncServiceImpl{
		dbProvider: dbProvider,
	}
}

func (s *ragSyncServiceImpl) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	if !s.dbProvider.IsSQLite() {
		// Only standalone mode (SQLite) pushes to cloud.
		return []RAGSyncRecord{}, nil
	}

	query := `
		SELECT memory_id, context, sync_status, last_sync_at
		FROM swarm_memory_embeddings
		WHERE sync_status = 'pending'
		LIMIT $1
	`
	rows, err := s.dbProvider.Query(ctx, query, limit)
	if err != nil {
		return nil, fmt.Errorf("failed to fetch pending syncs: %w", err)
	}
	defer rows.Close()

	var records []RAGSyncRecord
	for rows.Next() {
		var r RAGSyncRecord
		var lastSyncAt *time.Time
		var status string
		if err := rows.Scan(&r.ID, &r.Context, &status, &lastSyncAt); err != nil {
			return nil, fmt.Errorf("failed to scan row: %w", err)
		}
		r.SyncStatus = SyncStatus(status)
		if lastSyncAt != nil {
			r.LastSyncAt = *lastSyncAt
		}
		records = append(records, r)
	}

	if err := rows.Err(); err != nil {
		return nil, fmt.Errorf("row iteration error: %w", err)
	}

	return records, nil
}

func (s *ragSyncServiceImpl) MarkSynced(ctx context.Context, ids []string) error {
	if len(ids) == 0 {
		return nil
	}

	tx, err := s.dbProvider.Begin(ctx)
	if err != nil {
		return fmt.Errorf("failed to begin transaction: %w", err)
	}
	defer tx.Rollback(ctx)

	now := time.Now()
	for _, id := range ids {
		query := `
			UPDATE swarm_memory_embeddings
			SET sync_status = 'synced', last_sync_at = $1
			WHERE memory_id = $2
		`
		_, err := tx.Exec(ctx, query, now, id)
		if err != nil {
			RagSyncErrorsTotal.Add(ctx, 1)
			return fmt.Errorf("failed to mark record synced (id: %s): %w", id, err)
		}
		RagRecordsSyncedTotal.Add(ctx, 1)
	}

	if err := tx.Commit(ctx); err != nil {
		return fmt.Errorf("failed to commit transaction: %w", err)
	}

	return nil
}

func (s *ragSyncServiceImpl) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	if len(records) == 0 {
		return nil
	}

	tx, err := s.dbProvider.Begin(ctx)
	if err != nil {
		return fmt.Errorf("failed to begin transaction: %w", err)
	}
	defer tx.Rollback(ctx)

	for _, r := range records {
		// Use a simple ON CONFLICT DO UPDATE or similar depending on the DB
		// In Postgres we can use ON CONFLICT (memory_id) DO UPDATE.
		// For SQLite we can use INSERT OR REPLACE. Since we use `db.Provider` and standard sql,
		// we can try a simple check-then-update or insert, OR we use upsert if supported by both.
		// Since standard upsert syntax may differ, we will do a simple select then insert/update for compatibility.

		var exists bool
		checkQuery := `SELECT EXISTS(SELECT 1 FROM swarm_memory_embeddings WHERE memory_id = $1)`
		err := tx.QueryRow(ctx, checkQuery, r.ID).Scan(&exists)
		if err != nil {
			RagSyncErrorsTotal.Add(ctx, 1)
			return fmt.Errorf("failed to check existence (id: %s): %w", r.ID, err)
		}

		if exists {
			updateQuery := `
				UPDATE swarm_memory_embeddings
				SET context = $1, sync_status = 'synced', last_sync_at = $2
				WHERE memory_id = $3
			`
			_, err = tx.Exec(ctx, updateQuery, r.Context, time.Now(), r.ID)
			if err != nil {
				RagSyncErrorsTotal.Add(ctx, 1)
				return fmt.Errorf("failed to update record (id: %s): %w", r.ID, err)
			}
		} else {
			insertQuery := `
				INSERT INTO swarm_memory_embeddings (memory_id, context, sync_status, last_sync_at)
				VALUES ($1, $2, 'synced', $3)
			`
			_, err = tx.Exec(ctx, insertQuery, r.ID, r.Context, time.Now())
			if err != nil {
				RagSyncErrorsTotal.Add(ctx, 1)
				return fmt.Errorf("failed to insert record (id: %s): %w", r.ID, err)
			}
		}
		RagRecordsSyncedTotal.Add(ctx, 1)
	}

	if err := tx.Commit(ctx); err != nil {
		return fmt.Errorf("failed to commit transaction: %w", err)
	}

	return nil
}
