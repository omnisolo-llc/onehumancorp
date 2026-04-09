package hub

import (
	"context"
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
	Vector     []byte // Use []byte to represent vector embedding (BYTEA in pg, BLOB in sqlite)
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

type DefaultRAGSyncService struct {
	provider db.Provider
}

func NewDefaultRAGSyncService(provider db.Provider) *DefaultRAGSyncService {
	return &DefaultRAGSyncService{
		provider: provider,
	}
}

func (s *DefaultRAGSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	query := `
		SELECT memory_id, context, vector_embedding, sync_status, last_sync_at
		FROM swarm_memory_embeddings
		WHERE sync_status = 'pending'
		LIMIT $1
	`
	rows, err := s.provider.Query(ctx, query, limit)
	if err != nil {
		return nil, fmt.Errorf("failed to fetch pending syncs: %w", err)
	}
	defer rows.Close()

	var records []RAGSyncRecord
	for rows.Next() {
		var r RAGSyncRecord
		var status string
		var lastSyncAt *time.Time
		if err := rows.Scan(&r.ID, &r.Context, &r.Vector, &status, &lastSyncAt); err != nil {
			return nil, fmt.Errorf("failed to scan sync record: %w", err)
		}
		r.SyncStatus = SyncStatus(status)
		if lastSyncAt != nil {
			r.LastSyncAt = *lastSyncAt
		}
		records = append(records, r)
	}
	if err := rows.Err(); err != nil {
		return nil, fmt.Errorf("rows iteration error: %w", err)
	}

	return records, nil
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

	now := time.Now()
	syncedCount := 0

	for _, id := range ids {
		query := `
			UPDATE swarm_memory_embeddings
			SET sync_status = 'synced', last_sync_at = $1
			WHERE memory_id = $2 AND sync_status = 'pending'
		`
		affected, err := tx.Exec(ctx, query, now, id)
		if err != nil {
			telemetry.RecordRagSyncError(ctx)
			return fmt.Errorf("failed to update record %s: %w", id, err)
		}
		if affected > 0 {
			syncedCount++
		}
	}

	if err := tx.Commit(ctx); err != nil {
		return fmt.Errorf("failed to commit transaction: %w", err)
	}

	telemetry.RecordRagRecordSynced(ctx, syncedCount)
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

	for _, r := range records {
		// UPSERT approach: Update first, if 0 rows affected, Insert.
		updateQuery := `
			UPDATE swarm_memory_embeddings
			SET context = $1, vector_embedding = $2, sync_status = 'synced', last_sync_at = $3
			WHERE memory_id = $4
		`
		affected, err := tx.Exec(ctx, updateQuery, r.Context, r.Vector, r.LastSyncAt, r.ID)
		if err != nil {
			telemetry.RecordRagSyncError(ctx)
			return fmt.Errorf("failed to update incoming record %s: %w", r.ID, err)
		}

		if affected == 0 {
			insertQuery := `
				INSERT INTO swarm_memory_embeddings (memory_id, context, vector_embedding, sync_status, last_sync_at)
				VALUES ($1, $2, $3, 'synced', $4)
			`
			_, err := tx.Exec(ctx, insertQuery, r.ID, r.Context, r.Vector, r.LastSyncAt)
			if err != nil {
				telemetry.RecordRagSyncError(ctx)
				return fmt.Errorf("failed to insert incoming record %s: %w", r.ID, err)
			}
		}
	}

	if err := tx.Commit(ctx); err != nil {
		return fmt.Errorf("failed to commit transaction: %w", err)
	}

	return nil
}
