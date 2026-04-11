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
	Vector     []float32 // Convert to string internally for SQLite compat if needed
	SyncStatus SyncStatus
	LastSyncAt time.Time
}

type RAGSyncService interface {
	// FetchPendingSyncs retrieves records from the local DB that need syncing
	FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error)

	// MarkSynced updates the local DB after a successful sync to the cloud
	MarkSynced(ctx context.Context, ids []string) error

	// ProcessIncomingSync handles data pushed from a standalone client into the cloud DB
	ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error
}

type ragSyncService struct {
	provider db.Provider
}

// NewRAGSyncService creates a new RAGSyncService
func NewRAGSyncService(provider db.Provider) RAGSyncService {
	return &ragSyncService{provider: provider}
}

func (s *ragSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	query := `
		SELECT id, content, embedding, sync_status, last_sync_at
		FROM autodream_memories
		WHERE sync_status = 'pending'
		LIMIT $1
	`
	rows, err := s.provider.Query(ctx, query, limit)
	if err != nil {
		telemetry.RecordRagSyncError(ctx)
		return nil, fmt.Errorf("failed to fetch pending syncs: %w", err)
	}
	defer rows.Close()

	var records []RAGSyncRecord
	for rows.Next() {
		var r RAGSyncRecord
		var vectorStr *string
		var lastSyncAt *time.Time
		if err := rows.Scan(&r.ID, &r.Context, &vectorStr, &r.SyncStatus, &lastSyncAt); err != nil {
			telemetry.RecordRagSyncError(ctx)
			return nil, fmt.Errorf("failed to scan pending sync record: %w", err)
		}
		if lastSyncAt != nil {
			r.LastSyncAt = *lastSyncAt
		}
		if vectorStr != nil {
			var vector []float32
			if err := json.Unmarshal([]byte(*vectorStr), &vector); err == nil {
				r.Vector = vector
			}
		}
		records = append(records, r)
	}
	if err := rows.Err(); err != nil {
		telemetry.RecordRagSyncError(ctx)
		return nil, fmt.Errorf("row error when fetching pending syncs: %w", err)
	}

	return records, nil
}

func (s *ragSyncService) MarkSynced(ctx context.Context, ids []string) error {
	if len(ids) == 0 {
		return nil
	}

	tx, err := s.provider.Begin(ctx)
	if err != nil {
		telemetry.RecordRagSyncError(ctx)
		return fmt.Errorf("failed to begin transaction: %w", err)
	}
	defer tx.Rollback(ctx)

	now := time.Now()
	for _, id := range ids {
		query := `
			UPDATE autodream_memories
			SET sync_status = 'synced', last_sync_at = $1
			WHERE id = $2
		`
		if _, err := tx.Exec(ctx, query, now, id); err != nil {
			telemetry.RecordRagSyncError(ctx)
			return fmt.Errorf("failed to mark record %s as synced: %w", id, err)
		}
		telemetry.RecordRagRecordSynced(ctx)
	}

	if err := tx.Commit(ctx); err != nil {
		telemetry.RecordRagSyncError(ctx)
		return fmt.Errorf("failed to commit synced records: %w", err)
	}

	return nil
}

func (s *ragSyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	if len(records) == 0 {
		return nil
	}

	tx, err := s.provider.Begin(ctx)
	if err != nil {
		telemetry.RecordRagSyncError(ctx)
		return fmt.Errorf("failed to begin transaction: %w", err)
	}
	defer tx.Rollback(ctx)

	for _, r := range records {
		var vectorStr interface{} = nil
		if len(r.Vector) > 0 {
			b, err := json.Marshal(r.Vector)
			if err == nil {
				vectorStr = string(b)
			}
		}

		query := ""
		if s.provider.IsSQLite() {
			query = `
				INSERT INTO autodream_memories (id, content, embedding, sync_status, last_sync_at)
				VALUES ($1, $2, $3, 'synced', $4)
				ON CONFLICT(id) DO UPDATE SET
					content = excluded.content,
					embedding = excluded.embedding,
					sync_status = excluded.sync_status,
					last_sync_at = excluded.last_sync_at
			`
		} else {
			query = `
				INSERT INTO autodream_memories (id, content, embedding, sync_status, last_sync_at)
				VALUES ($1, $2, CASE WHEN $3::text IS NULL THEN NULL ELSE $3::text::vector END, 'synced', $4)
				ON CONFLICT(id) DO UPDATE SET
					content = excluded.content,
					embedding = excluded.embedding,
					sync_status = excluded.sync_status,
					last_sync_at = excluded.last_sync_at
			`
		}

		if _, err := tx.Exec(ctx, query, r.ID, r.Context, vectorStr, r.LastSyncAt); err != nil {
			telemetry.RecordRagSyncError(ctx)
			return fmt.Errorf("failed to upsert incoming record %s: %w", r.ID, err)
		}
		telemetry.RecordRagRecordSynced(ctx)
	}

	if err := tx.Commit(ctx); err != nil {
		telemetry.RecordRagSyncError(ctx)
		return fmt.Errorf("failed to commit incoming syncs: %w", err)
	}

	return nil
}
