package hub

import (
	"context"
	"fmt"
	"strings"
	"time"
	"encoding/json"

	"github.com/onehumancorp/mono/srcs/server/db"
	"go.opentelemetry.io/otel/metric"
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
	query := `
		SELECT id, content, embedding, sync_status, last_sync_at
		FROM autodream_memories
		WHERE sync_status = 'pending' OR sync_status IS NULL
		ORDER BY created_at ASC
		LIMIT $1
	`
	rows, err := s.dbProvider.Query(ctx, query, limit)
	if err != nil {
		ragSyncErrorsTotal.Add(ctx, 1, metric.WithAttributes())
		return nil, fmt.Errorf("failed to fetch pending syncs: %w", err)
	}
	defer rows.Close()

	var records []RAGSyncRecord
	for rows.Next() {
		var rec RAGSyncRecord
		var lastSyncAt *time.Time
		var syncStatus *string
		var embeddingStr *string

		if err := rows.Scan(&rec.ID, &rec.Context, &embeddingStr, &syncStatus, &lastSyncAt); err != nil {
			ragSyncErrorsTotal.Add(ctx, 1, metric.WithAttributes())
			return nil, fmt.Errorf("failed to scan sync record: %w", err)
		}

		if syncStatus != nil {
			rec.SyncStatus = SyncStatus(*syncStatus)
		} else {
			rec.SyncStatus = SyncStatusPending
		}

		if lastSyncAt != nil {
			rec.LastSyncAt = *lastSyncAt
		}

		if embeddingStr != nil {
			// Convert string to []float32 for vector
			var vec []float32
			if err := json.Unmarshal([]byte(*embeddingStr), &vec); err == nil {
				rec.Vector = vec
			}
		}

		records = append(records, rec)
	}

	if err := rows.Err(); err != nil {
		ragSyncErrorsTotal.Add(ctx, 1, metric.WithAttributes())
		return nil, fmt.Errorf("row error during fetch pending syncs: %w", err)
	}

	return records, nil
}

func (s *ragSyncServiceImpl) MarkSynced(ctx context.Context, ids []string) error {
	if len(ids) == 0 {
		return nil
	}

	placeholders := make([]string, len(ids))
	args := make([]interface{}, len(ids)+1)
	args[0] = time.Now()

	for i, id := range ids {
		placeholders[i] = fmt.Sprintf("$%d", i+2)
		args[i+1] = id
	}

	query := fmt.Sprintf(`
		UPDATE autodream_memories
		SET sync_status = 'synced', last_sync_at = $1
		WHERE id IN (%s)
	`, strings.Join(placeholders, ","))

	_, err := s.dbProvider.Exec(ctx, query, args...)
	if err != nil {
		ragSyncErrorsTotal.Add(ctx, 1, metric.WithAttributes())
		return fmt.Errorf("failed to mark synced: %w", err)
	}

	ragRecordsSyncedTotal.Add(ctx, int64(len(ids)), metric.WithAttributes())

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

	for _, rec := range records {
		var embeddingStr *string
		if rec.Vector != nil {
			b, err := json.Marshal(rec.Vector)
			if err == nil {
				str := string(b)
				embeddingStr = &str
			}
		}

		// Use Last-Write-Wins (LWW) or simple upsert logic
		query := `
			INSERT INTO autodream_memories (id, content, embedding, sync_status, last_sync_at)
			VALUES ($1, $2, $3, 'synced', $4)
			ON CONFLICT (id) DO UPDATE SET
				content = EXCLUDED.content,
				embedding = EXCLUDED.embedding,
				sync_status = 'synced',
				last_sync_at = EXCLUDED.last_sync_at
		`

		_, err := tx.Exec(ctx, query, rec.ID, rec.Context, embeddingStr, rec.LastSyncAt)
		if err != nil {
			ragSyncErrorsTotal.Add(ctx, 1, metric.WithAttributes())
			return fmt.Errorf("failed to upsert incoming sync record: %w", err)
		}
	}

	if err := tx.Commit(ctx); err != nil {
		return fmt.Errorf("failed to commit transaction: %w", err)
	}

	ragRecordsSyncedTotal.Add(ctx, int64(len(records)), metric.WithAttributes())

	return nil
}
