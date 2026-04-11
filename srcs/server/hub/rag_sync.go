package hub

import (
	"context"
	"database/sql"
	"encoding/json"
	"errors"
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
	dbProvider db.Provider
}

func NewRAGSyncService(dbProvider db.Provider) RAGSyncService {
	return &ragSyncService{
		dbProvider: dbProvider,
	}
}

func (s *ragSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	query := `
		SELECT id, content, CAST(embedding AS TEXT), sync_status, last_sync_at
		FROM autodream_memories
		WHERE sync_status = 'pending'
		LIMIT $1
	`
	rows, err := s.dbProvider.Query(ctx, query, limit)
	if err != nil {
		if errors.Is(err, sql.ErrNoRows) {
			return []RAGSyncRecord{}, nil
		}
		telemetry.RecordRagSyncError(ctx)
		return nil, fmt.Errorf("failed to query pending syncs: %w", err)
	}
	defer rows.Close()

	var records []RAGSyncRecord
	for rows.Next() {
		var r RAGSyncRecord
		var lastSyncAt sql.NullTime
		var vectorStr *string

		if err := rows.Scan(&r.ID, &r.Context, &vectorStr, &r.SyncStatus, &lastSyncAt); err != nil {
			telemetry.RecordRagSyncError(ctx)
			return nil, fmt.Errorf("failed to scan record: %w", err)
		}

		if lastSyncAt.Valid {
			r.LastSyncAt = lastSyncAt.Time
		}

		if vectorStr != nil && *vectorStr != "null" && *vectorStr != "" {
			// Parse vector string
			if err := json.Unmarshal([]byte(*vectorStr), &r.Vector); err != nil {
				telemetry.RecordRagSyncError(ctx)
				return nil, fmt.Errorf("failed to unmarshal vector: %w", err)
			}
		}

		records = append(records, r)
	}

	return records, nil
}

func (s *ragSyncService) MarkSynced(ctx context.Context, ids []string) error {
	if len(ids) == 0 {
		return nil
	}

	now := time.Now()
	// Using transaction since we might be updating multiple rows
	tx, err := s.dbProvider.Begin(ctx)
	if err != nil {
		telemetry.RecordRagSyncError(ctx)
		return fmt.Errorf("failed to begin transaction: %w", err)
	}
	defer tx.Rollback(ctx)

	query := `UPDATE autodream_memories SET sync_status = 'synced', last_sync_at = $1 WHERE id = $2`
	for _, id := range ids {
		if _, err := tx.Exec(ctx, query, now, id); err != nil {
			telemetry.RecordRagSyncError(ctx)
			return fmt.Errorf("failed to update record %s: %w", id, err)
		}
	}

	if err := tx.Commit(ctx); err != nil {
		telemetry.RecordRagSyncError(ctx)
		return fmt.Errorf("failed to commit transaction: %w", err)
	}

	telemetry.RecordRagRecordsSynced(ctx, int64(len(ids)))
	return nil
}

func (s *ragSyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	if len(records) == 0 {
		return nil
	}

	tx, err := s.dbProvider.Begin(ctx)
	if err != nil {
		telemetry.RecordRagSyncError(ctx)
		return fmt.Errorf("failed to begin transaction: %w", err)
	}
	defer tx.Rollback(ctx)

	query := `
		INSERT INTO autodream_memories (id, content, embedding, sync_status, last_sync_at)
		VALUES ($1, $2, $3, $4, $5)
		ON CONFLICT (id) DO UPDATE SET
			content = EXCLUDED.content,
			embedding = EXCLUDED.embedding,
			sync_status = EXCLUDED.sync_status,
			last_sync_at = EXCLUDED.last_sync_at
	`

	for _, r := range records {
		var vectorStr *string
		if len(r.Vector) > 0 {
			b, err := json.Marshal(r.Vector)
			if err != nil {
				telemetry.RecordRagSyncError(ctx)
				return fmt.Errorf("failed to marshal vector: %w", err)
			}
			s := string(b)
			vectorStr = &s
		}

		var lastSync sql.NullTime
		if !r.LastSyncAt.IsZero() {
			lastSync.Valid = true
			lastSync.Time = r.LastSyncAt
		}

		if _, err := tx.Exec(ctx, query, r.ID, r.Context, vectorStr, r.SyncStatus, lastSync); err != nil {
			telemetry.RecordRagSyncError(ctx)
			return fmt.Errorf("failed to upsert incoming sync record: %w", err)
		}
	}

	if err := tx.Commit(ctx); err != nil {
		telemetry.RecordRagSyncError(ctx)
		return fmt.Errorf("failed to commit transaction: %w", err)
	}

	return nil
}
