package hub

import (
	"context"
	"encoding/json"
	"fmt"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
	"github.com/onehumancorp/mono/srcs/server/telemetry"
)

type ragSyncService struct {
	provider db.Provider
}

// NewRAGSyncService creates a new instance of RAGSyncService.
func NewRAGSyncService(provider db.Provider) RAGSyncService {
	return &ragSyncService{
		provider: provider,
	}
}

func (s *ragSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	query := `
		SELECT id, content, embedding, sync_status, last_sync_at
		FROM autodream_memories
		WHERE sync_status = $1
		LIMIT $2
	`

	if s.provider.IsSQLite() {
		query = `
			SELECT id, content, embedding, sync_status, last_sync_at
			FROM autodream_memories
			WHERE sync_status = ?
			LIMIT ?
		`
	}

	rows, err := s.provider.Query(ctx, query, string(SyncStatusPending), limit)
	if err != nil {
		telemetry.RecordRagSyncError(ctx)
		return nil, fmt.Errorf("failed to fetch pending syncs: %w", err)
	}
	defer rows.Close()

	var records []RAGSyncRecord
	for rows.Next() {
		var r RAGSyncRecord
		var syncStatus string
		var lastSyncAt *time.Time
		var embeddingStr *string

		if err := rows.Scan(&r.ID, &r.Context, &embeddingStr, &syncStatus, &lastSyncAt); err != nil {
			telemetry.RecordRagSyncError(ctx)
			return nil, fmt.Errorf("failed to scan pending sync record: %w", err)
		}

		r.SyncStatus = SyncStatus(syncStatus)
		if lastSyncAt != nil {
			r.LastSyncAt = *lastSyncAt
		}

		if embeddingStr != nil {
			if err := json.Unmarshal([]byte(*embeddingStr), &r.Vector); err != nil {
				// Handle potential vector formatting issues if postgres returns unparseable
				// Try falling back if necessary or just skip if it's purely for sync transit.
				// In sqlite, we store it as json string.
				telemetry.RecordRagSyncError(ctx)
				// we might want to log this but continue syncing the context.
			}
		}

		records = append(records, r)
	}

	if err := rows.Err(); err != nil {
		telemetry.RecordRagSyncError(ctx)
		return nil, fmt.Errorf("error iterating pending syncs: %w", err)
	}

	return records, nil
}

func (s *ragSyncService) MarkSynced(ctx context.Context, ids []string) error {
	if len(ids) == 0 {
		return nil
	}

	// For simplicity, execute in a transaction or a loop, loop is simpler for multi-db
	tx, err := s.provider.Begin(ctx)
	if err != nil {
		telemetry.RecordRagSyncError(ctx)
		return fmt.Errorf("failed to begin transaction: %w", err)
	}
	defer tx.Rollback(ctx)

	updateQuery := `UPDATE autodream_memories SET sync_status = $1, last_sync_at = CURRENT_TIMESTAMP WHERE id = $2`
	if s.provider.IsSQLite() {
		updateQuery = `UPDATE autodream_memories SET sync_status = ?, last_sync_at = CURRENT_TIMESTAMP WHERE id = ?`
	}

	for _, id := range ids {
		_, err := tx.Exec(ctx, updateQuery, string(SyncStatusSynced), id)
		if err != nil {
			telemetry.RecordRagSyncError(ctx)
			return fmt.Errorf("failed to mark record as synced (%s): %w", id, err)
		}
	}

	if err := tx.Commit(ctx); err != nil {
		telemetry.RecordRagSyncError(ctx)
		return fmt.Errorf("failed to commit transaction: %w", err)
	}

	telemetry.RecordRagRecordSynced(ctx, int64(len(ids)))
	return nil
}

func (s *ragSyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	if len(records) == 0 {
		return nil
	}

	tx, err := s.provider.Begin(ctx)
	if err != nil {
		telemetry.RecordRagSyncError(ctx)
		return fmt.Errorf("failed to begin transaction for incoming sync: %w", err)
	}
	defer tx.Rollback(ctx)

	insertQuery := `
		INSERT INTO autodream_memories (id, content, embedding, sync_status, last_sync_at)
		VALUES ($1, $2, CASE WHEN $3::text IS NULL THEN NULL ELSE $3::text::vector END, $4, $5)
		ON CONFLICT (id) DO UPDATE SET
			content = EXCLUDED.content,
			embedding = EXCLUDED.embedding,
			sync_status = EXCLUDED.sync_status,
			last_sync_at = EXCLUDED.last_sync_at
	`
	if s.provider.IsSQLite() {
		insertQuery = `
			INSERT INTO autodream_memories (id, content, embedding, sync_status, last_sync_at)
			VALUES (?, ?, ?, ?, ?)
			ON CONFLICT (id) DO UPDATE SET
				content = EXCLUDED.content,
				embedding = EXCLUDED.embedding,
				sync_status = EXCLUDED.sync_status,
				last_sync_at = EXCLUDED.last_sync_at
		`
	}

	for _, record := range records {
		var embeddingStr *string
		if len(record.Vector) > 0 {
			embBytes, err := json.Marshal(record.Vector)
			if err != nil {
				telemetry.RecordRagSyncError(ctx)
				return fmt.Errorf("failed to marshal vector for incoming sync: %w", err)
			}
			s := string(embBytes)
			embeddingStr = &s
		}

		if s.provider.IsSQLite() {
			_, err = tx.Exec(ctx, insertQuery, record.ID, record.Context, embeddingStr, string(SyncStatusSynced), record.LastSyncAt)
		} else {
			_, err = tx.Exec(ctx, insertQuery, record.ID, record.Context, embeddingStr, string(SyncStatusSynced), record.LastSyncAt)
		}

		if err != nil {
			telemetry.RecordRagSyncError(ctx)
			return fmt.Errorf("failed to process incoming sync record (%s): %w", record.ID, err)
		}
	}

	if err := tx.Commit(ctx); err != nil {
		telemetry.RecordRagSyncError(ctx)
		return fmt.Errorf("failed to commit incoming sync transaction: %w", err)
	}

	return nil
}
