package hub

import (
	"context"
	"database/sql"
	"encoding/json"
	"errors"
	"fmt"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
)

type defaultRAGSyncService struct {
	db db.Provider
}

// NewRAGSyncService creates a new RAGSyncService backed by the given db Provider.
func NewRAGSyncService(db db.Provider) RAGSyncService {
	return &defaultRAGSyncService{
		db: db,
	}
}

func (s *defaultRAGSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	query := `
		SELECT id, content, CAST(embedding AS TEXT), sync_status, last_sync_timestamp
		FROM autodream_memories
		WHERE sync_status = $1
		LIMIT $2`

	rows, err := s.db.Query(ctx, query, string(SyncStatusPending), limit)
	if err != nil {
		SyncErrorsTotal.Add(ctx, 1)
		return nil, fmt.Errorf("failed to fetch pending syncs: %w", err)
	}
	defer rows.Close()

	var records []RAGSyncRecord
	for rows.Next() {
		var r RAGSyncRecord
		var embeddingStr *string
		var lastSync *time.Time

		err := rows.Scan(&r.ID, &r.Context, &embeddingStr, &r.SyncStatus, &lastSync)
		if err != nil {
			SyncErrorsTotal.Add(ctx, 1)
			return nil, fmt.Errorf("failed to scan record: %w", err)
		}

		if lastSync != nil {
			r.LastSyncAt = *lastSync
		}

		// Try to parse the embedding string back to float array if possible.
		// PostgreSQL with pgvector might return it differently, but for SQLite it's string.
		if embeddingStr != nil && *embeddingStr != "" {
			var vec []float32
			if err := json.Unmarshal([]byte(*embeddingStr), &vec); err == nil {
				r.Vector = vec
			}
		}

		records = append(records, r)
	}

	if err := rows.Err(); err != nil {
		SyncErrorsTotal.Add(ctx, 1)
		return nil, fmt.Errorf("rows error: %w", err)
	}

	return records, nil
}

func (s *defaultRAGSyncService) MarkSynced(ctx context.Context, ids []string) error {
	if len(ids) == 0 {
		return nil
	}

	tx, err := s.db.Begin(ctx)
	if err != nil {
		SyncErrorsTotal.Add(ctx, 1)
		return fmt.Errorf("failed to begin transaction: %w", err)
	}
	defer tx.Rollback(ctx)

	for _, id := range ids {
		query := `UPDATE autodream_memories SET sync_status = $1, last_sync_timestamp = $2 WHERE id = $3`
		_, err := tx.Exec(ctx, query, string(SyncStatusSynced), time.Now(), id)
		if err != nil {
			SyncErrorsTotal.Add(ctx, 1)
			return fmt.Errorf("failed to update record %s: %w", id, err)
		}
	}

	if err := tx.Commit(ctx); err != nil {
		SyncErrorsTotal.Add(ctx, 1)
		return fmt.Errorf("failed to commit transaction: %w", err)
	}

	RecordsSyncedTotal.Add(ctx, int64(len(ids)))
	return nil
}

func (s *defaultRAGSyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	if len(records) == 0 {
		return nil
	}

	tx, err := s.db.Begin(ctx)
	if err != nil {
		SyncErrorsTotal.Add(ctx, 1)
		return fmt.Errorf("failed to begin transaction: %w", err)
	}
	defer tx.Rollback(ctx)

	for _, r := range records {
		var embeddingStr string
		if len(r.Vector) > 0 {
			b, err := json.Marshal(r.Vector)
			if err == nil {
				embeddingStr = string(b)
			}
		} else {
			// fallback if empty
			embeddingStr = "[]"
		}

		// UPSERT approach: check if exists, then update or insert.
		// For simplicity and to avoid specific dialect ON CONFLICT handling,
		// we first query to see if it exists.
		var exists int
		err := tx.QueryRow(ctx, "SELECT 1 FROM autodream_memories WHERE id = $1", r.ID).Scan(&exists)

		if errors.Is(err, sql.ErrNoRows) {
			// Doesn't exist, insert
			query := `
				INSERT INTO autodream_memories (id, content, embedding, sync_status, last_sync_timestamp)
				VALUES ($1, $2, $3, $4, $5)`

			// Note: embedding is string. PostgreSQL would need CAST if it was typed as vector,
			// but we handle it transparently as text per the constraints for SQLite compatibility.
			_, err = tx.Exec(ctx, query, r.ID, r.Context, embeddingStr, string(SyncStatusSynced), time.Now())
			if err != nil {
				SyncErrorsTotal.Add(ctx, 1)
				return fmt.Errorf("failed to insert record %s: %w", r.ID, err)
			}
		} else if err != nil {
			SyncErrorsTotal.Add(ctx, 1)
			return fmt.Errorf("failed to check existence for record %s: %w", r.ID, err)
		} else {
			// Exists, update
			query := `
				UPDATE autodream_memories
				SET content = $1, embedding = $2, sync_status = $3, last_sync_timestamp = $4
				WHERE id = $5`
			_, err = tx.Exec(ctx, query, r.Context, embeddingStr, string(SyncStatusSynced), time.Now(), r.ID)
			if err != nil {
				SyncErrorsTotal.Add(ctx, 1)
				return fmt.Errorf("failed to update record %s: %w", r.ID, err)
			}
		}
	}

	if err := tx.Commit(ctx); err != nil {
		SyncErrorsTotal.Add(ctx, 1)
		return fmt.Errorf("failed to commit transaction: %w", err)
	}

	RecordsSyncedTotal.Add(ctx, int64(len(records)))
	return nil
}
