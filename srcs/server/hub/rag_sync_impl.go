package hub

import (
	"context"
	"encoding/json"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
	"github.com/onehumancorp/mono/srcs/server/telemetry"
)

type ragSyncServiceImpl struct {
	dbProvider db.Provider
}

func NewRAGSyncService(dbProvider db.Provider) RAGSyncService {
	return &ragSyncServiceImpl{
		dbProvider: dbProvider,
	}
}

func (s *ragSyncServiceImpl) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	rows, err := s.dbProvider.Query(ctx, "SELECT id, content, embedding, sync_status, last_sync_at FROM autodream_memories WHERE sync_status = 'pending' LIMIT $1", limit)
	if err != nil {
		return nil, err
	}
	defer rows.Close()

	var records []RAGSyncRecord
	for rows.Next() {
		var r RAGSyncRecord
		var lastSyncAt *time.Time
		var embeddingStr *string
		if err := rows.Scan(&r.ID, &r.Context, &embeddingStr, &r.SyncStatus, &lastSyncAt); err != nil {
			return nil, err
		}
		if embeddingStr != nil {
			var vec []float32
			if err := json.Unmarshal([]byte(*embeddingStr), &vec); err == nil {
				r.Vector = vec
			}
		}
		if lastSyncAt != nil {
			r.LastSyncAt = *lastSyncAt
		}
		records = append(records, r)
	}
	return records, nil
}

func (s *ragSyncServiceImpl) MarkSynced(ctx context.Context, ids []string) error {
	if len(ids) == 0 {
		return nil
	}

	tx, err := s.dbProvider.Begin(ctx)
	if err != nil {
		return err
	}
	defer tx.Rollback(ctx)

	for _, id := range ids {
		_, err := tx.Exec(ctx, "UPDATE autodream_memories SET sync_status = 'synced', last_sync_at = $1 WHERE id = $2", time.Now().UTC(), id)
		if err != nil {
			return err
		}
	}

	if err := tx.Commit(ctx); err != nil {
		return err
	}

	if telemetry.SyncCompletedCount != nil {
		telemetry.SyncCompletedCount.Add(ctx, int64(len(ids)))
	}

	return nil
}

func (s *ragSyncServiceImpl) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	if len(records) == 0 {
		return nil
	}

	tx, err := s.dbProvider.Begin(ctx)
	if err != nil {
		return err
	}
	defer tx.Rollback(ctx)

	var errs int64

	for _, r := range records {
		var vecStr *string
		if r.Vector != nil {
			b, err := json.Marshal(r.Vector)
			if err != nil {
				errs++
				continue
			}
			str := string(b)
			vecStr = &str
		}

		// Use ON CONFLICT DO UPDATE to handle hybrid setup properly
		// To handle SQLite TEXT types without Postgres ::text::vector cast
		// we'll pass parameters. SQLite and Postgres both accept this basic query format,
		// except postgres requires explicit casting for vectors on input if parameterized without type.
		// However, dbProvider might be able to handle it if we use standard insert,
		// but since we read memory rule: "In shared database queries for Hybrid architecture (SQLite & PostgreSQL), avoid PostgreSQL-specific extension casts like CAST($n AS VECTOR) during inserts/upserts."

		_, err := tx.Exec(ctx, `
			INSERT INTO autodream_memories (id, content, embedding, sync_status, last_sync_at)
			VALUES ($1, $2, $3, $4, $5)
			ON CONFLICT (id) DO UPDATE SET
				content = EXCLUDED.content,
				embedding = EXCLUDED.embedding,
				sync_status = EXCLUDED.sync_status,
				last_sync_at = EXCLUDED.last_sync_at
		`, r.ID, r.Context, vecStr, r.SyncStatus, r.LastSyncAt)

		if err != nil {
			errs++
		}
	}

	if err := tx.Commit(ctx); err != nil {
		return err
	}

	if telemetry.SyncFailedCount != nil {
		telemetry.SyncFailedCount.Add(ctx, errs)
	}
	if telemetry.SyncCompletedCount != nil {
		telemetry.SyncCompletedCount.Add(ctx, int64(len(records))-errs)
	}

	return nil
}
