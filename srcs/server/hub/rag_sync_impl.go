package hub

import (
	"context"
	"encoding/json"
	"fmt"
	"database/sql"

	"github.com/jackc/pgx/v5/pgxpool"
	"github.com/onehumancorp/mono/srcs/server/db"
	"github.com/onehumancorp/mono/srcs/server/telemetry"
)

type ragSyncServiceImpl struct {
	pool     *pgxpool.Pool
	provider db.Provider
}

// NewRAGSyncService creates a new RAG sync service.
func NewRAGSyncService(pool *pgxpool.Pool, provider db.Provider) RAGSyncService {
	return &ragSyncServiceImpl{
		pool:     pool,
		provider: provider,
	}
}

func (s *ragSyncServiceImpl) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	if s.provider == nil {
		return nil, fmt.Errorf("database provider is not set")
	}

	// Must select vector. In SQLite we might store it as text, in Postgres as vector type.
	// Cast to TEXT for generic fetching from both.
	query := `
		SELECT id, content, CAST(embedding AS TEXT), sync_status, last_sync_at
		FROM autodream_memories
		WHERE sync_status = 'pending'
		LIMIT $1`

	rows, err := s.provider.Query(ctx, query, limit)
	if err != nil {
		return nil, fmt.Errorf("failed to fetch pending syncs: %w", err)
	}
	defer rows.Close()

	var records []RAGSyncRecord
	for rows.Next() {
		var r RAGSyncRecord
		var lastSync sql.NullTime
		var vectorStr sql.NullString
		if err := rows.Scan(&r.ID, &r.Context, &vectorStr, &r.SyncStatus, &lastSync); err != nil {
			return nil, fmt.Errorf("failed to scan record: %w", err)
		}

		if vectorStr.Valid && vectorStr.String != "" {
			var vec []float32
			if err := json.Unmarshal([]byte(vectorStr.String), &vec); err == nil {
				r.Vector = vec
			}
		}

		if lastSync.Valid {
			r.LastSyncAt = lastSync.Time
		}
		records = append(records, r)
	}

	return records, nil
}

func (s *ragSyncServiceImpl) MarkSynced(ctx context.Context, ids []string) error {
	if s.provider == nil {
		return fmt.Errorf("database provider is not set")
	}

	if len(ids) == 0 {
		return nil
	}

	// Start transaction
	tx, err := s.provider.Begin(ctx)
	if err != nil {
		telemetry.RecordRAGSyncErrorsTotal(ctx, 1)
		return fmt.Errorf("failed to begin transaction: %w", err)
	}
	defer tx.Rollback(ctx)

	for _, id := range ids {
		query := `UPDATE autodream_memories SET sync_status = 'synced', last_sync_at = CURRENT_TIMESTAMP WHERE id = $1`
		_, err := tx.Exec(ctx, query, id)
		if err != nil {
			telemetry.RecordRAGSyncErrorsTotal(ctx, 1)
			return fmt.Errorf("failed to mark synced for id %s: %w", id, err)
		}
	}

	if err := tx.Commit(ctx); err != nil {
		telemetry.RecordRAGSyncErrorsTotal(ctx, 1)
		return fmt.Errorf("failed to commit transaction: %w", err)
	}

	telemetry.RecordRAGRecordsSyncedTotal(ctx, len(ids))
	return nil
}

func (s *ragSyncServiceImpl) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	if s.pool == nil {
		return fmt.Errorf("cloud database pool is not set")
	}

	if len(records) == 0 {
		return nil
	}

	tx, err := s.pool.Begin(ctx)
	if err != nil {
		telemetry.RecordRAGSyncErrorsTotal(ctx, 1)
		return fmt.Errorf("failed to begin transaction: %w", err)
	}
	defer tx.Rollback(ctx)

	for _, r := range records {
		var vecStr sql.NullString
		if len(r.Vector) > 0 {
			b, _ := json.Marshal(r.Vector)
			vecStr = sql.NullString{String: string(b), Valid: true}
		}

		query := `
			INSERT INTO autodream_memories (id, content, embedding, sync_status, last_sync_at)
			VALUES ($1, $2, CASE WHEN $3::text IS NULL THEN NULL ELSE $3::text::vector END, 'synced', CURRENT_TIMESTAMP)
			ON CONFLICT (id) DO UPDATE SET
				content = EXCLUDED.content,
				embedding = CASE WHEN EXCLUDED.embedding IS NULL THEN autodream_memories.embedding ELSE EXCLUDED.embedding END,
				sync_status = 'synced',
				last_sync_at = CURRENT_TIMESTAMP
		`
		var vecParam interface{} = nil
		if vecStr.Valid {
			vecParam = vecStr.String
		}
		_, err := tx.Exec(ctx, query, r.ID, r.Context, vecParam)
		if err != nil {
			telemetry.RecordRAGSyncErrorsTotal(ctx, 1)
			return fmt.Errorf("failed to process incoming sync for id %s: %w", r.ID, err)
		}
	}

	if err := tx.Commit(ctx); err != nil {
		telemetry.RecordRAGSyncErrorsTotal(ctx, 1)
		return fmt.Errorf("failed to commit transaction: %w", err)
	}

	telemetry.RecordRAGRecordsSyncedTotal(ctx, len(records))
	return nil
}
