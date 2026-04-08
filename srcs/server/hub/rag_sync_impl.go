package hub

import (
	"context"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
)

type DefaultRAGSyncService struct {
	pool db.Provider
}

func NewRAGSyncService(pool db.Provider) RAGSyncService {
	return &DefaultRAGSyncService{pool: pool}
}

func (s *DefaultRAGSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	query := `
		SELECT id, content, sync_status, last_sync_at
		FROM autodream_memories
		WHERE sync_status = $1
		LIMIT $2
	`
	// SQLite specific placeholder adjustments might be needed depending on db wrapper,
	// Assuming db wrapper handles $1, $2 transparently or using ? based on pool.IsSQLite()
	if s.pool.IsSQLite() {
		query = `
			SELECT id, content, sync_status, last_sync_at
			FROM autodream_memories
			WHERE sync_status = ?
			LIMIT ?
		`
	}

	rows, err := s.pool.Query(ctx, query, SyncStatusPending, limit)
	if err != nil {
		RagSyncErrorsTotal.Add(ctx, 1)
		return nil, err
	}
	defer rows.Close()

	var records []RAGSyncRecord
	for rows.Next() {
		var rec RAGSyncRecord
		var lastSyncAt *time.Time
		var syncStatusStr string
		err := rows.Scan(&rec.ID, &rec.Context, &syncStatusStr, &lastSyncAt)
		if err != nil {
			continue
		}
		rec.SyncStatus = SyncStatus(syncStatusStr)
		if lastSyncAt != nil {
			rec.LastSyncAt = *lastSyncAt
		}
		records = append(records, rec)
	}

	return records, rows.Err()
}

func (s *DefaultRAGSyncService) MarkSynced(ctx context.Context, ids []string) error {
	if len(ids) == 0 {
		return nil
	}

	tx, err := s.pool.Begin(ctx)
	if err != nil {
		RagSyncErrorsTotal.Add(ctx, 1)
		return err
	}
	defer tx.Rollback(ctx)

	query := `UPDATE autodream_memories SET sync_status = $1, last_sync_at = $2 WHERE id = $3`
	if s.pool.IsSQLite() {
		query = `UPDATE autodream_memories SET sync_status = ?, last_sync_at = ? WHERE id = ?`
	}

	for _, id := range ids {
		_, err := tx.Exec(ctx, query, SyncStatusSynced, time.Now(), id)
		if err != nil {
			RagSyncErrorsTotal.Add(ctx, 1)
			return err
		}
	}

	if err := tx.Commit(ctx); err != nil {
		RagSyncErrorsTotal.Add(ctx, 1)
		return err
	}

	RagRecordsSyncedTotal.Add(ctx, int64(len(ids)))
	return nil
}

func (s *DefaultRAGSyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	if len(records) == 0 {
		return nil
	}

	tx, err := s.pool.Begin(ctx)
	if err != nil {
		RagSyncErrorsTotal.Add(ctx, 1)
		return err
	}
	defer tx.Rollback(ctx)

	// In cloud Postgres we might upsert the record and vector
	query := `
		INSERT INTO autodream_memories (id, content, sync_status, last_sync_at)
		VALUES ($1, $2, $3, $4)
		ON CONFLICT (id) DO UPDATE SET
			content = EXCLUDED.content,
			sync_status = EXCLUDED.sync_status,
			last_sync_at = EXCLUDED.last_sync_at
	`
	if s.pool.IsSQLite() {
		query = `
			INSERT INTO autodream_memories (id, content, sync_status, last_sync_at)
			VALUES (?, ?, ?, ?)
			ON CONFLICT (id) DO UPDATE SET
				content = EXCLUDED.content,
				sync_status = EXCLUDED.sync_status,
				last_sync_at = EXCLUDED.last_sync_at
		`
	}

	for _, rec := range records {
		_, err := tx.Exec(ctx, query, rec.ID, rec.Context, rec.SyncStatus, rec.LastSyncAt)
		if err != nil {
			RagSyncErrorsTotal.Add(ctx, 1)
			return err
		}
	}

	if err := tx.Commit(ctx); err != nil {
		RagSyncErrorsTotal.Add(ctx, 1)
		return err
	}

	return nil
}
