package hub

import (
	"context"
	"database/sql"
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
	ID           string
	Context      string
	Vector       []byte
	SyncStatus   SyncStatus
	LastSyncAt   time.Time
}

type RAGSyncService interface {
	FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error)
	MarkSynced(ctx context.Context, ids []string) error
	ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error
}

type dbRAGSyncService struct {
	provider db.Provider
}

// NewRAGSyncService creates a new RAGSyncService backed by the database.
func NewRAGSyncService(provider db.Provider) RAGSyncService {
	return &dbRAGSyncService{
		provider: provider,
	}
}

func (s *dbRAGSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	var query string
	if s.provider.IsSQLite() {
		query = "SELECT memory_id, context, vector_embedding, sync_status, last_sync_at FROM swarm_memory_embeddings WHERE sync_status = 'pending' LIMIT ?"
	} else {
		query = "SELECT memory_id, context, vector_embedding, sync_status, last_sync_at FROM swarm_memory_embeddings WHERE sync_status = 'pending' LIMIT $1 FOR UPDATE SKIP LOCKED"
	}

	rows, err := s.provider.Query(ctx, query, limit)
	if err != nil {
		return nil, fmt.Errorf("failed to fetch pending syncs: %w", err)
	}
	defer rows.Close()

	var records []RAGSyncRecord
	for rows.Next() {
		var r RAGSyncRecord
		var lastSyncAt sql.NullTime
		if err := rows.Scan(&r.ID, &r.Context, &r.Vector, &r.SyncStatus, &lastSyncAt); err != nil {
			return nil, fmt.Errorf("failed to scan record: %w", err)
		}
		if lastSyncAt.Valid {
			r.LastSyncAt = lastSyncAt.Time
		}
		records = append(records, r)
	}
	if err := rows.Err(); err != nil {
		return nil, fmt.Errorf("rows error: %w", err)
	}

	// For Postgres, we might want to update the status to "in_progress" to avoid other workers picking them up,
	// but the lock is released when the transaction ends (Query is not a tx).
	// To strictly follow "in_progress" transition, we should use a Tx.
	if !s.provider.IsSQLite() && len(records) > 0 {
		tx, err := s.provider.Begin(ctx)
		if err == nil {
			for _, r := range records {
				tx.Exec(ctx, "UPDATE swarm_memory_embeddings SET sync_status = 'in_progress' WHERE memory_id = $1", r.ID)
			}
			tx.Commit(ctx)
		}
	}

	return records, nil
}

func (s *dbRAGSyncService) MarkSynced(ctx context.Context, ids []string) error {
	if len(ids) == 0 {
		return nil
	}

	tx, err := s.provider.Begin(ctx)
	if err != nil {
		return fmt.Errorf("failed to begin transaction: %w", err)
	}
	defer tx.Rollback(ctx)

	for _, id := range ids {
		var query string
		if s.provider.IsSQLite() {
			query = "UPDATE swarm_memory_embeddings SET sync_status = 'synced', last_sync_at = CURRENT_TIMESTAMP WHERE memory_id = ?"
		} else {
			query = "UPDATE swarm_memory_embeddings SET sync_status = 'synced', last_sync_at = CURRENT_TIMESTAMP WHERE memory_id = $1"
		}

		_, err := tx.Exec(ctx, query, id)
		if err != nil {
			telemetry.RecordRagSyncError(ctx, err.Error())
			return fmt.Errorf("failed to mark synced for %s: %w", id, err)
		}
		telemetry.RecordRagRecordSynced(ctx, "system")
	}

	return tx.Commit(ctx)
}

func (s *dbRAGSyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	if len(records) == 0 {
		return nil
	}

	tx, err := s.provider.Begin(ctx)
	if err != nil {
		return fmt.Errorf("failed to begin transaction: %w", err)
	}
	defer tx.Rollback(ctx)

	for _, r := range records {
		var query string
		if s.provider.IsSQLite() {
			query = `
			INSERT INTO swarm_memory_embeddings (memory_id, context, vector_embedding, sync_status, last_sync_at)
			VALUES (?, ?, ?, 'synced', CURRENT_TIMESTAMP)
			ON CONFLICT(memory_id) DO UPDATE SET
				context = excluded.context,
				vector_embedding = excluded.vector_embedding,
				sync_status = 'synced',
				last_sync_at = CURRENT_TIMESTAMP
			`
			_, err = tx.Exec(ctx, query, r.ID, r.Context, r.Vector)
		} else {
			query = `
			INSERT INTO swarm_memory_embeddings (memory_id, context, vector_embedding, sync_status, last_sync_at)
			VALUES ($1, $2, $3, 'synced', CURRENT_TIMESTAMP)
			ON CONFLICT(memory_id) DO UPDATE SET
				context = excluded.context,
				vector_embedding = excluded.vector_embedding,
				sync_status = 'synced',
				last_sync_at = CURRENT_TIMESTAMP
			`
			_, err = tx.Exec(ctx, query, r.ID, r.Context, r.Vector)
		}

		if err != nil {
			telemetry.RecordRagSyncError(ctx, err.Error())
			return fmt.Errorf("failed to process incoming sync for %s: %w", r.ID, err)
		}
		telemetry.RecordRagRecordSynced(ctx, "system")
	}

	return tx.Commit(ctx)
}
