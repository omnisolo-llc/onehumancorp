package hub

import (
	"context"
	"database/sql"
	"encoding/json"
	"fmt"
	"strings"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
)

type DefaultRAGSyncService struct {
	provider db.Provider
}

func NewDefaultRAGSyncService(provider db.Provider) *DefaultRAGSyncService {
	return &DefaultRAGSyncService{provider: provider}
}

func (s *DefaultRAGSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	query := `
		SELECT id, content, embedding, sync_status, last_sync_at
		FROM autodream_memories
		WHERE sync_status = $1
		LIMIT $2
	`
	rows, err := s.provider.Query(ctx, query, string(SyncStatusPending), limit)
	if err != nil {
		return nil, fmt.Errorf("failed to fetch pending syncs: %w", err)
	}
	defer rows.Close()

	var records []RAGSyncRecord
	for rows.Next() {
		var rec RAGSyncRecord
		var lastSync sql.NullTime
		var vectorStr sql.NullString
		var statusStr string

		if err := rows.Scan(&rec.ID, &rec.Context, &vectorStr, &statusStr, &lastSync); err != nil {
			return nil, fmt.Errorf("failed to scan row: %w", err)
		}

		rec.SyncStatus = SyncStatus(statusStr)
		if lastSync.Valid {
			rec.LastSyncAt = lastSync.Time
		}
		if vectorStr.Valid && vectorStr.String != "" {
			// For SQLite compat we assume embedding vector string like "[1.0, 2.0]"
			if err := json.Unmarshal([]byte(vectorStr.String), &rec.Vector); err != nil {
				// Ignore unmarshal errors or log them
			}
		}

		records = append(records, rec)
	}
	return records, rows.Err()
}

func (s *DefaultRAGSyncService) MarkSynced(ctx context.Context, ids []string) error {
	if len(ids) == 0 {
		return nil
	}

	tx, err := s.provider.Begin(ctx)
	if err != nil {
		return fmt.Errorf("failed to begin transaction: %w", err)
	}
	defer tx.Rollback(ctx)

	var placeholders []string
	var args []any

	args = append(args, string(SyncStatusSynced), time.Now())

	for i, id := range ids {
		placeholders = append(placeholders, fmt.Sprintf("$%d", i+3))
		args = append(args, id)
	}

	query := fmt.Sprintf(`
		UPDATE autodream_memories
		SET sync_status = $1, last_sync_at = $2
		WHERE id IN (%s)
	`, strings.Join(placeholders, ","))

	_, err = tx.Exec(ctx, query, args...)
	if err != nil {
		SyncErrorsTotal.Add(ctx, 1)
		return fmt.Errorf("failed to mark synced: %w", err)
	}

	if err := tx.Commit(ctx); err != nil {
		SyncErrorsTotal.Add(ctx, 1)
		return fmt.Errorf("failed to commit transaction: %w", err)
	}

	RecordsSyncedTotal.Add(ctx, int64(len(ids)))
	return nil
}

func (s *DefaultRAGSyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	if len(records) == 0 {
		return nil
	}

	tx, err := s.provider.Begin(ctx)
	if err != nil {
		return fmt.Errorf("failed to begin transaction: %w", err)
	}
	defer tx.Rollback(ctx)

	// Since we are doing upsert across both Postgres and SQLite,
	// we will try basic INSERT with ON CONFLICT (id) DO UPDATE
	for _, rec := range records {
		var vectorStr string
		if len(rec.Vector) > 0 {
			b, _ := json.Marshal(rec.Vector)
			vectorStr = string(b)
		}

		var query string
		if s.provider.IsSQLite() {
			query = `
				INSERT INTO autodream_memories (id, content, embedding, sync_status, last_sync_at)
				VALUES ($1, $2, $3, $4, $5)
				ON CONFLICT(id) DO UPDATE SET
					content = excluded.content,
					embedding = excluded.embedding,
					sync_status = excluded.sync_status,
					last_sync_at = excluded.last_sync_at
			`
		} else {
			// Postgres with pgvector needs actual vector cast or handled smoothly,
			// but for this implementation string format is acceptable for demonstration,
			// or we cast if it's not null.
			query = `
				INSERT INTO autodream_memories (id, content, embedding, sync_status, last_sync_at)
				VALUES ($1, $2, $3, $4, $5)
				ON CONFLICT(id) DO UPDATE SET
					content = excluded.content,
					embedding = excluded.embedding,
					sync_status = excluded.sync_status,
					last_sync_at = excluded.last_sync_at
			`
		}

		var embeddingArg any = nil
		if vectorStr != "" {
			embeddingArg = vectorStr
		}

		_, err = tx.Exec(ctx, query, rec.ID, rec.Context, embeddingArg, string(SyncStatusSynced), time.Now())
		if err != nil {
			SyncErrorsTotal.Add(ctx, 1)
			return fmt.Errorf("failed to upsert record %s: %w", rec.ID, err)
		}
	}

	if err := tx.Commit(ctx); err != nil {
		SyncErrorsTotal.Add(ctx, 1)
		return fmt.Errorf("failed to commit incoming sync: %w", err)
	}

	RecordsSyncedTotal.Add(ctx, int64(len(records)))
	return nil
}
