package hub

import (
	"context"
	"encoding/json"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
)

type RAGSyncServiceImpl struct {
	provider db.Provider
}

func NewRAGSyncService(provider db.Provider) RAGSyncService {
	return &RAGSyncServiceImpl{provider: provider}
}

func (s *RAGSyncServiceImpl) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	query := `SELECT id, content, CAST(embedding AS TEXT), sync_status, last_sync_at FROM autodream_memories WHERE sync_status = $1 LIMIT $2`
	rows, err := s.provider.Query(ctx, query, string(SyncStatusPending), limit)
	if err != nil {
		return nil, err
	}
	defer rows.Close()

	var records []RAGSyncRecord
	for rows.Next() {
		var r RAGSyncRecord
		var embeddingStr *string
		var lastSyncAt *time.Time
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
	return records, rows.Err()
}

func (s *RAGSyncServiceImpl) MarkSynced(ctx context.Context, ids []string) error {
	if len(ids) == 0 {
		return nil
	}
	// Basic implementation for SQLite/PG: use simple transaction and loop
	// (or construct IN clause, but this is simpler and dialect-agnostic)
	tx, err := s.provider.Begin(ctx)
	if err != nil {
		return err
	}
	defer tx.Rollback(ctx)

	now := time.Now()
	for _, id := range ids {
		_, err := tx.Exec(ctx, `UPDATE autodream_memories SET sync_status = $1, last_sync_at = $2 WHERE id = $3`, string(SyncStatusSynced), now, id)
		if err != nil {
			return err
		}
	}

	if err := tx.Commit(ctx); err != nil {
		return err
	}

	ragRecordsSyncedTotal.Add(ctx, int64(len(ids)))
	return nil
}

func (s *RAGSyncServiceImpl) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	if len(records) == 0 {
		return nil
	}
	tx, err := s.provider.Begin(ctx)
	if err != nil {
		return err
	}
	defer tx.Rollback(ctx)

	for _, r := range records {
		var embeddingStr *string
		if r.Vector != nil {
			b, _ := json.Marshal(r.Vector)
			s := string(b)
			embeddingStr = &s
		}

		query := `INSERT INTO autodream_memories (id, content, embedding, sync_status, last_sync_at)
			VALUES ($1, $2, $3, $4, $5)
			ON CONFLICT (id) DO UPDATE SET
			content = EXCLUDED.content,
			embedding = EXCLUDED.embedding,
			sync_status = EXCLUDED.sync_status,
			last_sync_at = EXCLUDED.last_sync_at`

		if !s.provider.IsSQLite() {
			// PG supports ON CONFLICT
			// But wait, the table might not have embedding cast in EXCLUDED.
			// Let's dynamically add the vector cast if it's PG
			query = `INSERT INTO autodream_memories (id, content, embedding, sync_status, last_sync_at)
				VALUES ($1, $2, CASE WHEN $3::text IS NULL THEN NULL ELSE $3::text::vector END, $4, $5)
				ON CONFLICT (id) DO UPDATE SET
				content = EXCLUDED.content,
				embedding = EXCLUDED.embedding,
				sync_status = EXCLUDED.sync_status,
				last_sync_at = EXCLUDED.last_sync_at`
		} else {
			// SQLite UPSERT syntax
			query = `INSERT INTO autodream_memories (id, content, embedding, sync_status, last_sync_at)
			VALUES ($1, $2, $3, $4, $5)
			ON CONFLICT (id) DO UPDATE SET
			content = EXCLUDED.content,
			embedding = EXCLUDED.embedding,
			sync_status = EXCLUDED.sync_status,
			last_sync_at = EXCLUDED.last_sync_at`
		}

		now := time.Now()
		_, err := tx.Exec(ctx, query, r.ID, r.Context, embeddingStr, string(SyncStatusSynced), now)
		if err != nil {
			ragRecordsSyncErrorsTotal.Add(ctx, 1)
			return err
		}
	}

	if err := tx.Commit(ctx); err != nil {
		return err
	}
	ragRecordsSyncedTotal.Add(ctx, int64(len(records)))
	return nil
}
