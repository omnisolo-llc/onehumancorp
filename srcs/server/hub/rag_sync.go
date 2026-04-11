package hub

import (
	"context"
	"encoding/json"
	"fmt"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
	"github.com/onehumancorp/mono/srcs/server/telemetry"
)

type SyncStatus string

const (
	SyncStatusPending SyncStatus = "pending"
	SyncStatusSynced  SyncStatus = "synced"
	SyncStatusError   SyncStatus = "error"
)

type RAGSyncRecord struct {
	ID         string
	Context    string
	Vector     []float32
	SyncStatus SyncStatus
	LastSyncAt time.Time
}

type RAGSyncService interface {
	FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error)
	MarkSynced(ctx context.Context, ids []string) error
	ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error
}

type ragSyncServiceImpl struct {
	provider db.Provider
}

func NewRAGSyncService(provider db.Provider) RAGSyncService {
	return &ragSyncServiceImpl{
		provider: provider,
	}
}

func (s *ragSyncServiceImpl) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	query := `SELECT id, content, embedding, sync_status, last_sync_timestamp
			  FROM autodream_memories
			  WHERE sync_status = 'pending' LIMIT $1`
	if s.provider.IsSQLite() {
		query = `SELECT id, content, embedding, sync_status, last_sync_timestamp
				 FROM autodream_memories
				 WHERE sync_status = 'pending' LIMIT ?`
	}

	rows, err := s.provider.Query(ctx, query, limit)
	if err != nil {
		return nil, fmt.Errorf("failed to fetch pending syncs: %w", err)
	}
	defer rows.Close()

	var records []RAGSyncRecord
	for rows.Next() {
		var rec RAGSyncRecord
		var vecStr *string
		var status *string
		var lastSync *time.Time
		if err := rows.Scan(&rec.ID, &rec.Context, &vecStr, &status, &lastSync); err != nil {
			return nil, fmt.Errorf("failed to scan sync record: %w", err)
		}

		if vecStr != nil && *vecStr != "" {
			if err := json.Unmarshal([]byte(*vecStr), &rec.Vector); err != nil {
				// Log but continue
				fmt.Printf("Warning: failed to unmarshal vector for record %s: %v\n", rec.ID, err)
			}
		}
		if status != nil {
			rec.SyncStatus = SyncStatus(*status)
		} else {
			rec.SyncStatus = SyncStatusPending
		}
		if lastSync != nil {
			rec.LastSyncAt = *lastSync
		}

		records = append(records, rec)
	}
	if err := rows.Err(); err != nil {
		return nil, fmt.Errorf("rows iteration error: %w", err)
	}

	return records, nil
}

func (s *ragSyncServiceImpl) MarkSynced(ctx context.Context, ids []string) error {
	if len(ids) == 0 {
		return nil
	}

	tx, err := s.provider.Begin(ctx)
	if err != nil {
		telemetry.RecordRagSyncError(ctx)
		return fmt.Errorf("failed to begin tx: %w", err)
	}
	defer tx.Rollback(ctx)

	query := `UPDATE autodream_memories
			  SET sync_status = 'synced', last_sync_timestamp = CURRENT_TIMESTAMP
			  WHERE id = $1`
	if s.provider.IsSQLite() {
		query = `UPDATE autodream_memories
				 SET sync_status = 'synced', last_sync_timestamp = CURRENT_TIMESTAMP
				 WHERE id = ?`
	}

	for _, id := range ids {
		if _, err := tx.Exec(ctx, query, id); err != nil {
			telemetry.RecordRagSyncError(ctx)
			return fmt.Errorf("failed to update sync status for id %s: %w", id, err)
		}
	}

	if err := tx.Commit(ctx); err != nil {
		telemetry.RecordRagSyncError(ctx)
		return fmt.Errorf("failed to commit tx: %w", err)
	}

	telemetry.RecordRagRecordsSynced(ctx, len(ids))
	return nil
}

func (s *ragSyncServiceImpl) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	if len(records) == 0 {
		return nil
	}

	tx, err := s.provider.Begin(ctx)
	if err != nil {
		telemetry.RecordRagSyncError(ctx)
		return fmt.Errorf("failed to begin tx: %w", err)
	}
	defer tx.Rollback(ctx)

	queryPostgres := `INSERT INTO autodream_memories (id, content, embedding, sync_status, last_sync_timestamp)
					  VALUES ($1, $2, CASE WHEN $3::text IS NULL THEN NULL ELSE $3::text::vector END, 'synced', $4)
					  ON CONFLICT (id) DO UPDATE SET
					  content = EXCLUDED.content,
					  embedding = EXCLUDED.embedding,
					  sync_status = 'synced',
					  last_sync_timestamp = EXCLUDED.last_sync_timestamp`

	querySQLite := `INSERT INTO autodream_memories (id, content, embedding, sync_status, last_sync_timestamp)
					VALUES (?, ?, ?, 'synced', ?)
					ON CONFLICT (id) DO UPDATE SET
					content = excluded.content,
					embedding = excluded.embedding,
					sync_status = 'synced',
					last_sync_timestamp = excluded.last_sync_timestamp`

	for _, rec := range records {
		var vecStr *string
		if len(rec.Vector) > 0 {
			b, err := json.Marshal(rec.Vector)
			if err != nil {
				telemetry.RecordRagSyncError(ctx)
				return fmt.Errorf("failed to marshal vector: %w", err)
			}
			s := string(b)
			vecStr = &s
		}

		if s.provider.IsSQLite() {
			if _, err := tx.Exec(ctx, querySQLite, rec.ID, rec.Context, vecStr, rec.LastSyncAt); err != nil {
				telemetry.RecordRagSyncError(ctx)
				return fmt.Errorf("failed to process incoming sync for id %s: %w", rec.ID, err)
			}
		} else {
			if _, err := tx.Exec(ctx, queryPostgres, rec.ID, rec.Context, vecStr, rec.LastSyncAt); err != nil {
				telemetry.RecordRagSyncError(ctx)
				return fmt.Errorf("failed to process incoming sync for id %s: %w", rec.ID, err)
			}
		}
	}

	if err := tx.Commit(ctx); err != nil {
		telemetry.RecordRagSyncError(ctx)
		return fmt.Errorf("failed to commit tx: %w", err)
	}

	telemetry.RecordRagRecordsSynced(ctx, len(records))
	return nil
}
