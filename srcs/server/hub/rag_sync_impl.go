package hub

import (
	"context"
	"database/sql"
	"encoding/json"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
	"github.com/onehumancorp/mono/srcs/server/telemetry"
)

type RAGSyncServiceImpl struct {
	db db.Provider
}

func NewRAGSyncServiceImpl(provider db.Provider) *RAGSyncServiceImpl {
	return &RAGSyncServiceImpl{db: provider}
}

func (s *RAGSyncServiceImpl) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {


	query := `
		SELECT id, content, embedding, sync_status, last_sync_at
		FROM autodream_memories
		WHERE sync_status = 'pending'
		LIMIT $1
	`
	rows, err := s.db.Query(ctx, query, limit)
	if err != nil {
		return nil, err
	}
	defer rows.Close()

	var records []RAGSyncRecord
	for rows.Next() {
		var rec RAGSyncRecord
		var embeddingStr sql.NullString
		var syncStatus sql.NullString
		var lastSyncAt sql.NullTime

		if err := rows.Scan(&rec.ID, &rec.Context, &embeddingStr, &syncStatus, &lastSyncAt); err != nil {
			return nil, err
		}

		if embeddingStr.Valid && embeddingStr.String != "" {
			var vec []float32
			if err := json.Unmarshal([]byte(embeddingStr.String), &vec); err != nil {
				// We can ignore unmarshal errors here and proceed with nil vector
				rec.Vector = nil
			} else {
				rec.Vector = vec
			}
		}

		if syncStatus.Valid {
			rec.SyncStatus = SyncStatus(syncStatus.String)
		} else {
			rec.SyncStatus = SyncStatusPending
		}

		if lastSyncAt.Valid {
			rec.LastSyncAt = lastSyncAt.Time
		}

		records = append(records, rec)
	}

	if err = rows.Err(); err != nil {
		return nil, err
	}

	return records, nil
}

func (s *RAGSyncServiceImpl) MarkSynced(ctx context.Context, ids []string) error {
	if len(ids) == 0 {
		return nil
	}

	tx, err := s.db.Begin(ctx)
	if err != nil {
		return err
	}
	defer tx.Rollback(ctx)

	query := `
		UPDATE autodream_memories
		SET sync_status = 'synced', last_sync_at = $1
		WHERE id = $2
	`

	now := time.Now()
	successCount := 0
	for _, id := range ids {
		rowsAff, err := tx.Exec(ctx, query, now, id)
		if err != nil {
			telemetry.RecordRagSyncErrorsTotal(ctx, 1)
			return err
		}
		successCount += int(rowsAff)
	}

	if err := tx.Commit(ctx); err != nil {
		return err
	}

	if successCount > 0 {
		telemetry.RecordRagRecordsSyncedTotal(ctx, int64(successCount))
	}

	return nil
}

func (s *RAGSyncServiceImpl) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	if len(records) == 0 {
		return nil
	}

	tx, err := s.db.Begin(ctx)
	if err != nil {
		return err
	}
	defer tx.Rollback(ctx)

	// Handle Conflict using Last-Write-Wins (LWW) mechanism if possible.
	// Since ON CONFLICT is standard across Postgre/SQLite, we'll use an UPSERT style query.
	query := `
		INSERT INTO autodream_memories (id, content, embedding, sync_status, last_sync_at)
		VALUES ($1, $2, $3, $4, $5)
		ON CONFLICT(id) DO UPDATE SET
			content = excluded.content,
			embedding = excluded.embedding,
			sync_status = excluded.sync_status,
			last_sync_at = excluded.last_sync_at
	`

	for _, rec := range records {
		var embeddingVal interface{}
		if len(rec.Vector) > 0 {
			b, err := json.Marshal(rec.Vector)
			if err == nil {
				embeddingVal = string(b)
			}
		}

		var lastSyncAtVal interface{}
		if !rec.LastSyncAt.IsZero() {
			lastSyncAtVal = rec.LastSyncAt
		}

		_, err = tx.Exec(ctx, query, rec.ID, rec.Context, embeddingVal, "synced", lastSyncAtVal)
		if err != nil {
			return err
		}
	}

	return tx.Commit(ctx)
}
