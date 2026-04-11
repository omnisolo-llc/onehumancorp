package hub

import (
	"context"
	"encoding/json"
	"fmt"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
)

type DatabaseRAGSyncService struct {
	provider db.Provider
}

func NewDatabaseRAGSyncService(provider db.Provider) *DatabaseRAGSyncService {
	return &DatabaseRAGSyncService{provider: provider}
}

func (s *DatabaseRAGSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	query := `
		SELECT id, content, embedding, sync_status, last_sync_at
		FROM autodream_memories
		WHERE sync_status = $1
		LIMIT $2
	`

	// Ensure safe column reading for vector, we cast to TEXT for SQLite vs Postgres difference
	if s.provider.IsSQLite() {
		query = `
			SELECT id, content, CAST(embedding AS TEXT), sync_status, last_sync_at
			FROM autodream_memories
			WHERE sync_status = $1
			LIMIT $2
		`
	} else {
		// For Postgres vector, CAST to text
		query = `
			SELECT id, content, embedding::text, sync_status, last_sync_at
			FROM autodream_memories
			WHERE sync_status = $1
			LIMIT $2
		`
	}

	rows, err := s.provider.Query(ctx, query, string(SyncStatusPending), limit)
	if err != nil {
		return nil, err
	}
	defer rows.Close()

	var records []RAGSyncRecord
	for rows.Next() {
		var rec RAGSyncRecord
		var vectorStr *string
		var statusStr string
		var lastSync *time.Time
		if err := rows.Scan(&rec.ID, &rec.Context, &vectorStr, &statusStr, &lastSync); err != nil {
			return nil, err
		}

		rec.SyncStatus = SyncStatus(statusStr)
		if lastSync != nil {
			rec.LastSyncAt = *lastSync
		}

		if vectorStr != nil && *vectorStr != "" {
			var vec []float32
			if err := json.Unmarshal([]byte(*vectorStr), &vec); err == nil {
				rec.Vector = vec
			}
		}

		records = append(records, rec)
	}

	return records, nil
}

func (s *DatabaseRAGSyncService) MarkSynced(ctx context.Context, ids []string) error {
	if len(ids) == 0 {
		return nil
	}

	tx, err := s.provider.Begin(ctx)
	if err != nil {
		return err
	}
	defer tx.Rollback(ctx)

	// Update records one by one in a transaction to handle both Postgres and SQLite.
	// SQLite doesn't natively support id = ANY($1) array passing without extensions.
	query := `
		UPDATE autodream_memories
		SET sync_status = $1, last_sync_at = $2
		WHERE id = $3
	`

	updatedCount := 0
	now := time.Now()
	for _, id := range ids {
		_, err := tx.Exec(ctx, query, string(SyncStatusSynced), now, id)
		if err != nil {
			RagSyncErrorsTotal.Add(ctx, 1)
			return err
		}
		updatedCount++
	}

	if err := tx.Commit(ctx); err != nil {
		return err
	}

	RagRecordsSyncedTotal.Add(ctx, int64(updatedCount))
	return nil
}

func (s *DatabaseRAGSyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	if len(records) == 0 {
		return nil
	}

	tx, err := s.provider.Begin(ctx)
	if err != nil {
		return err
	}
	defer tx.Rollback(ctx)

	now := time.Now()
	for _, rec := range records {
		var vecStr *string
		if len(rec.Vector) > 0 {
			b, err := json.Marshal(rec.Vector)
			if err == nil {
				s := string(b)
				vecStr = &s
			}
		}

		var query string
		var execErr error
		if s.provider.IsSQLite() {
			query = `
				INSERT INTO autodream_memories (id, content, embedding, sync_status, last_sync_at)
				VALUES ($1, $2, CAST($3 AS TEXT), $4, $5)
				ON CONFLICT (id) DO UPDATE SET
					content = EXCLUDED.content,
					embedding = EXCLUDED.embedding,
					sync_status = EXCLUDED.sync_status,
					last_sync_at = EXCLUDED.last_sync_at
			`
			_, execErr = tx.Exec(ctx, query, rec.ID, rec.Context, vecStr, string(SyncStatusSynced), now)
		} else {
			query = `
				INSERT INTO autodream_memories (id, content, embedding, sync_status, last_sync_at)
				VALUES ($1, $2, $3::vector, $4, $5)
				ON CONFLICT (id) DO UPDATE SET
					content = EXCLUDED.content,
					embedding = EXCLUDED.embedding,
					sync_status = EXCLUDED.sync_status,
					last_sync_at = EXCLUDED.last_sync_at
			`
			_, execErr = tx.Exec(ctx, query, rec.ID, rec.Context, vecStr, string(SyncStatusSynced), now)
		}

		if execErr != nil {
			RagSyncErrorsTotal.Add(ctx, 1)
			return fmt.Errorf("failed to process record %s: %w", rec.ID, execErr)
		}
	}

	if err := tx.Commit(ctx); err != nil {
		return err
	}

	RagRecordsSyncedTotal.Add(ctx, int64(len(records)))
	return nil
}
